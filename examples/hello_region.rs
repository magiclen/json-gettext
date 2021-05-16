#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_accept_language;

#[macro_use]
extern crate json_gettext;

use std::error::Error;

use rocket::State;

use rocket_accept_language::unic_langid::subtags::Region;
use rocket_accept_language::AcceptLanguage;

use json_gettext::{JSONGetTextManager, Key};

const REGION_US: Region = region!("us");

#[get("/")]
fn index(ctx: &State<JSONGetTextManager>, accept_language: &AcceptLanguage) -> String {
    let region = accept_language.get_first_region().unwrap_or(REGION_US);

    format!("Ron: {}", get_text!(ctx, Key(region), "hello").unwrap().as_str().unwrap())
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build()
        .attach(static_json_gettext_build_for_rocket!(
            key!("us");
            key!("us") => "langs/en_US.json",
            key!("tw") => "langs/zh_TW.json",
        ))
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
