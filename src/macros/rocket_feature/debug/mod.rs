#[macro_export]
macro_rules! static_json_gettext_build_rocketly {
    ( $default_key:expr, $($key:expr, $path:expr), * ) => {
        {
            let mut v = Vec::new();

            $(
                v.push(($key, $path));
            )*

            ($default_key ,v)
        }
    };
}
