/*!
# JSON Get Text

This is a library for getting text from JSON usually for internationalization.

## Example

```rust
#[macro_use] extern crate json_gettext;

let ctx = static_json_gettext_build!(
    "en_US",
    "en_US", "langs/en_US.json",
    "zh_TW", "langs/zh_TW.json"
).unwrap();

assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
```

## Rocket Support

This crate supports the Rocket framework. In order to reload changed json files instead of recompiling the program you have to enable the `rocketly` feature for this crate.

```toml
[dependencies.json-gettext]
version = "*"
features = ["rocketly"]
```

Then, use the `static_json_gettext_build_rocketly` macro instead of the `static_json_gettext_build` macro to build a `JSONGetText`(`JSONGetTextManager`).

```rust,ignore
#[macro_use] extern crate json_gettext;

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::response::Redirect;

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
            static_json_gettext_build_rocketly!("en_US",
                "en_US", "langs/en_US.json",
                "zh_TW", "langs/zh_TW.json"
            )
        }))
        .mount("/", routes![index, hello])
        .launch();
}
```

If you are not using the `release` profile, `JSONGetTextManager` can reload the json files automatically if needed.
*/

pub extern crate regex;
pub extern crate serde_json;
#[macro_use]
extern crate serde;
#[cfg(feature = "rocketly")]
extern crate rocket;

#[cfg(all(debug_assertions, feature = "rocketly"))]
mod mutate;
mod json_gettext_value;
mod json_gettext;
mod builder;
#[cfg(feature = "rocketly")]
mod manager;
#[cfg(feature = "rocketly")]
mod fairing;
mod macros;

use std::collections::HashMap;

pub use serde_json::Value;

#[cfg(all(debug_assertions, feature = "rocketly"))]
use mutate::DebuggableMutate;
pub use json_gettext_value::JSONGetTextValue;
pub use json_gettext::JSONGetText;
pub use builder::{JSONGetTextBuilder, JSONGetTextBuildError};
#[cfg(feature = "rocketly")]
pub use manager::JSONGetTextManager;
#[cfg(feature = "rocketly")]
use fairing::JSONGetTextFairing;

pub type Context<'a> = HashMap<String, HashMap<String, JSONGetTextValue<'a>>>;