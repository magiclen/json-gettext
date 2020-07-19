use std::fmt::{self, Display, Formatter};

use crate::unic_langid::subtags::Region;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Key(pub Region);

impl Display for Key {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.0.as_str())
    }
}

impl PartialEq<Region> for Key {
    #[inline]
    fn eq(&self, other: &Region) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<Key> for Region {
    #[inline]
    fn eq(&self, other: &Key) -> bool {
        self.eq(&other.0)
    }
}

/**
Create a literal key.

```rust
#[macro_use] extern crate json_gettext;

use std::str::FromStr;

use json_gettext::unic_langid::subtags::Region;
use json_gettext::Key;

let key = key!("us");

assert_eq!(Key(Region::from_str("us").unwrap()), key);
```
*/
#[macro_export]
macro_rules! key {
    ($key:expr) => {{
        $crate::Key($crate::unic_langid_macros::region!($key))
    }};
}
