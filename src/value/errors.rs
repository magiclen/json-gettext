use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::serde_json::Error as JSONError;

#[derive(Debug)]
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

impl Error for JSONGetTextValueError {}

impl From<JSONError> for JSONGetTextValueError {
    #[inline]
    fn from(error: JSONError) -> JSONGetTextValueError {
        JSONGetTextValueError::ParseError(error)
    }
}
