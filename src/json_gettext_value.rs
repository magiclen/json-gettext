use std::fmt::{self, Display, Formatter};

use serde::{Serialize, Serializer};
use serde_json::Value;

/// Represents any valid JSON value. Reference can also be wrapped.
#[derive(Debug, Clone, PartialEq)]
pub enum JSONGetTextValue<'a> {
    Str(&'a str),
    JSONValue(Value),
    JSONValueRef(&'a Value),
}

impl<'a> JSONGetTextValue<'a> {
    pub fn from_str(s: &'a str) -> JSONGetTextValue<'a> {
        JSONGetTextValue::Str(s)
    }

    pub fn from_string(s: String) -> JSONGetTextValue<'a> {
        JSONGetTextValue::JSONValue(serde_json::Value::String(s))
    }

    pub fn from_bool(b: bool) -> JSONGetTextValue<'a> {
        JSONGetTextValue::JSONValue(serde_json::Value::Bool(b))
    }

    pub fn from_i64(n: i64) -> JSONGetTextValue<'a> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::I64(n))))
    }

    pub fn from_u64(n: u64) -> JSONGetTextValue<'a> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::U64(n))))
    }

    pub fn from_f64(n: f64) -> JSONGetTextValue<'a> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::F64(n))))
    }

    pub fn from_json_value(v: serde_json::Value) -> JSONGetTextValue<'a> {
        JSONGetTextValue::JSONValue(v)
    }

    pub fn from_json_value_ref(v: &'a serde_json::Value) -> JSONGetTextValue<'a> {
        JSONGetTextValue::JSONValueRef(v)
    }

    pub fn null() -> JSONGetTextValue<'a> {
        JSONGetTextValue::JSONValue(serde_json::Value::Null)
    }

    /// Convert to a string for JSON format.
    pub fn to_json(&self) -> String {
        match self {
            JSONGetTextValue::Str(s) => {
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
            JSONGetTextValue::JSONValue(v) => {
                v.to_string()
            }
            JSONGetTextValue::JSONValueRef(v) => {
                v.to_string()
            }
        }
    }

    /// Convert to a string slice if it is possible (if it is a string).
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JSONGetTextValue::Str(s) => {
                Some(s)
            }
            JSONGetTextValue::JSONValue(v) => {
                match v {
                    serde_json::Value::String(s) => Some(&s),
                    _ => None
                }
            }
            JSONGetTextValue::JSONValueRef(v) => {
                match v {
                    serde_json::Value::String(s) => Some(&s),
                    _ => None
                }
            }
        }
    }
}

impl<'a> Serialize for JSONGetTextValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        match self {
            JSONGetTextValue::Str(s) => s.serialize(serializer),
            JSONGetTextValue::JSONValue(v) => v.serialize(serializer),
            JSONGetTextValue::JSONValueRef(v) => v.serialize(serializer)
        }
    }
}

impl<'a> PartialEq<JSONGetTextValue<'a>> for str {
    fn eq(&self, other: &JSONGetTextValue) -> bool {
        match other {
            JSONGetTextValue::Str(s) => s.eq(&self),
            JSONGetTextValue::JSONValue(v) => v.eq(self),
            JSONGetTextValue::JSONValueRef(v) => v.eq(&self)
        }
    }
}

impl<'a> PartialEq<JSONGetTextValue<'a>> for &'a str {
    fn eq(&self, other: &JSONGetTextValue) -> bool {
        match other {
            JSONGetTextValue::Str(s) => s.eq(self),
            JSONGetTextValue::JSONValue(v) => v.eq(self),
            JSONGetTextValue::JSONValueRef(v) => v.eq(self)
        }
    }
}

impl<'a> Display for JSONGetTextValue<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            JSONGetTextValue::Str(s) => s.fmt(f),
            JSONGetTextValue::JSONValue(v) => {
                match v.as_str() {
                    Some(s) => s.fmt(f),
                    None => v.fmt(f)
                }
            }
            JSONGetTextValue::JSONValueRef(v) => {
                match v.as_str() {
                    Some(s) => s.fmt(f),
                    None => v.fmt(f)
                }
            }
        }
    }
}