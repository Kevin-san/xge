//! 文件系统抽象层 — 提供平台无关的文件操作接口

use std::path::{Path, PathBuf};

/// 文件系统错误类型
#[derive(Debug)]
pub enum FileSystemError {
    Io(std::io::Error),
    NotFound(String),
    PermissionDenied(String),
}

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::NotFound(p) => write!(f, "Not found: {}", p),
            Self::PermissionDenied(p) => write!(f, "Permission denied: {}", p),
        }
    }
}

impl std::error::Error for FileSystemError {}

impl From<std::io::Error> for FileSystemError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound(err.to_string()),
            std::io::ErrorKind::PermissionDenied => Self::PermissionDenied(err.to_string()),
            _ => Self::Io(err),
        }
    }
}

/// 文件系统抽象 trait
pub trait FileSystem: Send + Sync {
    /// 读取文件为字节数组
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileSystemError>;

    /// 读取文件为字符串
    fn read_string(&self, path: &Path) -> Result<String, FileSystemError>;

    /// 写入字节数组到文件
    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileSystemError>;

    /// 写入字符串到文件
    fn write_string(&self, path: &Path, content: &str) -> Result<(), FileSystemError>;

    /// 检查文件或目录是否存在
    fn exists(&self, path: &Path) -> bool;

    /// 列出目录内容
    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>, FileSystemError>;

    /// 创建目录（递归）
    fn create_dir_all(&self, path: &Path) -> Result<(), FileSystemError>;

    /// 删除文件
    fn remove_file(&self, path: &Path) -> Result<(), FileSystemError>;

    /// 检查路径是否是目录
    fn is_dir(&self, path: &Path) -> bool;

    /// 规范化路径
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, FileSystemError>;
}

/// 基于标准库的默认文件系统实现
pub struct StdFileSystem;

impl FileSystem for StdFileSystem {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileSystemError> {
        std::fs::read(path).map_err(|e| e.into())
    }

    fn read_string(&self, path: &Path) -> Result<String, FileSystemError> {
        std::fs::read_to_string(path).map_err(|e| e.into())
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileSystemError> {
        std::fs::write(path, bytes).map_err(|e| e.into())
    }

    fn write_string(&self, path: &Path, content: &str) -> Result<(), FileSystemError> {
        std::fs::write(path, content.as_bytes()).map_err(|e| e.into())
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>, FileSystemError> {
        std::fs::read_dir(path)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(FileSystemError::Io)
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), FileSystemError> {
        std::fs::create_dir_all(path).map_err(|e| e.into())
    }

    fn remove_file(&self, path: &Path) -> Result<(), FileSystemError> {
        std::fs::remove_file(path).map_err(|e| e.into())
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf, FileSystemError> {
        std::fs::canonicalize(path).map_err(|e| e.into())
    }
}
