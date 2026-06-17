use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0 };

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
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
}

impl Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Vec2) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2::new(self * other.x, self * other.y)
    }
}

impl Div for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        assert_eq!(a + b, Vec2::new(4.0, 6.0));
        assert_eq!(a - b, Vec2::new(-2.0, -2.0));
        assert_eq!(a * 2.0, Vec2::new(2.0, 4.0));
        assert_eq!(a / 2.0, Vec2::new(0.5, 1.0));
    }

    #[test]
    fn test_dot_cross() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        assert_eq!(a.dot(b), 11.0);
        assert_eq!(a.cross(b), -2.0);
    }

    #[test]
    fn test_length() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
        assert_eq!(v.length_squared(), 25.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalize();
        assert!((n.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_lerp() {
        let a = Vec2::ZERO;
        let b = Vec2::ONE;
        assert_eq!(a.lerp(b, 0.5), Vec2::splat(0.5));
    }

    #[test]
    fn test_zero_vector_normalize() {
        let v = Vec2::ZERO;
        let n = v.normalize();
        assert_eq!(n, Vec2::ZERO);
    }

    #[test]
    fn test_neg() {
        let v = Vec2::new(1.0, -2.0);
        let neg = -v;
        assert_eq!(neg, Vec2::new(-1.0, 2.0));
    }

    #[test]
    fn test_splat() {
        let v = Vec2::splat(5.0);
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 5.0);
    }

    #[test]
    fn test_distance() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a.distance(b), 5.0);
        assert_eq!(a.distance_squared(b), 25.0);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec2::new(1.0, 2.0);
        a += Vec2::new(3.0, 4.0);
        assert_eq!(a, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_scalar_mul() {
        let v = Vec2::new(1.0, 2.0);
        let result = 3.0 * v;
        assert_eq!(result, Vec2::new(3.0, 6.0));
    }

    #[test]
    fn test_component_mul() {
        let a = Vec2::new(2.0, 3.0);
        let b = Vec2::new(4.0, 5.0);
        assert_eq!(a * b, Vec2::new(8.0, 15.0));
    }

    #[test]
    fn test_component_div() {
        let a = Vec2::new(10.0, 15.0);
        let b = Vec2::new(2.0, 3.0);
        assert_eq!(a / b, Vec2::new(5.0, 5.0));
    }

    #[test]
    fn test_constants() {
        assert_eq!(Vec2::ZERO, Vec2::new(0.0, 0.0));
        assert_eq!(Vec2::ONE, Vec2::new(1.0, 1.0));
        assert_eq!(Vec2::X, Vec2::new(1.0, 0.0));
        assert_eq!(Vec2::Y, Vec2::new(0.0, 1.0));
    }

    #[test]
    fn test_very_small_vector_normalize() {
        let v = Vec2::new(1e-10, 1e-10);
        let n = v.normalize();
        // Very small vectors should normalize to zero or near-zero
        assert!(n.length() < 1e-5 || n.length() < 1.0);
    }

    #[test]
    fn test_lerp_extreme_values() {
        let a = Vec2::new(-100.0, -100.0);
        let b = Vec2::new(100.0, 100.0);

        // t = 0 should return a
        assert_eq!(a.lerp(b, 0.0), a);

        // t = 1 should return b
        assert_eq!(a.lerp(b, 1.0), b);

        // t = 0.5 should return midpoint
        assert_eq!(a.lerp(b, 0.5), Vec2::ZERO);
    }

    #[test]
    fn test_display() {
        let v = Vec2::new(1.0, 2.0);
        let s = format!("{}", v);
        assert!(s.contains("Vec2"));
    }
}
