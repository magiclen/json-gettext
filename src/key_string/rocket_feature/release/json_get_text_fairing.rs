extern crate rocket;

use super::JSONGetTextManager;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;

const FAIRING_NAME: &str = "JSONGetText";

/// The fairing of `JSONGetTextManager`.
#[allow(clippy::type_complexity)]
pub struct JSONGetTextFairing {
    pub(crate) custom_callback:
        Box<dyn Fn() -> (&'static str, Vec<(&'static str, &'static str)>) + Send + Sync + 'static>,
}

impl Fairing for JSONGetTextFairing {
    #[inline]
    fn info(&self) -> Info {
        Info {
            name: FAIRING_NAME,
            kind: Kind::Attach,
        }
    }

    #[inline]
    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let (default_key, source) = (self.custom_callback)();

        let state = JSONGetTextManager::from_jsons(default_key, source).unwrap();

        Ok(rocket.manage(state))
    }
}
