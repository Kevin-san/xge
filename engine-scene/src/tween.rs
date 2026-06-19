//! Tween 补间动画系统

use engine_math::{Vec2, Vec3};
use engine_render::Color;

/// 缓动曲线（30+）
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Ease {
    /// 线性
    Linear,
    /// 二次方缓入
    InQuad,
    /// 二次方缓出
    OutQuad,
    /// 二次方缓入缓出
    InOutQuad,
    /// 三次方缓入
    InCubic,
    /// 三次方缓出
    OutCubic,
    /// 三次方缓入缓出
    InOutCubic,
    /// 四次方缓入
    InQuart,
    /// 四次方缓出
    OutQuart,
    /// 四次方缓入缓出
    InOutQuart,
    /// 五次方缓入
    InQuint,
    /// 五次方缓出
    OutQuint,
    /// 五次方缓入缓出
    InOutQuint,
    /// 正弦缓入
    InSine,
    /// 正弦缓出
    OutSine,
    /// 正弦缓入缓出
    InOutSine,
    /// 指数缓入
    InExpo,
    /// 指数缓出
    OutExpo,
    /// 指数缓入缓出
    InOutExpo,
    /// 圆弧缓入
    InCirc,
    /// 圆弧缓出
    OutCirc,
    /// 圆弧缓入缓出
    InOutCirc,
    /// 回退缓入
    InBack,
    /// 回退缓出
    OutBack,
    /// 回退缓入缓出
    InOutBack,
    /// 弹性缓入
    InElastic,
    /// 弹性缓出
    OutElastic,
    /// 弹性缓入缓出
    InOutElastic,
    /// 弹跳缓入
    InBounce,
    /// 弹跳缓出
    OutBounce,
    /// 弹跳缓入缓出
    InOutBounce,
}

impl Ease {
    /// 应用缓动曲线
    pub fn apply(self, t: f32) -> f32 {
        match self {
            Ease::Linear => t,
            Ease::InQuad => t * t,
            Ease::OutQuad => t * (2.0 - t),
            Ease::InOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Ease::InCubic => t * t * t,
            Ease::OutCubic => {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }
            Ease::InOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t2 = 2.0 * t - 2.0;
                    0.5 * t2 * t2 * t2 + 1.0
                }
            }
            Ease::InQuart => t * t * t * t,
            Ease::OutQuart => {
                let t1 = t - 1.0;
                1.0 - t1 * t1 * t1 * t1
            }
            Ease::InOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    let t1 = t - 1.0;
                    1.0 - 8.0 * t1 * t1 * t1 * t1
                }
            }
            Ease::InQuint => t * t * t * t * t,
            Ease::OutQuint => {
                let t1 = t - 1.0;
                t1 * t1 * t1 * t1 * t1 + 1.0
            }
            Ease::InOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    let t2 = 2.0 * t - 2.0;
                    0.5 * t2 * t2 * t2 * t2 * t2 + 1.0
                }
            }
            Ease::InSine => 1.0 - (t * std::f32::consts::FRAC_PI_2).cos(),
            Ease::OutSine => (t * std::f32::consts::FRAC_PI_2).sin(),
            Ease::InOutSine => -0.5 * ((std::f32::consts::PI * t).cos() - 1.0),
            Ease::InExpo => {
                if t == 0.0 {
                    0.0
                } else {
                    2.0f32.powf(10.0 * (t - 1.0))
                }
            }
            Ease::OutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0f32.powf(-10.0 * t)
                }
            }
            Ease::InOutExpo => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    2.0f32.powf(19.0 * 2.0 * t - 10.0) / 2.0
                } else {
                    (2.0 - 2.0f32.powf(-19.0 * 2.0 * t + 10.0)) / 2.0
                }
            }
            Ease::InCirc => 1.0 - (1.0 - t * t).sqrt(),
            Ease::OutCirc => (1.0 - (t - 1.0).powi(2)).sqrt(),
            Ease::InOutCirc => {
                if t < 0.5 {
                    (1.0 - (1.0 - 2.0 * t).powi(2)).sqrt() / 2.0
                } else {
                    ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
                }
            }
            Ease::InBack => {
                const C1: f32 = 1.70158;
                const C3: f32 = C1 + 1.0;
                C3 * t * t * t - C1 * t * t
            }
            Ease::OutBack => {
                const C1: f32 = 1.70158;
                const C3: f32 = C1 + 1.0;
                1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
            }
            Ease::InOutBack => {
                const C1: f32 = 1.70158;
                const C2: f32 = C1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
                }
            }
            Ease::InElastic => {
                const C4: f32 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    -2.0f32.powf(10.0 * t) * ((t * 10.0 - 10.75) * C4).sin()
                }
            }
            Ease::OutElastic => {
                const C4: f32 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
                }
            }
            Ease::InOutElastic => {
                const C5: f32 = (2.0 * std::f32::consts::PI) / 4.5;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    -(2.0f32.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0
                } else {
                    (2.0f32.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0 + 1.0
                }
            }
            Ease::InBounce => 1.0 - bounce_out(1.0 - t),
            Ease::OutBounce => bounce_out(t),
            Ease::InOutBounce => {
                if t < 0.5 {
                    (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
                }
            }
        }
    }
}

/// bounce_out 缓动函数
fn bounce_out(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;

    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t2 = t - 1.5 / D1;
        N1 * t2 * t2 + 0.75
    } else if t < 2.5 / D1 {
        let t2 = t - 2.25 / D1;
        N1 * t2 * t2 + 0.9375
    } else {
        let t2 = t - 2.625 / D1;
        N1 * t2 * t2 + 0.984375
    }
}

/// 补间值类型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TweenValue {
    /// 浮点数
    Float(f32),
    /// 二维向量
    Vec2(Vec2),
    /// 三维向量
    Vec3(Vec3),
    /// 颜色
    Color(Color),
    /// 角度（弧度）
    Angle(f32),
}

impl TweenValue {
    /// 线性插值
    fn lerp(&self, other: &TweenValue, t: f32) -> TweenValue {
        match (self, other) {
            (TweenValue::Float(a), TweenValue::Float(b)) => TweenValue::Float(a + (*b - *a) * t),
            (TweenValue::Vec2(a), TweenValue::Vec2(b)) => TweenValue::Vec2(a.lerp(*b, t)),
            (TweenValue::Vec3(a), TweenValue::Vec3(b)) => TweenValue::Vec3(a.lerp(*b, t)),
            (TweenValue::Color(a), TweenValue::Color(b)) => {
                TweenValue::Color(Color::lerp(*a, *b, t))
            }
            (TweenValue::Angle(a), TweenValue::Angle(b)) => {
                // 角度插值，取最短路径
                let diff = *b - *a;
                let pi = std::f32::consts::PI;
                let diff = if diff > pi {
                    diff - 2.0 * pi
                } else if diff < -pi {
                    diff + 2.0 * pi
                } else {
                    diff
                };
                TweenValue::Angle(*a + diff * t)
            }
            _ => self.clone(),
        }
    }
}

/// Tween 补间
pub struct Tween {
    start: TweenValue,
    end: TweenValue,
    duration: f32,
    elapsed: f32,
    ease: Ease,
    delay: f32,
    repeat_times: u32,
    repeat_count: u32,
    yoyo: bool,
    on_complete: Option<Box<dyn FnMut()>>,
}

impl std::fmt::Debug for Tween {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tween")
            .field("duration", &self.duration)
            .field("elapsed", &self.elapsed)
            .field("ease", &self.ease)
            .field("delay", &self.delay)
            .field("repeat_times", &self.repeat_times)
            .field("repeat_count", &self.repeat_count)
            .field("yoyo", &self.yoyo)
            .finish()
    }
}

impl Tween {
    /// 创建新的 Tween
    pub fn new(start: TweenValue, end: TweenValue, duration: f32, ease: Ease) -> Self {
        Self {
            start,
            end,
            duration: duration.max(0.0),
            elapsed: 0.0,
            ease,
            delay: 0.0,
            repeat_times: 0,
            repeat_count: 0,
            yoyo: false,
            on_complete: None,
        }
    }

    /// 设置延迟
    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }

    /// 设置重复次数
    pub fn with_repeat(mut self, times: u32) -> Self {
        self.repeat_times = times;
        self
    }

    /// 设置是否往返
    pub fn with_yoyo(mut self, yoyo: bool) -> Self {
        self.yoyo = yoyo;
        self
    }

    /// 设置完成回调
    pub fn on_complete(mut self, cb: impl FnMut() + 'static) -> Self {
        self.on_complete = Some(Box::new(cb));
        self
    }

    /// 获取当前值
    pub fn value(&self) -> TweenValue {
        let t = if self.duration == 0.0 {
            1.0
        } else {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        };
        let eased_t = self.ease.apply(t);

        if self.yoyo && self.repeat_count % 2 == 1 {
            self.end.lerp(&self.start, eased_t)
        } else {
            self.start.lerp(&self.end, eased_t)
        }
    }

    /// 获取进度 [0.0, 1.0]
    pub fn progress(&self) -> f32 {
        if self.duration == 0.0 {
            1.0
        } else {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        }
    }

    /// 更新 Tween
    pub fn update(&mut self, dt: f32) {
        if self.delay > 0.0 {
            self.delay -= dt;
            return;
        }

        self.elapsed += dt;

        if self.elapsed >= self.duration {
            if self.repeat_times == 0 {
                // 无重复 - 完成一次迭代后结束
                self.elapsed = self.duration;
                if let Some(ref mut cb) = self.on_complete {
                    cb();
                }
            } else if self.repeat_count >= self.repeat_times {
                // 所有重复完成
                self.elapsed = self.duration;
                if let Some(ref mut cb) = self.on_complete {
                    cb();
                }
            } else {
                // 继续重复
                self.elapsed = 0.0;
                self.repeat_count += 1;
            }
        }
    }

    /// 检查是否完成
    pub fn is_finished(&self) -> bool {
        if self.repeat_times == 0 {
            self.elapsed >= self.duration && self.delay <= 0.0
        } else {
            self.repeat_count >= self.repeat_times && self.elapsed >= self.duration
        }
    }

    /// 重置 Tween
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.repeat_count = 0;
    }
}

/// Tween 管理器
#[derive(Debug, Default)]
pub struct TweenManager {
    tweens: Vec<Tween>,
}

impl TweenManager {
    /// 创建新的 TweenManager
    pub fn new() -> Self {
        Self { tweens: Vec::new() }
    }

    /// 添加 Tween
    pub fn add(&mut self, tween: Tween) {
        self.tweens.push(tween);
    }

    /// 更新所有 Tween
    pub fn update(&mut self, dt: f32) {
        for tween in &mut self.tweens {
            tween.update(dt);
        }
        self.tweens.retain(|t| !t.is_finished());
    }

    /// 清空所有 Tween
    pub fn clear(&mut self) {
        self.tweens.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ease_linear() {
        assert!((Ease::Linear.apply(0.0) - 0.0).abs() < 1e-6);
        assert!((Ease::Linear.apply(0.5) - 0.5).abs() < 1e-6);
        assert!((Ease::Linear.apply(1.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_ease_in_out_cubic() {
        let start = Ease::InOutCubic.apply(0.0);
        let mid = Ease::InOutCubic.apply(0.5);
        let end = Ease::InOutCubic.apply(1.0);

        assert!((start - 0.0).abs() < 1e-6);
        assert!((end - 1.0).abs() < 1e-6);
        // InOutCubic 在中间点应该为 0.5
        assert!((mid - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_tween_progress() {
        let tween = Tween::new(
            TweenValue::Float(0.0),
            TweenValue::Float(100.0),
            1.0,
            Ease::Linear,
        );

        assert!((tween.progress() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_tween_repeat() {
        let mut tween = Tween::new(
            TweenValue::Float(0.0),
            TweenValue::Float(100.0),
            1.0,
            Ease::Linear,
        )
        .with_repeat(2);

        // 完成第一次
        tween.update(1.0);
        assert!(!tween.is_finished());

        // 完成第二次
        tween.update(1.0);
        assert!(!tween.is_finished());

        // 完成第三次
        tween.update(1.0);
        assert!(tween.is_finished());
    }

    #[test]
    fn test_tween_yoyo() {
        let mut tween = Tween::new(
            TweenValue::Float(0.0),
            TweenValue::Float(100.0),
            1.0,
            Ease::Linear,
        )
        .with_repeat(1) // 需要设置重复才能看到 yoyo 效果
        .with_yoyo(true);

        // 第一次：0 -> 100
        tween.update(1.0);
        if let TweenValue::Float(v) = tween.value() {
            assert!((v - 100.0).abs() < 1e-6);
        }

        // 第二次（yoyo）：100 -> 0
        tween.update(1.0);
        if let TweenValue::Float(v) = tween.value() {
            assert!((v - 0.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_tween_manager() {
        let mut manager = TweenManager::new();
        manager.add(Tween::new(
            TweenValue::Float(0.0),
            TweenValue::Float(100.0),
            1.0,
            Ease::Linear,
        ));

        assert_eq!(manager.tweens.len(), 1);

        manager.update(1.0);
        assert_eq!(manager.tweens.len(), 0);
    }

    #[test]
    fn test_tween_vec2() {
        let tween = Tween::new(
            TweenValue::Vec2(Vec2::ZERO),
            TweenValue::Vec2(Vec2::ONE),
            1.0,
            Ease::Linear,
        );

        if let TweenValue::Vec2(v) = tween.value() {
            assert!((v.x - 0.0).abs() < 1e-6);
            assert!((v.y - 0.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_tween_color() {
        let tween = Tween::new(
            TweenValue::Color(Color::BLACK),
            TweenValue::Color(Color::WHITE),
            1.0,
            Ease::Linear,
        );

        if let TweenValue::Color(c) = tween.value() {
            assert!((c.r - 0.0).abs() < 1e-6);
            assert!((c.g - 0.0).abs() < 1e-6);
        }
    }
}
