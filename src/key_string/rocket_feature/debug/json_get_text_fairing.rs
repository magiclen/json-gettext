extern crate rocket;

use super::JSONGetTextManager;

use rocket::data::Data;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::Request;
use rocket::{Build, Rocket};

const FAIRING_NAME: &str = "JSONGetText (Debug)";

/// The fairing of `JSONGetTextManager`.
#[allow(clippy::type_complexity)]
pub struct JSONGetTextFairing {
    pub(crate) custom_callback:
        Box<dyn Fn() -> (&'static str, Vec<(&'static str, &'static str)>) + Send + Sync + 'static>,
}

#[rocket::async_trait]
impl Fairing for JSONGetTextFairing {
    #[inline]
    fn info(&self) -> Info {
        Info {
            name: FAIRING_NAME,
            kind: Kind::Ignite | Kind::Request,
        }
    }

    #[inline]
    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let (default_key, source) = (self.custom_callback)();

        let state = JSONGetTextManager::from_files(default_key, source).unwrap();

        Ok(rocket.manage(state))
    }

    #[inline]
    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let ctx = req
            .rocket()
            .state::<JSONGetTextManager>()
            .expect("JSONGetTextManager registered in on_attach");

        ctx.reload_if_needed().unwrap();
    }
}
