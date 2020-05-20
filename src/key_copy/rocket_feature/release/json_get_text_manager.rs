extern crate rocket;

use std::ops::Deref;

use crate::{JSONGetText, JSONGetTextBuildError, JSONGetTextBuilder, JSONGetTextFairing, Key};

use rocket::fairing::Fairing;

#[derive(Debug)]
pub struct JSONGetTextManager {
    json_gettext: JSONGetText<'static>,
}

impl JSONGetTextManager {
    #[inline]
    pub fn from_jsons(
        default_key: Key,
        source: Vec<(Key, &'static str)>,
    ) -> Result<JSONGetTextManager, JSONGetTextBuildError> {
        let mut builder = JSONGetTextBuilder::new(default_key);

        for (key, json) in source {
            builder.add_json(key, json)?;
        }

        Ok(JSONGetTextManager {
            json_gettext: builder.build()?,
        })
    }
}

impl JSONGetTextManager {
    /// Create the fairing of `JSONGetTextManager`.
    pub fn fairing<F>(f: F) -> impl Fairing
    where
        F: Fn() -> (Key, Vec<(Key, &'static str)>) + Send + Sync + 'static, {
        JSONGetTextFairing {
            custom_callback: Box::new(f),
        }
    }
}

impl<'a> Deref for JSONGetTextManager {
    type Target = JSONGetText<'static>;

    #[inline]
    fn deref(&self) -> &JSONGetText<'static> {
        &self.json_gettext
    }
}
