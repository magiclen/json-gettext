use std::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

use crate::unic_langid::{
    subtags::{Language, Region},
    LanguageIdentifier, LanguageIdentifierError,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Key(pub Language, pub Option<Region>);

impl Display for Key {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.0.as_str())?;

        if let Some(region) = self.1 {
            f.write_char('_')?;
            f.write_str(region.as_str())?;
        }

        Ok(())
    }
}

impl FromStr for Key {
    type Err = LanguageIdentifierError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let langid = LanguageIdentifier::from_str(s)?;

        let language = langid.language;
        let region = langid.region;

        Ok(Key(language, region))
    }
}

/**
Create a literal key.

```rust
#[macro_use] extern crate json_gettext;

use std::str::FromStr;

use json_gettext::unic_langid::subtags::{Language, Region};
use json_gettext::Key;

let key = key!("en_US");

assert_eq!(Key(Language::from_str("en").unwrap(), Some(Region::from_str("US").unwrap())), key);
```
*/
#[macro_export]
macro_rules! key {
    ($key:expr) => {{
        let langid = $crate::unic_langid::langid!($key);

        $crate::Key(langid.language, langid.region)
    }};
}
