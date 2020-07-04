use std::collections::HashMap;

use crate::JSONGetTextValue;

mod json_get_text_builder;
mod json_gettext;
mod keys;

#[cfg(feature = "rocket")]
mod rocket_feature;

pub use self::json_gettext::*;

pub use json_get_text_builder::*;
pub use keys::*;

#[cfg(feature = "rocket")]
pub use rocket_feature::*;

pub type Context<'a> = HashMap<Key, HashMap<String, JSONGetTextValue<'a>>>;
