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

fn with_log_level<F, R>(f: F) -> R
where
    F: FnOnce(&mut Level) -> R,
{
    let mut level = LOG_LEVEL.lock().unwrap_or_else(|e| e.into_inner());
    f(&mut level)
}

/// 初始化日志系统
pub fn init(level: Level) {
    with_log_level(|l| *l = level);
}

/// 获取当前日志级别
pub fn current_level() -> Level {
    with_log_level(|l| *l)
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
    with_log_level(|l| *l = level);
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

    #[test]
    fn test_mutex_poisoning_resistance() {
        let original = current_level();
        set_level(Level::Warn);
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let guard = LOG_LEVEL.lock().unwrap();
            let _ = *guard;
            panic!("poison the mutex");
        }));
        let recovered = current_level();
        assert_eq!(recovered, Level::Warn);
        set_level(original);
    }

    #[test]
    fn test_level_ordering() {
        assert!(Level::Error < Level::Warn);
        assert!(Level::Warn < Level::Info);
        assert!(Level::Info < Level::Debug);
        assert!(Level::Debug < Level::Trace);
    }

    #[test]
    fn test_level_as_str() {
        assert_eq!(Level::Error.as_str(), "ERROR");
        assert_eq!(Level::Warn.as_str(), "WARN");
        assert_eq!(Level::Info.as_str(), "INFO");
        assert_eq!(Level::Debug.as_str(), "DEBUG");
        assert_eq!(Level::Trace.as_str(), "TRACE");
    }

    #[test]
    fn test_init_sets_level() {
        let original = current_level();
        init(Level::Debug);
        assert_eq!(current_level(), Level::Debug);
        assert!(enabled(Level::Debug));
        assert!(enabled(Level::Error));
        set_level(original);
    }
}
