use std::collections::HashMap;

mod keys;

// Only trait implementations for `Key` live here; nothing needs to be re-exported.
#[cfg(feature = "rocket")]
mod rocket_feature;

pub use keys::*;

use crate::{IntoKey, LookupKey};

impl LookupKey for Key {
    #[inline]
    fn lookup<'m, V>(&self, map: &'m HashMap<Key, V>) -> Option<&'m V> {
        map.get(self)
    }
}

impl LookupKey for &Key {
    #[inline]
    fn lookup<'m, V>(&self, map: &'m HashMap<Key, V>) -> Option<&'m V> {
        map.get(*self)
    }
}

impl IntoKey for Key {
    #[inline]
    fn into_key(self) -> Key {
        self
    }
}

impl IntoKey for &Key {
    #[inline]
    fn into_key(self) -> Key {
        *self
    }
}
