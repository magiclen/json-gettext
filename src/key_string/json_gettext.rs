use std::collections::{BTreeMap, HashMap};

#[cfg(feature = "regex")]
use regex::Regex;

use super::{Context, JSONGetTextBuilder, Key};
use crate::{JSONGetTextBuildError, JSONGetTextValue, text_lookup};

/// A wrapper for context and a default key. **Keys** are usually considered as locales.
#[derive(Debug)]
pub struct JSONGetText<'a> {
    default_key: String,
    context:     Context<'a>,
}

impl<'a> JSONGetText<'a> {
    /// Create a new `JSONGetTextBuilder` instance. You need to decide your default key at the stage.
    #[inline]
    pub fn build<S: Into<String>>(default_key: S) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder::new(default_key)
    }

    /// Create a new JSONGetText instance with context and a default key.
    pub(crate) fn from_context_with_default_key<S: Into<String>>(
        default_key: S,
        context: Context<'a>,
    ) -> Result<JSONGetText<'a>, JSONGetTextBuildError> {
        let default_key: String = default_key.into();

        let context = text_lookup::finalize_context(Key::from(default_key.clone()), context)?;

        Ok(JSONGetText {
            default_key,
            context,
        })
    }

    /// Get all keys in context.
    pub fn get_keys(&self) -> Vec<&str> {
        self.context.keys().map(|key| key.as_str()).collect()
    }

    /// Returns `true` if the context contains a value for the specified key.
    #[inline]
    pub fn contains_key<K: AsRef<str>>(&self, key: K) -> bool {
        self.context.contains_key(key.as_ref())
    }

    /// Get the default key.
    #[inline]
    pub fn get_default_key(&self) -> &str {
        &self.default_key
    }

    /// Get a string map from context by a key.
    #[inline]
    pub fn get<K: AsRef<str>>(&self, key: K) -> &HashMap<String, JSONGetTextValue<'a>> {
        match self.context.get(key.as_ref()) {
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
    pub fn get_text_with_key<K: AsRef<str>, T: AsRef<str>>(
        &self,
        key: K,
        text: T,
    ) -> Option<JSONGetTextValue<'_>> {
        let map = self
            .context
            .get(key.as_ref())
            .unwrap_or_else(|| self.context.get(&self.default_key).unwrap());

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
    pub fn get_multiple_text_with_key<'b, K: AsRef<str>, T: AsRef<str> + ?Sized>(
        &self,
        key: K,
        text_array: &[&'b T],
    ) -> Option<BTreeMap<&'b str, JSONGetTextValue<'_>>> {
        let map = self
            .context
            .get(key.as_ref())
            .unwrap_or_else(|| self.context.get(&self.default_key).unwrap());

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
    pub fn get_filtered_text_with_key<K: AsRef<str>>(
        &self,
        key: K,
        regex: &Regex,
    ) -> Option<BTreeMap<&str, JSONGetTextValue<'_>>> {
        let map = self
            .context
            .get(key.as_ref())
            .unwrap_or_else(|| self.context.get(&self.default_key).unwrap());

        Some(text_lookup::get_filtered_text(map, regex))
    }
}
