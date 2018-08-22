#![cfg(feature = "nightly")]

#[macro_use]
extern crate json_gettext;

extern crate serde_json;

use std::collections::HashMap;

use json_gettext::Value;

#[test]
fn mutiple_get() {
    let ctx = static_json_gettext_build!("en_US",
            "en_US", "langs/en_US.json",
            "zh_TW", "langs/zh_TW.json"
        ).unwrap();

    let map_en: HashMap<&str, Value> = get_text!(ctx, "en_US", "hello", "rust").unwrap();

    assert_eq!(&"Hello, world!", map_en.get("hello").unwrap());
    assert_eq!(&"Rust!", map_en.get("rust").unwrap());

    let map_de: HashMap<&str, Value> = get_text!(ctx, "de", "hello", "rust").unwrap();

    assert_eq!(&"Hello, world!", map_de.get("hello").unwrap());
    assert_eq!(&"Rust!", map_de.get("rust").unwrap());

    let map_zh: HashMap<&str, Value> = get_text!(ctx, "zh_TW", "hello", "rust").unwrap();

    assert_eq!(&"哈囉，世界！", map_zh.get("hello").unwrap());
    assert_eq!(&"Rust!", map_zh.get("rust").unwrap());
}