use std::{
    collections::HashMap,
    mem,
    ops::Deref,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
    time::SystemTime,
};

use rocket::fairing::Fairing;

use crate::{
    DebuggableMutate, JSONGetText, JSONGetTextBuildError, JSONGetTextBuilder, JSONGetTextFairing,
};

#[derive(Debug)]
pub struct JSONGetTextManager {
    empty:        JSONGetText<'static>,
    json_gettext: DebuggableMutate<JSONGetText<'static>>,
    files:        DebuggableMutate<HashMap<crate::Key, (PathBuf, Option<SystemTime>)>>,
    reloading:    AtomicBool,
}

impl JSONGetTextManager {
    pub fn from_files(
        default_key: crate::Key,
        source: Vec<(crate::Key, &'static str)>,
    ) -> Result<JSONGetTextManager, JSONGetTextBuildError> {
        let mut files = HashMap::with_capacity(source.len());

        for (key, json_file) in source {
            let metadata = Path::new(json_file).metadata()?;

            let mtime = metadata.modified().ok();

            files.insert(key, (PathBuf::from(json_file), mtime));
        }

        let mut builder = JSONGetTextBuilder::new(default_key);

        for (key, (path, _)) in &files {
            builder.add_json_file(key, path)?;
        }

        let json_gettext = builder.build()?;

        // Reuse the built default key so the empty context does not need another copy of it.
        let mut empty_builder = JSONGetTextBuilder::new(json_gettext.get_default_key());

        empty_builder.add_map(json_gettext.get_default_key(), HashMap::new())?;

        Ok(JSONGetTextManager {
            empty:        empty_builder.build()?,
            json_gettext: DebuggableMutate::new(json_gettext),
            files:        DebuggableMutate::new(files),
            reloading:    AtomicBool::new(false),
        })
    }

    pub fn reload_if_needed(&self) -> Result<(), JSONGetTextBuildError> {
        if self
            .reloading
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            let mut do_reload = false;

            let files = self.files.get_mut();

            for (path, mtime) in files.values_mut() {
                let metadata = path.metadata().inspect_err(|_| {
                    self.reloading.store(false, Ordering::Relaxed);
                })?;

                let (reload, new_mtime) = match mtime {
                    Some(mtime) => match metadata.modified() {
                        Ok(new_mtime) => (new_mtime > *mtime, Some(new_mtime)),
                        Err(_) => (true, None),
                    },
                    None => match metadata.modified() {
                        Ok(new_mtime) => (true, Some(new_mtime)),
                        Err(_) => (true, None),
                    },
                };

                if reload {
                    *mtime = new_mtime;

                    do_reload = true;
                }
            }

            if do_reload {
                let mut builder = JSONGetTextBuilder::new(self.get_default_key());

                for (key, (path, _)) in files.iter() {
                    builder.add_json_file(key, path).inspect_err(|_| {
                        self.reloading.store(false, Ordering::Relaxed);
                    })?;
                }

                let json_gettext = builder.build().inspect_err(|_| {
                    self.reloading.store(false, Ordering::Relaxed);
                })?;

                drop(mem::replace(self.json_gettext.get_mut(), json_gettext));
            }

            self.reloading.store(false, Ordering::Relaxed);
        }

        Ok(())
    }
}

impl JSONGetTextManager {
    /// Create the fairing of `JSONGetTextManager`.
    pub fn fairing<F>(f: F) -> impl Fairing
    where
        F: Fn() -> (crate::Key, Vec<(crate::Key, &'static str)>) + Send + Sync + 'static, {
        JSONGetTextFairing {
            custom_callback: Box::new(f)
        }
    }
}

impl Deref for JSONGetTextManager {
    type Target = JSONGetText<'static>;

    #[inline]
    fn deref(&self) -> &JSONGetText<'static> {
        // NOTICE: This is only sound enough for debug-time hot-reloading, not a production concurrency guarantee. A reader here can still race with the reloader; the empty context is returned while reloading to reduce the window.
        if self.reloading.load(Ordering::Relaxed) { &self.empty } else { self.json_gettext.get() }
    }
}
