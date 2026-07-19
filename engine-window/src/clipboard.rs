//! 剪贴板支持 — 基于 arboard，可自动初始化

use once_cell::sync::OnceCell;
use std::sync::Mutex;

/// 剪贴板接口
#[derive(Default)]
pub struct Clipboard {
    inner: Option<Mutex<arboard::Clipboard>>,
}

impl Clipboard {
    pub fn new() -> Result<Self, ClipboardError> {
        let inner = arboard::Clipboard::new()
            .map_err(|e| ClipboardError::NotInitialized(e.to_string()))?;
        Ok(Self { inner: Some(Mutex::new(inner)) })
    }

    /// 获取剪贴板文本
    pub fn get_text(&self) -> Option<String> {
        let mut guard = self.inner.as_ref()?.lock().ok()?;
        guard.get_text().ok()
    }

    /// 设置剪贴板文本
    pub fn set_text(&self, text: &str) -> Result<(), ClipboardError> {
        let mut guard = self.inner.as_ref()
            .ok_or(ClipboardError::NotInitialized("Clipboard not initialized".to_string()))?
            .lock()
            .map_err(|e| ClipboardError::NotInitialized(e.to_string()))?;
        guard.set_text(text)
            .map_err(|e| ClipboardError::NotInitialized(e.to_string()))
    }
}

/// 剪贴板错误类型
#[derive(Debug, Clone, thiserror::Error)]
pub enum ClipboardError {
    /// 剪贴板系统未初始化或不可用
    #[error("剪贴板系统未初始化或不可用: {0}")]
    NotInitialized(String),
    /// 系统剪贴板被其他程序占用
    #[error("系统剪贴板被占用: {0}")]
    SystemBusy(String),
    /// 剪贴板内容为空或格式不支持
    #[error("剪贴板内容为空或格式不支持")]
    ContentUnavailable,
    /// 文本编码错误
    #[error("文本编码错误: {0}")]
    EncodingError(String),
    /// IO 错误
    #[error("IO 错误: {0}")]
    IoError(String),
    /// 内存分配错误
    #[error("内存分配错误: {0}")]
    OutOfMemory(String),
    /// 其他未知错误
    #[error("剪贴板操作失败: {0}")]
    Unknown(String),
}

/// 全局剪贴板实例
static CLIPBOARD: OnceCell<Mutex<arboard::Clipboard>> = OnceCell::new();

/// 获取或初始化剪贴板实例
fn get_clipboard() -> Result<std::sync::MutexGuard<'static, arboard::Clipboard>, ClipboardError> {
    if CLIPBOARD.get().is_none() {
        let cb = arboard::Clipboard::new().map_err(|e| {
            ClipboardError::NotInitialized(format!(
                "无法访问系统剪贴板（可能窗口未初始化或平台不支持）: {:?}",
                e
            ))
        })?;
        // 尝试初始化；若同时有多个线程并发插入，保留第一个成功的实例
        let _ = CLIPBOARD.set(Mutex::new(cb));
    }
    CLIPBOARD.get().unwrap().lock().map_err(|e| {
        ClipboardError::Unknown(format!("剪贴板互斥锁被污染: {}", e))
    })
}

/// 从系统剪贴板获取文本内容
///
/// # 返回值
/// - `Ok(Some(text))` — 成功读取到文本
/// - `Ok(None)` — 剪贴板为空或内容不是文本
/// - `Err(ClipboardError)` — 读取失败
pub fn get_text() -> Result<Option<String>, ClipboardError> {
    let mut clipboard = get_clipboard()?;
    match clipboard.get_text() {
        Ok(text) => {
            if text.is_empty() {
                Ok(None)
            } else {
                Ok(Some(text))
            }
        }
        Err(arboard::Error::ContentNotAvailable) => Ok(None),
        Err(e) => {
            let msg = format!("{:?}", e);
            if msg.contains("not available") || msg.contains("empty") {
                Ok(None)
            } else {
                Err(ClipboardError::Unknown(format!("读取剪贴板失败: {}", e)))
            }
        }
    }
}

/// 设置系统剪贴板文本内容
///
/// # 参数
/// - `text` — 要设置的文本内容（不能为空字符串）
///
/// # 返回值
/// - `Ok(())` — 成功写入
/// - `Err(ClipboardError)` — 写入失败
pub fn set_text(text: &str) -> Result<(), ClipboardError> {
    if text.is_empty() {
        return Err(ClipboardError::ContentUnavailable);
    }
    let mut clipboard = get_clipboard()?;
    clipboard.set_text(text).map_err(|e| {
        let msg = format!("{:?}", e);
        if msg.contains("permission") || msg.contains("denied") {
            ClipboardError::SystemBusy("操作系统拒绝访问剪贴板，请稍后重试".to_string())
        } else if msg.contains("memory") || msg.contains("allocation") {
            ClipboardError::OutOfMemory("写入剪贴板时内存不足".to_string())
        } else {
            ClipboardError::Unknown(format!("写入剪贴板失败: {}", e))
        }
    })
}

/// 检查剪贴板当前是否包含文本内容
///
/// # 返回值
/// - `Ok(true)` — 包含文本
/// - `Ok(false)` — 不包含文本
/// - `Err(ClipboardError)` — 检查失败
pub fn has_text() -> Result<bool, ClipboardError> {
    Ok(get_text()?.is_some())
}

/// 清空剪贴板内容
///
/// # 返回值
/// - `Ok(())` — 成功清空
/// - `Err(ClipboardError)` — 清空失败
pub fn clear() -> Result<(), ClipboardError> {
    let mut clipboard = get_clipboard()?;
    clipboard.clear().map_err(|e| {
        ClipboardError::Unknown(format!("清空剪贴板失败: {}", e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_empty_text_returns_error() {
        let result = set_text("");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ClipboardError::ContentUnavailable));
    }

    #[test]
    fn test_clipboard_error_display() {
        let err = ClipboardError::NotInitialized("test message".to_string());
        let display = format!("{}", err);
        assert!(display.contains("test message"));
        assert!(display.contains("未初始化"));
    }
}
