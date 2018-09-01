#![cfg_attr(feature = "nightly", feature(map_get_key_value))]

extern crate serde;
pub extern crate serde_json;
extern crate regex;

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::fmt::{self, Display, Formatter};

use regex::Regex;

type Context<'a> = HashMap<String, HashMap<String, Value<'a>>>;

/// To build a JSONGetText instance, this struct can help you do that step by step.
#[derive(Debug)]
pub struct JSONGetTextBuilder<'a> {
    default_key: String,
    context: Context<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Str(&'a str),
    JSONValue(serde_json::Value),
    JSONValueRef(&'a serde_json::Value),
}

impl<'a> Value<'a> {
    pub fn from_str(s: &'a str) -> Value<'a> {
        Value::Str(s)
    }

    pub fn from_string(s: String) -> Value<'a> {
        Value::JSONValue(serde_json::Value::String(s))
    }

    pub fn from_bool(b: bool) -> Value<'a> {
        Value::JSONValue(serde_json::Value::Bool(b))
    }

    pub fn from_i64(n: i64) -> Value<'a> {
        Value::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::I64(n))))
    }

    pub fn from_u64(n: u64) -> Value<'a> {
        Value::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::U64(n))))
    }

    pub fn from_f64(n: f64) -> Value<'a> {
        Value::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::F64(n))))
    }

    pub fn from_json_value(v: serde_json::Value) -> Value<'a> {
        Value::JSONValue(v)
    }

    pub fn from_json_value_ref(v: &'a serde_json::Value) -> Value<'a> {
        Value::JSONValueRef(v)
    }

    pub fn null() -> Value<'a> {
        Value::JSONValue(serde_json::Value::Null)
    }

    /// Convert to a string for JSON format.
    pub fn to_json(&self) -> String {
        match self {
            Value::Str(s) => {
                let mut string = String::with_capacity(s.len() + 2);
                string.push('"');
                let mut from = 0;
                for (i, c) in s.char_indices() {
                    let esc = c.escape_debug();
                    if esc.len() != 1 {
                        string.push_str(&s[from..i]);
                        for c in esc {
                            string.push(c);
                        }
                        from = i + c.len_utf8();
                    }
                }
                string.push_str(&s[from..]);
                string.push('"');

                string
            }
            Value::JSONValue(v) => {
                v.to_string()
            }
            Value::JSONValueRef(v) => {
                v.to_string()
            }
        }
    }
}

impl<'a> serde::ser::Serialize for Value<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match self {
            Value::Str(s) => s.serialize(serializer),
            Value::JSONValue(v) => v.serialize(serializer),
            Value::JSONValueRef(v) => v.serialize(serializer)
        }
    }
}

impl<'a> PartialEq<Value<'a>> for str {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Str(s) => s.eq(&self),
            Value::JSONValue(v) => v.eq(self),
            Value::JSONValueRef(v) => v.eq(&self)
        }
    }
}

impl<'a> PartialEq<Value<'a>> for &'a str {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Str(s) => s.eq(self),
            Value::JSONValue(v) => v.eq(self),
            Value::JSONValueRef(v) => v.eq(self)
        }
    }
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => s.fmt(f),
            Value::JSONValue(v) => {
                match v.as_str() {
                    Some(s) => s.fmt(f),
                    None => v.fmt(f)
                }
            }
            Value::JSONValueRef(v) => {
                match v.as_str() {
                    Some(s) => s.fmt(f),
                    None => v.fmt(f)
                }
            }
        }
    }
}

impl<'a> JSONGetTextBuilder<'a> {
    /// Create a new JSONGetTextBuilder instance. You need to decide your default key at the stage.
    pub fn new(default_key: &str) -> JSONGetTextBuilder {
        JSONGetTextBuilder {
            default_key: default_key.to_string(),
            context: HashMap::new(),
        }
    }

    /// Add a JSON string to the context for a specify key. The JSON string must represent an object (key-value).
    pub fn add_json_string_to_context(&mut self, key: &str, json: &str) -> Result<&Self, String> {
        if self.context.contains_key(key) {
            return Err("The key exists.".to_string());
        }

        let data: HashMap<String, serde_json::Value> = serde_json::from_str(json).map_err(|err| { err.to_string() })?;

        let mut map = HashMap::new();

        for (k, v) in data {
            map.insert(k, Value::JSONValue(v));
        }

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Add JSON binary data to the context for a specify key. The JSON binary data must represent an object (key-value).
    pub fn add_json_bytes_to_context(&mut self, key: &str, json: &[u8]) -> Result<&Self, String> {
        if self.context.contains_key(key) {
            return Err("The key exists.".to_string());
        }

        let data: HashMap<String, serde_json::Value> = serde_json::from_slice(json).map_err(|err| { err.to_string() })?;

        let mut map = HashMap::new();

        for (k, v) in data {
            map.insert(k, Value::JSONValue(v));
        }

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Add JSON binary data from a file to the context for a specify key. The JSON binary data must represent an object (key-value).
    pub fn add_json_file_to_context<P: AsRef<Path>>(&mut self, key: &str, path: P) -> Result<&Self, String> {
        if self.context.contains_key(key) {
            return Err("The key exists.".to_string());
        }

        let file = File::open(path).map_err(|err| { err.to_string() })?;

        let data: HashMap<String, serde_json::Value> = serde_json::from_reader(&file).map_err(|err| { err.to_string() })?;

        let mut map = HashMap::new();

        for (k, v) in data {
            map.insert(k, Value::JSONValue(v));
        }

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Add a map to the context.
    pub fn add_map_to_context<P: AsRef<Path>>(&mut self, key: &str, map: HashMap<String, Value<'a>>) -> Result<&Self, String> {
        if self.context.contains_key(key) {
            return Err("The key exists.".to_string());
        }

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Build a JSONGetText instance.
    pub fn build(self) -> Result<JSONGetText<'a>, String> {
        JSONGetText::from_context_inner(self.default_key, self.context)
    }
}

/// A wrapper for context and a default key. **Keys** are usually considered as locales.
#[derive(Debug)]
pub struct JSONGetText<'a> {
    default_key: String,
    context: Context<'a>,
}

impl<'a> JSONGetText<'a> {
    /// Create a new JSONGetTextBuilder instance. You need to decide your default key at the stage.
    pub fn build(default_key: &str) -> JSONGetTextBuilder {
        JSONGetTextBuilder::new(default_key)
    }

    /// Create a new JSONGetText instance with context and a default key.
    pub fn from_context(default_key: &str, context: Context<'a>) -> Result<JSONGetText<'a>, String> {
        JSONGetText::from_context_inner(default_key.to_string(), context)
    }

    fn from_context_inner(default_key: String, mut context: Context<'a>) -> Result<JSONGetText<'a>, String> {
        if !context.contains_key(&default_key) {
            return Err("Cannot find the default key in the context.".to_string());
        }

        if !context.contains_key(&default_key) {
            return Err("Context should contain the default key.".to_string());
        }

        let default_map = context.remove(&default_key).unwrap();

        let mut inner_context = HashMap::new();

        {
            for (key, mut map) in context {
                {
                    for map_key in map.keys() {
                        if !default_map.contains_key(map_key) {
                            return Err(format! {"The text `{}` in the key `{}` is not in the map of the default key.", map_key, key});
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

                inner_context.insert(key.clone(), map);
            }

            inner_context.insert(default_key.clone(), default_map);
        }

        Ok(JSONGetText {
            default_key: default_key,
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
    pub fn get(&self, key: &str) -> &HashMap<String, Value> {
        match self.context.get(key) {
            Some(m) => m,
            None => self.context.get(&self.default_key).unwrap()
        }
    }

    /// Get text from context.
    pub fn get_text(&self, text: &str) -> Option<Value> {
        let map = self.context.get(&self.default_key).unwrap();

        map.get(text).map(|s| match s {
            Value::JSONValue(v) => Value::JSONValueRef(v),
            _ => Value::Str("")
        })
    }

    /// Get text from context with a specific key.
    pub fn get_text_with_key(&self, key: &str, text: &str) -> Option<Value> {
        let map = match self.context.get(key) {
            Some(m) => m,
            None => self.context.get(&self.default_key).unwrap()
        };

        map.get(text).map(|s| match s {
            Value::JSONValue(v) => Value::JSONValueRef(v),
            _ => Value::Str("")
        })
    }

    /// Get multiple text from context. The output map is usually used for serialization.
    #[cfg(feature = "nightly")]
    pub fn get_multiple_text(&self, text_array: &[&str]) -> Option<HashMap<&str, Value>> {
        let map = self.context.get(&self.default_key).unwrap();

        let mut new_map = HashMap::new();

        for &text in text_array.iter() {
            let (key, value) = map.get_key_value(text)?;
            new_map.insert(key.as_str(), Value::JSONValueRef(match value {
                Value::JSONValue(v) => v,
                _ => return None
            }));
        }

        Some(new_map)
    }

    /// Get multiple text from context with a specific key. The output map is usually used for serialization.
    #[cfg(feature = "nightly")]
    pub fn get_multiple_text_with_key(&self, key: &str, text_array: &[&str]) -> Option<HashMap<&str, Value>> {
        let map = match self.context.get(key) {
            Some(m) => m,
            None => self.context.get(&self.default_key).unwrap()
        };

        let mut new_map = HashMap::new();

        for &text in text_array.iter() {
            let (key, value) = map.get_key_value(text)?;
            new_map.insert(key.as_str(), Value::JSONValueRef(match value {
                Value::JSONValue(v) => v,
                _ => return None
            }));
        }

        Some(new_map)
    }

    /// Get filtered text from context by a Regex instance. The output map is usually used for serialization.
    pub fn get_filtered_text(&self, regex: &Regex) -> Option<HashMap<&str, Value>> {
        let map = self.context.get(&self.default_key).unwrap();

        let mut new_map = HashMap::new();

        for (key, value) in map.iter() {
            if !regex.is_match(key) {
                continue;
            }
            new_map.insert(key.as_str(), Value::JSONValueRef(match value {
                Value::JSONValue(v) => v,
                _ => return None
            }));
        }

        Some(new_map)
    }

    /// Get filtered text from context with a specific key by a Regex instance. The output map is usually used for serialization.
    pub fn get_filtered_text_with_key(&self, key: &str, regex: &Regex) -> Option<HashMap<&str, Value>> {
        let map = match self.context.get(key) {
            Some(m) => m,
            None => self.context.get(&self.default_key).unwrap()
        };

        let mut new_map = HashMap::new();

        for (key, value) in map.iter() {
            if !regex.is_match(key) {
                continue;
            }
            new_map.insert(key.as_str(), Value::JSONValueRef(match value {
                Value::JSONValue(v) => v,
                _ => return None
            }));
        }

        Some(new_map)
    }
}

/// Used for including json files into your executable binary file for building a JSONGetText instance.
///
/// ```
/// #[macro_use] extern crate json_gettext;
///
/// let ctx = static_json_gettext_build!("en_US",
///            "en_US", "langs/en_US.json",
///            "zh_TW", "langs/zh_TW.json"
///        ).unwrap();
///
/// println!("{:?}", ctx);
/// ```
#[macro_export]
macro_rules! static_json_gettext_build {
    ( $default_key:expr, $($key:expr, $path:expr), * ) => {
        {
            use ::json_gettext::JSONGetText;

            let mut builder = JSONGetText::build($default_key);

            $(
                {
                    let data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path));

                    builder.add_json_bytes_to_context($key, data).unwrap();
                }
            )*

            builder.build()
        }
    };
}

/// Used for getting single or multiple text from context.
///
/// ```
/// #[macro_use] extern crate json_gettext;
///
/// let ctx = static_json_gettext_build!("en_US",
///            "en_US", "langs/en_US.json",
///            "zh_TW", "langs/zh_TW.json"
///        ).unwrap();
///
/// assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
/// assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
/// ```
#[macro_export]
macro_rules! get_text {
    ( $ctx:ident, $text:expr ) => {
        {
            $ctx.get_text($text)
        }
    };
    ( $ctx:ident, $key:expr, $text:expr ) => {
        {
            $ctx.get_text_with_key($key, $text)
        }
    };
    ( $ctx:ident, $key:expr, $text:expr, $($text_array:expr), + ) => {
        {
            let mut text_array = vec![$text];

            $(
                {
                    text_array.push($text_array);
                }
            )*

            $ctx.get_multiple_text_with_key($key, &text_array)
        }
    };
}