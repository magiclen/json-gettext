#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_accept_language;

#[macro_use]
extern crate json_gettext;

use std::error::Error;

use rocket::State;

use rocket_accept_language::unic_langid::subtags::Language;
use rocket_accept_language::AcceptLanguage;

use json_gettext::{JSONGetTextManager, Key};

const LANGUAGE_EN: Language = language!("en");

#[get("/")]
fn index(ctx: State<JSONGetTextManager>, accept_language: &AcceptLanguage) -> String {
    let (language, region) =
        accept_language.get_first_language_region().unwrap_or((LANGUAGE_EN, None));

    format!("Ron: {}", get_text!(ctx, Key(language, region), "hello").unwrap().as_str().unwrap())
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build()
        .attach(static_json_gettext_build_for_rocket!(
            key!("en");
            key!("en") => "langs/en_US.json",
            key!("zh_TW") => "langs/zh_TW.json",
        ))
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
