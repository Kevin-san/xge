//! 光照模块
//!
//! 提供 DirectionalLight、PointLight、SpotLight 等 3D 光照类型。

use engine_math::{Vec3, Vec4};
use crate::geometry::Sphere;

/// 光源类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightType {
    /// 方向光
    Directional,
    /// 点光源
    Point,
    /// 聚光灯
    Spot,
}

/// 光源trait
pub trait Light3D {
    /// 获取光源类型
    fn light_type(&self) -> LightType;

    /// 获取光源位置（用于点光源和聚光灯）
    fn position(&self) -> Vec3;

    /// 获取光源方向（用于方向光和聚光灯）
    fn direction(&self) -> Vec3;

    /// 获取光源颜色
    fn color(&self) -> Vec4;

    /// 获取光源强度
    fn intensity(&self) -> f32;

    /// 检查是否投射阴影
    fn casts_shadow(&self) -> bool;

    /// 设置阴影投射
    fn set_casts_shadow(&mut self, casts: bool);
}

/// 方向光
#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
    /// 方向
    direction: Vec3,
    /// 颜色
    color: Vec4,
    /// 强度
    intensity: f32,
    /// 是否投射阴影
    casts_shadow: bool,
}

impl DirectionalLight {
    /// 创建方向光
    pub fn new(direction: Vec3, color: Vec4, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
            casts_shadow: false,
        }
    }

    /// 获取方向
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// 设置方向
    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction.normalize();
    }

    /// 获取颜色
    pub fn color(&self) -> Vec4 {
        self.color
    }

    /// 获取强度
    pub fn intensity(&self) -> f32 {
        self.intensity
    }
}

impl Light3D for DirectionalLight {
    fn light_type(&self) -> LightType {
        LightType::Directional
    }

    fn position(&self) -> Vec3 {
        Vec3::ZERO // 方向光没有位置概念
    }

    fn direction(&self) -> Vec3 {
        self.direction
    }

    fn color(&self) -> Vec4 {
        self.color * self.intensity
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    fn set_casts_shadow(&mut self, casts: bool) {
        self.casts_shadow = casts;
    }
}

/// 点光源
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    /// 位置
    position: Vec3,
    /// 颜色
    color: Vec4,
    /// 强度
    intensity: f32,
    /// 影响半径
    radius: f32,
    /// 是否投射阴影
    casts_shadow: bool,
}

impl PointLight {
    /// 创建点光源
    pub fn new(position: Vec3, color: Vec4, intensity: f32, radius: f32) -> Self {
        Self {
            position,
            color,
            intensity,
            radius,
            casts_shadow: false,
        }
    }

    /// 获取位置
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// 设置位置
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// 获取颜色
    pub fn color(&self) -> Vec4 {
        self.color
    }

    /// 获取强度
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// 获取半径
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// 设置半径
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    /// 计算距离衰减
    pub fn attenuation(&self, distance: f32) -> f32 {
        if distance > self.radius {
            return 0.0;
        }
        let ratio = distance / self.radius;
        1.0 - ratio * ratio
    }
}

impl Light3D for PointLight {
    fn light_type(&self) -> LightType {
        LightType::Point
    }

    fn position(&self) -> Vec3 {
        self.position
    }

    fn direction(&self) -> Vec3 {
        Vec3::ZERO // 点光源没有统一方向
    }

    fn color(&self) -> Vec4 {
        self.color * self.intensity
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    fn set_casts_shadow(&mut self, casts: bool) {
        self.casts_shadow = casts;
    }
}

/// 聚光灯
#[derive(Debug, Clone, Copy)]
pub struct SpotLight {
    /// 位置
    position: Vec3,
    /// 方向
    direction: Vec3,
    /// 内圆锥角（弧度）
    inner_angle: f32,
    /// 外圆锥角（弧度）
    outer_angle: f32,
    /// 颜色
    color: Vec4,
    /// 强度
    intensity: f32,
    /// 影响半径
    radius: f32,
    /// 是否投射阴影
    casts_shadow: bool,
}

impl SpotLight {
    /// 创建聚光灯
    pub fn new(position: Vec3, direction: Vec3, inner_angle: f32, outer_angle: f32, color: Vec4, intensity: f32, radius: f32) -> Self {
        Self {
            position,
            direction: direction.normalize(),
            inner_angle,
            outer_angle,
            color,
            intensity,
            radius,
            casts_shadow: false,
        }
    }

    /// 获取位置
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// 设置位置
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// 获取方向
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// 设置方向
    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction.normalize();
    }

    /// 获取内圆锥角
    pub fn inner_angle(&self) -> f32 {
        self.inner_angle
    }

    /// 获取外圆锥角
    pub fn outer_angle(&self) -> f32 {
        self.outer_angle
    }

    /// 获取颜色
    pub fn color(&self) -> Vec4 {
        self.color
    }

    /// 获取强度
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// 获取半径
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// 计算聚光灯衰减
    pub fn cone_attenuation(&self, dir_to_point: Vec3) -> f32 {
        let cos_angle = self.direction.dot(dir_to_point.normalize());
        let outer_cos = self.outer_angle.cos();
        let inner_cos = self.inner_angle.cos();

        if cos_angle < outer_cos {
            return 0.0;
        }

        if cos_angle > inner_cos {
            return 1.0;
        }

        (cos_angle - outer_cos) / (inner_cos - outer_cos)
    }
}

impl Light3D for SpotLight {
    fn light_type(&self) -> LightType {
        LightType::Spot
    }

    fn position(&self) -> Vec3 {
        self.position
    }

    fn direction(&self) -> Vec3 {
        self.direction
    }

    fn color(&self) -> Vec4 {
        self.color * self.intensity
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    fn set_casts_shadow(&mut self, casts: bool) {
        self.casts_shadow = casts;
    }
}

/// 环境光
#[derive(Debug, Clone, Copy)]
pub struct AmbientLight {
    color: Vec4,
    intensity: f32,
}

impl AmbientLight {
    /// 创建环境光
    pub fn new(color: Vec4, intensity: f32) -> Self {
        Self { color, intensity }
    }

    /// 获取颜色
    pub fn color(&self) -> Vec4 {
        self.color * self.intensity
    }
}

/// 半球光
#[derive(Debug, Clone, Copy)]
pub struct HemisphereLight {
    /// 天空颜色
    sky: Vec4,
    /// 地面颜色
    ground: Vec4,
    /// 强度
    intensity: f32,
}

impl HemisphereLight {
    /// 创建半球光
    pub fn new(sky: Vec4, ground: Vec4, intensity: f32) -> Self {
        Self { sky, ground, intensity }
    }

    /// 获取天空颜色
    pub fn sky(&self) -> Vec4 {
        self.sky * self.intensity
    }

    /// 获取地面颜色
    pub fn ground(&self) -> Vec4 {
        self.ground * self.intensity
    }
}

/// 光源管理器
#[derive(Debug, Clone)]
pub struct LightManager {
    /// 方向光列表
    directional_lights: Vec<DirectionalLight>,
    /// 点光源列表
    point_lights: Vec<PointLight>,
    /// 聚光灯列表
    spot_lights: Vec<SpotLight>,
    /// 环境光
    ambient: AmbientLight,
    /// 最大光源数量限制
    max_directional: usize,
    max_point: usize,
    max_spot: usize,
}

impl LightManager {
    /// 创建光源管理器
    pub fn new(max_directional: usize, max_point: usize, max_spot: usize) -> Self {
        Self {
            directional_lights: Vec::with_capacity(max_directional),
            point_lights: Vec::with_capacity(max_point),
            spot_lights: Vec::with_capacity(max_spot),
            ambient: AmbientLight::new(Vec4::new(0.05, 0.05, 0.1, 1.0), 1.0),
            max_directional,
            max_point,
            max_spot,
        }
    }

    /// 添加方向光
    pub fn add_directional(&mut self, light: DirectionalLight) -> Option<usize> {
        if self.directional_lights.len() < self.max_directional {
            self.directional_lights.push(light);
            Some(self.directional_lights.len() - 1)
        } else {
            None
        }
    }

    /// 添加点光源
    pub fn add_point(&mut self, light: PointLight) -> Option<usize> {
        if self.point_lights.len() < self.max_point {
            self.point_lights.push(light);
            Some(self.point_lights.len() - 1)
        } else {
            None
        }
    }

    /// 添加聚光灯
    pub fn add_spot(&mut self, light: SpotLight) -> Option<usize> {
        if self.spot_lights.len() < self.max_spot {
            self.spot_lights.push(light);
            Some(self.spot_lights.len() - 1)
        } else {
            None
        }
    }

    /// 设置环境光
    pub fn set_ambient(&mut self, ambient: AmbientLight) {
        self.ambient = ambient;
    }

    /// 获取环境光
    pub fn ambient(&self) -> &AmbientLight {
        &self.ambient
    }

    /// 获取所有方向光
    pub fn directional_lights(&self) -> &[DirectionalLight] {
        &self.directional_lights
    }

    /// 获取所有点光源
    pub fn point_lights(&self) -> &[PointLight] {
        &self.point_lights
    }

    /// 获取所有聚光灯
    pub fn spot_lights(&self) -> &[SpotLight] {
        &self.spot_lights
    }

    /// 获取方向光数量
    pub fn directional_count(&self) -> usize {
        self.directional_lights.len()
    }

    /// 获取点光源数量
    pub fn point_count(&self) -> usize {
        self.point_lights.len()
    }

    /// 获取聚光灯数量
    pub fn spot_count(&self) -> usize {
        self.spot_lights.len()
    }

    /// 清空所有光源
    pub fn clear(&mut self) {
        self.directional_lights.clear();
        self.point_lights.clear();
        self.spot_lights.clear();
    }
}

impl Default for LightManager {
    fn default() -> Self {
        Self::new(4, 8, 8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directional_light() {
        let light = DirectionalLight::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            1.0,
        );
        assert_eq!(light.light_type(), LightType::Directional);
        assert_eq!(light.direction(), Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_point_light() {
        let light = PointLight::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            1.0,
            10.0,
        );
        assert_eq!(light.light_type(), LightType::Point);
        assert_eq!(light.attenuation(5.0), 0.75);
    }

    #[test]
    fn test_spot_light() {
        let light = SpotLight::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            30.0f32.to_radians(),
            45.0f32.to_radians(),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            1.0,
            100.0,
        );
        assert_eq!(light.light_type(), LightType::Spot);
    }

    #[test]
    fn test_light_manager() {
        let mut manager = LightManager::new(2, 2, 2);

        let dir = DirectionalLight::new(Vec3::new(0.0, -1.0, 0.0), Vec4::new(1.0, 1.0, 1.0, 1.0), 1.0);
        manager.add_directional(dir);

        let point = PointLight::new(Vec3::new(0.0, 5.0, 0.0), Vec4::new(1.0, 1.0, 1.0, 1.0), 1.0, 50.0);
        manager.add_point(point);

        assert_eq!(manager.directional_count(), 1);
        assert_eq!(manager.point_count(), 1);
    }
}
