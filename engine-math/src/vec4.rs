use core::fmt;
use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    };
    pub const X: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub const Y: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 0.0,
    };
    pub const Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 0.0,
    };
    pub const W: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self / len
        } else {
            Self::ZERO
        }
    }

    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self / len
        } else {
            Self::ZERO
        }
    }

    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }

    #[inline]
    pub fn xyz(self) -> super::Vec3 {
        super::Vec3::new(self.x, self.y, self.z)
    }
}

impl Add for Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Vec4) -> Self {
        Self::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4::new(
            self * other.x,
            self * other.y,
            self * other.z,
            self * other.w,
        )
    }
}

impl Div for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self::new(
            self.x / other.x,
            self.y / other.y,
            self.z / other.z,
            self.w / other.w,
        )
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, scalar: f32) -> Self {
        Self::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}

impl Neg for Vec4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        assert_eq!(a + b, Vec4::new(6.0, 8.0, 10.0, 12.0));
        assert_eq!(a - b, Vec4::new(-4.0, -4.0, -4.0, -4.0));
        assert_eq!(a * 2.0, Vec4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn test_dot() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(a.dot(b), 10.0);
    }

    #[test]
    fn test_xyz() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.xyz(), crate::Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_zero_vector_normalize() {
        let v = Vec4::ZERO;
        let n = v.normalize();
        assert_eq!(n, Vec4::ZERO);
    }

    #[test]
    fn test_neg() {
        let v = Vec4::new(1.0, -2.0, 3.0, -4.0);
        let neg = -v;
        assert_eq!(neg, Vec4::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_splat() {
        let v = Vec4::splat(5.0);
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 5.0);
        assert_eq!(v.z, 5.0);
        assert_eq!(v.w, 5.0);
    }

    #[test]
    fn test_length() {
        let v = Vec4::new(1.0, 2.0, 2.0, 0.0);
        assert!((v.length() - 3.0).abs() < 1e-6);
        assert!((v.length_squared() - 9.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalize() {
        let v = Vec4::new(1.0, 2.0, 2.0, 0.0);
        let n = v.normalize();
        assert!((n.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_lerp() {
        let a = Vec4::ZERO;
        let b = Vec4::ONE;
        assert_eq!(a.lerp(b, 0.5), Vec4::splat(0.5));
    }

    #[test]
    fn test_scalar_mul() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let result = 3.0 * v;
        assert_eq!(result, Vec4::new(3.0, 6.0, 9.0, 12.0));
    }

    #[test]
    fn test_component_mul() {
        let a = Vec4::new(2.0, 3.0, 4.0, 5.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(a * b, Vec4::new(10.0, 18.0, 28.0, 40.0));
    }

    #[test]
    fn test_component_div() {
        let a = Vec4::new(10.0, 12.0, 14.0, 16.0);
        let b = Vec4::new(2.0, 3.0, 7.0, 4.0);
        assert_eq!(a / b, Vec4::new(5.0, 4.0, 2.0, 4.0));
    }

    #[test]
    fn test_scalar_div() {
        let v = Vec4::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(v / 10.0, Vec4::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_constants() {
        assert_eq!(Vec4::ZERO, Vec4::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4::ONE, Vec4::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(Vec4::X, Vec4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4::Y, Vec4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(Vec4::Z, Vec4::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(Vec4::W, Vec4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn test_very_small_vector_normalize() {
        let v = Vec4::new(1e-10, 1e-10, 1e-10, 1e-10);
        let n = v.normalize();
        // Very small vectors are still normalized (implementation uses sqrt)
        // The result should be a unit vector
        assert!((n.length() - 1.0).abs() < 1e-5 || n == Vec4::ZERO);
    }

    #[test]
    fn test_lerp_extreme_values() {
        let a = Vec4::new(-100.0, -100.0, -100.0, -100.0);
        let b = Vec4::new(100.0, 100.0, 100.0, 100.0);
        
        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);
        assert_eq!(a.lerp(b, 0.5), Vec4::ZERO);
    }

    #[test]
    fn test_display() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let s = format!("{}", v);
        assert!(s.contains("Vec4"));
    }

    #[test]
    fn test_normalize_or_zero() {
        let v = Vec4::ZERO;
        assert_eq!(v.normalize_or_zero(), Vec4::ZERO);
        
        let v2 = Vec4::new(1.0, 0.0, 0.0, 0.0);
        assert!((v2.normalize_or_zero().length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_dot_self() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.dot(v), v.length_squared());
    }

    #[test]
    fn test_homogeneous_point() {
        // Test that w=1 behaves as a point
        let point = Vec4::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(point.xyz(), crate::Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_homogeneous_direction() {
        // Test that w=0 behaves as a direction
        let direction = Vec4::new(1.0, 2.0, 3.0, 0.0);
        assert_eq!(direction.xyz(), crate::Vec3::new(1.0, 2.0, 3.0));
    }
}
