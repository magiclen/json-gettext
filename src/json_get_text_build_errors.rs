use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    io,
};

use crate::{serde_json::Error as JSONError, Key};

#[derive(Debug)]
pub enum JSONGetTextBuildError {
    DefaultKeyNotFound,
    TextInKeyNotInDefaultKey { key: Key, text: String },
    DuplicatedKey(Key),
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
            JSONGetTextBuildError::DuplicatedKey(key) => Display::fmt(key, f),
            JSONGetTextBuildError::IOError(err) => Display::fmt(err, f),
            JSONGetTextBuildError::SerdeJSONError(err) => Display::fmt(err, f),
        }
    }
}

impl Error for JSONGetTextBuildError {}

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
