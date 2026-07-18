//! 物理材质模块
//!
//! 定义碰撞体的物理属性：弹性、摩擦和密度。

/// 物理材质
///
/// 定义碰撞体的弹性系数、摩擦系数和密度。
#[derive(Debug, Clone, Copy)]
pub struct PhysicsMaterial {
    /// 弹性系数（0.0 = 完全非弹性，1.0 = 完全弹性）
    pub restitution: f32,
    /// 动摩擦系数
    pub friction: f32,
    /// 密度
    pub density: f32,
}

impl Default for PhysicsMaterial {
    fn default() -> Self {
        Self {
            restitution: 0.3,
            friction: 0.5,
            density: 1.0,
        }
    }
}

impl PhysicsMaterial {
    /// 创建新的物理材质
    pub fn new(restitution: f32, friction: f32, density: f32) -> Self {
        Self {
            restitution: restitution.clamp(0.0, 1.0),
            friction: friction.max(0.0),
            density: density.max(0.0),
        }
    }

    /// 创建弹性材质
    pub fn bouncy() -> Self {
        Self {
            restitution: 0.8,
            friction: 0.2,
            density: 1.0,
        }
    }

    /// 创建高摩擦材质
    pub fn rough() -> Self {
        Self {
            restitution: 0.1,
            friction: 0.9,
            density: 1.0,
        }
    }

    /// 创建重材质
    pub fn heavy() -> Self {
        Self {
            restitution: 0.2,
            friction: 0.5,
            density: 10.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_material_default() {
        let mat = PhysicsMaterial::default();
        assert!((mat.restitution - 0.3).abs() < 1e-6);
        assert!((mat.friction - 0.5).abs() < 1e-6);
        assert!((mat.density - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_physics_material_new() {
        let mat = PhysicsMaterial::new(0.5, 0.3, 2.0);
        assert!((mat.restitution - 0.5).abs() < 1e-6);
        assert!((mat.friction - 0.3).abs() < 1e-6);
        assert!((mat.density - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_physics_material_restitution_clamped() {
        let mat = PhysicsMaterial::new(2.0, 0.5, 1.0);
        assert!((mat.restitution - 1.0).abs() < 1e-6);
        let mat2 = PhysicsMaterial::new(-0.5, 0.5, 1.0);
        assert!((mat2.restitution - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_physics_material_friction_non_negative() {
        let mat = PhysicsMaterial::new(0.5, -1.0, 1.0);
        assert!((mat.friction - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_physics_material_density_non_negative() {
        let mat = PhysicsMaterial::new(0.5, 0.5, -1.0);
        assert!((mat.density - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_physics_material_bouncy() {
        let mat = PhysicsMaterial::bouncy();
        assert!(mat.restitution > 0.5);
    }

    #[test]
    fn test_physics_material_rough() {
        let mat = PhysicsMaterial::rough();
        assert!(mat.friction > 0.5);
    }

    #[test]
    fn test_physics_material_heavy() {
        let mat = PhysicsMaterial::heavy();
        assert!(mat.density > 1.0);
    }
}
