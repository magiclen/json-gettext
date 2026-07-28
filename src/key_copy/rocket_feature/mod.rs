use std::str::FromStr;

use rocket::{
    form::{self, FromFormField, ValueField},
    request::FromParam,
};

use super::Key;

#[rocket::async_trait]
impl<'v> FromFormField<'v> for Key {
    fn from_value(field: ValueField<'v>) -> form::Result<'v, Self> {
        Ok(Key::from_str(field.value).map_err(form::Error::custom)?)
    }
}

#[cfg(feature = "locale")]
impl<'a> FromParam<'a> for Key {
    type Error = crate::LanguageIdentifierError;

    #[inline]
    fn from_param(v: &'a str) -> Result<Self, Self::Error> {
        Key::from_str(v)
    }
}

#[cfg(any(feature = "language", feature = "region"))]
impl<'a> FromParam<'a> for Key {
    type Error = crate::ParserError;

    #[inline]
    fn from_param(v: &'a str) -> Result<Self, Self::Error> {
        Key::from_str(v)
    }
}
