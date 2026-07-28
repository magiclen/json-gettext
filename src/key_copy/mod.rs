use std::collections::HashMap;

mod keys;

#[cfg(feature = "rocket")]
mod rocket_feature;

pub use keys::*;
#[cfg(feature = "rocket")]
pub use rocket_feature::*;

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
