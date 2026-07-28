JSON Get Text
====================

[![CI](https://github.com/magiclen/json-gettext/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/json-gettext/actions/workflows/ci.yml)

This library reads texts from JSON, usually for internationalization (i18n).

Each **key** is a locale (for example `en_US` or `zh_TW`), and each locale maps **text names** to **values**. A value can be any JSON value, not only a string.

## Example

```rust
use json_gettext::{get_text, static_json_gettext_build};

let ctx = static_json_gettext_build!(
    "en_US";
    "en_US" => "langs/en_US.json",
    "zh_TW" => "langs/zh_TW.json",
)
.unwrap();

assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
```

The argument before `;` is the **default key**. Every other key may only contain text names that also exist in the default key; this is checked when the context is built. To drop such extra texts instead of failing, build with `JSONGetTextBuilder` and call `allow_extra_texts(true)`.

## Looking Up Texts

The lookup methods differ in how they fall back to the default key:

- `get_text(text)` reads a text from the default key.
- `get_text_with_key(key, text)` reads a text from `key`, and falls back to the default key when that single text is missing.
- `get_multiple_text_with_key(key, &[text, ...])` applies the same per-text fallback to many texts at once.
- `get(key)` returns the whole map of texts that `key` owns. Missing texts are **not** filled in from the default key; only when `key` itself does not exist is the default key's map returned instead.

The `get_text!` macro is a shortcut with three forms: `get_text!(ctx, text)`, `get_text!(ctx, key, text)`, and `get_text!(ctx, key, text1, text2, ...)`.

## Features

### Key Backends (mutually exclusive)

By default a key is an arbitrary `String`, matched by exact string comparison. If your keys are locale tags, you can enable one of the features below to store keys as fixed-size `Copy` values from the [`unic-langid`](https://crates.io/crates/unic-langid) crate. This makes key comparison faster and normalizes the tag format, so `en-US` and `en_US` become the same key.

| Feature | `Key` type | Example tag | Use when |
|---------|------------|-------------|----------|
| *(none)* | `Key(String)` | `en_US` | keys are arbitrary strings |
| `locale` | `Key(Language, Option<Region>)` | `en_US` | you key by language **and** region |
| `language` | `Key(Language)` | `en` | you key by language only |
| `region` | `Key(Region)` | `US` | you key by region only |

These three features each redefine `Key`, so **at most one** may be enabled. Enabling more than one, or enabling the internal `langid` feature on its own, fails to compile with a message telling you what to do.

When a langid backend is enabled, use the `key!` macro to build a `Key` from a string literal at compile time:

```rust
use json_gettext::{key, Key};

let key: Key = key!("en_US");
```

### `regex`

Enables `get_filtered_text` and `get_filtered_text_with_key`, which return every text whose **name** matches a [`Regex`](https://crates.io/crates/regex). These methods do not fall back to the default key.

```rust
use json_gettext::{static_json_gettext_build, regex::Regex};

let ctx = static_json_gettext_build!(
    "en_US";
    "en_US" => "langs/en_US.json",
    "zh_TW" => "langs/zh_TW.json",
)
.unwrap();

let re = Regex::new("^h").unwrap();
let map = ctx.get_filtered_text(&re).unwrap();

assert_eq!("Hello, world!", map.get("hello").unwrap());
```

### `rocket`

Integrates with the [Rocket](https://rocket.rs) web framework. See the section below.

## Rocket Support

Enable the `rocket` feature, then build the context with `static_json_gettext_build_for_rocket` instead of `static_json_gettext_build`. It registers a `JSONGetTextManager`, which derefs to `JSONGetText`.

```toml
[dependencies.json-gettext]
version = "*"
features = ["rocket"]
```

In debug builds the manager watches the JSON files and reloads them when they change, so translations can be edited without recompiling. In release builds the JSON is embedded into the binary and is never reloaded.

```rust
#[macro_use]
extern crate rocket;

use json_gettext::{get_text, static_json_gettext_build_for_rocket, JSONGetTextManager};
use rocket::response::Redirect;
use rocket::State;

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

### Rocket with a langid backend

With a langid backend, pass `Key` values (built with `key!`) instead of strings.

```rust
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_accept_language;

use json_gettext::{get_text, key, static_json_gettext_build_for_rocket, JSONGetTextManager, Key};
use rocket::State;
use rocket_accept_language::unic_langid::subtags::Language;
use rocket_accept_language::AcceptLanguage;

const LANGUAGE_EN: Language = language!("en");

#[get("/")]
fn index(ctx: &State<JSONGetTextManager>, accept_language: &AcceptLanguage) -> String {
    let (language, region) =
        accept_language.get_first_language_region().unwrap_or((LANGUAGE_EN, None));

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

## Crates.io

https://crates.io/crates/json-gettext

## Documentation

https://docs.rs/json-gettext

## License

[MIT](LICENSE)
