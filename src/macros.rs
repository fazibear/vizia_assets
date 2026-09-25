#[allow(unused_macros)]
macro_rules! svg {
    ($cx:ident, $name:ident, $path:literal) => {
        $cx.load_svg(
            stringify!($name),
            include_bytes!($path),
            ImageRetentionPolicy::Forever,
        );
    };
}

#[allow(unused_macros)]
macro_rules! png {
    ($cx:ident, $name:ident, $path:literal) => {
        $cx.load_image(
            stringify!($name),
            include_bytes!($path),
            ImageRetentionPolicy::Forever,
        );
    };
}

#[allow(unused_macros)]
macro_rules! css {
    ($cx:ident, $name:ident, $path:literal) => {{
        #[cfg(debug_assertions)]
        {
            $cx.add_stylesheet(CSS::from_file($path))
        }
        #[cfg(not(debug_assertions))]
        {
            $cx.add_stylesheet(CSS::from_string(include_str!($path)))
        }
    }};
}
