use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::serde_json::Error as JSONError;

#[derive(Debug)]
#[non_exhaustive]
pub enum JSONGetTextValueError {
    IntegerOutOfRange,
    ParseError(JSONError),
}

impl Display for JSONGetTextValueError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JSONGetTextValueError::IntegerOutOfRange => f.write_str("The integer is out of range."),
            JSONGetTextValueError::ParseError(error) => Display::fmt(error, f),
        }
    }
}

impl Error for JSONGetTextValueError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            JSONGetTextValueError::ParseError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<JSONError> for JSONGetTextValueError {
    #[inline]
    fn from(error: JSONError) -> JSONGetTextValueError {
        JSONGetTextValueError::ParseError(error)
    }
}
