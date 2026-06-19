//! 定时器系统

/// 定时器模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TimerMode {
    /// 单次触发
    Once,
    /// 重复触发
    Repeat,
}

/// 定时器
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Timer {
    duration: f32,
    elapsed: f32,
    mode: TimerMode,
    paused: bool,
}

impl Timer {
    /// 创建新的定时器
    pub fn new(duration: f32, mode: TimerMode) -> Self {
        Self {
            duration: duration.max(0.0),
            elapsed: 0.0,
            mode,
            paused: false,
        }
    }

    /// 每帧更新，返回是否触发
    pub fn tick(&mut self, dt: f32) -> bool {
        if self.paused {
            return false;
        }

        self.elapsed += dt;

        if self.elapsed >= self.duration {
            if self.mode == TimerMode::Repeat {
                self.elapsed %= self.duration;
                true
            } else {
                self.elapsed = self.duration;
                true
            }
        } else {
            false
        }
    }

    /// 检查是否已完成（Once 模式）
    pub fn finished(&self) -> bool {
        self.mode == TimerMode::Once && self.elapsed >= self.duration
    }

    /// 重置定时器
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
    }

    /// 获取剩余时间
    pub fn remaining(&self) -> f32 {
        (self.duration - self.elapsed).max(0.0)
    }

    /// 获取已用时间
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// 暂停
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// 继续
    pub fn resume(&mut self) {
        self.paused = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_once() {
        let mut timer = Timer::new(1.0, TimerMode::Once);

        assert!(!timer.tick(0.5));
        assert!(!timer.finished());

        assert!(timer.tick(0.6));
        assert!(timer.finished());
    }

    #[test]
    fn test_timer_repeat() {
        let mut timer = Timer::new(1.0, TimerMode::Repeat);

        assert!(!timer.tick(0.5));
        assert!(timer.tick(0.6));
        assert!(!timer.finished());
        assert!((timer.elapsed() - 0.1).abs() < 1e-6);

        // 继续运行
        assert!(timer.tick(1.0));
        assert!((timer.elapsed() - 0.1).abs() < 1e-6);
    }

    #[test]
    fn test_timer_pause_resume() {
        let mut timer = Timer::new(1.0, TimerMode::Once);

        timer.tick(0.5);
        timer.pause();
        assert!(timer.paused);

        timer.tick(1.0); // 不应增加
        assert!((timer.elapsed() - 0.5).abs() < 1e-6);

        timer.resume();
        assert!(!timer.paused);

        timer.tick(0.6);
        assert!(timer.finished());
    }

    #[test]
    fn test_timer_remaining() {
        let mut timer = Timer::new(2.0, TimerMode::Once);

        timer.tick(0.5);
        assert!((timer.remaining() - 1.5).abs() < 1e-6);

        timer.reset();
        assert!((timer.remaining() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_timer_zero_duration() {
        let mut timer = Timer::new(0.0, TimerMode::Once);
        assert!(timer.tick(0.1));
        assert!(timer.finished());
    }

    #[test]
    fn test_timer_repeat_cumulative() {
        let mut timer = Timer::new(1.0, TimerMode::Repeat);

        // 多次触发
        timer.tick(1.5);
        assert!((timer.elapsed() - 0.5).abs() < 1e-6);

        timer.tick(1.5);
        assert!((timer.elapsed() - 0.0).abs() < 1e-6 || (timer.elapsed() - 0.5).abs() < 1e-6);
    }
}
