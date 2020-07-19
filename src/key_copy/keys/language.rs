use std::fmt::{self, Display, Formatter};
use std::ops::Deref;

use crate::unic_langid::subtags::Language;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Key(pub Language);

impl Display for Key {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.0.as_str())
    }
}

impl PartialEq<Language> for Key {
    #[inline]
    fn eq(&self, other: &Language) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<Key> for Language {
    #[inline]
    fn eq(&self, other: &Key) -> bool {
        self.eq(&other.0)
    }
}

impl From<Language> for Key {
    #[inline]
    fn from(l: Language) -> Self {
        Key(l)
    }
}

impl Deref for Key {
    type Target = Language;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/**
Create a literal key.

```rust
#[macro_use] extern crate json_gettext;

use std::str::FromStr;

use json_gettext::unic_langid::subtags::Language;
use json_gettext::Key;

let key = key!("us");

assert_eq!(Key(Language::from_str("us").unwrap()), key);
```
*/
#[macro_export]
macro_rules! key {
    ($key:expr) => {{
        $crate::Key($crate::unic_langid_macros::lang!($key))
    }};
}
