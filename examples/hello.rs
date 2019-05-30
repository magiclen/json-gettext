#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate json_gettext;

use rocket::State;
use rocket::response::Redirect;

// For enabling the `rocketly` feature

#[cfg(feature = "rocketly")]
use json_gettext::JSONGetTextManager;

#[cfg(feature = "rocketly")]
#[get("/")]
fn index(ctx: State<JSONGetTextManager>) -> Redirect {
    Redirect::temporary(uri!(hello: lang = ctx.get_default_key()))
}

#[cfg(feature = "rocketly")]
#[get("/<lang>")]
fn hello(ctx: State<JSONGetTextManager>, lang: String) -> String {
    format!("Ron: {}", get_text!(ctx, lang, "hello").unwrap().as_str().unwrap())
}

#[cfg(feature = "rocketly")]
fn main() {
    rocket::ignite()
        .attach(JSONGetTextManager::fairing(|| {
            static_json_gettext_build_rocketly!("en_US",
                "en_US", "langs/en_US.json",
                "zh_TW", "langs/zh_TW.json"
            )
        }))
        .mount("/", routes![index, hello])
        .launch();
}

// For not enabling the `rocketly` feature

#[cfg(not(feature = "rocketly"))]
use json_gettext::JSONGetText;

#[cfg(not(feature = "rocketly"))]
#[get("/")]
fn index(ctx: State<JSONGetText>) -> Redirect {
    Redirect::temporary(uri!(hello: lang = ctx.get_default_key()))
}

#[cfg(not(feature = "rocketly"))]
#[get("/<lang>")]
fn hello(ctx: State<JSONGetText>, lang: String) -> String {
    format!("Ron: {}", get_text!(ctx, lang, "hello").unwrap().as_str().unwrap())
}

#[cfg(not(feature = "rocketly"))]
fn main() {
    let ctx = static_json_gettext_build!("en_US",
        "en_US", "langs/en_US.json",
        "zh_TW", "langs/zh_TW.json"
    ).unwrap();

    rocket::ignite()
        .manage(ctx)
        .mount("/", routes![index, hello])
        .launch();
}