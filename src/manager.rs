#[cfg(debug_assertions)]
use std::collections::HashMap;
#[cfg(debug_assertions)]
use std::path::{PathBuf, Path};
#[cfg(debug_assertions)]
use std::time::SystemTime;
use std::ops::Deref;
#[cfg(debug_assertions)]
use std::sync::atomic::{Ordering, AtomicBool};
#[cfg(debug_assertions)]
use std::mem;

use crate::rocket::fairing::Fairing;
use crate::{JSONGetTextBuilder, JSONGetText, JSONGetTextBuildError, JSONGetTextFairing};
#[cfg(debug_assertions)]
use crate::DebuggableMutate;

/// To monitor the state of `JSONGetText`.
#[cfg(debug_assertions)]
#[derive(Debug)]
pub struct JSONGetTextManager {
    empty: JSONGetText<'static>,
    json_gettext: DebuggableMutate<JSONGetText<'static>>,
    files: DebuggableMutate<HashMap<&'static str, (PathBuf, Option<SystemTime>)>>,
    reloading: AtomicBool,
}

/// To monitor the state of `JSONGetText`.
#[cfg(not(debug_assertions))]
#[derive(Debug)]
pub struct JSONGetTextManager {
    json_gettext: JSONGetText<'static>
}

impl JSONGetTextManager {
    #[cfg(debug_assertions)]
    pub fn from_files(default_key: &'static str, source: Vec<(&'static str, &'static str)>) -> Result<JSONGetTextManager, JSONGetTextBuildError> {
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
            empty: empty_builder.build()?,
            json_gettext: DebuggableMutate::new(builder.build()?),
            files: DebuggableMutate::new(files),
            reloading: AtomicBool::new(false),
        })
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    pub fn from_jsons(default_key: &'static str, source: Vec<(&'static str, &'static str)>) -> Result<JSONGetTextManager, JSONGetTextBuildError> {
        let mut builder = JSONGetTextBuilder::new(default_key);

        for (key, json) in source {
            builder.add_json(key, json)?;
        }

        Ok(JSONGetTextManager {
            json_gettext: builder.build()?,
        })
    }
}

#[cfg(debug_assertions)]
impl JSONGetTextManager {
    pub fn reload_if_needed(&self) -> Result<(), JSONGetTextBuildError> {
        if !self.reloading.compare_and_swap(false, true, Ordering::Relaxed) {
            let mut do_reload = false;

            let files = self.files.get_mut();

            for (_, (path, mtime)) in files.iter_mut() {
                let metadata = path.metadata()?;

                let (reload, new_mtime) = match mtime {
                    Some(mtime) => {
                        match metadata.modified() {
                            Ok(new_mtime) => {
                                (new_mtime > *mtime, Some(new_mtime))
                            }
                            Err(_) => {
                                (true, None)
                            }
                        }
                    }
                    None => {
                        match metadata.modified() {
                            Ok(new_mtime) => {
                                (true, Some(new_mtime))
                            }
                            Err(_) => {
                                (true, None)
                            }
                        }
                    }
                };

                if reload {
                    *mtime = new_mtime;

                    do_reload = true;
                }
            }

            if do_reload {
                let mut builder = JSONGetTextBuilder::new(self.get_default_key());

                for (&key, (path, _)) in files {
                    builder.add_json_file(key, path)?;
                }

                let json_gettext = builder.build()?;

                mem::replace(self.json_gettext.get_mut(), json_gettext);
            }

            self.reloading.store(false, Ordering::Relaxed);
        }

        Ok(())
    }
}

impl JSONGetTextManager {
    /// Create the fairing of `JSONGetTextManager`.
    pub fn fairing<F>(f: F) -> impl Fairing where F: Fn() -> (&'static str, Vec<(&'static str, &'static str)>) + Send + Sync + 'static {
        JSONGetTextFairing {
            custom_callback: Box::new(f),
        }
    }
}


#[cfg(debug_assertions)]
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

#[cfg(not(debug_assertions))]
impl<'a> Deref for JSONGetTextManager {
    type Target = JSONGetText<'static>;

    #[inline]
    fn deref(&self) -> &JSONGetText<'static> {
        &self.json_gettext
    }
}
