use std::fmt::{self, Display, Formatter};

use crate::unic_langid::subtags::Language;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Key(pub Language);

impl Display for Key {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.0.as_str())
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
