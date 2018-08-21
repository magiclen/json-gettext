#![cfg(feature = "nightly")]

#[macro_use]
extern crate json_gettext;

use std::collections::HashMap;

#[test]
fn mutiple_get() {
    let ctx = static_json_gettext_build!("en_US",
            "en_US", "langs/en_US.json",
            "zh_TW", "langs/zh_TW.json"
        ).unwrap();

    let map_en: HashMap<&str, &str> = get_text!(ctx, "en_US", "hello", "rust").unwrap();

    assert_eq!(&"Hello, world!", map_en.get("hello").unwrap());
    assert_eq!(&"Rust!", map_en.get("rust").unwrap());

    let map_zh: HashMap<&str, &str> = get_text!(ctx, "zh_TW", "hello", "rust").unwrap();

    assert_eq!(&"哈囉，世界！", map_zh.get("hello").unwrap());
    assert_eq!(&"Rust!", map_zh.get("rust").unwrap());
}