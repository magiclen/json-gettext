#[cfg(debug_assertions)]
use crate::rocket::data::Data;
use crate::rocket::fairing::{Fairing, Info, Kind};
#[cfg(debug_assertions)]
use crate::rocket::request::Request;
use crate::rocket::Rocket;
#[cfg(debug_assertions)]
use crate::rocket::State;

use crate::JSONGetTextManager;

const FAIRING_NAME: &str = "JSONGetText";

/// The fairing of `JSONGetTextManager`.
#[allow(clippy::type_complexity)]
pub struct JSONGetTextFairing {
    pub(crate) custom_callback:
        Box<dyn Fn() -> (&'static str, Vec<(&'static str, &'static str)>) + Send + Sync + 'static>,
}

impl Fairing for JSONGetTextFairing {
    #[cfg(debug_assertions)]
    fn info(&self) -> Info {
        Info {
            name: FAIRING_NAME,
            kind: Kind::Attach | Kind::Request,
        }
    }

    #[cfg(not(debug_assertions))]
    fn info(&self) -> Info {
        Info {
            name: FAIRING_NAME,
            kind: Kind::Attach,
        }
    }

    #[cfg(debug_assertions)]
    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let (default_key, source) = (self.custom_callback)();

        let state = JSONGetTextManager::from_files(default_key, source).unwrap();

        Ok(rocket.manage(state))
    }

    #[cfg(not(debug_assertions))]
    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let (default_key, source) = (self.custom_callback)();

        let state = JSONGetTextManager::from_jsons(default_key, source).unwrap();

        Ok(rocket.manage(state))
    }

    #[cfg(debug_assertions)]
    fn on_request(&self, req: &mut Request, _data: &Data) {
        let ctx = req
            .guard::<State<JSONGetTextManager>>()
            .expect("JSONGetTextManager registered in on_attach");

        ctx.reload_if_needed().unwrap();
    }
}
