use json_gettext::{Key, key};

pub fn keys() -> (Key, Key, Key) {
    #[cfg(not(feature = "langid"))]
    {
        (key!("en_US"), key!("zh_TW"), key!("de"))
    }
    #[cfg(feature = "locale")]
    {
        (key!("en_US"), key!("zh_TW"), key!("de_DE"))
    }
    #[cfg(feature = "language")]
    {
        (key!("en"), key!("zh"), key!("de"))
    }
    #[cfg(feature = "region")]
    {
        (key!("US"), key!("TW"), key!("DE"))
    }
}
