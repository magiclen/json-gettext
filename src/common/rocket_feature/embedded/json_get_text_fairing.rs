use rocket::{
    Build, Rocket,
    fairing::{Fairing, Info, Kind},
};

use super::JSONGetTextManager;

const FAIRING_NAME: &str = "JSONGetText";

/// The fairing of `JSONGetTextManager`.
#[allow(clippy::type_complexity)]
pub struct JSONGetTextFairing {
    pub(crate) custom_callback:
        Box<dyn Fn() -> (crate::Key, Vec<(crate::Key, &'static str)>) + Send + Sync + 'static>,
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

        match JSONGetTextManager::from_jsons(default_key, source) {
            Ok(state) => Ok(rocket.manage(state)),
            Err(error) => {
                rocket::error!("Failed to load JSONGetText: {error}");

                Err(rocket)
            },
        }
    }
}
