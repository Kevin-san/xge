use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

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

    fn color_code(&self) -> &'static str {
        match self {
            Level::Error => "\x1b[31m",
            Level::Warn => "\x1b[33m",
            Level::Info => "\x1b[32m",
            Level::Debug => "\x1b[34m",
            Level::Trace => "\x1b[36m",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RotationPolicy {
    Size(u64),
    Daily,
    None,
}

pub struct LogConfig {
    pub level: Level,
    pub enable_console: bool,
    pub enable_file: bool,
    pub file_path: Option<String>,
    pub rotation_policy: RotationPolicy,
    pub max_files: usize,
    pub use_color: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: Level::Info,
            enable_console: true,
            enable_file: false,
            file_path: None,
            rotation_policy: RotationPolicy::Size(10 * 1024 * 1024),
            max_files: 5,
            use_color: true,
        }
    }
}

struct LogWriter {
    console_writer: Option<io::Stderr>,
    file_writer: Option<BufWriter<File>>,
    file_path: Option<String>,
    rotation_policy: RotationPolicy,
    max_files: usize,
    use_color: bool,
    current_file_size: u64,
}

impl LogWriter {
    fn new(config: &LogConfig) -> Self {
        let file_writer = if config.enable_file {
            if let Some(path) = &config.file_path {
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .ok();
                file.map(BufWriter::new)
            } else {
                None
            }
        } else {
            None
        };

        let current_file_size = if let Some(path) = &config.file_path {
            fs::metadata(path).map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

        Self {
            console_writer: if config.enable_console {
                Some(io::stderr())
            } else {
                None
            },
            file_writer,
            file_path: config.file_path.clone(),
            rotation_policy: config.rotation_policy,
            max_files: config.max_files,
            use_color: config.use_color,
            current_file_size,
        }
    }

    fn rotate(&mut self) {
        if let Some(path) = &self.file_path {
            let path = Path::new(path);
            let parent = path.parent().unwrap_or_else(|| Path::new("."));
            let file_name = path.file_name().unwrap().to_string_lossy();
            
            for i in (1..self.max_files).rev() {
                let old_path = parent.join(format!("{}.{}", file_name, i));
                let new_path = parent.join(format!("{}.{}", file_name, i + 1));
                let _ = fs::rename(&old_path, &new_path);
            }
            
            let backup_path = parent.join(format!("{}.1", file_name));
            let _ = fs::rename(path, &backup_path);
            
            if let Ok(file) = OpenOptions::new().create(true).write(true).open(path) {
                self.file_writer = Some(BufWriter::new(file));
                self.current_file_size = 0;
            }
        }
    }

    fn should_rotate(&self, bytes_written: u64) -> bool {
        match self.rotation_policy {
            RotationPolicy::Size(max_size) => self.current_file_size + bytes_written > max_size,
            RotationPolicy::Daily => {
                if let Some(path) = &self.file_path {
                    if let Ok(metadata) = fs::metadata(path) {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(elapsed) = SystemTime::now().duration_since(modified) {
                                return elapsed.as_secs() > 24 * 60 * 60;
                            }
                        }
                    }
                }
                false
            }
            RotationPolicy::None => false,
        }
    }

    fn write(&mut self, level: Level, target: &str, msg: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        let console_line = if self.use_color {
            format!(
                "[{}][{}][{}] {}\x1b[0m\n",
                timestamp,
                level.color_code(),
                level.as_str(),
                msg
            )
        } else {
            format!("[{}][{}][{}] {}\n", timestamp, level.as_str(), target, msg)
        };

        let file_line = format!("[{}][{}][{}] {}\n", timestamp, level.as_str(), target, msg);

        if let Some(writer) = &mut self.console_writer {
            let _ = writer.write_all(console_line.as_bytes());
            let _ = writer.flush();
        }

        let needs_rotate = self.should_rotate(file_line.len() as u64);
        if let Some(writer) = &mut self.file_writer {
            if needs_rotate {
                let _ = writer.flush();
            }
        }
        if needs_rotate {
            self.rotate();
        }

        if let Some(writer) = &mut self.file_writer {
            let bytes_written = file_line.len() as u64;
            let _ = writer.write_all(file_line.as_bytes());
            let _ = writer.flush();
            self.current_file_size += bytes_written;
        }
    }
}

struct Logger {
    level: Level,
    writer: Mutex<LogWriter>,
}

static mut LOGGER: Option<Arc<Logger>> = None;

pub fn init_with_config(config: LogConfig) {
    let logger = Arc::new(Logger {
        level: config.level,
        writer: Mutex::new(LogWriter::new(&config)),
    });
    unsafe {
        LOGGER = Some(logger);
    }
}

pub fn init(level: Level) {
    let config = LogConfig {
        level,
        ..LogConfig::default()
    };
    init_with_config(config);
}

pub fn init_with_file(level: Level, file_path: &str) {
    let config = LogConfig {
        level,
        enable_file: true,
        file_path: Some(file_path.to_string()),
        ..LogConfig::default()
    };
    init_with_config(config);
}

fn get_logger() -> Option<Arc<Logger>> {
    unsafe { LOGGER.clone() }
}

fn log_impl(level: Level, target: &str, msg: &str) {
    if let Some(logger) = get_logger() {
        if level <= logger.level {
            let mut writer = logger.writer.lock().unwrap();
            writer.write(level, target, msg);
        }
    } else {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
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

#[macro_export]
macro_rules! log_error {
    ($msg:expr) => {
        $crate::log::error(module_path!(), $msg)
    };
    ($target:expr, $msg:expr) => {
        $crate::log::error($target, $msg)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr) => {
        $crate::log::warn(module_path!(), $msg)
    };
    ($target:expr, $msg:expr) => {
        $crate::log::warn($target, $msg)
    };
}

#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {
        $crate::log::info(module_path!(), $msg)
    };
    ($target:expr, $msg:expr) => {
        $crate::log::info($target, $msg)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($msg:expr) => {
        $crate::log::debug(module_path!(), $msg)
    };
    ($target:expr, $msg:expr) => {
        $crate::log::debug($target, $msg)
    };
}

#[macro_export]
macro_rules! log_trace {
    ($msg:expr) => {
        $crate::log::trace(module_path!(), $msg)
    };
    ($target:expr, $msg:expr) => {
        $crate::log::trace($target, $msg)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_log_init() {
        init(Level::Debug);
        debug("test", "debug message");
        info("test", "info message");
    }

    #[test]
    fn test_log_level_filter() {
        init(Level::Warn);
        debug("test", "should not appear");
        warn("test", "should appear");
        error("test", "should appear");
    }

    #[test]
    fn test_log_file_output() {
        let temp_path = "./test_log.txt";
        
        init_with_file(Level::Debug, temp_path);
        info("test", "file test message");
        
        let content = fs::read_to_string(temp_path).unwrap();
        assert!(content.contains("file test message"));
        
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_log_rotation() {
        let temp_path = "./test_rotate.txt";
        let _ = fs::remove_file(temp_path);
        
        let config = LogConfig {
            level: Level::Debug,
            enable_file: true,
            file_path: Some(temp_path.to_string()),
            rotation_policy: RotationPolicy::Size(100),
            max_files: 2,
            ..LogConfig::default()
        };
        init_with_config(config);
        
        for i in 0..20 {
            info("test", &format!("message {}", i));
        }
        
        assert!(Path::new(temp_path).exists());
        
        let _ = fs::remove_file(temp_path);
        let _ = fs::remove_file(format!("{}.1", temp_path));
        let _ = fs::remove_file(format!("{}.2", temp_path));
    }

    #[test]
    fn test_log_macros() {
        init(Level::Trace);
        log_error!("error macro");
        log_warn!("warn macro");
        log_info!("info macro");
        log_debug!("debug macro");
        log_trace!("trace macro");
    }

    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.level, Level::Info);
        assert!(config.enable_console);
        assert!(!config.enable_file);
        assert_eq!(config.max_files, 5);
    }
}