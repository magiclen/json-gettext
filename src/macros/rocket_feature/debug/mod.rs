#[macro_export]
macro_rules! static_json_gettext_build_for_rocket {
    ( $default_key:expr; $( $key:expr => $path:expr ), * $(,)* ) => {
        $crate::JSONGetTextManager::fairing(|| {
            let mut v = Vec::new();

            $(
                v.push(($key, $path));
            )*

            ($default_key ,v)
        })
    };
}
