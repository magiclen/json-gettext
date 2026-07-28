#[macro_export]
macro_rules! static_json_gettext_build_for_rocket {
    ( $default_key:expr; $( $key:expr => $path:expr ), * $(,)* ) => {
        $crate::JSONGetTextManager::fairing(|| {
            let mut v = Vec::new();

            $(
                v.push((
                    $crate::IntoKey::into_key($key),
                    $crate::manifest_dir_macros::not_directory_path!($path),
                ));
            )*

            ($crate::IntoKey::into_key($default_key), v)
        })
    };
}
