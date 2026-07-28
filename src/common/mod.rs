mod json_get_text_builder;
mod json_gettext;

use std::collections::HashMap;

pub use json_get_text_builder::*;

pub use self::json_gettext::*;
use crate::JSONGetTextValue;

/// The whole context: a map from each key (locale) to that key's texts.
pub type Context<'a> = HashMap<crate::Key, HashMap<String, JSONGetTextValue<'a>>>;

/// A value that can locate an entry in a context without allocating a new key.
///
/// The crate implements this for the enabled key backend, so you rarely implement it yourself.
pub trait LookupKey {
    fn lookup<'m, V>(&self, map: &'m HashMap<crate::Key, V>) -> Option<&'m V>;
}

/// A value that can be turned into a stored `Key`.
///
/// The crate implements this for the enabled key backend, so you rarely implement it yourself.
pub trait IntoKey {
    fn into_key(self) -> crate::Key;
}
