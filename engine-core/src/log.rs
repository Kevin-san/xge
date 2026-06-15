// 日志模块

use std::sync::Mutex;
use std::io::{self, Write};

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
fn current_level() -> Level {
    *LOG_LEVEL.lock().unwrap()
}

fn log_impl(level: Level, target: &str, msg: &str) {
    if level <= current_level() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(io::stderr(), "[{}][{}][{}] {}", timestamp, level.as_str(), target, msg);
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