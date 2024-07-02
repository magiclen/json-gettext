use std::{collections::HashMap, fs::File, path::Path};

use serde::Serialize;
use serde_json::{Map, Value};

use super::{Context, JSONGetText, JSONGetTextValue};
use crate::JSONGetTextBuildError;

/// To build a JSONGetText instance, this struct can help you do that step by step.
#[derive(Debug, Clone)]
pub struct JSONGetTextBuilder<'a> {
    default_key: String,
    context:     Context<'a>,
}

impl<'a> JSONGetTextBuilder<'a> {
    /// Create a new `JSONGetTextBuilder` instance. You need to decide your default key at the stage.
    #[inline]
    pub fn new<S: Into<String>>(default_key: S) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder {
            default_key: default_key.into(), context: HashMap::new()
        }
    }

    /// Add a JSON string to the context for a specify key. The JSON string must represent a map object (key-value).
    pub fn add_json<K: AsRef<str> + Into<String>, J: AsRef<str> + ?Sized>(
        &mut self,
        key: K,
        json: &'a J,
    ) -> Result<&mut Self, JSONGetTextBuildError> {
        if self.context.contains_key(key.as_ref()) {
            return Err(JSONGetTextBuildError::DuplicatedKey(key.into().into()));
        }

        let map: HashMap<String, JSONGetTextValue<'a>> = serde_json::from_str(json.as_ref())?;

        self.context.insert(key.into().into(), map);

        Ok(self)
    }

    /// Add a JSON string to the context for a specify key. The JSON string must represent a map object (key-value).
    pub fn add_json_owned<K: AsRef<str> + Into<String>, J: AsRef<str>>(
        &mut self,
        key: K,
        json: J,
    ) -> Result<&mut Self, JSONGetTextBuildError> {
        if self.context.contains_key(key.as_ref()) {
            return Err(JSONGetTextBuildError::DuplicatedKey(key.into().into()));
        }

        let value: Map<String, Value> = serde_json::from_str(json.as_ref())?;

        let mut map: HashMap<String, JSONGetTextValue<'static>> =
            HashMap::with_capacity(value.len());

        for (k, v) in value {
            map.insert(k, JSONGetTextValue::from_json_value(v));
        }

        self.context.insert(key.into().into(), map);

        Ok(self)
    }

    /// Add a JSON file to the context for a specify key. The JSON file must represent a map object (key-value).
    pub fn add_json_file<K: AsRef<str> + Into<String>, P: AsRef<Path>>(
        &mut self,
        key: K,
        path: P,
    ) -> Result<&mut Self, JSONGetTextBuildError> {
        if self.context.contains_key(key.as_ref()) {
            return Err(JSONGetTextBuildError::DuplicatedKey(key.into().into()));
        }

        let path = path.as_ref();

        let value: Map<String, Value> = serde_json::from_reader(File::open(path)?)?;

        let mut map: HashMap<String, JSONGetTextValue<'static>> =
            HashMap::with_capacity(value.len());

        for (k, v) in value {
            map.insert(k, JSONGetTextValue::from_json_value(v));
        }

        self.context.insert(key.into().into(), map);

        Ok(self)
    }

    /// Add any serializable value to the context for a specify key. The value must represent a map object (key-value).
    pub fn add_serialize<K: AsRef<str> + Into<String>, S: Serialize>(
        &mut self,
        key: K,
        value: S,
    ) -> Result<&mut Self, JSONGetTextBuildError> {
        if self.context.contains_key(key.as_ref()) {
            return Err(JSONGetTextBuildError::DuplicatedKey(key.into().into()));
        }

        let value: Value = serde_json::to_value(value)?;

        match value {
            Value::Object(value) => {
                let mut map: HashMap<String, JSONGetTextValue<'static>> =
                    HashMap::with_capacity(value.len());

                for (k, v) in value {
                    map.insert(k, JSONGetTextValue::from_json_value(v));
                }

                self.context.insert(key.into().into(), map);

                Ok(self)
            },
            _ => {
                serde_json::from_str::<Map<String, Value>>("\"MagicLen\"")?;

                unreachable!()
            },
        }
    }

    /// Add a map to the context.
    pub fn add_map<K: AsRef<str> + Into<String>>(
        &mut self,
        key: K,
        map: HashMap<String, JSONGetTextValue<'a>>,
    ) -> Result<&mut Self, JSONGetTextBuildError> {
        if self.context.contains_key(key.as_ref()) {
            return Err(JSONGetTextBuildError::DuplicatedKey(key.into().into()));
        }

        self.context.insert(key.into().into(), map);

        Ok(self)
    }

    /// Build a `JSONGetText` instance.
    pub fn build(self, ignore_extra_keys: bool) -> Result<JSONGetText<'a>, JSONGetTextBuildError> {
        JSONGetText::from_context_with_default_key(
            self.default_key,
            ignore_extra_keys,
            self.context,
        )
    }
}

impl<'a> From<String> for JSONGetTextBuilder<'a> {
    #[inline]
    fn from(v: String) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder::new(v)
    }
}
