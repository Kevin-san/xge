//! 结构化日志模块
//!
//! 提供 `debug! / info! / warn! / error!` 宏及运行时级别控制。
//! 支持按模块 target 前缀过滤。

use std::io::{self, Write};
use std::sync::Mutex;

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        }
    }
}

static LOG_LEVEL: Mutex<Level> = Mutex::new(Level::Info);
static TARGET_FILTER: Mutex<Option<String>> = Mutex::new(None);

/// 初始化日志系统
pub fn init(level: Level) {
    *LOG_LEVEL.lock().unwrap() = level;
}

/// 设置日志级别
pub fn set_level(level: Level) {
    *LOG_LEVEL.lock().unwrap() = level;
}

/// 检查指定级别是否启用
pub fn enabled(level: Level) -> bool {
    level <= current_level()
}

/// 设置 target 前缀过滤器（仅匹配该前缀的日志才会输出）
pub fn set_target_filter(prefix: impl Into<String>) {
    *TARGET_FILTER.lock().unwrap() = Some(prefix.into());
}

/// 清除 target 过滤器
pub fn clear_target_filter() {
    *TARGET_FILTER.lock().unwrap() = None;
}

/// 获取当前日志级别
pub fn current_level() -> Level {
    *LOG_LEVEL.lock().unwrap()
}

/// 检查 target 是否匹配过滤器
fn target_allowed(target: &str) -> bool {
    let filter = TARGET_FILTER.lock().unwrap();
    match filter.as_deref() {
        Some(prefix) => target.starts_with(prefix),
        None => true,
    }
}

fn log_impl(level: Level, target: &str, msg: &str) {
    if level <= current_level() && target_allowed(target) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(
            io::stderr(),
            "[{}][{}][{}] {}",
            timestamp,
            level.as_str(),
            target,
            msg
        );
    }
}

pub fn error(target: &str, msg: &str) {
    log_impl(Level::Error, target, msg);
}

pub fn warn(target: &str, msg: &str) {
    log_impl(Level::Warn, target, msg);
}

pub fn info(target: &str, msg: &str) {
    log_impl(Level::Info, target, msg);
}

pub fn debug(target: &str, msg: &str) {
    log_impl(Level::Debug, target, msg);
}

pub fn trace(target: &str, msg: &str) {
    log_impl(Level::Trace, target, msg);
}

/// 日志宏 — 自动填充 target 为模块路径
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::log::error(module_path!(), &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::log::warn(module_path!(), &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::log::info(module_path!(), &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::log::debug(module_path!(), &format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_ordering() {
        assert!(Level::Error < Level::Warn);
        assert!(Level::Warn < Level::Info);
        assert!(Level::Info < Level::Debug);
        assert!(Level::Debug < Level::Trace);
    }

    #[test]
    fn test_enabled() {
        init(Level::Info);
        assert!(enabled(Level::Error));
        assert!(enabled(Level::Warn));
        assert!(enabled(Level::Info));
        assert!(!enabled(Level::Debug));
        assert!(!enabled(Level::Trace));
    }

    #[test]
    fn test_set_level() {
        set_level(Level::Debug);
        assert!(enabled(Level::Debug));
        // 重置
        set_level(Level::Info);
        assert!(!enabled(Level::Debug));
    }

    #[test]
    fn test_target_filter() {
        set_level(Level::Trace);
        set_target_filter("engine_core");
        assert!(target_allowed("engine_core::module"));
        assert!(!target_allowed("engine_math::vec3"));
        clear_target_filter();
        assert!(target_allowed("anything"));
    }

    #[test]
    fn test_log_functions_dont_panic() {
        set_level(Level::Trace);
        clear_target_filter();
        error("test", "error message");
        warn("test", "warn message");
        info("test", "info message");
        debug("test", "debug message");
        trace("test", "trace message");
    }

    #[test]
    fn test_level_as_str() {
        assert_eq!(Level::Error.as_str(), "ERROR");
        assert_eq!(Level::Warn.as_str(), "WARN");
        assert_eq!(Level::Info.as_str(), "INFO");
        assert_eq!(Level::Debug.as_str(), "DEBUG");
        assert_eq!(Level::Trace.as_str(), "TRACE");
    }
}
