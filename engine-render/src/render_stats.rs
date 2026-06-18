//! RenderStats 模块 - 渲染统计信息
//!
//! 提供每帧渲染统计，包括 draw_calls、vertices、indices、batches 等。

use parking_lot::RwLock;

/// 渲染统计信息
///
/// 每帧重置一次，用于监控渲染性能。
#[derive(Clone, Debug, Default)]
pub struct RenderStats {
    /// 绘制调用次数
    pub draw_calls: u32,
    /// 顶点数量
    pub vertices: u32,
    /// 索引数量
    pub indices: u32,
    /// 批次数
    pub batches: u32,
    /// 纹理切换次数
    pub texture_switches: u32,
}

impl RenderStats {
    /// 创建新的渲染统计
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 重置统计
    #[inline]
    pub fn reset(&mut self) {
        self.draw_calls = 0;
        self.vertices = 0;
        self.indices = 0;
        self.batches = 0;
        self.texture_switches = 0;
    }

    /// 增加绘制调用
    #[inline]
    pub fn add_draw_call(&mut self, count: u32) {
        self.draw_calls += count;
    }

    /// 增加顶点
    #[inline]
    pub fn add_vertices(&mut self, count: u32) {
        self.vertices += count;
    }

    /// 增加索引
    #[inline]
    pub fn add_indices(&mut self, count: u32) {
        self.indices += count;
    }

    /// 增加批次
    #[inline]
    pub fn add_batch(&mut self, count: u32) {
        self.batches += count;
    }

    /// 增加纹理切换
    #[inline]
    pub fn add_texture_switch(&mut self) {
        self.texture_switches += 1;
    }

    /// 设置绘制调用
    #[inline]
    pub fn set_draw_calls(&mut self, count: u32) {
        self.draw_calls = count;
    }

    /// 设置顶点数量
    #[inline]
    pub fn set_vertices(&mut self, count: u32) {
        self.vertices = count;
    }

    /// 设置索引数量
    #[inline]
    pub fn set_indices(&mut self, count: u32) {
        self.indices = count;
    }

    /// 设置批次数量
    #[inline]
    pub fn set_batches(&mut self, count: u32) {
        self.batches = count;
    }

    /// 设置纹理切换次数
    #[inline]
    pub fn set_texture_switches(&mut self, count: u32) {
        self.texture_switches = count;
    }
}

/// 线程安全的渲染统计（用于全局统计）
pub struct RenderStatsHolder {
    stats: RwLock<RenderStats>,
}

impl RenderStatsHolder {
    /// 创建新的统计持有者
    pub fn new() -> Self {
        Self {
            stats: RwLock::new(RenderStats::new()),
        }
    }

    /// 获取统计副本
    pub fn get(&self) -> RenderStats {
        self.stats.read().clone()
    }

    /// 重置统计
    pub fn reset(&self) {
        self.stats.write().reset();
    }

    /// 增加绘制调用
    pub fn add_draw_call(&self, count: u32) {
        self.stats.write().add_draw_call(count);
    }

    /// 增加顶点
    pub fn add_vertices(&self, count: u32) {
        self.stats.write().add_vertices(count);
    }

    /// 增加索引
    pub fn add_indices(&self, count: u32) {
        self.stats.write().add_indices(count);
    }

    /// 增加批次
    pub fn add_batch(&self, count: u32) {
        self.stats.write().add_batch(count);
    }

    /// 增加纹理切换
    pub fn add_texture_switch(&self) {
        self.stats.write().add_texture_switch();
    }
}

impl Default for RenderStatsHolder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_stats_default() {
        let stats = RenderStats::default();
        assert_eq!(stats.draw_calls, 0);
        assert_eq!(stats.vertices, 0);
        assert_eq!(stats.indices, 0);
        assert_eq!(stats.batches, 0);
    }

    #[test]
    fn test_render_stats_reset() {
        let mut stats = RenderStats::new();
        stats.add_draw_call(5);
        stats.add_vertices(100);
        stats.reset();
        assert_eq!(stats.draw_calls, 0);
        assert_eq!(stats.vertices, 0);
    }

    #[test]
    fn test_render_stats_holder() {
        let holder = RenderStatsHolder::new();
        holder.add_draw_call(10);
        holder.add_vertices(200);
        let stats = holder.get();
        assert_eq!(stats.draw_calls, 10);
        assert_eq!(stats.vertices, 200);
        holder.reset();
        let stats = holder.get();
        assert_eq!(stats.draw_calls, 0);
    }
}
