// Only trait implementations for `Key` live here; nothing needs to be re-exported.
#[cfg(feature = "rocket")]
mod rocket_feature;

use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    ops::Deref,
};

use crate::{IntoKey, LookupKey};

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

impl From<Key> for String {
    #[inline]
    fn from(key: Key) -> Self {
        key.0
    }
}

impl From<&Key> for String {
    #[inline]
    fn from(key: &Key) -> Self {
        key.0.clone()
    }
}

impl AsRef<str> for Key {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
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

// Anything that looks like a string can be used to look up a key.
impl<S: AsRef<str>> LookupKey for S {
    #[inline]
    fn lookup<'m, V>(&self, map: &'m HashMap<Key, V>) -> Option<&'m V> {
        map.get(self.as_ref())
    }
}

// Anything that turns into a `String` can be used as a stored key.
impl<S: Into<String>> IntoKey for S {
    #[inline]
    fn into_key(self) -> Key {
        Key(self.into())
    }
}

/**
Create a literal key.

```rust
use json_gettext::{key, Key};

let key = key!("en_US");

assert_eq!(Key(String::from("en_US")), key);
```
*/
#[macro_export]
macro_rules! key {
    ($key:expr) => {
        $crate::Key(format!($key))
    };
}
