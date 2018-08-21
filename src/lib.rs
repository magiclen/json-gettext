#![cfg_attr(feature = "nightly", feature(map_get_key_value))]

extern crate serde_json;

use std::collections::{HashMap, hash_map::Keys};
use std::path::Path;
use std::fs::File;

type Context = HashMap<String, HashMap<String, String>>;

/// To build a JSONGetText instance, this struct can help you do that step by step.
#[derive(Debug)]
pub struct JSONGetTextBuilder {
    default_key: String,
    context: Context,
}

impl JSONGetTextBuilder {
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

        let map: HashMap<String, String> = serde_json::from_str(json).map_err(|err| { err.to_string() })?;

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Add JSON binary data to the context for a specify key. The JSON binary data must represent an object (key-value).
    pub fn add_json_bytes_to_context(&mut self, key: &str, json: &[u8]) -> Result<&Self, String> {
        if self.context.contains_key(key) {
            return Err("The key exists.".to_string());
        }

        let map: HashMap<String, String> = serde_json::from_slice(json).map_err(|err| { err.to_string() })?;

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Add JSON binary data from a file to the context for a specify key. The JSON binary data must represent an object (key-value).
    pub fn add_json_file_to_context<P: AsRef<Path>>(&mut self, key: &str, path: P) -> Result<&Self, String> {
        if self.context.contains_key(key) {
            return Err("The key exists.".to_string());
        }

        let file = File::open(path).map_err(|err| { err.to_string() })?;

        let map: HashMap<String, String> = serde_json::from_reader(&file).map_err(|err| { err.to_string() })?;

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Add a map to the context.
    pub fn add_map_to_context<P: AsRef<Path>>(&mut self, key: &str, map: HashMap<String, String>) -> Result<&Self, String> {
        if self.context.contains_key(key) {
            return Err("The key exists.".to_string());
        }

        self.context.insert(key.to_string(), map);

        Ok(self)
    }

    /// Build a JSONGetText instance.
    pub fn build(self) -> Result<JSONGetText, String> {
        JSONGetText::from_context_inner(self.default_key, self.context)
    }
}

/// A wrapper for context and a default key. **Keys** are usually considered as locales.
#[derive(Debug)]
pub struct JSONGetText {
    default_key: String,
    context: Context,
}

impl JSONGetText {
    /// Create a new JSONGetTextBuilder instance. You need to decide your default key at the stage.
    pub fn build(default_key: &str) -> JSONGetTextBuilder {
        JSONGetTextBuilder::new(default_key)
    }

    /// Create a new JSONGetText instance with context and a default key.
    pub fn from_context(default_key: &str, context: Context) -> Result<JSONGetText, String> {
        JSONGetText::from_context_inner(default_key.to_string(), context)
    }

    fn from_context_inner(default_key: String, mut context: Context) -> Result<JSONGetText, String> {
        if !context.contains_key(&default_key) {
            return Err("Cannot find the default key in the context.".to_string());
        }

        if !context.contains_key(&default_key) {
            return Err("Context should contain the default key.".to_string());
        }

        {
            let default_map = context.remove(&default_key).unwrap();

            for (key, map) in context.iter_mut() {
                {
                    let map_keys = map.keys();

                    for map_key in map_keys {
                        if !default_map.contains_key(map_key) {
                            return Err(format! {"The text `{}` in the key `{}` is not in the map of the default key.", map_key, key});
                        }
                    }
                }

                let map_keys = default_map.keys();

                for map_key in map_keys {
                    if !map.contains_key(map_key) {
                        map.insert(map_key.clone(), default_map.get(map_key).unwrap().clone());
                    }
                }
            }

            context.insert(default_key.clone(), default_map);
        }

        Ok(JSONGetText {
            default_key: default_key,
            context,
        })
    }

    /// Get all keys in context.
    pub fn get_keys(&self) -> Keys<String, HashMap<String, String>> {
        self.context.keys()
    }

    /// Get the default key.
    pub fn get_default_key(&self) -> &str {
        &self.default_key
    }

    /// Get a string map from context by a key.
    pub fn get(&self, key: &str) -> Option<&HashMap<String, String>> {
        self.context.get(key)
    }

    /// Get text from context.
    pub fn get_text(&self, text: &str) -> Option<&str> {
        self.get_text_with_key(&self.default_key, text)
    }

    /// Get text from context with a specific key.
    pub fn get_text_with_key(&self, key: &str, text: &str) -> Option<&str> {
        self.context.get(key)?.get(text).map(|s| s.as_str())
    }

    /// Get multiple text from context. The output map is usually be used for serialization.
    #[cfg(feature = "nightly")]
    pub fn get_multiple_text(&self, text_array: &[&str]) -> Option<HashMap<&str, &str>> {
        self.get_multiple_text_with_key(&self.default_key, text_array)
    }

    /// Get multiple text from context with a specific key. The output map is usually be used for serialization.
    #[cfg(feature = "nightly")]
    pub fn get_multiple_text_with_key(&self, key: &str, text_array: &[&str]) -> Option<HashMap<&str, &str>> {
        let map = self.context.get(key)?;

        let mut new_map = HashMap::new();

        for &text in text_array.iter() {
            let (key, value) = map.get_key_value(text)?;
            new_map.insert(key.as_str(), value.as_str());
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
            use self::json_gettext::JSONGetText;

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