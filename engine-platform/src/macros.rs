//! 编译期平台检测

/// 编译期目标 OS 名称
pub const TARGET_OS: &str = {
    #[cfg(target_os = "windows")]
    {
        "windows"
    }
    #[cfg(all(not(target_os = "windows"), target_os = "linux"))]
    {
        "linux"
    }
    #[cfg(all(
        not(target_os = "windows"),
        not(target_os = "linux"),
        target_os = "macos"
    ))]
    {
        "macos"
    }
    #[cfg(all(
        not(target_os = "windows"),
        not(target_os = "linux"),
        not(target_os = "macos")
    ))]
    {
        "unknown"
    }
};

/// 编译期判断是否为 Windows
pub const IS_WINDOWS: bool = cfg!(target_os = "windows");

/// 编译期判断是否为 Linux
pub const IS_LINUX: bool = cfg!(target_os = "linux");

/// 编译期判断是否为 macOS
pub const IS_MACOS: bool = cfg!(target_os = "macos");

/// 编译期判断是否为桌面平台
pub const IS_DESKTOP: bool = IS_WINDOWS || IS_LINUX || IS_MACOS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_os_name() {
        assert!(!TARGET_OS.is_empty());
    }

    #[test]
    fn test_platform_flags_consistency() {
        // 当前平台至少有一个为 true
        assert!(IS_WINDOWS || IS_LINUX || IS_MACOS || !IS_DESKTOP);

        // 不可能同时为多个
        let count = IS_WINDOWS as usize + IS_LINUX as usize + IS_MACOS as usize;
        assert!(count <= 1);
    }

    #[test]
    fn test_desktop_flag() {
        #[cfg(target_os = "linux")]
        assert!(IS_DESKTOP);
        #[cfg(target_os = "windows")]
        assert!(IS_DESKTOP);
        #[cfg(target_os = "macos")]
        assert!(IS_DESKTOP);
    }
}
