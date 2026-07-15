#[macro_export]
macro_rules! target_os_cfg {
    ($os:expr, $($tt:tt)*) => {
        #[cfg(target_os = $os)]
        {
            $($tt)*
        }
    };
}

#[macro_export]
macro_rules! platform_cfg {
    (windows, $($tt:tt)*) => {
        target_os_cfg!("windows", $($tt)*);
    };
    (linux, $($tt:tt)*) => {
        target_os_cfg!("linux", $($tt)*);
    };
    (macos, $($tt:tt)*) => {
        target_os_cfg!("macos", $($tt)*);
    };
    (android, $($tt:tt)*) => {
        target_os_cfg!("android", $($tt)*);
    };
    (ios, $($tt:tt)*) => {
        target_os_cfg!("ios", $($tt)*);
    };
    (web, $($tt:tt)*) => {
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        {
            $($tt)*
        }
    };
    (desktop, $($tt:tt)*) => {
        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        {
            $($tt)*
        }
    };
    (mobile, $($tt:tt)*) => {
        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            $($tt)*
        }
    };
}
