/// Used for including json files into your executable binary file for building a JSONGetText instance.
///
/// ```
/// #[macro_use] extern crate json_gettext;
///
/// let ctx = static_json_gettext_build!("en_US",
///         "en_US", "langs/en_US.json",
///         "zh_TW", "langs/zh_TW.json"
///     ).unwrap();
///
/// println!("{:?}", ctx);
/// ```
#[macro_export]
macro_rules! static_json_gettext_build {
    ( $default_key:expr, $($key:expr, $path:expr), * ) => {
        {
            use crate::json_gettext::JSONGetText;

            let mut builder = JSONGetText::build($default_key);

            $(
                builder.add_json($key, include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))).unwrap();
            )*

            builder.build()
        }
    };
}

/// Used for getting single or multiple text from context.
///
/// ```
/// #[macro_use] extern crate json_gettext;
///
/// let ctx = static_json_gettext_build!("en_US",
///         "en_US", "langs/en_US.json",
///         "zh_TW", "langs/zh_TW.json"
///     ).unwrap();
///
/// assert_eq!("Hello, world!", get_text!(ctx, "hello").unwrap());
/// assert_eq!("哈囉，世界！", get_text!(ctx, "zh_TW", "hello").unwrap());
/// ```
#[macro_export]
macro_rules! get_text {
    ( $ctx:ident, $text:expr ) => {
        {
            $ctx.get_text($text)
        }
    };
    ( $ctx:ident, $key:expr, $text:expr ) => {
        {
            $ctx.get_text_with_key($key, $text)
        }
    };
    ( $ctx:ident, $key:expr, $text:expr, $($text_array:expr), + ) => {
        {
            let mut text_array = vec![$text];

            $(
                {
                    text_array.push($text_array);
                }
            )*

            $ctx.get_multiple_text_with_key($key, &text_array)
        }
    };
}