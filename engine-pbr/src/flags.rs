//! PBR Material Flags - Bitflags for material features

use core::ops::{BitAnd, BitOr, BitOrAssign};

/// Bitflags indicating which features a PBR material uses.
///
/// These flags are used for shader permutation selection and GPU binding optimization.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PbrMaterialFlags(u32);

impl PbrMaterialFlags {
    /// Material has an albedo (base color) texture map
    pub const HAS_ALBEDO_MAP: Self = Self(1 << 0);
    /// Material has a normal map
    pub const HAS_NORMAL_MAP: Self = Self(1 << 1);
    /// Material has a metallic map
    pub const HAS_METALLIC_MAP: Self = Self(1 << 2);
    /// Material has a roughness map
    pub const HAS_ROUGHNESS_MAP: Self = Self(1 << 3);
    /// Material has an ambient occlusion map
    pub const HAS_AO_MAP: Self = Self(1 << 4);
    /// Material has an emissive map
    pub const HAS_EMISSIVE_MAP: Self = Self(1 << 5);
    /// Material has a height/displacement map
    pub const HAS_HEIGHT_MAP: Self = Self(1 << 6);
    /// Material uses image-based lighting
    pub const USE_IBL: Self = Self(1 << 7);
    /// Material has clear coat layer
    pub const USE_CLEAR_COAT: Self = Self(1 << 8);
    /// Material has anisotropic reflections
    pub const USE_ANISOTROPY: Self = Self(1 << 9);
    /// Material has sheen (fabric-like soft highlights)
    pub const USE_SHEEN: Self = Self(1 << 10);
    /// Material has subsurface scattering
    pub const USE_SUBSURFACE: Self = Self(1 << 11);
    /// Material uses parallax occlusion mapping
    pub const USE_PARALLAX: Self = Self(1 << 12);

    /// Create empty flags (no features enabled)
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Create flags with all features enabled
    #[inline]
    pub const fn all() -> Self {
        Self(0xFFFF)
    }

    /// Check if a specific flag is set
    #[inline]
    pub const fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    /// Set a flag
    #[inline]
    pub const fn insert(&mut self, flag: Self) {
        self.0 |= flag.0;
    }

    /// Remove a flag
    #[inline]
    pub const fn remove(&mut self, flag: Self) {
        self.0 &= !flag.0;
    }

    /// Get the raw bit value
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Create flags from raw bits
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Check if any texture map flags are set
    #[inline]
    pub const fn has_any_texture(self) -> bool {
        self.contains(Self::HAS_ALBEDO_MAP)
            || self.contains(Self::HAS_NORMAL_MAP)
            || self.contains(Self::HAS_METALLIC_MAP)
            || self.contains(Self::HAS_ROUGHNESS_MAP)
            || self.contains(Self::HAS_AO_MAP)
            || self.contains(Self::HAS_EMISSIVE_MAP)
            || self.contains(Self::HAS_HEIGHT_MAP)
    }
}

impl BitOr for PbrMaterialFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for PbrMaterialFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for PbrMaterialFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags_empty() {
        let flags = PbrMaterialFlags::empty();
        assert!(!flags.contains(PbrMaterialFlags::HAS_ALBEDO_MAP));
        assert!(!flags.contains(PbrMaterialFlags::HAS_NORMAL_MAP));
    }

    #[test]
    fn test_flags_insert() {
        let mut flags = PbrMaterialFlags::empty();
        flags.insert(PbrMaterialFlags::HAS_ALBEDO_MAP);
        assert!(flags.contains(PbrMaterialFlags::HAS_ALBEDO_MAP));
        assert!(!flags.contains(PbrMaterialFlags::HAS_NORMAL_MAP));
    }

    #[test]
    fn test_flags_combine() {
        let flags = PbrMaterialFlags::HAS_ALBEDO_MAP | PbrMaterialFlags::HAS_NORMAL_MAP;
        assert!(flags.contains(PbrMaterialFlags::HAS_ALBEDO_MAP));
        assert!(flags.contains(PbrMaterialFlags::HAS_NORMAL_MAP));
    }

    #[test]
    fn test_flags_remove() {
        let mut flags = PbrMaterialFlags::HAS_ALBEDO_MAP | PbrMaterialFlags::HAS_NORMAL_MAP;
        flags.remove(PbrMaterialFlags::HAS_ALBEDO_MAP);
        assert!(!flags.contains(PbrMaterialFlags::HAS_ALBEDO_MAP));
        assert!(flags.contains(PbrMaterialFlags::HAS_NORMAL_MAP));
    }

    #[test]
    fn test_flags_bits() {
        let flags = PbrMaterialFlags::HAS_ALBEDO_MAP | PbrMaterialFlags::USE_IBL;
        assert_eq!(flags.bits(), 1 | (1 << 7));
    }

    #[test]
    fn test_flags_from_bits() {
        let flags = PbrMaterialFlags::from_bits(3);
        assert!(flags.contains(PbrMaterialFlags::HAS_ALBEDO_MAP));
        assert!(flags.contains(PbrMaterialFlags::HAS_NORMAL_MAP));
    }

    #[test]
    fn test_has_any_texture() {
        let flags = PbrMaterialFlags::HAS_ALBEDO_MAP;
        assert!(flags.has_any_texture());

        let flags = PbrMaterialFlags::USE_IBL;
        assert!(!flags.has_any_texture());
    }
}
