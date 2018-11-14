/*!
# JSON GetText

This is a library for getting text from JSON usually for internationalization.

## Example

```rust
#[macro_use] extern crate json_gettext;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate lazy_static_include;

let ctx = static_json_gettext_build!(
            "en_US",
            "en_US", "langs/en_US.json",
            "zh_TW", "langs/zh_TW.json"
        ).unwrap();

assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
```

In order to reduce the compilation time, the `static_json_gettext_build` macro has files compiled into your executable binary file together, only when you are using the **release** profile.
*/

pub extern crate serde_json;
extern crate serde;
extern crate regex;

mod json_gettext_value;

pub use json_gettext_value::JSONGetTextValue;

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io;

use regex::Regex;
pub use serde_json::Value;

type Context<'a> = HashMap<String, HashMap<String, JSONGetTextValue<'a>>>;

/// To build a JSONGetText instance, this struct can help you do that step by step.
#[derive(Debug, Clone)]
pub struct JSONGetTextBuilder<'a> {
    default_key: String,
    context: Context<'a>,
}

#[derive(Debug)]
pub enum JSONGetTextBuilderError {
    KeyRepeat(String),
    IOError(io::Error),
    SerdeError(serde_json::Error),
}

impl<'a, 'e> JSONGetTextBuilder<'a> {
    /// Create a new JSONGetTextBuilder instance. You need to decide your default key at the stage.
    pub fn new<S: AsRef<str>>(default_key: S) -> JSONGetTextBuilder<'a> {
        Self::from_default_key_str(default_key)
    }

    /// Create a new JSONGetTextBuilder instance. You need to decide your default key at the stage.
    pub fn from_default_key_str<S: AsRef<str>>(default_key: S) -> JSONGetTextBuilder<'a> {
        Self::from_default_key_string(default_key.as_ref().to_string())
    }

    /// Create a new JSONGetTextBuilder instance. You need to decide your default key at the stage.
    pub fn from_default_key_string(default_key: String) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder {
            default_key: default_key,
            context: HashMap::new(),
        }
    }

    /// Add a JSON string to the context for a specify key. The JSON string must represent an object (key-value).
    pub fn add_json_string_to_context<K: AsRef<str>, J: AsRef<str>>(&mut self, key: K, json: J) -> Result<&Self, JSONGetTextBuilderError> {
        let key = key.as_ref();

        if self.context.contains_key(key) {
            return Err(JSONGetTextBuilderError::KeyRepeat(key.to_string()));
        }

        let data: HashMap<String, serde_json::Value> = serde_json::from_str(json.as_ref()).map_err(|err| JSONGetTextBuilderError::SerdeError(err))?;

        let mut map = HashMap::new();

        for (k, v) in data {
            map.insert(k, JSONGetTextValue::JSONValue(v));
        }

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Add JSON binary data to the context for a specify key. The JSON binary data must represent an object (key-value).
    pub fn add_json_bytes_to_context<K: AsRef<str> + 'e, J: ?Sized + AsRef<[u8]>>(&mut self, key: K, json: &J) -> Result<&Self, JSONGetTextBuilderError> {
        let key = key.as_ref();

        if self.context.contains_key(key) {
            return Err(JSONGetTextBuilderError::KeyRepeat(key.to_string()));
        }

        let data: HashMap<String, serde_json::Value> = serde_json::from_slice(json.as_ref()).map_err(|err| JSONGetTextBuilderError::SerdeError(err))?;

        let mut map = HashMap::new();

        for (k, v) in data {
            map.insert(k, JSONGetTextValue::JSONValue(v));
        }

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Add JSON binary data from a file to the context for a specify key. The JSON binary data must represent an object (key-value).
    pub fn add_json_file_to_context<K: AsRef<str> + 'e, P: AsRef<Path>>(&mut self, key: K, path: P) -> Result<&Self, JSONGetTextBuilderError> {
        let key = key.as_ref();

        if self.context.contains_key(key) {
            return Err(JSONGetTextBuilderError::KeyRepeat(key.to_string()));
        }

        let file = File::open(path).map_err(|err| JSONGetTextBuilderError::IOError(err))?;

        let data: HashMap<String, serde_json::Value> = serde_json::from_reader(&file).map_err(|err| JSONGetTextBuilderError::SerdeError(err))?;

        let mut map = HashMap::new();

        for (k, v) in data {
            map.insert(k, JSONGetTextValue::JSONValue(v));
        }

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Add a map to the context.
    pub fn add_map_to_context<K: AsRef<str> + 'e>(&mut self, key: K, map: HashMap<String, JSONGetTextValue<'a>>) -> Result<&Self, JSONGetTextBuilderError> {
        let key = key.as_ref();

        if self.context.contains_key(key) {
            return Err(JSONGetTextBuilderError::KeyRepeat(key.to_string()));
        }

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Build a JSONGetText instance.
    pub fn build(self) -> Result<JSONGetText<'a>, JSONGetTextError> {
        JSONGetText::from_context_inner(self.default_key, self.context)
    }
}

/// A wrapper for context and a default key. **Keys** are usually considered as locales.
#[derive(Debug)]
pub struct JSONGetText<'a> {
    default_key: String,
    context: Context<'a>,
}

#[derive(Debug)]
pub enum JSONGetTextError {
    DefaultKeyNotFound,
    TextInKeyNotInDefaultKey {
        key: String,
        text: String,
    },
}

impl<'a> JSONGetText<'a> {
    /// Create a new JSONGetTextBuilder instance. You need to decide your default key at the stage.
    pub fn build<S: AsRef<str>>(default_key: S) -> JSONGetTextBuilder<'a> {
        Self::build_with_default_key_str(default_key)
    }

    /// Create a new JSONGetTextBuilder instance. You need to decide your default key at the stage.
    pub fn build_with_default_key_str<S: AsRef<str>>(default_key: S) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder::from_default_key_str(default_key)
    }

    /// Create a new JSONGetTextBuilder instance. You need to decide your default key at the stage.
    pub fn build_with_default_key_string(default_key: String) -> JSONGetTextBuilder<'a> {
        JSONGetTextBuilder::from_default_key_string(default_key)
    }

    /// Create a new JSONGetText instance with context and a default key.
    pub fn from_context_with_default_key_str<S: AsRef<str>>(default_key: S, context: Context<'a>) -> Result<JSONGetText<'a>, JSONGetTextError> {
        JSONGetText::from_context_inner(default_key.as_ref().to_string(), context)
    }

    /// Create a new JSONGetText instance with context and a default key.
    pub fn from_context_with_default_key_string(default_key: String, context: Context<'a>) -> Result<JSONGetText<'a>, JSONGetTextError> {
        JSONGetText::from_context_inner(default_key, context)
    }

    fn from_context_inner(default_key: String, mut context: Context<'a>) -> Result<JSONGetText<'a>, JSONGetTextError> {
        if !context.contains_key(&default_key) {
            return Err(JSONGetTextError::DefaultKeyNotFound);
        }

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
        let mut vec = Vec::new();

        for key in self.context.keys() {
            vec.push(key.as_str());
        }

        vec
    }

    /// Get the default key.
    pub fn get_default_key(&self) -> &str {
        &self.default_key
    }

    /// Get a string map from context by a key.
    pub fn get<K: AsRef<str>>(&self, key: K) -> &HashMap<String, JSONGetTextValue> {
        match self.context.get(key.as_ref()) {
            Some(m) => m,
            None => self.context.get(&self.default_key).unwrap()
        }
    }

    /// Get text from context.
    pub fn get_text<T: AsRef<str>>(&self, text: T) -> Option<JSONGetTextValue> {
        let map = self.context.get(&self.default_key).unwrap();

        map.get(text.as_ref()).map(|s| match s {
            JSONGetTextValue::JSONValue(v) => JSONGetTextValue::JSONValueRef(v),
            _ => JSONGetTextValue::Str("")
        })
    }

    /// Get text from context with a specific key.
    pub fn get_text_with_key<K: AsRef<str>, T: AsRef<str>>(&self, key: K, text: T) -> Option<JSONGetTextValue> {
        let map = match self.context.get(key.as_ref()) {
            Some(m) => m,
            None => self.context.get(&self.default_key).unwrap()
        };

        map.get(text.as_ref()).map(|s| match s {
            JSONGetTextValue::JSONValue(v) => JSONGetTextValue::JSONValueRef(v),
            _ => JSONGetTextValue::Str("")
        })
    }

    /// Get multiple text from context. The output map is usually used for serialization.
    pub fn get_multiple_text<'b, T: AsRef<str>>(&self, text_array: &[&'b T]) -> Option<HashMap<&'b str, JSONGetTextValue>> {
        let map = self.context.get(&self.default_key).unwrap();

        let mut new_map = HashMap::new();

        for &text in text_array.iter() {
            let text = text.as_ref();
            let value = map.get(text)?;
            new_map.insert(text, JSONGetTextValue::JSONValueRef(match value {
                JSONGetTextValue::JSONValue(v) => v,
                _ => return None
            }));
        }

        Some(new_map)
    }

    /// Get multiple text from context with a specific key. The output map is usually used for serialization.
    pub fn get_multiple_text_with_key<'b, K: AsRef<str>, T: ?Sized + AsRef<str>>(&self, key: K, text_array: &[&'b T]) -> Option<HashMap<&'b str, JSONGetTextValue>> {
        let map = match self.context.get(key.as_ref()) {
            Some(m) => m,
            None => self.context.get(&self.default_key).unwrap()
        };

        let mut new_map = HashMap::new();

        for &text in text_array.iter() {
            let text = text.as_ref();
            let value = map.get(text)?;
            new_map.insert(text, JSONGetTextValue::JSONValueRef(match value {
                JSONGetTextValue::JSONValue(v) => v,
                _ => return None
            }));
        }

        Some(new_map)
    }

    /// Get filtered text from context by a Regex instance. The output map is usually used for serialization.
    pub fn get_filtered_text(&self, regex: &Regex) -> Option<HashMap<&str, JSONGetTextValue>> {
        let map = self.context.get(&self.default_key).unwrap();

        let mut new_map = HashMap::new();

        for (key, value) in map.iter() {
            if !regex.is_match(key) {
                continue;
            }
            new_map.insert(key.as_str(), JSONGetTextValue::JSONValueRef(match value {
                JSONGetTextValue::JSONValue(v) => v,
                _ => return None
            }));
        }

        Some(new_map)
    }

    /// Get filtered text from context with a specific key by a Regex instance. The output map is usually used for serialization.
    pub fn get_filtered_text_with_key<K: AsRef<str>>(&self, key: K, regex: &Regex) -> Option<HashMap<&str, JSONGetTextValue>> {
        let map = match self.context.get(key.as_ref()) {
            Some(m) => m,
            None => self.context.get(&self.default_key).unwrap()
        };

        let mut new_map = HashMap::new();

        for (key, value) in map.iter() {
            if !regex.is_match(key) {
                continue;
            }
            new_map.insert(key.as_str(), JSONGetTextValue::JSONValueRef(match value {
                JSONGetTextValue::JSONValue(v) => v,
                _ => return None
            }));
        }

        Some(new_map)
    }
}

mod macros;