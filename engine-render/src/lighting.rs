//! 光照系统模块
//!
//! 提供 2D 光照效果实现。

use engine_math::{Vec2, Vec3, Vec4};

/// 光源类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightType {
    /// 点光源
    Point,
    /// 平行光源
    Directional,
    /// 聚光灯
    Spot,
}

/// 光源结构
#[derive(Debug, Clone)]
pub struct Light {
    /// 光源类型
    pub light_type: LightType,
    /// 位置（用于点光源和聚光灯）
    pub position: Vec2,
    /// 方向（用于平行光源和聚光灯）
    pub direction: Vec2,
    /// 颜色
    pub color: Vec4,
    /// 强度
    pub intensity: f32,
    /// 影响半径（用于点光源）
    pub radius: f32,
    /// 散射角度（用于聚光灯，弧度）
    pub spread: f32,
    /// 衰减系数
    pub attenuation: f32,
}

impl Light {
    /// 创建点光源
    pub fn point(position: Vec2, color: Vec4, intensity: f32, radius: f32) -> Self {
        Self {
            light_type: LightType::Point,
            position,
            direction: Vec2::ZERO,
            color,
            intensity,
            radius,
            spread: std::f32::consts::PI * 2.0,
            attenuation: 0.1,
        }
    }

    /// 创建平行光源
    pub fn directional(direction: Vec2, color: Vec4, intensity: f32) -> Self {
        Self {
            light_type: LightType::Directional,
            position: Vec2::ZERO,
            direction: direction.normalize(),
            color,
            intensity,
            radius: f32::INFINITY,
            spread: std::f32::consts::PI * 2.0,
            attenuation: 0.0,
        }
    }

    /// 创建聚光灯
    pub fn spot(position: Vec2, direction: Vec2, color: Vec4, intensity: f32, radius: f32, spread: f32) -> Self {
        Self {
            light_type: LightType::Spot,
            position,
            direction: direction.normalize(),
            color,
            intensity,
            radius,
            spread,
            attenuation: 0.1,
        }
    }

    /// 计算光照强度
    pub fn intensity_at(&self, pos: Vec2) -> f32 {
        match self.light_type {
            LightType::Point => self.point_intensity_at(pos),
            LightType::Directional => self.directional_intensity_at(pos),
            LightType::Spot => self.spot_intensity_at(pos),
        }
    }

    /// 计算点光源强度
    fn point_intensity_at(&self, pos: Vec2) -> f32 {
        let dist = (pos - self.position).length();
        if dist > self.radius {
            return 0.0;
        }
        let falloff = 1.0 - (dist / self.radius).powf(self.attenuation);
        self.intensity * falloff
    }

    /// 计算平行光源强度
    fn directional_intensity_at(&self, _pos: Vec2) -> f32 {
        // 平行光源在所有位置强度相同
        self.intensity
    }

    /// 计算聚光灯强度
    fn spot_intensity_at(&self, pos: Vec2) -> f32 {
        let to_light = pos - self.position;
        let dist = to_light.length();
        
        if dist > self.radius {
            return 0.0;
        }

        let dir_to_pos = to_light.normalize();
        let angle = self.direction.dot(dir_to_pos).acos();
        
        if angle > self.spread / 2.0 {
            return 0.0;
        }

        let falloff = 1.0 - (dist / self.radius).powf(self.attenuation);
        let spot_falloff = 1.0 - (angle / (self.spread / 2.0));
        
        self.intensity * falloff * spot_falloff
    }
}

/// 光照系统管理器
pub struct LightingSystem {
    /// 所有光源
    lights: Vec<Light>,
    /// 环境光颜色
    ambient: Vec4,
    /// 最大光源数
    max_lights: usize,
}

impl LightingSystem {
    /// 创建光照系统
    pub fn new(max_lights: usize) -> Self {
        Self {
            lights: Vec::with_capacity(max_lights),
            ambient: Vec4::new(0.05, 0.05, 0.1, 1.0),
            max_lights,
        }
    }

    /// 添加光源
    pub fn add_light(&mut self, light: Light) -> Option<usize> {
        if self.lights.len() < self.max_lights {
            self.lights.push(light);
            Some(self.lights.len() - 1)
        } else {
            None
        }
    }

    /// 移除光源
    pub fn remove_light(&mut self, index: usize) {
        if index < self.lights.len() {
            self.lights.remove(index);
        }
    }

    /// 设置环境光
    pub fn set_ambient(&mut self, color: Vec4) {
        self.ambient = color;
    }

    /// 获取环境光
    pub fn ambient(&self) -> Vec4 {
        self.ambient
    }

    /// 获取所有光源
    pub fn lights(&self) -> &[Light] {
        &self.lights
    }

    /// 获取光源数量
    pub fn light_count(&self) -> usize {
        self.lights.len()
    }

    /// 清空所有光源
    pub fn clear(&mut self) {
        self.lights.clear();
    }

    /// 计算某点的总光照强度
    pub fn total_intensity_at(&self, pos: Vec2) -> f32 {
        let mut total = self.ambient.w * self.ambient.a;
        
        for light in &self.lights {
            total += light.intensity_at(pos);
        }
        
        total.min(1.0)
    }

    /// 计算某点的总光照颜色
    pub fn color_at(&self, pos: Vec2) -> Vec4 {
        let mut total_color = self.ambient * self.ambient.w;
        
        for light in &self.lights {
            let intensity = light.intensity_at(pos);
            if intensity > 0.0 {
                total_color = total_color + light.color * intensity;
            }
        }
        
        // 限制颜色范围
        Vec4::new(
            total_color.x.min(1.0),
            total_color.y.min(1.0),
            total_color.z.min(1.0),
            1.0,
        )
    }
}

impl Default for LightingSystem {
    fn default() -> Self {
        Self::new(16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_light() {
        let light = Light::point(Vec2::new(100.0, 100.0), Vec4::new(1.0, 1.0, 1.0, 1.0), 1.0, 100.0);
        
        assert_eq!(light.light_type, LightType::Point);
        assert_eq!(light.intensity_at(Vec2::new(100.0, 100.0)), 1.0);
        assert_eq!(light.intensity_at(Vec2::new(200.0, 100.0)), 0.0);
    }

    #[test]
    fn test_directional_light() {
        let light = Light::directional(Vec2::new(1.0, 0.0), Vec4::new(1.0, 1.0, 1.0, 1.0), 0.5);
        
        assert_eq!(light.light_type, LightType::Directional);
        assert_eq!(light.intensity_at(Vec2::new(0.0, 0.0)), 0.5);
        assert_eq!(light.intensity_at(Vec2::new(1000.0, 1000.0)), 0.5);
    }

    #[test]
    fn test_lighting_system() {
        let mut system = LightingSystem::new(4);
        
        let light1 = Light::point(Vec2::new(100.0, 100.0), Vec4::new(1.0, 0.0, 0.0, 1.0), 1.0, 100.0);
        let light2 = Light::point(Vec2::new(200.0, 100.0), Vec4::new(0.0, 1.0, 0.0, 1.0), 1.0, 100.0);
        
        system.add_light(light1);
        system.add_light(light2);
        
        assert_eq!(system.light_count(), 2);
        
        system.clear();
        assert_eq!(system.light_count(), 0);
    }
}
