extern crate regex;

use std::collections::HashMap;

use crate::{JSONGetTextBuildError, JSONGetTextValue};

use super::{Context, JSONGetTextBuilder, Key};

use regex::Regex;

/// A wrapper for context and a default key. **Keys** are usually considered as locales.
#[derive(Debug)]
pub struct JSONGetText<'a> {
    default_key: Key,
    context: Context<'a>,
}

impl<'a> JSONGetText<'a> {
    /// Create a new `JSONGetTextBuilder` instance. You need to decide your default key at the stage.
    #[inline]
    pub fn build(default_key: Key) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder::new(default_key)
    }

    /// Create a new JSONGetText instance with context and a default key.
    pub(crate) fn from_context_with_default_key(
        default_key: Key,
        mut context: Context<'a>,
    ) -> Result<JSONGetText<'a>, JSONGetTextBuildError> {
        if !context.contains_key(&default_key) {
            return Err(JSONGetTextBuildError::DefaultKeyNotFound);
        }

        let default_map = context.remove(&default_key).unwrap();

        let mut inner_context = HashMap::new();

        {
            for (key, mut map) in context {
                {
                    for map_key in map.keys() {
                        if !default_map.contains_key(map_key) {
                            return Err(JSONGetTextBuildError::TextInKeyNotInDefaultKey {
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

            inner_context.insert(default_key, default_map);
        }

        Ok(JSONGetText {
            default_key,
            context: inner_context,
        })
    }

    /// Get all keys in context.
    pub fn get_keys(&self) -> Vec<Key> {
        self.context.keys().copied().collect()
    }

    /// Returns `true` if the context contains a value for the specified key.
    #[inline]
    pub fn contains_key(&self, key: Key) -> bool {
        self.context.contains_key(&key)
    }

    /// Get the default key.
    #[inline]
    pub fn get_default_key(&self) -> Key {
        self.default_key
    }

    /// Get a string map from context by a key.
    #[inline]
    pub fn get(&self, key: Key) -> &HashMap<String, JSONGetTextValue<'a>> {
        match self.context.get(&key) {
            Some(m) => m,
            None => self.context.get(&self.default_key).unwrap(),
        }
    }

    /// Get text from context.
    #[inline]
    pub fn get_text<T: AsRef<str>>(&'a self, text: T) -> Option<JSONGetTextValue<'a>> {
        let map = self.context.get(&self.default_key).unwrap();

        map.get(text.as_ref()).map(|v| v.clone_borrowed())
    }

    /// Get text from context with a specific key.
    #[inline]
    pub fn get_text_with_key<T: AsRef<str>>(
        &'a self,
        key: Key,
        text: T,
    ) -> Option<JSONGetTextValue<'a>> {
        let map =
            self.context.get(&key).unwrap_or_else(|| self.context.get(&self.default_key).unwrap());

        map.get(text.as_ref()).map(|v| v.clone_borrowed())
    }

    /// Get multiple text from context. The output map is usually used for serialization.
    pub fn get_multiple_text<'b, T: AsRef<str> + ?Sized>(
        &self,
        text_array: &[&'b T],
    ) -> Option<HashMap<&'b str, JSONGetTextValue>> {
        let map = self.context.get(&self.default_key).unwrap();

        let mut new_map = HashMap::new();

        for &text in text_array.iter() {
            let text = text.as_ref();
            let value = map.get(text)?;
            new_map.insert(text, value.clone_borrowed());
        }

        Some(new_map)
    }

    /// Get multiple text from context with a specific key. The output map is usually used for serialization.
    pub fn get_multiple_text_with_key<'b, T: AsRef<str> + ?Sized>(
        &'a self,
        key: Key,
        text_array: &[&'b T],
    ) -> Option<HashMap<&'b str, JSONGetTextValue<'a>>> {
        let map =
            self.context.get(&key).unwrap_or_else(|| self.context.get(&self.default_key).unwrap());

        let mut new_map = HashMap::new();

        for &text in text_array.iter() {
            let text = text.as_ref();
            let value = map.get(text)?;
            new_map.insert(text, value.clone_borrowed());
        }

        Some(new_map)
    }

    /// Get filtered text from context by a Regex instance. The output map is usually used for serialization.
    pub fn get_filtered_text(
        &'a self,
        regex: &Regex,
    ) -> Option<HashMap<&'a str, JSONGetTextValue<'a>>> {
        let map = self.context.get(&self.default_key).unwrap();

        let mut new_map = HashMap::new();

        for (key, value) in map.iter() {
            if !regex.is_match(key) {
                continue;
            }
            new_map.insert(key.as_str(), value.clone_borrowed());
        }

        Some(new_map)
    }

    /// Get filtered text from context with a specific key by a Regex instance. The output map is usually used for serialization.
    pub fn get_filtered_text_with_key(
        &'a self,
        key: Key,
        regex: &Regex,
    ) -> Option<HashMap<&'a str, JSONGetTextValue<'a>>> {
        let map =
            self.context.get(&key).unwrap_or_else(|| self.context.get(&self.default_key).unwrap());

        let mut new_map = HashMap::new();

        for (key, value) in map.iter() {
            if !regex.is_match(key) {
                continue;
            }
            new_map.insert(key.as_str(), value.clone_borrowed());
        }

        Some(new_map)
    }
}
