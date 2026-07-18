//! 显示器与视频模式枚举

/// 显示器信息
#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub name: Option<String>,
    pub width: u32,
    pub height: u32,
    pub refresh_rate_millihertz: u32,
    pub scale_factor: f64,
}

/// 视频模式
#[derive(Debug, Clone)]
pub struct VideoModeInfo {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u16,
    pub refresh_rate_millihertz: u32,
    pub monitor_name: Option<String>,
}

/// 显示器管理器
pub struct MonitorManager;

impl MonitorManager {
    /// 枚举所有可用显示器
    /// 注意：此方法需要在事件循环可用时调用
    pub fn enumerate(event_loop: &crate::EventLoop<()>) -> Vec<MonitorInfo> {
        event_loop
            .available_monitors()
            .map(|handle| {
                let name = handle.name().map(|s| s.to_string());
                let size = handle.size();
                let refresh = handle.refresh_rate_millihertz().unwrap_or(0);
                let scale = handle.scale_factor();
                MonitorInfo {
                    name,
                    width: size.width,
                    height: size.height,
                    refresh_rate_millihertz: refresh,
                    scale_factor: scale,
                }
            })
            .collect()
    }

    /// 获取主显示器
    pub fn primary(event_loop: &crate::EventLoop<()>) -> Option<MonitorInfo> {
        event_loop.primary_monitor().map(|handle| {
            let name = handle.name().map(|s| s.to_string());
            let size = handle.size();
            let refresh = handle.refresh_rate_millihertz().unwrap_or(0);
            let scale = handle.scale_factor();
            MonitorInfo {
                name,
                width: size.width,
                height: size.height,
                refresh_rate_millihertz: refresh,
                scale_factor: scale,
            }
        })
    }

    /// 枚举显示器支持的视频模式
    pub fn video_modes(handle: &crate::MonitorHandle) -> Vec<VideoModeInfo> {
        handle
            .video_modes()
            .map(|vm| {
                let size = vm.size();
                VideoModeInfo {
                    width: size.width,
                    height: size.height,
                    bit_depth: vm.bit_depth(),
                    refresh_rate_millihertz: vm.refresh_rate_millihertz(),
                    monitor_name: vm.monitor().name().map(|s| s.to_string()),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_info_debug() {
        let info = MonitorInfo {
            name: Some("Test Monitor".to_string()),
            width: 1920,
            height: 1080,
            refresh_rate_millihertz: 60000,
            scale_factor: 1.0,
        };
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("Test Monitor"));
    }

    #[test]
    fn test_video_mode_info_debug() {
        let mode = VideoModeInfo {
            width: 1920,
            height: 1080,
            bit_depth: 32,
            refresh_rate_millihertz: 60000,
            monitor_name: Some("Test".to_string()),
        };
        let debug_str = format!("{:?}", mode);
        assert!(debug_str.contains("1920"));
    }
}
