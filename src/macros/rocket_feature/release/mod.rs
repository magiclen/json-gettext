#[macro_export]
macro_rules! static_json_gettext_build_rocketly {
    ( $default_key:expr, $($key:expr, $path:expr), * ) => {
        {
            let mut v = Vec::new();

            $(
                v.push(($key, include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))));
            )*

            ($default_key ,v)
        }
    };
}
