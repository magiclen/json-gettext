#[deprecated(
    since = "3.2.4",
    note = "Please use the `static_json_gettext_build_for_rocket` macro instead."
)]
#[macro_export]
macro_rules! static_json_gettext_build_rocketly {
    ( $default_key:expr $(, $key:expr, $path:expr)* $(,)* ) => {
        $crate::static_json_gettext_build_for_rocket!($default_key $(, $key, $path)*);
    };
}

#[macro_export]
macro_rules! static_json_gettext_build_for_rocket {
    ( $default_key:expr $(, $key:expr, $path:expr)* $(,)* ) => {
        {
            let mut v = Vec::new();

            $(
                v.push(($key, $path));
            )*

            ($default_key ,v)
        }
    };
}
