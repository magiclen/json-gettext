JSON Get Text
====================

[![Build Status](https://travis-ci.org/magiclen/json-gettext.svg?branch=master)](https://travis-ci.org/magiclen/json-gettext)
[![Build status](https://ci.appveyor.com/api/projects/status/s62mpv7x0y54wycy/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/json-gettext/branch/master)

This is a library for getting text from JSON usually for internationalization.

## Sample Code

```rust
#[macro_use]
extern crate json_gettext;

let ctx = static_json_gettext_build!("en_US", 
            "en_US", "langs/en_US.json",
            "zh_TW", "langs/zh_TW.json"
        ).unwrap();

assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
```

In order to reduce the compilation time, the `static_json_gettext_build` macro has files compiled into your executable binary file together, only when you are using the **release** profile.

## Crates.io

https://crates.io/crates/json-gettext

## Documentation

https://docs.rs/json-gettext

## License

[MIT](LICENSE)