#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate json_gettext;

use rocket::response::Redirect;
use rocket::State;

use json_gettext::JSONGetTextManager;

#[get("/")]
fn index(ctx: State<JSONGetTextManager>) -> Redirect {
    Redirect::temporary(uri!(hello: lang = ctx.get_default_key()))
}

#[get("/<lang>")]
fn hello(ctx: State<JSONGetTextManager>, lang: String) -> String {
    format!("Ron: {}", get_text!(ctx, lang, "hello").unwrap().as_str().unwrap())
}

fn main() {
    rocket::ignite()
        .attach(JSONGetTextManager::fairing(|| {
            static_json_gettext_build_rocketly!(
                "en_US",
                "en_US",
                "langs/en_US.json",
                "zh_TW",
                "langs/zh_TW.json"
            )
        }))
        .mount("/", routes![index, hello])
        .launch();
}
