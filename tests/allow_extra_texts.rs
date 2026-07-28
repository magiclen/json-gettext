#![cfg(not(feature = "langid"))]

use json_gettext::JSONGetText;

#[test]
fn allow_extra_texts_drops_unknown() {
    let mut builder = JSONGetText::build("en_US");

    builder.allow_extra_texts(true);
    builder.add_json_owned("en_US", r#"{"hello": "Hello, world!"}"#).unwrap();
    builder.add_json_owned("zh_TW", r#"{"hello": "哈囉，世界！", "extra": "額外"}"#).unwrap();

    let ctx = builder.build().unwrap();

    assert_eq!("哈囉，世界！", ctx.get_text_with_key("zh_TW", "hello").unwrap());

    // The text `extra` is not in the default key, so it was dropped while building.
    let map = ctx.get("zh_TW");

    assert!(map.get("hello").is_some());
    assert!(map.get("extra").is_none());
}
