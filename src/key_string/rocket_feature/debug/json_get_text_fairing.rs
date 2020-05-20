extern crate rocket;

use super::JSONGetTextManager;

use rocket::data::Data;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::Request;
use rocket::{Rocket, State};

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
            kind: Kind::Attach | Kind::Request,
        }
    }

    #[inline]
    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let (default_key, source) = (self.custom_callback)();

        let state = JSONGetTextManager::from_files(default_key, source).unwrap();

        Ok(rocket.manage(state))
    }

    #[inline]
    fn on_request(&self, req: &mut Request, _data: &Data) {
        let ctx = req
            .guard::<State<JSONGetTextManager>>()
            .expect("JSONGetTextManager registered in on_attach");

        ctx.reload_if_needed().unwrap();
    }
}
