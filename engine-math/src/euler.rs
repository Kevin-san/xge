use core::ops::{Add, Sub, Mul, Div};
use core::fmt;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub struct Euler {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Euler {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn to_radians(self) -> Self {
        Self::new(
            self.x * core::f32::consts::PI / 180.0,
            self.y * core::f32::consts::PI / 180.0,
            self.z * core::f32::consts::PI / 180.0,
        )
    }

    #[inline]
    pub fn to_degrees(self) -> Self {
        Self::new(
            self.x * 180.0 / core::f32::consts::PI,
            self.y * 180.0 / core::f32::consts::PI,
            self.z * 180.0 / core::f32::consts::PI,
        )
    }
}

impl Add for Euler {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Euler {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Euler {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f32> for Euler {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl fmt::Display for Euler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Euler({:.4}, {:.4}, {:.4})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radians_conversion() {
        let euler = Euler::new(180.0, 90.0, 45.0);
        let rad = euler.to_radians();
        assert!((rad.x - core::f32::consts::PI).abs() < 1e-5);
        assert!((rad.y - core::f32::consts::PI / 2.0).abs() < 1e-5);
    }
}
