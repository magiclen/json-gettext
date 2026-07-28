use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    io,
};

use crate::{Key, serde_json::Error as JSONError};

#[derive(Debug)]
#[non_exhaustive]
pub enum JSONGetTextBuildError {
    DefaultKeyNotFound,
    TextInKeyNotInDefaultKey {
        key:  Key,
        text: String,
    },
    DuplicatedKey(Key),
    /// The added value does not represent a JSON map object.
    NotObject,
    IOError(io::Error),
    SerdeJSONError(JSONError),
}

impl Display for JSONGetTextBuildError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            JSONGetTextBuildError::DefaultKeyNotFound => {
                f.write_str("The default key is not found.")
            },
            JSONGetTextBuildError::TextInKeyNotInDefaultKey {
                key,
                text,
            } => f.write_fmt(format_args!(
                "The text `{}` in the key `{}` is not found in the default key.",
                text, key
            )),
            JSONGetTextBuildError::DuplicatedKey(key) => {
                f.write_fmt(format_args!("The key `{key}` is duplicated."))
            },
            JSONGetTextBuildError::NotObject => {
                f.write_str("The added value does not represent a JSON map object.")
            },
            JSONGetTextBuildError::IOError(err) => Display::fmt(err, f),
            JSONGetTextBuildError::SerdeJSONError(err) => Display::fmt(err, f),
        }
    }
}

impl Error for JSONGetTextBuildError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            JSONGetTextBuildError::IOError(err) => Some(err),
            JSONGetTextBuildError::SerdeJSONError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for JSONGetTextBuildError {
    #[inline]
    fn from(v: io::Error) -> JSONGetTextBuildError {
        JSONGetTextBuildError::IOError(v)
    }
}

impl From<JSONError> for JSONGetTextBuildError {
    #[inline]
    fn from(v: JSONError) -> JSONGetTextBuildError {
        JSONGetTextBuildError::SerdeJSONError(v)
    }
}
