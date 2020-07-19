extern crate rocket;

#[cfg(debug_assertions)]
mod debug;

#[cfg(not(debug_assertions))]
mod release;

use std::str::FromStr;
use std::error::Error;

#[cfg(debug_assertions)]
pub use debug::*;

#[cfg(not(debug_assertions))]
pub use release::*;

use rocket::http::RawStr;
use rocket::request::{FromFormValue, FromParam};

use super::Key;

impl<'a> FromFormValue<'a> for Key {
    type Error = Box<dyn Error>;

    #[inline]
    fn from_form_value(v: &'a RawStr) -> Result<Self, Self::Error> {
        Ok(Key::from_str(v)?)
    }
}

impl<'a> FromParam<'a> for Key {
    type Error = Box<dyn Error>;

    #[inline]
    fn from_param(v: &'a RawStr) -> Result<Self, Self::Error> {
        Ok(Key::from_str(v)?)
    }
}
