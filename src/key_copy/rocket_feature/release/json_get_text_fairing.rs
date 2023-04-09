extern crate rocket;

use rocket::{
    fairing::{Fairing, Info, Kind},
    Build, Rocket,
};

use super::JSONGetTextManager;
use crate::Key;

const FAIRING_NAME: &str = "JSONGetText";

/// The fairing of `JSONGetTextManager`.
#[allow(clippy::type_complexity)]
pub struct JSONGetTextFairing {
    pub(crate) custom_callback:
        Box<dyn Fn() -> (Key, Vec<(Key, &'static str)>) + Send + Sync + 'static>,
}

#[rocket::async_trait]
impl Fairing for JSONGetTextFairing {
    #[inline]
    fn info(&self) -> Info {
        Info {
            name: FAIRING_NAME, kind: Kind::Ignite
        }
    }

    #[inline]
    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let (default_key, source) = (self.custom_callback)();

        let state = JSONGetTextManager::from_jsons(default_key, source).unwrap();

        Ok(rocket.manage(state))
    }
}
