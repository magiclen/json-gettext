#![cfg(not(feature = "langid"))]

#[macro_use]
extern crate json_gettext;

use json_gettext::JSONGetTextBuilder;

#[test]
fn single_get() {
    let ctx = static_json_gettext_build!(
        "en_US";
        "en_US" => "langs/en_US.json",
        "zh_TW" => "langs/zh_TW.json",
    )
    .unwrap();

    assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
    assert_eq!("Hello, world!", get_text!(ctx, "en_US", "hello").unwrap());
    assert_eq!("Hello, world!", get_text!(ctx, "de", "hello").unwrap());
    assert_eq!("Rust!", get_text!(ctx, "en_US", "rust").unwrap());
    assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
    assert_eq!("Rust!", get_text!(ctx, "zh_TW", "rust").unwrap());
}

#[test]
fn map_get() {
    let ctx = static_json_gettext_build!(
        "en_US";
        "en_US" => "langs/en_US.json",
        "zh_TW" => "langs/zh_TW.json",
    )
    .unwrap();

    let map = ctx.get("en_US");

    assert_eq!("Hello, world!", map.get("hello").unwrap());
    assert_eq!("Rust!", map.get("rust").unwrap());

    let map = ctx.get("de");

    assert_eq!("Hello, world!", map.get("hello").unwrap());
    assert_eq!("Rust!", map.get("rust").unwrap());

    let map = ctx.get("zh_TW");

    assert_eq!("哈囉，世界！", map.get("hello").unwrap());
    assert_eq!("Rust!", map.get("rust").unwrap());
}

#[test]
fn extra_keys() {
    let mut builder = JSONGetTextBuilder::new("en_US");
    builder.add_json_file("en_US", "langs/en_US.json").unwrap();
    builder.add_json_file("zh_TW", "langs/zh_TW_extra_key.json").unwrap();
    let ctx = builder.build(true).unwrap();

    let map = ctx.get("zh_TW");
    assert_eq!("哈囉，世界！", map.get("hello").unwrap());
    assert_eq!(None, map.get("goodbye"));
}
