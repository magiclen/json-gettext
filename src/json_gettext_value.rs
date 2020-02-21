use std::convert::{TryFrom, TryInto};
use std::fmt::{self, Display, Formatter};
#[cfg(feature = "rocketly")]
use std::io::Cursor;

#[cfg(feature = "rocketly")]
use crate::rocket::http::RawStr;
#[cfg(feature = "rocketly")]
use crate::rocket::request::{FromFormValue, FromParam, Request};
#[cfg(feature = "rocketly")]
use crate::rocket::response::{self, Responder, Response};
use crate::serde::de::{Error as DeError, MapAccess, SeqAccess, Visitor};
use crate::serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "rocketly")]
use crate::serde_json::Error as JSONError;
use crate::serde_json::{Map, Value};
use std::str::FromStr;

/// Represents any valid JSON value. Reference can also be wrapped.
#[derive(Debug, Clone, PartialEq)]
pub enum JSONGetTextValue<'a> {
    Str(&'a str),
    JSONValue(Value),
    JSONValueRef(&'a Value),
}

impl<'a> JSONGetTextValue<'a> {
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str> + ?Sized>(s: &'a S) -> JSONGetTextValue<'a> {
        JSONGetTextValue::Str(s.as_ref())
    }

    #[inline]
    pub fn from_string<S: Into<String>>(s: S) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(Value::String(s.into()))
    }

    #[inline]
    pub fn from_bool(b: bool) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(Value::Bool(b))
    }

    #[inline]
    pub fn from_i8(n: i8) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_i16(n: i16) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_i32(n: i32) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_i64(n: i64) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_i128(n: i128) -> Result<JSONGetTextValue<'static>, String> {
        if n > i128::from(i64::max_value()) {
            Err(format!("{} is bigger than {}.", n, i64::max_value()))
        } else if n < i128::from(i64::min_value()) {
            Err(format!("{} is smaller than {}.", n, i64::min_value()))
        } else {
            Ok(JSONGetTextValue::JSONValue(json!(n)))
        }
    }

    #[inline]
    pub fn from_isize(n: isize) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_u8(n: u8) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_u16(n: u16) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_u32(n: u32) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_u64(n: u64) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_u128(n: u128) -> Result<JSONGetTextValue<'static>, String> {
        if n > u128::from(u64::max_value()) {
            Err(format!("{} is bigger than {}.", n, u64::max_value()))
        } else if n < u128::from(u64::min_value()) {
            Err(format!("{} is smaller than {}.", n, u64::min_value()))
        } else {
            Ok(JSONGetTextValue::JSONValue(json!(n)))
        }
    }

    #[inline]
    pub fn from_usize(n: usize) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_f32(n: f32) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_f64(n: f64) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(json!(n))
    }

    #[inline]
    pub fn from_json_value(v: Value) -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(v)
    }

    #[inline]
    pub fn from_json_value_ref(v: &'a Value) -> JSONGetTextValue<'a> {
        JSONGetTextValue::JSONValueRef(v)
    }

    #[inline]
    pub fn null() -> JSONGetTextValue<'static> {
        JSONGetTextValue::JSONValue(Value::Null)
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
            JSONGetTextValue::JSONValue(v) => v.to_string(),
            JSONGetTextValue::JSONValueRef(v) => v.to_string(),
        }
    }

    /// Convert to a string slice if it is possible (if it is a string).
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JSONGetTextValue::Str(s) => Some(s),
            JSONGetTextValue::JSONValue(v) => {
                match v {
                    Value::String(s) => Some(&s),
                    _ => None,
                }
            }
            JSONGetTextValue::JSONValueRef(v) => {
                match v {
                    Value::String(s) => Some(&s),
                    _ => None,
                }
            }
        }
    }

    /// Clone the reference of this `JSONGetTextValue` instance.
    #[inline]
    pub fn clone_borrowed(&'a self) -> JSONGetTextValue<'a> {
        match self {
            JSONGetTextValue::Str(s) => JSONGetTextValue::Str(*s),
            JSONGetTextValue::JSONValue(v) => JSONGetTextValue::JSONValueRef(v),
            JSONGetTextValue::JSONValueRef(v) => JSONGetTextValue::JSONValueRef(*v),
        }
    }
}

impl<'a> Serialize for JSONGetTextValue<'a> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer, {
        match self {
            JSONGetTextValue::Str(s) => s.serialize(serializer),
            JSONGetTextValue::JSONValue(v) => v.serialize(serializer),
            JSONGetTextValue::JSONValueRef(v) => v.serialize(serializer),
        }
    }
}

struct JSONGetTextValueVisitor;

impl<'de> Visitor<'de> for JSONGetTextValueVisitor {
    type Value = JSONGetTextValue<'de>;

    serde_if_integer128! {
        #[inline]
        fn visit_i128<E>(self, v: i128) -> Result<JSONGetTextValue<'static>, E> where E: DeError {
            Ok(JSONGetTextValue::from_i128(v).map_err(DeError::custom)?)
        }
    }

    serde_if_integer128! {
        #[inline]
        fn visit_u128<E>(self, v: u128) -> Result<JSONGetTextValue<'static>, E> where E: DeError {
            Ok(JSONGetTextValue::from_u128(v).map_err(DeError::custom)?)
        }
    }

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a json value")
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<JSONGetTextValue<'static>, E>
        where
            E: DeError, {
        Ok(JSONGetTextValue::from_bool(v))
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<JSONGetTextValue<'static>, E>
        where
            E: DeError, {
        Ok(JSONGetTextValue::from_i64(v))
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<JSONGetTextValue<'static>, E>
        where
            E: DeError, {
        Ok(JSONGetTextValue::from_u64(v))
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<JSONGetTextValue<'static>, E>
        where
            E: DeError, {
        Ok(JSONGetTextValue::from_f64(v))
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<JSONGetTextValue<'static>, E>
        where
            E: DeError, {
        Ok(JSONGetTextValue::from_string(v.to_string()))
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<JSONGetTextValue<'de>, E>
        where
            E: DeError, {
        Ok(JSONGetTextValue::from_str(v))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<JSONGetTextValue<'static>, E>
        where
            E: DeError, {
        Ok(JSONGetTextValue::from_string(v))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<JSONGetTextValue<'static>, E>
        where
            E: DeError, {
        Ok(JSONGetTextValue::null())
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<JSONGetTextValue<'static>, A::Error>
        where
            A: SeqAccess<'de>, {
        let mut v = match seq.size_hint() {
            Some(size) => Vec::with_capacity(size),
            None => Vec::new(),
        };

        while let Some(e) = seq.next_element()? {
            v.push(e);
        }

        Ok(JSONGetTextValue::from_json_value(Value::Array(v)))
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<JSONGetTextValue<'static>, A::Error>
        where
            A: MapAccess<'de>, {
        let mut v = match map.size_hint() {
            Some(size) => Map::with_capacity(size),
            None => Map::new(),
        };

        while let Some((k, e)) = map.next_entry()? {
            v.insert(k, e);
        }

        Ok(JSONGetTextValue::from_json_value(Value::Object(v)))
    }
}

impl<'de> Deserialize<'de> for JSONGetTextValue<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>, {
        deserializer.deserialize_str(JSONGetTextValueVisitor)
    }
}

impl<'a> PartialEq<JSONGetTextValue<'a>> for str {
    #[inline]
    fn eq(&self, other: &JSONGetTextValue) -> bool {
        match other {
            JSONGetTextValue::Str(s) => s.eq(&self),
            JSONGetTextValue::JSONValue(v) => v.eq(self),
            JSONGetTextValue::JSONValueRef(v) => v.eq(&self),
        }
    }
}

impl<'a> PartialEq<JSONGetTextValue<'a>> for &'a str {
    #[inline]
    fn eq(&self, other: &JSONGetTextValue) -> bool {
        match other {
            JSONGetTextValue::Str(s) => s.eq(self),
            JSONGetTextValue::JSONValue(v) => v.eq(self),
            JSONGetTextValue::JSONValueRef(v) => v.eq(self),
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
                    None => v.fmt(f),
                }
            }
            JSONGetTextValue::JSONValueRef(v) => {
                match v.as_str() {
                    Some(s) => s.fmt(f),
                    None => v.fmt(f),
                }
            }
        }
    }
}

impl<'a> From<&'a str> for JSONGetTextValue<'a> {
    #[inline]
    fn from(v: &'a str) -> JSONGetTextValue<'a> {
        JSONGetTextValue::from_str(v)
    }
}

impl From<String> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: String) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_string(v)
    }
}

impl From<bool> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: bool) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_bool(v)
    }
}

impl From<i8> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: i8) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_i8(v)
    }
}

impl From<i16> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: i16) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_i16(v)
    }
}

impl From<i32> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: i32) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_i32(v)
    }
}

impl From<i64> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: i64) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_i64(v)
    }
}

impl From<isize> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: isize) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_isize(v)
    }
}

impl TryFrom<i128> for JSONGetTextValue<'static> {
    type Error = String;

    #[inline]
    fn try_from(v: i128) -> Result<JSONGetTextValue<'static>, String> {
        JSONGetTextValue::from_i128(v)
    }
}

impl From<u8> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: u8) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_u8(v)
    }
}

impl From<u16> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: u16) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_u16(v)
    }
}

impl From<u32> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: u32) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_u32(v)
    }
}

impl From<u64> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: u64) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_u64(v)
    }
}

impl From<usize> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: usize) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_usize(v)
    }
}

impl TryFrom<u128> for JSONGetTextValue<'static> {
    type Error = String;

    #[inline]
    fn try_from(v: u128) -> Result<JSONGetTextValue<'static>, String> {
        JSONGetTextValue::from_u128(v)
    }
}

impl From<f32> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: f32) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_f32(v)
    }
}

impl From<f64> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: f64) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_f64(v)
    }
}

impl From<Value> for JSONGetTextValue<'static> {
    #[inline]
    fn from(v: Value) -> JSONGetTextValue<'static> {
        JSONGetTextValue::from_json_value(v)
    }
}

impl<'a> From<&'a Value> for JSONGetTextValue<'a> {
    #[inline]
    fn from(v: &'a Value) -> JSONGetTextValue<'a> {
        JSONGetTextValue::from_json_value_ref(v)
    }
}

impl<'a, T: Into<JSONGetTextValue<'a>>> From<Option<T>> for JSONGetTextValue<'a> {
    #[inline]
    fn from(v: Option<T>) -> JSONGetTextValue<'a> {
        match v {
            Some(v) => v.into(),
            None => JSONGetTextValue::null(),
        }
    }
}

impl TryFrom<Option<i128>> for JSONGetTextValue<'static> {
    type Error = String;

    #[inline]
    fn try_from(v: Option<i128>) -> Result<JSONGetTextValue<'static>, String> {
        match v {
            Some(v) => v.try_into(),
            None => Ok(JSONGetTextValue::null()),
        }
    }
}

impl TryFrom<Option<u128>> for JSONGetTextValue<'static> {
    type Error = String;

    #[inline]
    fn try_from(v: Option<u128>) -> Result<JSONGetTextValue<'static>, String> {
        match v {
            Some(v) => v.try_into(),
            None => Ok(JSONGetTextValue::null()),
        }
    }
}

impl FromStr for JSONGetTextValue<'static> {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(JSONGetTextValue::from_string(s))
    }
}

#[cfg(feature = "rocketly")]
impl<'a> Responder<'a> for JSONGetTextValue<'a> {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        let mut response = Response::build();

        response
            .raw_header("Content-Type", "application/json")
            .sized_body(Cursor::new(self.to_json()));

        response.ok()
    }
}

#[cfg(feature = "rocketly")]
impl<'a> FromParam<'a> for JSONGetTextValue<'a> {
    type Error = JSONError;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let value: Value = serde_json::from_str(param)?;

        Ok(JSONGetTextValue::from_json_value(value))
    }
}

#[cfg(feature = "rocketly")]
impl<'a> FromFormValue<'a> for JSONGetTextValue<'a> {
    type Error = JSONError;

    fn from_form_value(form_value: &'a RawStr) -> Result<Self, Self::Error> {
        let value: Value = serde_json::from_str(form_value)?;

        Ok(JSONGetTextValue::from_json_value(value))
    }
}
