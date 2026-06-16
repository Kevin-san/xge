//! Lighting module - 2D/3D lighting system
//!
//! Provides different light types and a lighting system manager
//! for computing lighting in real-time rendering.

use engine_math::{Vec2, Vec3, Vec4};

/// Light types supported by the rendering system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightType {
    /// Directional light (sun-like, parallel rays)
    Directional,
    /// Point light (spherical falloff from a position)
    Point,
    /// Spot light (cone-shaped illumination)
    Spot,
}

/// Represents a light source with position, color, and attenuation
#[derive(Debug, Clone)]
pub struct Light {
    /// Type of light
    pub light_type: LightType,
    /// Position for point/spot lights
    pub position: Vec2,
    /// 3D position for 3D rendering
    pub position_3d: Vec3,
    /// Direction (for directional/spot lights)
    pub direction: Vec2,
    /// 3D direction
    pub direction_3d: Vec3,
    /// RGBA color
    pub color: Vec4,
    /// Intensity multiplier
    pub intensity: f32,
    /// Falloff radius (for point lights)
    pub radius: f32,
    /// Spotlight cone angle (radians)
    pub spot_angle: f32,
    /// Spotlight outer cone angle (for soft edges)
    pub spot_outer_angle: f32,
    /// Constant attenuation term
    pub attenuation_const: f32,
    /// Linear attenuation term
    pub attenuation_linear: f32,
    /// Quadratic attenuation term
    pub attenuation_quad: f32,
}

impl Light {
    /// Create a new directional light
    pub fn directional(direction: Vec2, color: Vec4, intensity: f32) -> Self {
        let dir_3d = Vec3::new(direction.x, 0.0, direction.y).normalize();
        Self {
            light_type: LightType::Directional,
            position: Vec2::ZERO,
            position_3d: Vec3::ZERO,
            direction,
            direction_3d: dir_3d,
            color,
            intensity,
            radius: f32::INFINITY,
            spot_angle: std::f32::consts::PI / 4.0,
            spot_outer_angle: std::f32::consts::PI / 3.0,
            attenuation_const: 1.0,
            attenuation_linear: 0.0,
            attenuation_quad: 0.0,
        }
    }
    
    /// Create a 3D directional light
    pub fn directional_3d(direction: Vec3, color: Vec4, intensity: f32) -> Self {
        let dir_2d = Vec2::new(direction.x, direction.z).normalize();
        Self {
            light_type: LightType::Directional,
            position: Vec2::ZERO,
            position_3d: Vec3::ZERO,
            direction: dir_2d,
            direction_3d: direction.normalize(),
            color,
            intensity,
            radius: f32::INFINITY,
            spot_angle: std::f32::consts::PI / 4.0,
            spot_outer_angle: std::f32::consts::PI / 3.0,
            attenuation_const: 1.0,
            attenuation_linear: 0.0,
            attenuation_quad: 0.0,
        }
    }
    
    /// Create a new point light
    pub fn point(position: Vec2, color: Vec4, intensity: f32, radius: f32) -> Self {
        Self {
            light_type: LightType::Point,
            position,
            position_3d: Vec3::new(position.x, 0.0, position.y),
            direction: Vec2::ZERO,
            direction_3d: Vec3::ZERO,
            color,
            intensity,
            radius,
            spot_angle: std::f32::consts::PI,
            spot_outer_angle: std::f32::consts::PI,
            attenuation_const: 1.0,
            attenuation_linear: 0.09,
            attenuation_quad: 0.032,
        }
    }
    
    /// Create a 3D point light
    pub fn point_3d(position: Vec3, color: Vec4, intensity: f32, radius: f32) -> Self {
        Self {
            light_type: LightType::Point,
            position: Vec2::new(position.x, position.z),
            position_3d: position,
            direction: Vec2::ZERO,
            direction_3d: Vec3::ZERO,
            color,
            intensity,
            radius,
            spot_angle: std::f32::consts::PI,
            spot_outer_angle: std::f32::consts::PI,
            attenuation_const: 1.0,
            attenuation_linear: 0.09,
            attenuation_quad: 0.032,
        }
    }
    
    /// Create a spotlight
    pub fn spot(
        position: Vec2,
        direction: Vec2,
        color: Vec4,
        intensity: f32,
        radius: f32,
        angle: f32,
    ) -> Self {
        Self {
            light_type: LightType::Spot,
            position,
            position_3d: Vec3::new(position.x, 0.0, position.y),
            direction: direction.normalize(),
            direction_3d: Vec3::new(direction.x, 0.0, direction.y).normalize(),
            color,
            intensity,
            radius,
            spot_angle: angle,
            spot_outer_angle: angle * 1.2,
            attenuation_const: 1.0,
            attenuation_linear: 0.09,
            attenuation_quad: 0.032,
        }
    }
    
    /// Calculate intensity at a 2D position
    pub fn intensity_at(&self, pos: Vec2) -> f32 {
        match self.light_type {
            LightType::Directional => self.intensity,
            LightType::Point => self.point_intensity_at(pos),
            LightType::Spot => self.spot_intensity_at(pos),
        }
    }
    
    /// Calculate intensity at a 3D position
    pub fn intensity_at_3d(&self, pos: Vec3) -> f32 {
        match self.light_type {
            LightType::Directional => self.intensity,
            LightType::Point => self.point_intensity_at_3d(pos),
            LightType::Spot => self.spot_intensity_at_3d(pos),
        }
    }
    
    /// Point light intensity at 2D position
    fn point_intensity_at(&self, pos: Vec2) -> f32 {
        let dist = (pos - self.position).length();
        if dist > self.radius {
            return 0.0;
        }
        self.intensity / (self.attenuation_const 
            + self.attenuation_linear * dist 
            + self.attenuation_quad * dist * dist)
    }
    
    /// Point light intensity at 3D position
    fn point_intensity_at_3d(&self, pos: Vec3) -> f32 {
        let dist = (pos - self.position_3d).length();
        if dist > self.radius {
            return 0.0;
        }
        self.intensity / (self.attenuation_const 
            + self.attenuation_linear * dist 
            + self.attenuation_quad * dist * dist)
    }
    
    /// Spotlight intensity at 2D position
    fn spot_intensity_at(&self, pos: Vec2) -> f32 {
        let to_light = pos - self.position;
        let dist = to_light.length();
        
        if dist > self.radius {
            return 0.0;
        }
        
        let dir_to_pos = to_light.normalize();
        let angle = self.direction.dot(dir_to_pos).acos();
        
        if angle > self.spot_outer_angle {
            return 0.0;
        }
        
        // Soft edge falloff
        let edge_fade = if angle > self.spot_angle {
            1.0 - (angle - self.spot_angle) / (self.spot_outer_angle - self.spot_angle)
        } else {
            1.0
        };
        
        let attenuation = self.intensity / (self.attenuation_const 
            + self.attenuation_linear * dist 
            + self.attenuation_quad * dist * dist);
        
        attenuation * edge_fade
    }
    
    /// Spotlight intensity at 3D position
    fn spot_intensity_at_3d(&self, pos: Vec3) -> f32 {
        let to_light = pos - self.position_3d;
        let dist = to_light.length();
        
        if dist > self.radius {
            return 0.0;
        }
        
        let dir_to_pos = to_light.normalize();
        let angle = self.direction_3d.dot(dir_to_pos).acos();
        
        if angle > self.spot_outer_angle {
            return 0.0;
        }
        
        let edge_fade = if angle > self.spot_angle {
            1.0 - (angle - self.spot_angle) / (self.spot_outer_angle - self.spot_angle)
        } else {
            1.0
        };
        
        let attenuation = self.intensity / (self.attenuation_const 
            + self.attenuation_linear * dist 
            + self.attenuation_quad * dist * dist);
        
        attenuation * edge_fade
    }
    
    /// Get light position as Vec3
    pub fn get_position_3d(&self) -> Vec3 {
        self.position_3d
    }
    
    /// Get light direction as Vec3
    pub fn get_direction_3d(&self) -> Vec3 {
        self.direction_3d
    }
}

/// Manages multiple lights and computes lighting
pub struct LightingSystem {
    /// All lights in the scene
    lights: Vec<Light>,
    /// Ambient light color
    ambient: Vec4,
    /// Maximum number of lights
    max_lights: usize,
    /// Whether to use shadows
    shadows_enabled: bool,
    /// Shadow map resolution
    shadow_map_resolution: u32,
}

impl LightingSystem {
    /// Create a new lighting system
    pub fn new(max_lights: usize) -> Self {
        Self {
            lights: Vec::with_capacity(max_lights),
            ambient: Vec4::new(0.03, 0.03, 0.05, 1.0),
            max_lights,
            shadows_enabled: true,
            shadow_map_resolution: 2048,
        }
    }
    
    /// Add a light to the system
    pub fn add_light(&mut self, light: Light) -> Option<usize> {
        if self.lights.len() < self.max_lights {
            self.lights.push(light);
            Some(self.lights.len() - 1)
        } else {
            None
        }
    }
    
    /// Remove a light by index
    pub fn remove_light(&mut self, index: usize) {
        if index < self.lights.len() {
            self.lights.remove(index);
        }
    }
    
    /// Get a light by index
    pub fn get_light(&self, index: usize) -> Option<&Light> {
        self.lights.get(index)
    }
    
    /// Get a mutable light by index
    pub fn get_light_mut(&mut self, index: usize) -> Option<&mut Light> {
        self.lights.get_mut(index)
    }
    
    /// Set ambient light color
    pub fn set_ambient(&mut self, color: Vec4) {
        self.ambient = color;
    }
    
    /// Get ambient light color
    pub fn ambient(&self) -> Vec4 {
        self.ambient
    }
    
    /// Get all lights
    pub fn lights(&self) -> &[Light] {
        &self.lights
    }
    
    /// Get number of lights
    pub fn light_count(&self) -> usize {
        self.lights.len()
    }
    
    /// Clear all lights
    pub fn clear(&mut self) {
        self.lights.clear();
    }
    
    /// Enable/disable shadows
    pub fn set_shadows_enabled(&mut self, enabled: bool) {
        self.shadows_enabled = enabled;
    }
    
    /// Check if shadows are enabled
    pub fn shadows_enabled(&self) -> bool {
        self.shadows_enabled
    }
    
    /// Set shadow map resolution
    pub fn set_shadow_map_resolution(&mut self, resolution: u32) {
        self.shadow_map_resolution = resolution;
    }
    
    /// Get shadow map resolution
    pub fn shadow_map_resolution(&self) -> u32 {
        self.shadow_map_resolution
    }
    
    /// Calculate total intensity at a 2D position
    pub fn total_intensity_at(&self, pos: Vec2) -> f32 {
        let mut total = self.ambient.w * self.ambient.w;
        
        for light in &self.lights {
            total += light.intensity_at(pos);
        }
        
        total.min(1.0)
    }
    
    /// Calculate total intensity at a 3D position
    pub fn total_intensity_at_3d(&self, pos: Vec3) -> f32 {
        let mut total = self.ambient.w * self.ambient.w;
        
        for light in &self.lights {
            total += light.intensity_at_3d(pos);
        }
        
        total.min(1.0)
    }
    
    /// Calculate combined color at a 2D position
    pub fn color_at(&self, pos: Vec2) -> Vec4 {
        let mut total_color = self.ambient * self.ambient.w;
        
        for light in &self.lights {
            let intensity = light.intensity_at(pos);
            if intensity > 0.0 {
                total_color = total_color + light.color * intensity;
            }
        }
        
        // Clamp color values
        Vec4::new(
            total_color.x.min(1.0),
            total_color.y.min(1.0),
            total_color.z.min(1.0),
            1.0,
        )
    }
    
    /// Calculate combined color at a 3D position
    pub fn color_at_3d(&self, pos: Vec3) -> Vec4 {
        let mut total_color = self.ambient * self.ambient.w;
        
        for light in &self.lights {
            let intensity = light.intensity_at_3d(pos);
            if intensity > 0.0 {
                total_color = total_color + light.color * intensity;
            }
        }
        
        // Clamp color values
        Vec4::new(
            total_color.x.min(1.0),
            total_color.y.min(1.0),
            total_color.z.min(1.0),
            1.0,
        )
    }
    
    /// Get the main directional light (for shadows)
    pub fn get_main_light(&self) -> Option<&Light> {
        self.lights.iter().find(|l| l.light_type == LightType::Directional)
    }
    
    /// Get all point lights
    pub fn get_point_lights(&self) -> Vec<&Light> {
        self.lights.iter().filter(|l| l.light_type == LightType::Point).collect()
    }
    
    /// Get all spot lights
    pub fn get_spot_lights(&self) -> Vec<&Light> {
        self.lights.iter().filter(|l| l.light_type == LightType::Spot).collect()
    }
}

impl Default for LightingSystem {
    fn default() -> Self {
        Self::new(8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_directional_light() {
        let light = Light::directional(
            Vec2::new(-0.5, -1.0).normalize(),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            1.0,
        );
        
        assert_eq!(light.light_type, LightType::Directional);
        assert_eq!(light.intensity_at(Vec2::new(100.0, 100.0)), 1.0);
    }
    
    #[test]
    fn test_point_light() {
        let light = Light::point(
            Vec2::new(100.0, 100.0),
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            2.0,
            100.0,
        );
        
        assert_eq!(light.light_type, LightType::Point);
        
        // At center
        let intensity = light.intensity_at(Vec2::new(100.0, 100.0));
        assert!(intensity > 0.0 && intensity <= 2.0);
        
        // Outside radius
        let intensity = light.intensity_at(Vec2::new(300.0, 100.0));
        assert_eq!(intensity, 0.0);
    }
    
    #[test]
    fn test_lighting_system() {
        let mut system = LightingSystem::new(4);
        
        let light1 = Light::point(
            Vec2::new(100.0, 100.0),
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            1.0,
            100.0,
        );
        let light2 = Light::point(
            Vec2::new(200.0, 100.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            1.0,
            100.0,
        );
        
        system.add_light(light1);
        system.add_light(light2);
        
        assert_eq!(system.light_count(), 2);
        
        system.clear();
        assert_eq!(system.light_count(), 0);
    }
}
