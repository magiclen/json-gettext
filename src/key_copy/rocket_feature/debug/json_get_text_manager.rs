extern crate rocket;

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
    Key,
};

#[derive(Debug)]
pub struct JSONGetTextManager {
    empty:        JSONGetText<'static>,
    json_gettext: DebuggableMutate<JSONGetText<'static>>,
    files:        DebuggableMutate<HashMap<Key, (PathBuf, Option<SystemTime>)>>,
    reloading:    AtomicBool,
}

impl JSONGetTextManager {
    pub fn from_files(
        default_key: Key,
        source: Vec<(Key, &'static str)>,
    ) -> Result<JSONGetTextManager, JSONGetTextBuildError> {
        let mut builder = JSONGetTextBuilder::new(default_key);

        let mut files = HashMap::with_capacity(source.len());

        for (key, json_file) in source {
            let metadata = Path::new(json_file).metadata()?;

            let mtime = metadata.modified().ok();

            builder.add_json_file(key, json_file)?;

            let json_file_path: PathBuf = json_file.into();

            files.insert(key, (json_file_path, mtime));
        }

        let mut empty_builder = JSONGetTextBuilder::new(default_key);

        empty_builder.add_map(default_key, HashMap::new())?;

        Ok(JSONGetTextManager {
            empty:        empty_builder.build()?,
            json_gettext: DebuggableMutate::new(builder.build()?),
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

            for (_, (path, mtime)) in files.iter_mut() {
                let metadata = path.metadata().map_err(|err| {
                    self.reloading.store(false, Ordering::Relaxed);

                    err
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

                for (&key, (path, _)) in files {
                    builder.add_json_file(key, path).map_err(|err| {
                        self.reloading.store(false, Ordering::Relaxed);

                        err
                    })?;
                }

                let json_gettext = builder.build().map_err(|err| {
                    self.reloading.store(false, Ordering::Relaxed);

                    err
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
        F: Fn() -> (Key, Vec<(Key, &'static str)>) + Send + Sync + 'static, {
        JSONGetTextFairing {
            custom_callback: Box::new(f)
        }
    }
}

impl<'a> Deref for JSONGetTextManager {
    type Target = JSONGetText<'static>;

    #[inline]
    fn deref(&self) -> &JSONGetText<'static> {
        // NOTICE: Not always safe!
        if self.reloading.load(Ordering::Relaxed) {
            &self.empty
        } else {
            self.json_gettext.get()
        }
    }
}
