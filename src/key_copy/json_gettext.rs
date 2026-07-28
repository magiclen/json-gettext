use std::collections::{BTreeMap, HashMap};

#[cfg(feature = "regex")]
use regex::Regex;

use super::{Context, JSONGetTextBuilder, Key};
use crate::{JSONGetTextBuildError, JSONGetTextValue, text_lookup};

/// A wrapper for context and a default key. **Keys** are usually considered as locales.
#[derive(Debug)]
pub struct JSONGetText<'a> {
    default_key: Key,
    context:     Context<'a>,
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
        context: Context<'a>,
    ) -> Result<JSONGetText<'a>, JSONGetTextBuildError> {
        let context = text_lookup::finalize_context(default_key, context)?;

        Ok(JSONGetText {
            default_key,
            context,
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
    pub fn get_text<T: AsRef<str>>(&self, text: T) -> Option<JSONGetTextValue<'_>> {
        let map = self.context.get(&self.default_key).unwrap();

        text_lookup::get_text(map, text.as_ref())
    }

    /// Get text from context with a specific key.
    #[inline]
    pub fn get_text_with_key<T: AsRef<str>>(
        &self,
        key: Key,
        text: T,
    ) -> Option<JSONGetTextValue<'_>> {
        let map =
            self.context.get(&key).unwrap_or_else(|| self.context.get(&self.default_key).unwrap());

        text_lookup::get_text(map, text.as_ref())
    }

    /// Get multiple text from context. The output map is usually used for serialization.
    pub fn get_multiple_text<'b, T: AsRef<str> + ?Sized>(
        &self,
        text_array: &[&'b T],
    ) -> Option<BTreeMap<&'b str, JSONGetTextValue<'_>>> {
        let map = self.context.get(&self.default_key).unwrap();

        text_lookup::get_multiple_text(map, text_array)
    }

    /// Get multiple text from context with a specific key. The output map is usually used for serialization.
    pub fn get_multiple_text_with_key<'b, T: AsRef<str> + ?Sized>(
        &self,
        key: Key,
        text_array: &[&'b T],
    ) -> Option<BTreeMap<&'b str, JSONGetTextValue<'_>>> {
        let map =
            self.context.get(&key).unwrap_or_else(|| self.context.get(&self.default_key).unwrap());

        text_lookup::get_multiple_text(map, text_array)
    }

    /// Get filtered text from context by a Regex instance. The output map is usually used for serialization.
    #[cfg(feature = "regex")]
    pub fn get_filtered_text(&self, regex: &Regex) -> Option<BTreeMap<&str, JSONGetTextValue<'_>>> {
        let map = self.context.get(&self.default_key).unwrap();

        Some(text_lookup::get_filtered_text(map, regex))
    }

    /// Get filtered text from context with a specific key by a Regex instance. The output map is usually used for serialization.
    #[cfg(feature = "regex")]
    pub fn get_filtered_text_with_key(
        &self,
        key: Key,
        regex: &Regex,
    ) -> Option<BTreeMap<&str, JSONGetTextValue<'_>>> {
        let map =
            self.context.get(&key).unwrap_or_else(|| self.context.get(&self.default_key).unwrap());

        Some(text_lookup::get_filtered_text(map, regex))
    }
}
