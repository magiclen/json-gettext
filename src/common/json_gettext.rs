use std::collections::{BTreeMap, HashMap};

#[cfg(feature = "regex")]
use regex::Regex;

use super::{Context, IntoKey, JSONGetTextBuilder, LookupKey};
use crate::{JSONGetTextBuildError, JSONGetTextValue, text_lookup};

/// A wrapper for context and a default key. **Keys** are usually considered as locales.
#[derive(Debug)]
pub struct JSONGetText<'a> {
    default_key: crate::Key,
    context:     Context<'a>,
}

impl<'a> JSONGetText<'a> {
    /// Create a new `JSONGetTextBuilder` instance. You need to decide your default key at this stage.
    #[inline]
    pub fn build<K: IntoKey>(default_key: K) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder::new(default_key)
    }

    /// Create a new JSONGetText instance with context and a default key.
    pub(crate) fn from_context_with_default_key(
        default_key: crate::Key,
        context: Context<'a>,
        allow_extra_texts: bool,
    ) -> Result<JSONGetText<'a>, JSONGetTextBuildError> {
        let context = text_lookup::finalize_context(&default_key, context, allow_extra_texts)?;

        Ok(JSONGetText {
            default_key,
            context,
        })
    }

    /// Get the default key.
    #[cfg(not(feature = "langid"))]
    #[inline]
    pub fn get_default_key(&self) -> &str {
        self.default_key.as_str()
    }

    /// Get the default key.
    #[cfg(feature = "langid")]
    #[inline]
    pub fn get_default_key(&self) -> crate::Key {
        self.default_key
    }

    /// Get all keys in context.
    #[cfg(not(feature = "langid"))]
    pub fn get_keys(&self) -> Vec<&str> {
        self.context.keys().map(|key| key.as_str()).collect()
    }

    /// Get all keys in context.
    #[cfg(feature = "langid")]
    pub fn get_keys(&self) -> Vec<crate::Key> {
        self.context.keys().copied().collect()
    }

    /// Returns `true` if the context contains a value for the specified key.
    #[inline]
    pub fn contains_key<K: LookupKey>(&self, key: K) -> bool {
        key.lookup(&self.context).is_some()
    }

    /// Get the texts owned by a key. Missing texts are **not** filled in from the default key; the default key's map is returned only when the key itself is absent.
    #[inline]
    pub fn get<K: LookupKey>(&self, key: K) -> &HashMap<String, JSONGetTextValue<'a>> {
        key.lookup(&self.context).unwrap_or_else(|| self.default_map())
    }

    /// Get text from context.
    #[inline]
    pub fn get_text<T: AsRef<str>>(&self, text: T) -> Option<JSONGetTextValue<'_>> {
        let map = self.default_map();

        text_lookup::get_text(map, map, text.as_ref())
    }

    /// Get text from context with a specific key, falling back to the default key when the text is missing.
    #[inline]
    pub fn get_text_with_key<K: LookupKey, T: AsRef<str>>(
        &self,
        key: K,
        text: T,
    ) -> Option<JSONGetTextValue<'_>> {
        let default_map = self.default_map();
        let map = key.lookup(&self.context).unwrap_or(default_map);

        text_lookup::get_text(map, default_map, text.as_ref())
    }

    /// Get multiple text from context. The output map is usually used for serialization.
    pub fn get_multiple_text<'b, T: AsRef<str> + ?Sized>(
        &self,
        text_array: &[&'b T],
    ) -> Option<BTreeMap<&'b str, JSONGetTextValue<'_>>> {
        let map = self.default_map();

        text_lookup::get_multiple_text(map, map, text_array)
    }

    /// Get multiple text from context with a specific key, falling back to the default key per text. The output map is usually used for serialization.
    pub fn get_multiple_text_with_key<'b, K: LookupKey, T: AsRef<str> + ?Sized>(
        &self,
        key: K,
        text_array: &[&'b T],
    ) -> Option<BTreeMap<&'b str, JSONGetTextValue<'_>>> {
        let default_map = self.default_map();
        let map = key.lookup(&self.context).unwrap_or(default_map);

        text_lookup::get_multiple_text(map, default_map, text_array)
    }

    /// Get filtered text from context by a Regex instance. The output map is usually used for serialization.
    #[cfg(feature = "regex")]
    pub fn get_filtered_text(&self, regex: &Regex) -> Option<BTreeMap<&str, JSONGetTextValue<'_>>> {
        Some(text_lookup::get_filtered_text(self.default_map(), regex))
    }

    /// Get filtered text owned by a specific key by a Regex instance. Texts are not filled in from the default key. The output map is usually used for serialization.
    /// If the key itself is absent, the default key's map is used.
    #[cfg(feature = "regex")]
    pub fn get_filtered_text_with_key<K: LookupKey>(
        &self,
        key: K,
        regex: &Regex,
    ) -> Option<BTreeMap<&str, JSONGetTextValue<'_>>> {
        let map = key.lookup(&self.context).unwrap_or_else(|| self.default_map());

        Some(text_lookup::get_filtered_text(map, regex))
    }

    /// The map of texts owned by the default key. The default key is always present, so this never panics.
    #[inline]
    fn default_map(&self) -> &HashMap<String, JSONGetTextValue<'a>> {
        self.context.get(&self.default_key).unwrap()
    }

    /// Compare each text's JSON output, including the order of fields inside objects.
    #[cfg(all(debug_assertions, feature = "rocket"))]
    pub(super) fn has_same_texts(&self, other: &Self) -> bool {
        self.default_key == other.default_key
            && self.context.len() == other.context.len()
            && self.context.iter().all(|(key, texts)| {
                other.context.get(key).is_some_and(|other_texts| {
                    texts.len() == other_texts.len()
                        && texts.iter().all(|(text, value)| {
                            other_texts.get(text).is_some_and(|other_value| {
                                value.to_json_string() == other_value.to_json_string()
                            })
                        })
                })
            })
    }
}
