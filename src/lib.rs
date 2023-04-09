/*!
# JSON Get Text

This is a library for getting text from JSON usually for internationalization.

## Example

```rust,ignore
#[macro_use] extern crate json_gettext;

let ctx = static_json_gettext_build!(
    "en_US";
    "en_US" => "langs/en_US.json",
    "zh_TW" => "langs/zh_TW.json"
).unwrap();

assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
```

## Rocket Support

This crate supports the Rocket framework. In order to reload changed json files instead of recompiling the program you have to enable the `rocket` feature for this crate.

```toml
[dependencies.json-gettext]
version = "*"
features = ["rocket"]
```

Then, use the `static_json_gettext_build_for_rocket` macro instead of the `static_json_gettext_build` macro to build a `JSONGetText`(`JSONGetTextManager`).

```rust,ignore
#[macro_use] extern crate json_gettext;

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::response::Redirect;

use json_gettext::JSONGetTextManager;

#[get("/")]
fn index(ctx: &State<JSONGetTextManager>) -> Redirect {
    Redirect::temporary(uri!(hello(lang = ctx.get_default_key())))
}

#[get("/<lang>")]
fn hello(ctx: &State<JSONGetTextManager>, lang: String) -> String {
    format!("Ron: {}", get_text!(ctx, lang, "hello").unwrap().as_str().unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(static_json_gettext_build_for_rocket!(
            "en_US";
            "en_US" => "langs/en_US.json",
            "zh_TW" => "langs/zh_TW.json"
        ))
        .mount("/", routes![index, hello])
}
```

If you are not using the `release` profile, `JSONGetTextManager` can reload the json files automatically if needed.

## `unic-langid` Support

Since string comparison could be slow, the `language_region_pair` feature, the `language` feature or the `region` feature can be enabled to change key's type to `(Language, Option<Region>)`, `Language` or `Region` respectively where `Language` and `Region` structs are in the `unic-langid` crate.

In this case, the `key!` macro would be useful for generating a `Key` instance from a literal string.

For example,

```toml
[dependencies.json-gettext]
version = "*"
features = ["language_region_pair", "rocket"]
```

```rust,ignore
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_accept_language;

#[macro_use]
extern crate json_gettext;

use rocket::State;

use rocket_accept_language::unic_langid::subtags::Language;
use rocket_accept_language::AcceptLanguage;

use json_gettext::{JSONGetTextManager, Key};

const LANGUAGE_EN: Language = language!("en");

#[get("/")]
fn index(ctx: &State<JSONGetTextManager>, accept_language: &AcceptLanguage) -> String {
    let (language, region) = accept_language.get_first_language_region().unwrap_or((LANGUAGE_EN, None));

    format!("Ron: {}", get_text!(ctx, Key(language, region), "hello").unwrap().as_str().unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(static_json_gettext_build_for_rocket!(
            key!("en");
            key!("en") => "langs/en_US.json",
            key!("zh_TW") => "langs/zh_TW.json",
        ))
        .mount("/", routes![index])
}
```
*/

pub extern crate serde_json;

#[cfg(feature = "langid")]
pub extern crate unic_langid;

#[cfg(feature = "langid")]
pub extern crate unic_langid_macros;

#[doc(hidden)]
pub extern crate manifest_dir_macros;

mod json_get_text_build_errors;
mod macros;
mod value;

#[cfg(all(debug_assertions, feature = "rocket"))]
mod mutate;

#[cfg(all(feature = "langid"))]
mod key_copy;

#[cfg(not(feature = "langid"))]
mod key_string;

pub use json_get_text_build_errors::*;
#[cfg(feature = "langid")]
pub use key_copy::*;
#[cfg(not(feature = "langid"))]
pub use key_string::*;
#[cfg(all(debug_assertions, feature = "rocket"))]
use mutate::DebuggableMutate;
#[cfg(any(feature = "language", feature = "region"))]
pub use unic_langid::parser::ParserError;
#[cfg(feature = "language_region_pair")]
pub use unic_langid::LanguageIdentifierError;
pub use value::*;
