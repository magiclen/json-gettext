use std::collections::HashMap;
use std::io;

use crate::serde_json::Error as JSONError;

use crate::{Context, JSONGetTextValue, JSONGetText, JSONGetTextError};

#[derive(Debug)]
pub enum JSONGetTextBuilderError {
    DuplicatedKey(String),
    IOError(io::Error),
    SerdeJSONError(JSONError),
}

impl ToString for JSONGetTextBuilderError {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            JSONGetTextBuilderError::DuplicatedKey(s) => s.clone(),
            JSONGetTextBuilderError::IOError(err) => err.to_string(),
            JSONGetTextBuilderError::SerdeJSONError(err) => err.to_string()
        }
    }
}

impl From<io::Error> for JSONGetTextBuilderError {
    #[inline]
    fn from(v: io::Error) -> JSONGetTextBuilderError {
        JSONGetTextBuilderError::IOError(v)
    }
}

impl From<JSONError> for JSONGetTextBuilderError {
    #[inline]
    fn from(v: JSONError) -> JSONGetTextBuilderError {
        JSONGetTextBuilderError::SerdeJSONError(v)
    }
}

/// To build a JSONGetText instance, this struct can help you do that step by step.
#[derive(Debug, Clone)]
pub struct JSONGetTextBuilder<'a> {
    default_key: String,
    context: Context<'a>,
}

impl<'a> JSONGetTextBuilder<'a> {
    /// Create a new `JSONGetTextBuilder` instance. You need to decide your default key at the stage.
    #[inline]
    pub fn new<S: Into<String>>(default_key: S) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder {
            default_key: default_key.into(),
            context: HashMap::new(),
        }
    }

    /// Add a JSON string to the context for a specify key. The JSON string must represent a map object (key-value).
    pub fn add_json<K: AsRef<str> + Into<String>, J: AsRef<str> + ?Sized>(&mut self, key: K, json: &'a J) -> Result<&mut Self, JSONGetTextBuilderError> {
        if self.context.contains_key(key.as_ref()) {
            return Err(JSONGetTextBuilderError::DuplicatedKey(key.into()));
        }

        let map: HashMap<String, JSONGetTextValue<'a>> = serde_json::from_str(json.as_ref())?;

        let key = key.into();

        self.context.insert(key, map);

        Ok(self)
    }

    /// Add a map to the context.
    pub fn add_map<K: AsRef<str> + Into<String>>(&mut self, key: K, map: HashMap<String, JSONGetTextValue<'a>>) -> Result<&mut Self, JSONGetTextBuilderError> {
        if self.context.contains_key(key.as_ref()) {
            return Err(JSONGetTextBuilderError::DuplicatedKey(key.into()));
        }

        let key = key.into();

        self.context.insert(key, map);

        Ok(self)
    }

    /// Build a `JSONGetText` instance.
    pub fn build(self) -> Result<JSONGetText<'a>, JSONGetTextError> {
        JSONGetText::from_context_with_default_key(self.default_key, self.context)
    }
}

impl<'a> From<String> for JSONGetTextBuilder<'a> {
    #[inline]
    fn from(v: String) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder::new(v)
    }
}