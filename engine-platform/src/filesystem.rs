//! 文件系统抽象 — 平台层定义

use std::path::{Path, PathBuf};

/// 文件系统错误
#[derive(Debug)]
pub enum FsError {
    Io(std::io::Error),
    NotFound(String),
    PermissionDenied(String),
}

impl std::fmt::Display for FsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::NotFound(p) => write!(f, "Not found: {}", p),
            Self::PermissionDenied(p) => write!(f, "Permission denied: {}", p),
        }
    }
}

impl std::error::Error for FsError {}

impl From<std::io::Error> for FsError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound(err.to_string()),
            std::io::ErrorKind::PermissionDenied => Self::PermissionDenied(err.to_string()),
            _ => Self::Io(err),
        }
    }
}

/// 平台文件系统抽象 trait
pub trait FileSystem: Send + Sync {
    /// 读取文件字节内容
    fn read(&self, path: &Path) -> Result<Vec<u8>, FsError>;

    /// 读取文件字符串内容
    fn read_string(&self, path: &Path) -> Result<String, FsError>;

    /// 写入字节数据
    fn write(&self, path: &Path, data: &[u8]) -> Result<(), FsError>;

    /// 写入字符串
    fn write_string(&self, path: &Path, s: &str) -> Result<(), FsError>;

    /// 判断路径是否存在
    fn exists(&self, path: &Path) -> bool;

    /// 列出目录条目
    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>, FsError>;

    /// 递归创建目录
    fn create_dir_all(&self, path: &Path) -> Result<(), FsError>;

    /// 删除文件
    fn remove_file(&self, path: &Path) -> Result<(), FsError>;

    /// 是否是目录
    fn is_dir(&self, path: &Path) -> bool;

    /// 规范化路径
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, FsError>;
}
