#![cfg(feature = "regex")]

use json_gettext::{regex::Regex, static_json_gettext_build};

mod common;

#[test]
fn filtered_get() {
    let (ref en, ref zh, ref de) = common::keys();
    let ctx = static_json_gettext_build!(
        en;
        en => "langs/en_US.json",
        zh => "langs/zh_TW.json",
    )
    .unwrap();
    let regex = Regex::new("^(hello|rust)$").unwrap();
    let default = ctx.get_filtered_text(&regex).unwrap();

    assert_eq!(2, default.len());
    assert_eq!("Hello, world!", default.get("hello").unwrap());
    assert_eq!("Rust!", default.get("rust").unwrap());

    let translated = ctx.get_filtered_text_with_key(zh, &regex).unwrap();

    assert_eq!(1, translated.len());
    assert_eq!("哈囉，世界！", translated.get("hello").unwrap());
    assert!(!translated.contains_key("rust"));
    assert_eq!(default, ctx.get_filtered_text_with_key(de, &regex).unwrap());
}
