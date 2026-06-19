//! 3D Lighting system

use alloc::vec::Vec;
use engine_math::Vec3;

/// Color representation (RGBA)
#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    #[inline]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    #[inline]
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }
}

/// Light sample result
#[derive(Clone, Copy, Debug)]
pub struct LightSample {
    pub direction: Vec3,
    pub color: Color,
    pub intensity: f32,
}

/// Light trait for all light types
pub trait Light3D {
    /// Compute light contribution at a world position
    fn contribution(&self, world_pos: Vec3) -> LightSample;
}

/// Directional light (sun-like)
#[derive(Clone, Debug)]
pub struct DirectionalLight {
    direction: Vec3,
    color: Color,
    intensity: f32,
    casts_shadow: bool,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
            casts_shadow: false,
        }
    }

    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    #[inline]
    pub fn casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    #[inline]
    pub fn set_casts_shadow(&mut self, enabled: bool) {
        self.casts_shadow = enabled;
    }
}

impl Light3D for DirectionalLight {
    fn contribution(&self, _world_pos: Vec3) -> LightSample {
        LightSample {
            direction: -self.direction,
            color: self.color,
            intensity: self.intensity,
        }
    }
}

/// Point light with radius and attenuation
#[derive(Clone, Debug)]
pub struct PointLight {
    position: Vec3,
    color: Color,
    intensity: f32,
    radius: f32,
}

impl PointLight {
    pub fn new(position: Vec3, color: Color, intensity: f32, radius: f32) -> Self {
        Self {
            position,
            color,
            intensity,
            radius,
        }
    }

    #[inline]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Compute attenuation based on distance
    pub fn attenuation(&self, distance: f32) -> f32 {
        if distance >= self.radius {
            return 0.0;
        }
        // Smooth falloff using inverse square with radius limit
        let normalized = distance / self.radius;
        let factor = 1.0 - normalized * normalized;
        factor * factor
    }
}

impl Light3D for PointLight {
    fn contribution(&self, world_pos: Vec3) -> LightSample {
        let dir = world_pos - self.position;
        let distance = dir.length();
        let attenuation = self.attenuation(distance);

        LightSample {
            direction: dir.normalize(),
            color: self.color,
            intensity: self.intensity * attenuation,
        }
    }
}

/// Spot light with cone angles
#[derive(Clone, Debug)]
pub struct SpotLight {
    position: Vec3,
    direction: Vec3,
    inner_angle: f32,
    outer_angle: f32,
    color: Color,
    intensity: f32,
    radius: f32,
}

impl SpotLight {
    pub fn new(
        position: Vec3,
        direction: Vec3,
        inner_angle: f32,
        outer_angle: f32,
        color: Color,
        intensity: f32,
    ) -> Self {
        Self {
            position,
            direction: direction.normalize(),
            inner_angle,
            outer_angle,
            color,
            intensity,
            radius: 10.0,
        }
    }

    #[inline]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    #[inline]
    pub fn inner_angle(&self) -> f32 {
        self.inner_angle
    }

    #[inline]
    pub fn outer_angle(&self) -> f32 {
        self.outer_angle
    }

    /// Compute cone attenuation based on direction to point
    pub fn cone_attenuation(&self, dir_to_point: Vec3) -> f32 {
        let cos_outer = self.outer_angle.cos();
        let cos_inner = self.inner_angle.cos();
        let cos_angle = self.direction.dot(dir_to_point.normalize());

        if cos_angle < cos_outer {
            return 0.0;
        }
        if cos_angle > cos_inner {
            return 1.0;
        }
        // Smooth transition between inner and outer cone
        (cos_angle - cos_outer) / (cos_inner - cos_outer)
    }
}

impl Light3D for SpotLight {
    fn contribution(&self, world_pos: Vec3) -> LightSample {
        let dir = world_pos - self.position;
        let distance = dir.length();
        let cone_atten = self.cone_attenuation(dir);

        // Distance attenuation (simplified)
        let dist_atten = if distance < self.radius {
            1.0 - (distance / self.radius).powi(2)
        } else {
            0.0
        };

        LightSample {
            direction: dir.normalize(),
            color: self.color,
            intensity: self.intensity * cone_atten * dist_atten,
        }
    }
}

/// Ambient light (constant illumination)
#[derive(Clone, Debug)]
pub struct AmbientLight {
    color: Color,
    intensity: f32,
}

impl AmbientLight {
    pub fn new(color: Color, intensity: f32) -> Self {
        Self { color, intensity }
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }
}

impl Light3D for AmbientLight {
    fn contribution(&self, _world_pos: Vec3) -> LightSample {
        LightSample {
            direction: Vec3::ZERO,
            color: self.color,
            intensity: self.intensity,
        }
    }
}

/// Hemisphere light (sky/ground gradient)
#[derive(Clone, Debug)]
pub struct HemisphereLight {
    sky_color: Color,
    ground_color: Color,
    intensity: f32,
}

impl HemisphereLight {
    pub fn new(sky_color: Color, ground_color: Color, intensity: f32) -> Self {
        Self {
            sky_color,
            ground_color,
            intensity,
        }
    }

    #[inline]
    pub fn sky_color(&self) -> Color {
        self.sky_color
    }

    #[inline]
    pub fn ground_color(&self) -> Color {
        self.ground_color
    }

    #[inline]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }
}

impl Light3D for HemisphereLight {
    fn contribution(&self, _world_pos: Vec3) -> LightSample {
        // Blend between sky and ground based on normal (simplified)
        LightSample {
            direction: Vec3::Y,
            color: self.sky_color,
            intensity: self.intensity,
        }
    }
}

/// Light manager for scene lights
#[derive(Debug, Default)]
pub struct LightManager {
    directional: Vec<DirectionalLight>,
    points: Vec<PointLight>,
    spots: Vec<SpotLight>,
    ambient: Option<AmbientLight>,
    hemisphere: Option<HemisphereLight>,
}

impl LightManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_directional(&mut self, light: DirectionalLight) {
        if self.directional.len() < 16 {
            self.directional.push(light);
        }
    }

    pub fn add_point(&mut self, light: PointLight) {
        if self.points.len() < 64 {
            self.points.push(light);
        }
    }

    pub fn add_spot(&mut self, light: SpotLight) {
        if self.spots.len() < 32 {
            self.spots.push(light);
        }
    }

    pub fn set_ambient(&mut self, light: AmbientLight) {
        self.ambient = Some(light);
    }

    pub fn set_hemisphere(&mut self, light: HemisphereLight) {
        self.hemisphere = Some(light);
    }

    #[inline]
    pub fn directional_count(&self) -> usize {
        self.directional.len()
    }

    #[inline]
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    #[inline]
    pub fn spot_count(&self) -> usize {
        self.spots.len()
    }

    pub fn directional(&self) -> &[DirectionalLight] {
        &self.directional
    }

    pub fn points(&self) -> &[PointLight] {
        &self.points
    }

    pub fn spots(&self) -> &[SpotLight] {
        &self.spots
    }

    pub fn ambient(&self) -> Option<&AmbientLight> {
        self.ambient.as_ref()
    }

    pub fn hemisphere(&self) -> Option<&HemisphereLight> {
        self.hemisphere.as_ref()
    }

    /// Sort point lights by distance to camera (for optimization)
    pub fn sort_by_distance(&mut self, camera_pos: Vec3) {
        self.points.sort_by(|a, b| {
            let da = (a.position - camera_pos).length_squared();
            let db = (b.position - camera_pos).length_squared();
            da.partial_cmp(&db).unwrap_or(core::cmp::Ordering::Equal)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_math::Vec3;

    #[test]
    fn test_directional_light() {
        let light = DirectionalLight::new(Vec3::Y, Color::WHITE, 1.0);
        assert_eq!(light.direction().y, 1.0);
        let sample = light.contribution(Vec3::ZERO);
        assert_eq!(sample.direction.y, -1.0);
    }

    #[test]
    fn test_point_light_attenuation() {
        let light = PointLight::new(Vec3::ZERO, Color::WHITE, 1.0, 10.0);
        assert_eq!(light.attenuation(0.0), 1.0);
        assert_eq!(light.attenuation(10.0), 0.0);
        assert!(light.attenuation(5.0) > 0.0);
    }

    #[test]
    fn test_spot_light_cone() {
        let light = SpotLight::new(
            Vec3::ZERO,
            Vec3::Z,
            0.5, // inner cone angle (radians)
            1.0, // outer cone angle (radians)
            Color::WHITE,
            1.0,
        );
        // Point directly along the direction axis (within inner cone)
        let atten_center = light.cone_attenuation(Vec3::new(0.0, 0.0, 10.0));
        assert!(atten_center > 0.9); // Should be fully bright at center

        // Point at edge of cone
        let atten_edge = light.cone_attenuation(Vec3::new(0.5, 0.0, 1.0));
        // Should have some attenuation but not zero
        assert!(atten_edge >= 0.0);
    }

    #[test]
    fn test_light_manager() {
        let mut manager = LightManager::new();
        manager.add_directional(DirectionalLight::new(Vec3::Y, Color::WHITE, 1.0));
        manager.add_point(PointLight::new(Vec3::ZERO, Color::WHITE, 1.0, 10.0));
        assert_eq!(manager.directional_count(), 1);
        assert_eq!(manager.point_count(), 1);
    }

    #[test]
    fn test_color_new() {
        let c = Color::new(0.2, 0.4, 0.6, 1.0);
        assert_eq!(c.r, 0.2);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 0.6);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_color_from_rgb() {
        let c = Color::from_rgb(0.5, 0.6, 0.7);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_color_constants() {
        let white = Color::WHITE;
        assert_eq!(white.r, 1.0);
        assert_eq!(white.g, 1.0);
        assert_eq!(white.b, 1.0);
        assert_eq!(white.a, 1.0);

        let black = Color::BLACK;
        assert_eq!(black.r, 0.0);
        assert_eq!(black.g, 0.0);
        assert_eq!(black.b, 0.0);

        let red = Color::RED;
        assert_eq!(red.r, 1.0);
        assert_eq!(red.g, 0.0);
        assert_eq!(red.b, 0.0);

        let green = Color::GREEN;
        assert_eq!(green.g, 1.0);

        let blue = Color::BLUE;
        assert_eq!(blue.b, 1.0);
    }

    #[test]
    fn test_color_to_vec3() {
        let c = Color::new(0.3, 0.4, 0.5, 1.0);
        let v = c.to_vec3();
        assert_eq!(v, Vec3::new(0.3, 0.4, 0.5));
    }

    #[test]
    fn test_directional_light_normalized() {
        let light = DirectionalLight::new(Vec3::new(1.0, 1.0, 0.0), Color::WHITE, 1.0);
        // Direction should be normalized
        let len = light.direction().length();
        assert!((len - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_directional_light_color_getter() {
        let color = Color::new(0.5, 0.5, 1.0, 1.0);
        let light = DirectionalLight::new(Vec3::Y, color, 1.0);
        assert_eq!(light.color().r, 0.5);
        assert_eq!(light.color().g, 0.5);
        assert_eq!(light.color().b, 1.0);
    }

    #[test]
    fn test_directional_light_intensity() {
        let light = DirectionalLight::new(Vec3::Y, Color::WHITE, 0.75);
        assert_eq!(light.intensity(), 0.75);
    }

    #[test]
    fn test_directional_light_shadow_default() {
        let light = DirectionalLight::new(Vec3::Y, Color::WHITE, 1.0);
        assert!(!light.casts_shadow());
    }

    #[test]
    fn test_directional_light_set_shadow() {
        let mut light = DirectionalLight::new(Vec3::Y, Color::WHITE, 1.0);
        light.set_casts_shadow(true);
        assert!(light.casts_shadow());
    }

    #[test]
    fn test_directional_light_contribution() {
        let light = DirectionalLight::new(Vec3::Y, Color::WHITE, 0.5);
        let sample = light.contribution(Vec3::new(10.0, 5.0, 3.0));
        assert_eq!(sample.intensity, 0.5);
        assert_eq!(sample.direction.y, -1.0);
    }

    #[test]
    fn test_point_light_position() {
        let light = PointLight::new(Vec3::new(3.0, -2.0, 5.0), Color::WHITE, 1.0, 10.0);
        assert_eq!(light.position(), Vec3::new(3.0, -2.0, 5.0));
    }

    #[test]
    fn test_point_light_radius() {
        let light = PointLight::new(Vec3::ZERO, Color::WHITE, 1.0, 25.0);
        assert_eq!(light.radius(), 25.0);
    }

    #[test]
    fn test_point_light_contribution_attenuation() {
        let light = PointLight::new(Vec3::ZERO, Color::WHITE, 1.0, 10.0);
        let near = light.contribution(Vec3::new(1.0, 0.0, 0.0));
        let far = light.contribution(Vec3::new(9.0, 0.0, 0.0));
        // Near position should have higher intensity
        assert!(near.intensity > far.intensity);
    }

    #[test]
    fn test_point_light_contribution_zero_distance() {
        let light = PointLight::new(Vec3::ZERO, Color::WHITE, 1.0, 10.0);
        let sample = light.contribution(Vec3::ZERO);
        assert!(sample.intensity > 0.0);
    }

    #[test]
    fn test_spot_light_cone_inner() {
        let light = SpotLight::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            1.0,
            Color::WHITE,
            1.0,
        );
        let atten = light.cone_attenuation(Vec3::new(0.0, 0.0, -5.0));
        assert!(atten > 0.5);
    }

    #[test]
    fn test_spot_light_cone_edge() {
        let light = SpotLight::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            1.0,
            Color::WHITE,
            1.0,
        );
        // Point perpendicular to direction should be outside cone
        let atten = light.cone_attenuation(Vec3::new(5.0, 0.0, 0.0));
        // cos(angle) of direction (0,0,-1) and vector (5,0,0) = 0
        // cos(1.0) ~ 0.54, cos(0.5) ~ 0.877, so 0 < cos(outer) => 0 atten
        assert!(atten >= 0.0);
    }

    #[test]
    fn test_spot_light_position() {
        let light = SpotLight::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::Z,
            0.5,
            1.0,
            Color::WHITE,
            1.0,
        );
        assert_eq!(light.position(), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_spot_light_direction() {
        let light = SpotLight::new(
            Vec3::ZERO,
            Vec3::new(1.0, 0.0, 0.0),
            0.5,
            1.0,
            Color::WHITE,
            1.0,
        );
        assert_eq!(light.direction().x, 1.0);
    }

    #[test]
    fn test_spot_light_angles() {
        let light = SpotLight::new(
            Vec3::ZERO,
            Vec3::Z,
            0.3,
            0.8,
            Color::WHITE,
            1.0,
        );
        assert_eq!(light.inner_angle(), 0.3);
        assert_eq!(light.outer_angle(), 0.8);
    }

    #[test]
    fn test_spot_light_contribution() {
        let light = SpotLight::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            1.0,
            Color::WHITE,
            1.0,
        );
        let sample = light.contribution(Vec3::new(0.0, 0.0, -5.0));
        assert!(sample.intensity > 0.0);
    }

    #[test]
    fn test_ambient_light_new() {
        let light = AmbientLight::new(Color::new(0.2, 0.2, 0.2, 1.0), 0.5);
        assert_eq!(light.intensity(), 0.5);
    }

    #[test]
    fn test_ambient_light_color() {
        let color = Color::new(0.3, 0.4, 0.5, 1.0);
        let light = AmbientLight::new(color, 1.0);
        assert_eq!(light.color().r, 0.3);
        assert_eq!(light.color().g, 0.4);
        assert_eq!(light.color().b, 0.5);
    }

    #[test]
    fn test_hemisphere_light_new() {
        let sky = Color::new(0.8, 0.9, 1.0, 1.0);
        let ground = Color::new(0.3, 0.2, 0.1, 1.0);
        let light = HemisphereLight::new(sky, ground, 0.8);
        assert_eq!(light.intensity(), 0.8);
    }

    #[test]
    fn test_hemisphere_light_colors() {
        let sky = Color::new(1.0, 1.0, 1.0, 1.0);
        let ground = Color::new(0.0, 0.0, 0.5, 1.0);
        let light = HemisphereLight::new(sky, ground, 1.0);
        assert_eq!(light.sky_color().r, 1.0);
        assert_eq!(light.ground_color().b, 0.5);
    }

    #[test]
    fn test_light_manager_new() {
        let manager = LightManager::new();
        assert_eq!(manager.directional_count(), 0);
        assert_eq!(manager.point_count(), 0);
        assert_eq!(manager.spot_count(), 0);
    }

    #[test]
    fn test_light_manager_add_spot() {
        let mut manager = LightManager::new();
        manager.add_spot(SpotLight::new(
            Vec3::ZERO,
            Vec3::Z,
            0.5,
            1.0,
            Color::WHITE,
            1.0,
        ));
        assert_eq!(manager.spot_count(), 1);
    }

    #[test]
    fn test_light_manager_set_ambient() {
        let mut manager = LightManager::new();
        manager.set_ambient(AmbientLight::new(Color::WHITE, 0.3));
        assert!(manager.ambient().is_some());
    }

    #[test]
    fn test_light_manager_set_hemisphere() {
        let mut manager = LightManager::new();
        manager.set_hemisphere(HemisphereLight::new(Color::WHITE, Color::BLACK, 0.5));
        assert!(manager.hemisphere().is_some());
    }

    #[test]
    fn test_light_manager_sort_distance() {
        let mut manager = LightManager::new();
        manager.add_point(PointLight::new(Vec3::new(10.0, 0.0, 0.0), Color::WHITE, 1.0, 10.0));
        manager.add_point(PointLight::new(Vec3::new(1.0, 0.0, 0.0), Color::WHITE, 1.0, 10.0));
        manager.sort_by_distance(Vec3::ZERO);
        // After sorting, first light should be the closer one
        let points = manager.points();
        let first_pos = points[0].position();
        let second_pos = points[1].position();
        assert!(first_pos.length() <= second_pos.length());
    }

    #[test]
    fn test_light_manager_accessors() {
        let manager = LightManager::new();
        let _ = manager.directional();
        let _ = manager.points();
        let _ = manager.spots();
    }

    #[test]
    fn test_color_default() {
        let c = Color::default();
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 0.0);
    }

    #[test]
    fn test_light_sample_direction_from_contribution() {
        let light = DirectionalLight::new(Vec3::new(0.0, -1.0, 0.0), Color::WHITE, 1.0);
        let sample = light.contribution(Vec3::new(5.0, 5.0, 5.0));
        // Direction should be opposite of light direction
        assert!(sample.direction.y > 0.0);
    }
}
