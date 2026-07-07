use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    pub const X: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const Y: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
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
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    #[inline]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Vec3) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Div for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(a - b, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(a * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_dot_cross() {
        let a = Vec3::X;
        let b = Vec3::Y;
        assert_eq!(a.dot(b), 0.0);
        assert_eq!(a.cross(b), Vec3::Z);
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert!((v.length() - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        let n = v.normalize();
        assert!((n.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_lerp() {
        let a = Vec3::ZERO;
        let b = Vec3::ONE;
        assert_eq!(a.lerp(b, 0.5), Vec3::splat(0.5));
    }

    #[test]
    fn test_zero_vector_normalize() {
        let v = Vec3::ZERO;
        let n = v.normalize();
        assert_eq!(n, Vec3::ZERO);
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let neg = -v;
        assert_eq!(neg, Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_splat() {
        let v = Vec3::splat(5.0);
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 5.0);
        assert_eq!(v.z, 5.0);
    }

    #[test]
    fn test_distance() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 2.0, 2.0);
        assert!((a.distance(b) - 3.0).abs() < 1e-6);
        assert!((a.distance_squared(b) - 9.0).abs() < 1e-6);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        a += Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vec3::new(10.0, 10.0, 10.0);
        a -= Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a, Vec3::new(9.0, 8.0, 7.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        a *= 2.0;
        assert_eq!(a, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_scalar_mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = 3.0 * v;
        assert_eq!(result, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_component_mul() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        assert_eq!(a * b, Vec3::new(10.0, 18.0, 28.0));
    }

    #[test]
    fn test_component_div() {
        let a = Vec3::new(10.0, 12.0, 14.0);
        let b = Vec3::new(2.0, 3.0, 7.0);
        assert_eq!(a / b, Vec3::new(5.0, 4.0, 2.0));
    }

    #[test]
    fn test_scalar_div() {
        let v = Vec3::new(10.0, 20.0, 30.0);
        assert_eq!(v / 10.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_abs() {
        let v = Vec3::new(-1.0, -2.0, 3.0);
        assert_eq!(v.abs(), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_constants() {
        assert_eq!(Vec3::ZERO, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(Vec3::ONE, Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(Vec3::X, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(Vec3::Y, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(Vec3::Z, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_cross_orthogonal() {
        let x = Vec3::X;
        let y = Vec3::Y;
        let z = Vec3::Z;

        assert_eq!(x.cross(y), z);
        assert_eq!(y.cross(z), x);
        assert_eq!(z.cross(x), y);

        // Anti-commutative
        assert_eq!(y.cross(x), -z);
        assert_eq!(z.cross(y), -x);
        assert_eq!(x.cross(z), -y);
    }

    #[test]
    fn test_cross_self() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.cross(v), Vec3::ZERO);
    }

    #[test]
    fn test_dot_perpendicular() {
        let x = Vec3::X;
        let y = Vec3::Y;
        let z = Vec3::Z;

        assert_eq!(x.dot(y), 0.0);
        assert_eq!(y.dot(z), 0.0);
        assert_eq!(x.dot(z), 0.0);
    }

    #[test]
    fn test_very_small_vector_normalize() {
        let v = Vec3::new(1e-10, 1e-10, 1e-10);
        let n = v.normalize();
        assert!(n.length() < 1e-5 || n.length() < 1.0);
    }

    #[test]
    fn test_lerp_extreme_values() {
        let a = Vec3::new(-100.0, -100.0, -100.0);
        let b = Vec3::new(100.0, 100.0, 100.0);

        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);
        assert_eq!(a.lerp(b, 0.5), Vec3::ZERO);
    }

    #[test]
    fn test_display() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let s = format!("{}", v);
        assert!(s.contains("Vec3"));
    }

    #[test]
    fn test_normalize_or_zero() {
        let v = Vec3::ZERO;
        assert_eq!(v.normalize_or_zero(), Vec3::ZERO);

        let v2 = Vec3::new(1.0, 0.0, 0.0);
        assert!((v2.normalize_or_zero().length() - 1.0).abs() < 1e-6);
    }
}
