#[macro_use]
extern crate json_gettext;

#[test]
fn single_get() {
    let ctx = static_json_get_text_build!("en_US",
            "en_US", "langs/en_US.json",
            "zh_TW", "langs/zh_TW.json"
        ).unwrap();

    assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
    assert_eq!("Hello, world!", get_text!(ctx, "en_US", "hello").unwrap());
    assert_eq!("Rust!", get_text!(ctx, "en_US", "rust").unwrap());
    assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
    assert_eq!("Rust!", get_text!(ctx, "zh_TW", "rust").unwrap());
}