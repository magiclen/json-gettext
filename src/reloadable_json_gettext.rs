use std::collections::HashMap;
use std::sync::Mutex;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::{Context, JSONGetTextValue, JSONGetTextBuilder, JSONGetTextError};
use crate::regex::Regex;

/// A reloadable wrapper for context and a default key. **Keys** are usually considered as locales.
#[derive(Debug)]
pub struct ReloadableJSONGetText<'a> {
    default_key: String,
    context: Mutex<(HashMap<String, (PathBuf, Option<SystemTime>)>, Context<'a>)>,
}

impl<'a> ReloadableJSONGetText<'a> {
    /// Create a new `JSONGetTextBuilder` instance. You need to decide your default key at the stage.
    #[inline]
    pub fn build<S: Into<String>>(default_key: S) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder::new(default_key)
    }

    pub(crate) fn from_context_with_default_key<S: AsRef<str> + Into<String>>(default_key: S, mut context: Context<'a>, file_table: HashMap<String, (PathBuf, Option<SystemTime>)>) -> Result<ReloadableJSONGetText<'a>, JSONGetTextError> {
        if !context.contains_key(default_key.as_ref()) {
            return Err(JSONGetTextError::DefaultKeyNotFound);
        }

        let default_key = default_key.into();

        let default_map = context.remove(&default_key).unwrap();

        let mut inner_context = HashMap::new();

        {
            for (key, mut map) in context {
                {
                    for map_key in map.keys() {
                        if !default_map.contains_key(map_key) {
                            return Err(JSONGetTextError::TextInKeyNotInDefaultKey {
                                key,
                                text: map_key.clone(),
                            });
                        }
                    }
                }

                {
                    for map_key in default_map.keys() {
                        if !map.contains_key(map_key) {
                            map.insert(map_key.clone(), default_map.get(map_key).unwrap().clone());
                        }
                    }
                }

                inner_context.insert(key, map);
            }

            inner_context.insert(default_key.clone(), default_map);
        }

        Ok(ReloadableJSONGetText {
            default_key,
            context: Mutex::new(inner_context),
            file_table: Mutex::new(file_table),
        })
    }
}