#[macro_use]
extern crate json_gettext;

use std::collections::BTreeMap;

use json_gettext::JSONGetTextValue;

mod common;

#[test]
fn multiple_get() {
    let (ref en, ref zh, ref de) = common::keys();
    let ctx = static_json_gettext_build!(
        en;
        en => "langs/en_US.json",
        zh => "langs/zh_TW.json",
    )
    .unwrap();

    let map_en: BTreeMap<&str, JSONGetTextValue> = get_text!(ctx, en, "hello", "rust").unwrap();

    assert_eq!(&"Hello, world!", map_en.get("hello").unwrap());
    assert_eq!(&"Rust!", map_en.get("rust").unwrap());

    let map_de: BTreeMap<&str, JSONGetTextValue> = get_text!(ctx, de, "hello", "rust").unwrap();

    assert_eq!(&"Hello, world!", map_de.get("hello").unwrap());
    assert_eq!(&"Rust!", map_de.get("rust").unwrap());

    let map_zh: BTreeMap<&str, JSONGetTextValue> = get_text!(ctx, zh, "hello", "rust").unwrap();

    assert_eq!(&"哈囉，世界！", map_zh.get("hello").unwrap());
    assert_eq!(&"Rust!", map_zh.get("rust").unwrap());
}
