#![cfg(feature = "rocket")]

use json_gettext::{
    JSONGetTextBuildError, JSONGetTextManager, JSONGetTextValue,
    static_json_gettext_build_for_rocket,
};
use rocket::{
    State,
    http::{ContentType, Status},
    local::blocking::Client,
};

mod common;

#[rocket::get("/<name>")]
fn text<'a>(ctx: &'a State<JSONGetTextManager>, name: &str) -> Option<JSONGetTextValue<'a>> {
    ctx.get_text(name)
}

#[test]
fn serve_json_value() {
    let client = Client::tracked(
        rocket::build()
            .attach(static_json_gettext_build_for_rocket!(
                common::keys().0;
                common::keys().0 => "tests/data/values.json",
            ))
            .mount("/", rocket::routes![text]),
    )
    .unwrap();
    let response = client.get("/boolean").dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(Some(ContentType::JSON), response.content_type());
    assert_eq!("true", response.into_string().unwrap());
}

#[test]
fn duplicated_key() {
    #[cfg(debug_assertions)]
    let result = JSONGetTextManager::from_files(common::keys().0, vec![
        (common::keys().0, concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/values.json")),
        (common::keys().0, concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/values.json")),
    ]);
    #[cfg(not(debug_assertions))]
    let result = JSONGetTextManager::from_jsons(common::keys().0, vec![
        (common::keys().0, include_str!("data/values.json")),
        (common::keys().0, include_str!("data/values.json")),
    ]);

    assert!(
        matches!(result, Err(JSONGetTextBuildError::DuplicatedKey(key)) if key == common::keys().0)
    );
}

#[test]
fn failed_ignition() {
    let result = Client::tracked(
        rocket::build().attach(JSONGetTextManager::fairing(|| (common::keys().0, Vec::new()))),
    );

    let Err(error) = result else {
        panic!("Ignition should fail.");
    };

    assert!(matches!(error.kind(), rocket::error::ErrorKind::FailedFairings(_)));
}

#[cfg(debug_assertions)]
mod reloadable {
    use std::{
        fs::{self, File},
        ptr,
        sync::mpsc,
        thread,
        time::{Duration, SystemTime},
    };

    use super::*;

    const MODIFIED: Duration = Duration::from_secs(1_000_000_000);

    struct SourceFile {
        path: &'static str,
    }

    impl SourceFile {
        fn new(path: &'static str, json: &str) -> Self {
            let source = SourceFile {
                path,
            };

            source.write(json, SystemTime::UNIX_EPOCH + MODIFIED);

            source
        }

        fn write(&self, json: &str, modified: SystemTime) {
            fs::write(self.path, json).unwrap();
            File::options().write(true).open(self.path).unwrap().set_modified(modified).unwrap();
        }
    }

    impl Drop for SourceFile {
        fn drop(&mut self) {
            fs::remove_file(self.path).unwrap();
        }
    }

    #[test]
    fn borrowed_values_survive_reload() {
        let source = SourceFile::new(
            concat!(env!("CARGO_TARGET_TMPDIR"), "/borrowed_values_survive_reload.json"),
            r#"{"plain":"old"}"#,
        );
        let manager =
            JSONGetTextManager::from_files(common::keys().0, vec![(common::keys().0, source.path)])
                .unwrap();
        let manager_ref = &manager;
        let (ready_tx, ready_rx) = mpsc::channel();
        let (reloaded_tx, reloaded_rx) = mpsc::channel();

        thread::scope(|scope| {
            let reader = scope.spawn(move || {
                let borrowed = manager_ref.get_text("plain").unwrap();
                let map = manager_ref.get(manager_ref.get_default_key());

                ready_tx.send(()).unwrap();
                reloaded_rx.recv().unwrap();

                assert_eq!("old", borrowed);
                assert_eq!("old", map.get("plain").unwrap());
            });

            ready_rx.recv().unwrap();
            source.write(
                r#"{"plain":"new"}"#,
                SystemTime::UNIX_EPOCH + MODIFIED - Duration::from_secs(1),
            );
            manager.reload_if_needed().unwrap();

            assert_eq!("new", manager.get_text("plain").unwrap());

            reloaded_tx.send(()).unwrap();
            reader.join().unwrap();
        });
    }

    #[test]
    fn unchanged_content_keeps_snapshot() {
        let source = SourceFile::new(
            concat!(env!("CARGO_TARGET_TMPDIR"), "/unchanged_content_keeps_snapshot.json"),
            r#"{"plain":"old"}"#,
        );
        let manager =
            JSONGetTextManager::from_files(common::keys().0, vec![(common::keys().0, source.path)])
                .unwrap();
        let snapshot = &*manager;

        // Only the modification time changes, like `touch`.
        source.write(
            r#"{"plain":"old"}"#,
            SystemTime::UNIX_EPOCH + MODIFIED + Duration::from_secs(1),
        );
        manager.reload_if_needed().unwrap();

        assert!(ptr::eq(snapshot, &*manager));
    }

    #[test]
    fn reload_error_keeps_serving_previous_text() {
        let source = SourceFile::new(
            concat!(env!("CARGO_TARGET_TMPDIR"), "/reload_error_keeps_serving_previous_text.json"),
            r#"{"plain":"old"}"#,
        );
        let path = source.path;
        let client = Client::tracked(
            rocket::build()
                .attach(JSONGetTextManager::fairing(move || {
                    (common::keys().0, vec![(common::keys().0, path)])
                }))
                .mount("/", rocket::routes![text]),
        )
        .unwrap();

        assert_eq!(r#""old""#, client.get("/plain").dispatch().into_string().unwrap());

        let modified = SystemTime::UNIX_EPOCH + MODIFIED + Duration::from_secs(1);

        source.write("{", modified);

        assert_eq!(r#""old""#, client.get("/plain").dispatch().into_string().unwrap());

        source.write(r#"{"plain":"new"}"#, modified);

        assert_eq!(r#""new""#, client.get("/plain").dispatch().into_string().unwrap());
    }
}
