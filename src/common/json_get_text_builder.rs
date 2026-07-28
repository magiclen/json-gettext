use std::{collections::HashMap, fs::File, path::Path};

use serde::Serialize;
use serde_json::{Map, Value};

use super::{Context, IntoKey, JSONGetText};
use crate::{JSONGetTextBuildError, JSONGetTextValue};

/// To build a JSONGetText instance, this struct can help you do that step by step.
#[derive(Debug, Clone)]
pub struct JSONGetTextBuilder<'a> {
    default_key:       crate::Key,
    context:           Context<'a>,
    allow_extra_texts: bool,
}

impl<'a> JSONGetTextBuilder<'a> {
    /// Create a new `JSONGetTextBuilder` instance. You need to decide your default key at this stage.
    #[inline]
    pub fn new<K: IntoKey>(default_key: K) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder {
            default_key:       default_key.into_key(),
            context:           HashMap::new(),
            allow_extra_texts: false,
        }
    }

    /// Set whether a non-default key may contain texts that the default key does not define.
    /// When enabled, such texts are dropped while building instead of causing a `TextInKeyNotInDefaultKey` error.
    #[inline]
    pub fn allow_extra_texts(&mut self, allow: bool) -> &mut Self {
        self.allow_extra_texts = allow;

        self
    }

    /// Add a JSON string to the context for a specific key. The JSON string must represent a map object (key-value).
    pub fn add_json<K: IntoKey, J: AsRef<str> + ?Sized>(
        &mut self,
        key: K,
        json: &'a J,
    ) -> Result<&mut Self, JSONGetTextBuildError> {
        let key = key.into_key();

        if self.context.contains_key(&key) {
            return Err(JSONGetTextBuildError::DuplicatedKey(key));
        }

        let map: HashMap<String, JSONGetTextValue<'a>> = serde_json::from_str(json.as_ref())?;

        self.context.insert(key, map);

        Ok(self)
    }

    /// Add a JSON string to the context for a specific key. The JSON string must represent a map object (key-value).
    pub fn add_json_owned<K: IntoKey, J: AsRef<str>>(
        &mut self,
        key: K,
        json: J,
    ) -> Result<&mut Self, JSONGetTextBuildError> {
        let key = key.into_key();

        if self.context.contains_key(&key) {
            return Err(JSONGetTextBuildError::DuplicatedKey(key));
        }

        let value: Map<String, Value> = serde_json::from_str(json.as_ref())?;

        let mut map: HashMap<String, JSONGetTextValue<'static>> =
            HashMap::with_capacity(value.len());

        for (k, v) in value {
            map.insert(k, JSONGetTextValue::from_json_value(v));
        }

        self.context.insert(key, map);

        Ok(self)
    }

    /// Add a JSON file to the context for a specific key. The JSON file must represent a map object (key-value).
    pub fn add_json_file<K: IntoKey, P: AsRef<Path>>(
        &mut self,
        key: K,
        path: P,
    ) -> Result<&mut Self, JSONGetTextBuildError> {
        let key = key.into_key();

        if self.context.contains_key(&key) {
            return Err(JSONGetTextBuildError::DuplicatedKey(key));
        }

        let path = path.as_ref();

        let value: Map<String, Value> = serde_json::from_reader(File::open(path)?)?;

        let mut map: HashMap<String, JSONGetTextValue<'static>> =
            HashMap::with_capacity(value.len());

        for (k, v) in value {
            map.insert(k, JSONGetTextValue::from_json_value(v));
        }

        self.context.insert(key, map);

        Ok(self)
    }

    /// Add any serializable value to the context for a specific key. The value must represent a map object (key-value).
    pub fn add_serialize<K: IntoKey, S: Serialize>(
        &mut self,
        key: K,
        value: S,
    ) -> Result<&mut Self, JSONGetTextBuildError> {
        let key = key.into_key();

        if self.context.contains_key(&key) {
            return Err(JSONGetTextBuildError::DuplicatedKey(key));
        }

        let value: Value = serde_json::to_value(value)?;

        match value {
            Value::Object(value) => {
                let mut map: HashMap<String, JSONGetTextValue<'static>> =
                    HashMap::with_capacity(value.len());

                for (k, v) in value {
                    map.insert(k, JSONGetTextValue::from_json_value(v));
                }

                self.context.insert(key, map);

                Ok(self)
            },
            _ => Err(JSONGetTextBuildError::NotObject),
        }
    }

    /// Add a map to the context.
    pub fn add_map<K: IntoKey>(
        &mut self,
        key: K,
        map: HashMap<String, JSONGetTextValue<'a>>,
    ) -> Result<&mut Self, JSONGetTextBuildError> {
        let key = key.into_key();

        if self.context.contains_key(&key) {
            return Err(JSONGetTextBuildError::DuplicatedKey(key));
        }

        self.context.insert(key, map);

        Ok(self)
    }

    /// Build a `JSONGetText` instance.
    pub fn build(self) -> Result<JSONGetText<'a>, JSONGetTextBuildError> {
        JSONGetText::from_context_with_default_key(
            self.default_key,
            self.context,
            self.allow_extra_texts,
        )
    }
}

impl<'a> From<crate::Key> for JSONGetTextBuilder<'a> {
    #[inline]
    fn from(v: crate::Key) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder::new(v)
    }
}
