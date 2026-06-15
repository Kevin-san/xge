#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Euler {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Euler {
    pub fn new(pitch_deg: f32, yaw_deg: f32, roll_deg: f32) -> Self {
        Self {
            pitch: pitch_deg,
            yaw: yaw_deg,
            roll: roll_deg,
        }
    }

    pub fn from_radians(pitch_rad: f32, yaw_rad: f32, roll_rad: f32) -> Self {
        Self {
            pitch: pitch_rad.to_degrees(),
            yaw: yaw_rad.to_degrees(),
            roll: roll_rad.to_degrees(),
        }
    }

    pub fn to_radians(&self) -> (f32, f32, f32) {
        (
            self.pitch.to_radians(),
            self.yaw.to_radians(),
            self.roll.to_radians(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euler_from_radians() {
        let euler = Euler::from_radians(
            core::f32::consts::PI,
            core::f32::consts::FRAC_PI_2,
            0.0,
        );
        assert_eq!(euler.pitch, 180.0);
        assert_eq!(euler.yaw, 90.0);
        assert_eq!(euler.roll, 0.0);
    }

    #[test]
    fn euler_to_radians() {
        let euler = Euler::new(180.0, 90.0, 0.0);
        let (p, y, r) = euler.to_radians();
        assert_eq!(p, core::f32::consts::PI);
        assert_eq!(y, core::f32::consts::FRAC_PI_2);
        assert_eq!(r, 0.0);
    }
}
