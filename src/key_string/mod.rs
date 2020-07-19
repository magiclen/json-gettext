mod json_get_text_builder;
mod json_gettext;

#[cfg(feature = "rocket")]
mod rocket_feature;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;

use crate::JSONGetTextValue;

pub use self::json_gettext::*;

pub use json_get_text_builder::*;

#[cfg(feature = "rocket")]
pub use rocket_feature::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Key(pub String);

impl Display for Key {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.0.as_str())
    }
}

impl PartialEq<String> for Key {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<Key> for String {
    #[inline]
    fn eq(&self, other: &Key) -> bool {
        self.eq(&other.0)
    }
}

impl From<String> for Key {
    #[inline]
    fn from(s: String) -> Self {
        Key(s)
    }
}

impl Borrow<str> for Key {
    #[inline]
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}

impl Borrow<String> for Key {
    #[inline]
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl Deref for Key {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
