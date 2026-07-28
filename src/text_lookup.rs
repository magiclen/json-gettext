use std::collections::{BTreeMap, HashMap};

#[cfg(feature = "regex")]
use regex::Regex;

use crate::{Context, JSONGetTextBuildError, JSONGetTextValue, Key};

/// A map from a text name to its value within a single key (locale).
type TextMap<'a> = HashMap<String, JSONGetTextValue<'a>>;

/// Validate that every non-default key only contains texts that also exist in the default key.
/// Each key keeps only its own texts; missing texts are resolved at lookup time by falling back to the default key.
pub(crate) fn finalize_context<'a>(
    default_key: &Key,
    mut context: Context<'a>,
) -> Result<Context<'a>, JSONGetTextBuildError> {
    // Recover the owned key from the map so it can be re-inserted without cloning.
    let (default_key, default_map) = match context.remove_entry(default_key) {
        Some(entry) => entry,
        None => return Err(JSONGetTextBuildError::DefaultKeyNotFound),
    };

    let mut inner_context = HashMap::with_capacity(context.len() + 1);

    for (key, map) in context {
        for map_key in map.keys() {
            if !default_map.contains_key(map_key) {
                return Err(JSONGetTextBuildError::TextInKeyNotInDefaultKey {
                    key,
                    text: map_key.clone(),
                });
            }
        }

        inner_context.insert(key, map);
    }

    inner_context.insert(default_key, default_map);

    Ok(inner_context)
}

/// Look up a single text, falling back to the default key's map when the text is missing.
#[inline]
pub(crate) fn get_text<'r>(
    map: &'r TextMap<'_>,
    default_map: &'r TextMap<'_>,
    text: &str,
) -> Option<JSONGetTextValue<'r>> {
    map.get(text).or_else(|| default_map.get(text)).map(|value| value.clone_borrowed())
}

/// Look up multiple texts, falling back to the default key per text. Returns `None` as soon as any text is missing from both.
pub(crate) fn get_multiple_text<'r, 'b, T: AsRef<str> + ?Sized>(
    map: &'r TextMap<'_>,
    default_map: &'r TextMap<'_>,
    text_array: &[&'b T],
) -> Option<BTreeMap<&'b str, JSONGetTextValue<'r>>> {
    let mut new_map = BTreeMap::new();

    for &text in text_array {
        let text = text.as_ref();
        let value = map.get(text).or_else(|| default_map.get(text))?;
        new_map.insert(text, value.clone_borrowed());
    }

    Some(new_map)
}

/// Collect the texts whose names match the regex from an already-resolved map, without any fallback.
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
