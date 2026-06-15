#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Transform {
    pub translation: crate::Vec3,
    pub rotation: crate::Quat,
    pub scale: crate::Vec3,
}

impl Transform {
    pub fn new(translation: crate::Vec3, rotation: crate::Quat, scale: crate::Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn from_translation(v: crate::Vec3) -> Self {
        Self {
            translation: v,
            rotation: crate::Quat::IDENTITY,
            scale: crate::Vec3::ONE,
        }
    }

    pub fn matrix(&self) -> crate::Mat4 {
        let scale_mat = crate::Mat4::from_scale(self.scale);
        let rot_mat = crate::Mat4::from_quat(self.rotation);
        let trans_mat = crate::Mat4::from_translation(self.translation);
        trans_mat * rot_mat * scale_mat
    }

    pub fn inverse(&self) -> Self {
        Self {
            translation: -(self.rotation.inverse().mul_vec3(self.translation * self.scale)),
            rotation: self.rotation.inverse(),
            scale: crate::Vec3::new(
                1.0 / self.scale.x,
                1.0 / self.scale.y,
                1.0 / self.scale.z,
            ),
        }
    }
}

impl crate::Quat {
    pub fn mul_vec3(self, v: crate::Vec3) -> crate::Vec3 {
        let qv = crate::Vec3::new(self.x, self.y, self.z);
        let uv = qv.cross(v);
        let uuv = qv.cross(uv);
        v + (uv * self.w + uuv) * 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_new() {
        let t = Transform::new(
            crate::Vec3::new(1.0, 2.0, 3.0),
            crate::Quat::IDENTITY,
            crate::Vec3::ONE,
        );
        assert_eq!(t.translation, crate::Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn transform_from_translation() {
        let t = Transform::from_translation(crate::Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(t.translation, crate::Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(t.scale, crate::Vec3::ONE);
    }

    #[test]
    fn transform_matrix() {
        let t = Transform::from_translation(crate::Vec3::new(1.0, 2.0, 3.0));
        let m = t.matrix();
        let v = m.mul_vec4(crate::Vec4::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(v, crate::Vec4::new(1.0, 2.0, 3.0, 1.0));
    }
}
