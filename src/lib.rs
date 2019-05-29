pub extern crate regex;
pub extern crate serde_json;
#[macro_use]
extern crate serde;

mod json_gettext_value;
mod json_gettext;
//mod reloadable_json_gettext;
mod builder;
mod macros;

use std::collections::HashMap;

pub use serde_json::Value;

pub use json_gettext_value::JSONGetTextValue;
pub use json_gettext::{JSONGetText, JSONGetTextError};
pub use builder::{JSONGetTextBuilder, JSONGetTextBuilderError};

pub type Context<'a> = HashMap<String, HashMap<String, JSONGetTextValue<'a>>>;