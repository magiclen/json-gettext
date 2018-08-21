JSON Get Text
====================

[![Build Status](https://travis-ci.org/magiclen/json-gettext.svg?branch=master)](https://travis-ci.org/magiclen/json-gettext)

This is a library for getting text from JSON usually for internationalization.

## Sample Code

```rust
#[macro_use]
extern crate json_gettext;

let ctx = static_json_get_text_build!("en_US", 
            "en_US", "langs/en_US.json",
            "zh_TW", "langs/zh_TW.json"
        ).unwrap();

assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
```

## Crates.io

https://crates.io/crates/json-gettext

## Documentation

https://docs.rs/json-gettext

## License

[MIT](LICENSE)