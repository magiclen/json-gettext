use std::collections::HashMap;

use crate::JSONGetTextValue;

mod json_get_text_builder;
mod json_gettext;

#[cfg(feature = "rocket")]
mod rocket_feature;

pub use self::json_gettext::*;

pub use json_get_text_builder::*;

#[cfg(feature = "rocket")]
pub use rocket_feature::*;

pub type Key = String;

pub type Context<'a> = HashMap<Key, HashMap<String, JSONGetTextValue<'a>>>;

/**
Create a literal key.

```rust
#[macro_use] extern crate json_gettext;

let key = key!("en_US");

assert_eq!(String::from("en_US"), key);
```
*/
#[macro_export]
macro_rules! key {
    ($key:expr) => {
        format!($key)
    };
}
