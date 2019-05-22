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
    #[inline]
    pub fn from_str<S: AsRef<str> + ?Sized>(s: &'a S) -> JSONGetTextValue<'a> {
        JSONGetTextValue::Str(s.as_ref())
    }

    #[inline]
    pub fn from_string<S: Into<String>>(s: S) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::String(s.into()))
    }

    #[inline]
    pub fn from_bool(b: bool) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Bool(b))
    }

    #[inline]
    pub fn from_i8(n: i8) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::I64(n as i64))))
    }

    #[inline]
    pub fn from_i16(n: i16) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::I64(n as i64))))
    }

    #[inline]
    pub fn from_i32(n: i32) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::I64(n as i64))))
    }

    #[inline]
    pub fn from_i64(n: i64) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::I64(n))))
    }

    #[inline]
    pub fn from_u8(n: u8) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::U64(n as u64))))
    }

    #[inline]
    pub fn from_u16(n: u16) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::U64(n as u64))))
    }

    #[inline]
    pub fn from_u32(n: u32) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::U64(n as u64))))
    }

    #[inline]
    pub fn from_u64(n: u64) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::U64(n))))
    }

    #[inline]
    pub fn from_f32(n: f32) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::F64(n as f64))))
    }

    #[inline]
    pub fn from_f64(n: f64) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(serde_json::Value::Number(serde_json::Number::from(serde_json::de::ParserNumber::F64(n))))
    }

    #[inline]
    pub fn from_json_value(v: serde_json::Value) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(v)
    }

    #[inline]
    pub fn from_json_value_ref(v: &'a serde_json::Value) -> JSONGetTextValue<'a> {
        JSONGetTextValue::JSONValueRef(v)
    }

    #[inline]
    pub fn null() -> JSONGetTextValue<'static> {
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
    #[inline]
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
    #[inline]
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
    #[inline]
    fn eq(&self, other: &JSONGetTextValue) -> bool {
        match other {
            JSONGetTextValue::Str(s) => s.eq(&self),
            JSONGetTextValue::JSONValue(v) => v.eq(self),
            JSONGetTextValue::JSONValueRef(v) => v.eq(&self)
        }
    }
}

impl<'a> PartialEq<JSONGetTextValue<'a>> for &'a str {
    #[inline]
    fn eq(&self, other: &JSONGetTextValue) -> bool {
        match other {
            JSONGetTextValue::Str(s) => s.eq(self),
            JSONGetTextValue::JSONValue(v) => v.eq(self),
            JSONGetTextValue::JSONValueRef(v) => v.eq(self)
        }
    }
}

impl<'a> Display for JSONGetTextValue<'a> {
    #[inline]
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