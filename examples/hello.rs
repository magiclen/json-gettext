// TODO: Need mutually exclusive features to remove those cfg attributes below.

#[cfg(not(any(feature = "language", feature = "region", feature = "language_region_pair")))]
#[macro_use]
extern crate rocket;

#[cfg(not(any(feature = "language", feature = "region", feature = "language_region_pair")))]
#[macro_use]
extern crate json_gettext;

use std::error::Error;

#[cfg(not(any(feature = "language", feature = "region", feature = "language_region_pair")))]
use rocket::response::Redirect;

#[cfg(not(any(feature = "language", feature = "region", feature = "language_region_pair")))]
use rocket::State;

#[cfg(not(any(feature = "language", feature = "region", feature = "language_region_pair")))]
use json_gettext::JSONGetTextManager;

#[cfg(not(any(feature = "language", feature = "region", feature = "language_region_pair")))]
#[get("/")]
fn index(ctx: &State<JSONGetTextManager>) -> Redirect {
    Redirect::temporary(uri!(hello(lang = ctx.get_default_key())))
}

#[cfg(not(any(feature = "language", feature = "region", feature = "language_region_pair")))]
#[get("/<lang>")]
fn hello(ctx: &State<JSONGetTextManager>, lang: String) -> String {
    format!("Ron: {}", get_text!(ctx, lang, "hello").unwrap().as_str().unwrap())
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(not(any(feature = "language", feature = "region", feature = "language_region_pair")))]
    {
        rocket::build()
            .attach(static_json_gettext_build_for_rocket!(
                "en_US";
                "en_US" => "langs/en_US.json",
                "zh_TW" => "langs/zh_TW.json",
            ))
            .mount("/", routes![index, hello])
            .launch()
            .await?;
    }

    Ok(())
}
