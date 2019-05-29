use std::collections::HashMap;

use crate::serde_json::{Value, Map, Error as JSONError};

use crate::{Context, JSONGetTextValue, JSONGetTextBuilder};
use crate::regex::Regex;

#[derive(Debug)]
pub enum JSONGetTextError {
    DefaultKeyNotFound,
    KeyNotFound,
    TextInKeyNotInDefaultKey {
        key: String,
        text: String,
    },
    SerdeJSONError(JSONError),
}

impl From<JSONError> for JSONGetTextError {
    #[inline]
    fn from(v: JSONError) -> JSONGetTextError {
        JSONGetTextError::SerdeJSONError(v)
    }
}

/// A wrapper for context and a default key. **Keys** are usually considered as locales.
#[derive(Debug)]
pub struct JSONGetText<'a> {
    default_key: String,
    context: Context<'a>,
}

impl<'a> JSONGetText<'a> {
    /// Create a new `JSONGetTextBuilder` instance. You need to decide your default key at the stage.
    #[inline]
    pub fn build<S: Into<String>>(default_key: S) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder::new(default_key)
    }

    /// Create a new JSONGetText instance with context and a default key.
    pub(crate) fn from_context_with_default_key<S: AsRef<str> + Into<String>>(default_key: S, mut context: Context<'a>) -> Result<JSONGetText<'a>, JSONGetTextError> {
        if !context.contains_key(default_key.as_ref()) {
            return Err(JSONGetTextError::DefaultKeyNotFound);
        }

        let default_key = default_key.into();

        let default_map = context.remove(&default_key).unwrap();

        let mut inner_context = HashMap::new();

        {
            for (key, mut map) in context {
                {
                    for map_key in map.keys() {
                        if !default_map.contains_key(map_key) {
                            return Err(JSONGetTextError::TextInKeyNotInDefaultKey {
                                key,
                                text: map_key.clone(),
                            });
                        }
                    }
                }

                {
                    for map_key in default_map.keys() {
                        if !map.contains_key(map_key) {
                            map.insert(map_key.clone(), default_map.get(map_key).unwrap().clone());
                        }
                    }
                }

                inner_context.insert(key, map);
            }

            inner_context.insert(default_key.clone(), default_map);
        }

        Ok(JSONGetText {
            default_key,
            context: inner_context,
        })
    }

    /// Get all keys in context.
    pub fn get_keys(&self) -> Vec<&str> {
        let mut vec = Vec::with_capacity(self.context.len());

        for key in self.context.keys() {
            vec.push(key.as_str());
        }

        vec
    }

    /// Get the default key.
    #[inline]
    pub fn get_default_key(&self) -> &str {
        &self.default_key
    }

    /// Get a string map from context by a key.
    #[inline]
    pub fn get<K: AsRef<str>>(&self, key: K) -> &HashMap<String, JSONGetTextValue<'a>> {
        match self.context.get(key.as_ref()) {
            Some(m) => m,
            None => self.context.get(&self.default_key).unwrap()
        }
    }

    /// Get text from context.
    #[inline]
    pub fn get_text<T: AsRef<str>>(&'a self, text: T) -> Option<JSONGetTextValue<'a>> {
        let map = self.context.get(&self.default_key).unwrap();

        map.get(text.as_ref()).map(|v| v.clone_borrowed())
    }

    /// Get text from context with a specific key.
    #[inline]
    pub fn get_text_with_key<K: AsRef<str>, T: AsRef<str>>(&'a self, key: K, text: T) -> Option<JSONGetTextValue<'a>> {
        let map = self.context.get(key.as_ref()).unwrap_or(self.context.get(&self.default_key).unwrap());

        map.get(text.as_ref()).map(|v| v.clone_borrowed())
    }

    /// Get multiple text from context. The output map is usually used for serialization.
    pub fn get_multiple_text<'b, T: AsRef<str> + ?Sized>(&self, text_array: &[&'b T]) -> Option<HashMap<&'b str, JSONGetTextValue>> {
        let map = self.context.get(&self.default_key).unwrap();

        let mut new_map = HashMap::new();

        for &text in text_array.iter() {
            let text = text.as_ref();
            let value = map.get(text)?;
            new_map.insert(text, value.clone_borrowed());
        }

        Some(new_map)
    }

    /// Get multiple text from context with a specific key. The output map is usually used for serialization.
    pub fn get_multiple_text_with_key<'b, K: AsRef<str>, T: AsRef<str> + ?Sized>(&'a self, key: K, text_array: &[&'b T]) -> Option<HashMap<&'b str, JSONGetTextValue<'a>>> {
        let map = self.context.get(key.as_ref()).unwrap_or(self.context.get(&self.default_key).unwrap());

        let mut new_map = HashMap::new();

        for &text in text_array.iter() {
            let text = text.as_ref();
            let value = map.get(text)?;
            new_map.insert(text, value.clone_borrowed());
        }

        Some(new_map)
    }

    /// Get filtered text from context by a Regex instance. The output map is usually used for serialization.
    pub fn get_filtered_text(&'a self, regex: &Regex) -> Option<HashMap<&str, JSONGetTextValue<'a>>> {
        let map = self.context.get(&self.default_key).unwrap();

        let mut new_map = HashMap::new();

        for (key, value) in map.iter() {
            if !regex.is_match(key) {
                continue;
            }
            new_map.insert(key.as_str(), value.clone_borrowed());
        }

        Some(new_map)
    }

    /// Get filtered text from context with a specific key by a Regex instance. The output map is usually used for serialization.
    pub fn get_filtered_text_with_key<K: AsRef<str>>(&'a self, key: K, regex: &Regex) -> Option<HashMap<&str, JSONGetTextValue<'a>>> {
        let map = self.context.get(key.as_ref()).unwrap_or(self.context.get(&self.default_key).unwrap());

        let mut new_map = HashMap::new();

        for (key, value) in map.iter() {
            if !regex.is_match(key) {
                continue;
            }
            new_map.insert(key.as_str(), value.clone_borrowed());
        }

        Some(new_map)
    }
}

impl<'a> JSONGetText<'a> {
    pub fn set_json<K: AsRef<str> + Into<String>, J: AsRef<str> + ?Sized>(&mut self, key: K, json: &'a J) -> Result<&mut Self, JSONGetTextError> {
        let key_str = key.as_ref();

        if self.default_key.eq(key_str) {
            let default_map_old = self.context.remove(key_str).unwrap();

            let default_map: HashMap<String, JSONGetTextValue<'a>> = serde_json::from_str(json.as_ref())?;

            let mut err = None;

            for (key, map) in &self.context {
                for map_key in map.keys() {
                    if !default_map.contains_key(map_key) {
                        err = Some(JSONGetTextError::TextInKeyNotInDefaultKey {
                            key: key.clone(),
                            text: map_key.clone(),
                        });

                        break;
                    }
                }
            }

            let key = key.into();

            if let Some(err) = err {
                self.context.insert(key, default_map_old);

                Err(err)
            } else {
                self.context.insert(key, default_map);

                Ok(self)
            }
        } else {
            let map_old = self.context.remove(key_str);

            match map_old {
                Some(map_old) => {
                    let map: HashMap<String, JSONGetTextValue<'a>> = serde_json::from_str(json.as_ref())?;

                    let mut err = None;

                    let key = key.into();

                    {
                        let default_map = self.context.get(&self.default_key).unwrap();

                        for map_key in map.keys() {
                            if !default_map.contains_key(map_key) {
                                err = Some(JSONGetTextError::TextInKeyNotInDefaultKey {
                                    key: key.clone(),
                                    text: map_key.clone(),
                                });

                                break;
                            }
                        }
                    }

                    if let Some(err) = err {
                        self.context.insert(key, map_old);

                        Err(err)
                    } else {
                        self.context.insert(key, map);

                        Ok(self)
                    }
                }
                None => Err(JSONGetTextError::KeyNotFound)
            }
        }
    }

    pub fn set_json_owned<K: AsRef<str> + Into<String>, J: AsRef<str>>(&mut self, key: K, json: J) -> Result<&mut Self, JSONGetTextError> {
        let key_str = key.as_ref();

        if self.default_key.eq(key_str) {
            let default_map_old = self.context.remove(key_str).unwrap();

            let value: Map<String, Value> = serde_json::from_str(json.as_ref())?;

            let mut default_map: HashMap<String, JSONGetTextValue<'static>> = HashMap::with_capacity(value.len());

            for (k, v) in value {
                default_map.insert(k, JSONGetTextValue::from_json_value(v));
            }

            let mut err = None;

            for (key, map) in &self.context {
                for map_key in map.keys() {
                    if !default_map.contains_key(map_key) {
                        err = Some(JSONGetTextError::TextInKeyNotInDefaultKey {
                            key: key.clone(),
                            text: map_key.clone(),
                        });

                        break;
                    }
                }
            }

            let key = key.into();

            if let Some(err) = err {
                self.context.insert(key, default_map_old);

                Err(err)
            } else {
                self.context.insert(key, default_map);

                Ok(self)
            }
        } else {
            let map_old = self.context.remove(key_str);

            match map_old {
                Some(map_old) => {
                    let value: Map<String, Value> = serde_json::from_str(json.as_ref())?;

                    let mut map: HashMap<String, JSONGetTextValue<'static>> = HashMap::with_capacity(value.len());

                    for (k, v) in value {
                        map.insert(k, JSONGetTextValue::from_json_value(v));
                    }

                    let mut err = None;

                    let key = key.into();

                    {
                        let default_map = self.context.get(&self.default_key).unwrap();

                        for map_key in map.keys() {
                            if !default_map.contains_key(map_key) {
                                err = Some(JSONGetTextError::TextInKeyNotInDefaultKey {
                                    key: key.clone(),
                                    text: map_key.clone(),
                                });

                                break;
                            }
                        }
                    }

                    if let Some(err) = err {
                        self.context.insert(key, map_old);

                        Err(err)
                    } else {
                        self.context.insert(key, map);

                        Ok(self)
                    }
                }
                None => Err(JSONGetTextError::KeyNotFound)
            }
        }
    }
}