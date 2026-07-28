use std::collections::{BTreeMap, HashMap};

#[cfg(feature = "regex")]
use regex::Regex;

use crate::{Context, JSONGetTextBuildError, JSONGetTextValue, Key};

/// A map from a text name to its value within a single key (locale).
type TextMap<'a> = HashMap<String, JSONGetTextValue<'a>>;

/// Validate every non-default key against the default key and back-fill missing texts from the default key.
pub(crate) fn finalize_context<'a>(
    default_key: Key,
    mut context: Context<'a>,
) -> Result<Context<'a>, JSONGetTextBuildError> {
    if !context.contains_key(&default_key) {
        return Err(JSONGetTextBuildError::DefaultKeyNotFound);
    }

    let default_map = context.remove(&default_key).unwrap();

    let mut inner_context = HashMap::with_capacity(context.len() + 1);

    for (key, mut map) in context {
        for map_key in map.keys() {
            if !default_map.contains_key(map_key) {
                return Err(JSONGetTextBuildError::TextInKeyNotInDefaultKey {
                    key,
                    text: map_key.clone(),
                });
            }
        }

        for (map_key, value) in default_map.iter() {
            if !map.contains_key(map_key) {
                map.insert(map_key.clone(), value.clone());
            }
        }

        inner_context.insert(key, map);
    }

    inner_context.insert(default_key, default_map);

    Ok(inner_context)
}

/// Look up a single text in an already-resolved map.
#[inline]
pub(crate) fn get_text<'r>(map: &'r TextMap<'_>, text: &str) -> Option<JSONGetTextValue<'r>> {
    map.get(text).map(|value| value.clone_borrowed())
}

/// Look up multiple texts in an already-resolved map. Returns `None` as soon as any text is missing.
pub(crate) fn get_multiple_text<'r, 'b, T: AsRef<str> + ?Sized>(
    map: &'r TextMap<'_>,
    text_array: &[&'b T],
) -> Option<BTreeMap<&'b str, JSONGetTextValue<'r>>> {
    let mut new_map = BTreeMap::new();

    for &text in text_array {
        let text = text.as_ref();
        let value = map.get(text)?;
        new_map.insert(text, value.clone_borrowed());
    }

    Some(new_map)
}

/// Collect the texts whose names match the regex from an already-resolved map.
#[cfg(feature = "regex")]
pub(crate) fn get_filtered_text<'r>(
    map: &'r TextMap<'_>,
    regex: &Regex,
) -> BTreeMap<&'r str, JSONGetTextValue<'r>> {
    let mut new_map = BTreeMap::new();

    for (key, value) in map.iter() {
        if regex.is_match(key) {
            new_map.insert(key.as_str(), value.clone_borrowed());
        }
    }

    new_map
}
