extern crate rocket;

#[cfg(debug_assertions)]
mod debug;

#[cfg(not(debug_assertions))]
mod release;

#[cfg(debug_assertions)]
pub use debug::*;

#[cfg(not(debug_assertions))]
pub use release::*;

use rocket::form::{self, FromFormField, ValueField};
use rocket::request::FromParam;

use super::Key;

#[rocket::async_trait]
impl<'v> FromFormField<'v> for Key {
    fn from_value(field: ValueField<'v>) -> form::Result<'v, Self> {
        Ok(Key(String::from(field.value)))
    }
}

impl<'a> FromParam<'a> for Key {
    type Error = ();

    #[inline]
    fn from_param(v: &'a str) -> Result<Self, Self::Error> {
        Ok(Key(String::from(v)))
    }
}
