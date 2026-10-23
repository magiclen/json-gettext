use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    ops::Deref,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, TryLockError,
        atomic::{AtomicPtr, Ordering},
    },
    time::SystemTime,
};

use rocket::fairing::Fairing;

use crate::{JSONGetText, JSONGetTextBuildError, JSONGetTextBuilder, JSONGetTextFairing};

/// Holds a `JSONGetText` and reloads it from the JSON files when they change.
/// Keeps each published snapshot alive until the manager is dropped, so borrowed texts stay valid after a reload.
pub struct JSONGetTextManager {
    json_gettext: AtomicPtr<JSONGetText<'static>>,
    reload_state: Mutex<ReloadState>,
}

struct ReloadState {
    snapshots: Vec<Arc<JSONGetText<'static>>>,
    files:     HashMap<crate::Key, SourceFile>,
}

struct SourceFile {
    path:          PathBuf,
    modified:      Option<SystemTime>,
    next_modified: Option<SystemTime>,
}

impl JSONGetTextManager {
    /// Load the JSON files and remember their modification times.
    pub fn from_files(
        default_key: crate::Key,
        source: Vec<(crate::Key, &'static str)>,
    ) -> Result<JSONGetTextManager, JSONGetTextBuildError> {
        let mut files = HashMap::with_capacity(source.len());

        for (key, json_file) in source {
            if files.contains_key(&key) {
                return Err(JSONGetTextBuildError::DuplicatedKey(key));
            }

            let metadata = Path::new(json_file).metadata()?;

            let modified = metadata.modified().ok();

            files.insert(key, SourceFile {
                path: PathBuf::from(json_file),
                modified,
                next_modified: modified,
            });
        }

        let mut builder = JSONGetTextBuilder::new(default_key);

        for (key, file) in &files {
            builder.add_json_file(key, &file.path)?;
        }

        let json_gettext = Arc::new(builder.build()?);
        let pointer = Arc::as_ptr(&json_gettext).cast_mut();

        Ok(JSONGetTextManager {
            json_gettext: AtomicPtr::new(pointer),
            reload_state: Mutex::new(ReloadState {
                snapshots: vec![json_gettext],
                files,
            }),
        })
    }

    /// Reload all JSON files if any modification time has changed.
    /// If another reload is running, this returns `Ok(())` without waiting.
    /// On error, the previous texts are kept and the next call tries again.
    pub fn reload_if_needed(&self) -> Result<(), JSONGetTextBuildError> {
        let mut state = match self.reload_state.try_lock() {
            Ok(state) => state,
            Err(TryLockError::WouldBlock) => return Ok(()),
            Err(TryLockError::Poisoned(error)) => error.into_inner(),
        };
        let mut do_reload = false;

        for file in state.files.values_mut() {
            file.next_modified = file.path.metadata()?.modified().ok();
            do_reload |= file.next_modified.is_none() || file.next_modified != file.modified;
        }

        if !do_reload {
            return Ok(());
        }

        let mut builder = JSONGetTextBuilder::new(self.get_default_key());

        for (key, file) in &state.files {
            builder.add_json_file(key, &file.path)?;
        }

        let json_gettext = builder.build()?;

        // Snapshots are never freed, so skip publishing when only modification times changed (e.g. `touch`).
        if !self.has_same_texts(&json_gettext) {
            let snapshot = Arc::new(json_gettext);
            let pointer = Arc::as_ptr(&snapshot).cast_mut();

            // Keep ownership before publishing; old snapshots must never be removed or changed.
            state.snapshots.push(snapshot);
            self.json_gettext.store(pointer, Ordering::Release);
        }

        for file in state.files.values_mut() {
            file.modified = file.next_modified;
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
        let pointer = self.json_gettext.load(Ordering::Acquire);

        // SAFETY: Acquire sees a complete, immutable snapshot that reload_state keeps alive until this manager is dropped.
        unsafe { &*pointer }
    }
}

impl Debug for JSONGetTextManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("JSONGetTextManager")
            .field("json_gettext", self.deref())
            .finish_non_exhaustive()
    }
}
