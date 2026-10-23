#[macro_use]
extern crate json_gettext;

use std::collections::HashMap;

mod common;

#[test]
fn single_get() {
    let (ref en, ref zh, ref de) = common::keys();
    let ctx = static_json_gettext_build!(
        en;
        en => "langs/en_US.json",
        zh => "langs/zh_TW.json",
    )
    .unwrap();

    assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
    assert_eq!("Hello, world!", get_text!(ctx, en, "hello").unwrap());
    assert_eq!("Hello, world!", get_text!(ctx, de, "hello").unwrap());
    assert_eq!("Rust!", get_text!(ctx, en, "rust").unwrap());
    assert_eq!("哈囉，世界！", get_text!(ctx, zh, "hello").unwrap());
    assert_eq!("Rust!", get_text!(ctx, zh, "rust").unwrap());
}

#[test]
fn map_get() {
    let (ref en, ref zh, ref de) = common::keys();
    let ctx = static_json_gettext_build!(
        en;
        en => "langs/en_US.json",
        zh => "langs/zh_TW.json",
    )
    .unwrap();

    let map = ctx.get(en);

    assert_eq!("Hello, world!", map.get("hello").unwrap());
    assert_eq!("Rust!", map.get("rust").unwrap());

    // An absent key falls back to the default key's map.
    let map = ctx.get(de);

    assert_eq!("Hello, world!", map.get("hello").unwrap());
    assert_eq!("Rust!", map.get("rust").unwrap());

    // `get` returns only the texts the key actually owns; `rust` is not filled in from the default key.
    let map = ctx.get(zh);

    assert_eq!("哈囉，世界！", map.get("hello").unwrap());
    assert!(!map.contains_key("rust"));
}

#[test]
fn json_values() {
    let (ref en, ..) = common::keys();
    let ctx = static_json_gettext_build!(
        en;
        en => "tests/data/values.json",
    )
    .unwrap();
    let expected: HashMap<String, serde_json::Value> =
        serde_json::from_str(include_str!("data/values.json")).unwrap();

    for (name, value) in expected {
        assert_eq!(value, serde_json::to_value(ctx.get_text(name).unwrap()).unwrap());
    }

    assert!(matches!(ctx.get_text("plain"), Some(json_gettext::JSONGetTextValue::Str("Hello"))));
}

#[cfg(not(feature = "langid"))]
#[test]
fn literal_key() {
    assert_eq!("{{tenant}}", key!("{{tenant}}").as_str());
    assert_eq!("{tenant}", key!("{tenant}").as_str());
}
