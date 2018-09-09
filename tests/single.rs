#[macro_use]
extern crate lazy_static_include;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate json_gettext;

#[test]
fn single_get() {
    let ctx = static_json_gettext_build!("en_US",
            "en_US", "langs/en_US.json",
            "zh_TW", "langs/zh_TW.json"
        ).unwrap();

    assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
    assert_eq!("Hello, world!", get_text!(ctx, "en_US", "hello").unwrap());
    assert_eq!("Hello, world!", get_text!(ctx, "de", "hello").unwrap());
    assert_eq!("Rust!", get_text!(ctx, "en_US", "rust").unwrap());
    assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
    assert_eq!("Rust!", get_text!(ctx, "zh_TW", "rust").unwrap());
}

#[test]
fn map_get() {
    let ctx = static_json_gettext_build!("en_US",
            "en_US", "langs/en_US.json",
            "zh_TW", "langs/zh_TW.json"
        ).unwrap();

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