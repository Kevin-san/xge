//! PBR Material - Core material definition for physically-based rendering

use engine_math::Vec3;
use serde::{Deserialize, Serialize};

use crate::PbrMaterialFlags;

/// Alpha blending mode for materials
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlphaMode {
    /// Fully opaque, no transparency
    #[default]
    Opaque,
    /// Alpha mask with cutoff threshold (binary transparency)
    Mask,
    /// Alpha blending for smooth transparency
    Blend,
}

/// PBR Material definition following metal/roughness workflow.
///
/// Supports albedo, metallic, roughness, normal, AO, emissive, and height maps
/// with corresponding constant values. Additional features include clear coat,
/// anisotropy, sheen, and subsurface scattering.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PbrMaterial {
    // Base color
    /// Albedo/base color constant value
    pub albedo: [f32; 3],
    /// Albedo texture path (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albedo_map: Option<String>,

    // Metallic
    /// Metallic factor (0.0 = non-metal, 1.0 = metal)
    pub metallic: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallic_map: Option<String>,

    // Roughness
    /// Roughness factor (0.0 = smooth, 1.0 = rough)
    pub roughness: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roughness_map: Option<String>,

    // Normal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_map: Option<String>,
    /// Normal map intensity multiplier
    pub normal_strength: f32,

    // Ambient Occlusion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ao_map: Option<String>,
    /// AO strength multiplier
    pub ao_strength: f32,

    // Emissive
    /// Emissive color constant
    pub emissive: [f32; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive_map: Option<String>,
    /// Emissive intensity multiplier
    pub emissive_intensity: f32,

    // Height/Parallax
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_map: Option<String>,
    /// Parallax occlusion mapping strength
    pub parallax_strength: f32,

    // Clear Coat
    /// Clear coat layer intensity
    pub clear_coat: f32,
    /// Clear coat roughness
    pub clear_coat_roughness: f32,

    // Anisotropy
    /// Anisotropic reflection strength
    pub anisotropy: f32,

    // Sheen
    /// Sheen color (fabric-like soft highlights)
    pub sheen: [f32; 3],
    /// Sheen roughness
    pub sheen_roughness: f32,

    // Subsurface
    /// Subsurface scattering intensity
    pub subsurface: f32,

    // Alpha
    /// Alpha blending mode
    pub alpha_mode: AlphaMode,
    /// Alpha cutoff threshold for Mask mode
    pub alpha_cutoff: f32,

    // Rendering flags
    /// Render both sides of geometry
    pub double_sided: bool,
    /// Material casts shadows
    pub casts_shadow: bool,
    /// Material receives shadows
    pub receives_shadow: bool,
}

impl PbrMaterial {
    /// Create a default white material with no textures
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a material with a specific albedo color
    pub fn from_albedo(color: Vec3) -> Self {
        Self {
            albedo: [color.x, color.y, color.z],
            ..Self::default()
        }
    }

    /// Get albedo as Vec3
    pub fn albedo_vec3(&self) -> Vec3 {
        Vec3::new(self.albedo[0], self.albedo[1], self.albedo[2])
    }

    /// Set albedo from Vec3
    pub fn set_albedo_vec3(&mut self, color: Vec3) {
        self.albedo = [color.x, color.y, color.z];
    }

    /// Get emissive as Vec3
    pub fn emissive_vec3(&self) -> Vec3 {
        Vec3::new(self.emissive[0], self.emissive[1], self.emissive[2])
    }

    /// Set emissive from Vec3
    pub fn set_emissive_vec3(&mut self, color: Vec3) {
        self.emissive = [color.x, color.y, color.z];
    }

    /// Get sheen as Vec3
    pub fn sheen_vec3(&self) -> Vec3 {
        Vec3::new(self.sheen[0], self.sheen[1], self.sheen[2])
    }

    /// Set sheen from Vec3
    pub fn set_sheen_vec3(&mut self, color: Vec3) {
        self.sheen = [color.x, color.y, color.z];
    }

    /// Get the material flags based on enabled features
    pub fn flags(&self) -> PbrMaterialFlags {
        let mut flags = PbrMaterialFlags::empty();

        if self.albedo_map.is_some() {
            flags.insert(PbrMaterialFlags::HAS_ALBEDO_MAP);
        }
        if self.normal_map.is_some() {
            flags.insert(PbrMaterialFlags::HAS_NORMAL_MAP);
        }
        if self.metallic_map.is_some() {
            flags.insert(PbrMaterialFlags::HAS_METALLIC_MAP);
        }
        if self.roughness_map.is_some() {
            flags.insert(PbrMaterialFlags::HAS_ROUGHNESS_MAP);
        }
        if self.ao_map.is_some() {
            flags.insert(PbrMaterialFlags::HAS_AO_MAP);
        }
        if self.emissive_map.is_some() {
            flags.insert(PbrMaterialFlags::HAS_EMISSIVE_MAP);
        }
        if self.height_map.is_some() {
            flags.insert(PbrMaterialFlags::HAS_HEIGHT_MAP);
        }

        if self.clear_coat > 0.0 {
            flags.insert(PbrMaterialFlags::USE_CLEAR_COAT);
        }
        if self.anisotropy > 0.0 {
            flags.insert(PbrMaterialFlags::USE_ANISOTROPY);
        }
        if self.sheen_roughness > 0.0 {
            flags.insert(PbrMaterialFlags::USE_SHEEN);
        }
        if self.subsurface > 0.0 {
            flags.insert(PbrMaterialFlags::USE_SUBSURFACE);
        }
        if self.parallax_strength > 0.0 && self.height_map.is_some() {
            flags.insert(PbrMaterialFlags::USE_PARALLAX);
        }

        flags
    }

    /// Serialize material to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize material from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Default for PbrMaterial {
    fn default() -> Self {
        Self {
            albedo: [1.0, 1.0, 1.0],
            albedo_map: None,
            metallic: 0.0,
            metallic_map: None,
            roughness: 0.5,
            roughness_map: None,
            normal_map: None,
            normal_strength: 1.0,
            ao_map: None,
            ao_strength: 1.0,
            emissive: [0.0, 0.0, 0.0],
            emissive_map: None,
            emissive_intensity: 0.0,
            height_map: None,
            parallax_strength: 0.0,
            clear_coat: 0.0,
            clear_coat_roughness: 0.0,
            anisotropy: 0.0,
            sheen: [0.0, 0.0, 0.0],
            sheen_roughness: 0.0,
            subsurface: 0.0,
            alpha_mode: AlphaMode::Opaque,
            alpha_cutoff: 0.5,
            double_sided: false,
            casts_shadow: true,
            receives_shadow: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_default() {
        let mat = PbrMaterial::default();
        assert_eq!(mat.albedo, [1.0, 1.0, 1.0]);
        assert_eq!(mat.metallic, 0.0);
        assert_eq!(mat.roughness, 0.5);
        assert_eq!(mat.alpha_mode, AlphaMode::Opaque);
    }

    #[test]
    fn test_material_from_albedo() {
        let mat = PbrMaterial::from_albedo(Vec3::new(0.5, 0.3, 0.2));
        assert_eq!(mat.albedo, [0.5, 0.3, 0.2]);
        assert_eq!(mat.metallic, 0.0);
    }

    #[test]
    fn test_material_flags_empty() {
        let mat = PbrMaterial::default();
        let flags = mat.flags();
        assert!(!flags.contains(PbrMaterialFlags::HAS_ALBEDO_MAP));
        assert!(!flags.contains(PbrMaterialFlags::HAS_NORMAL_MAP));
    }

    #[test]
    fn test_material_flags_with_textures() {
        let mat = PbrMaterial {
            albedo_map: Some("albedo.png".to_string()),
            normal_map: Some("normal.png".to_string()),
            ..PbrMaterial::default()
        };
        let flags = mat.flags();
        assert!(flags.contains(PbrMaterialFlags::HAS_ALBEDO_MAP));
        assert!(flags.contains(PbrMaterialFlags::HAS_NORMAL_MAP));
    }

    #[test]
    fn test_material_flags_with_clear_coat() {
        let mat = PbrMaterial {
            clear_coat: 1.0,
            ..PbrMaterial::default()
        };
        let flags = mat.flags();
        assert!(flags.contains(PbrMaterialFlags::USE_CLEAR_COAT));
    }

    #[test]
    fn test_material_json_roundtrip() {
        let mat = PbrMaterial {
            albedo: [0.8, 0.2, 0.1],
            metallic: 0.9,
            roughness: 0.3,
            ..PbrMaterial::default()
        };
        let json = mat.to_json().unwrap();
        let parsed = PbrMaterial::from_json(&json).unwrap();
        assert_eq!(mat.albedo, parsed.albedo);
        assert_eq!(mat.metallic, parsed.metallic);
        assert_eq!(mat.roughness, parsed.roughness);
    }

    #[test]
    fn test_material_vec3_methods() {
        let mut mat = PbrMaterial::default();
        mat.set_albedo_vec3(Vec3::new(0.5, 0.6, 0.7));
        assert_eq!(mat.albedo_vec3(), Vec3::new(0.5, 0.6, 0.7));

        mat.set_emissive_vec3(Vec3::new(1.0, 0.5, 0.0));
        assert_eq!(mat.emissive_vec3(), Vec3::new(1.0, 0.5, 0.0));

        mat.set_sheen_vec3(Vec3::new(0.2, 0.3, 0.4));
        assert_eq!(mat.sheen_vec3(), Vec3::new(0.2, 0.3, 0.4));
    }
}