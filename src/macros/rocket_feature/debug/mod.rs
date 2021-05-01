#[macro_export]
macro_rules! static_json_gettext_build_for_rocket {
    ( $default_key:expr; $( $key:expr => $path:expr ), * $(,)* ) => {
        $crate::JSONGetTextManager::fairing(|| {
            let mut v = Vec::new();

            $(
                v.push(($key, $crate::slash_formatter::concat_with_file_separator_debug_release!(env!("CARGO_MANIFEST_DIR"), $path)));
            )*

            ($default_key ,v)
        })
    };
}
