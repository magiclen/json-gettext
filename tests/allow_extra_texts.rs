use json_gettext::JSONGetText;

mod common;

#[test]
fn allow_extra_texts_drops_unknown() {
    let (ref en, ref zh, _) = common::keys();
    let mut builder = JSONGetText::build(en);

    builder.allow_extra_texts(true);
    builder.add_json_owned(en, r#"{"hello": "Hello, world!"}"#).unwrap();
    builder.add_json_owned(zh, r#"{"hello": "哈囉，世界！", "extra": "額外"}"#).unwrap();

    let ctx = builder.build().unwrap();

    assert_eq!("哈囉，世界！", ctx.get_text_with_key(zh, "hello").unwrap());

    // The text `extra` is not in the default key, so it was dropped while building.
    let map = ctx.get(zh);

    assert!(map.contains_key("hello"));
    assert!(!map.contains_key("extra"));
}
