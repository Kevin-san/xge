// 日志模块

use std::io::{self, Write};
use std::sync::Mutex;

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Level {
    fn as_str(&self) -> &'static str {
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

/// 初始化日志系统
pub fn init(level: Level) {
    *LOG_LEVEL.lock().unwrap() = level;
}

/// 获取当前日志级别
pub fn current_level() -> Level {
    *LOG_LEVEL.lock().unwrap()
}

fn log_impl(level: Level, target: &str, msg: &str) {
    if level <= current_level() {
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

/// 设置日志级别
pub fn set_level(level: Level) {
    *LOG_LEVEL.lock().unwrap() = level;
}

/// 检查指定级别是否启用
pub fn enabled(level: Level) -> bool {
    level <= current_level()
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

/// 日志宏 — 简化调用，自动填充 target 为调用模块
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
    fn test_set_level() {
        let original = current_level();
        set_level(Level::Error);
        assert_eq!(current_level(), Level::Error);
        assert!(!enabled(Level::Info));
        assert!(enabled(Level::Error));
        set_level(original);
    }

    #[test]
    fn test_enabled() {
        let original = current_level();
        set_level(Level::Warn);
        assert!(enabled(Level::Error));
        assert!(enabled(Level::Warn));
        assert!(!enabled(Level::Info));
        assert!(!enabled(Level::Trace));
        set_level(original);
    }
}
