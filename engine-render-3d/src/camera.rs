//! 3D Camera system

use engine_math::{Mat4, Vec2, Vec3, Vec4};
use crate::ray::Ray3;

/// Projection mode for Camera3D
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProjectionMode {
    Perspective,
    Orthographic,
}

/// 3D Camera with perspective and orthographic projection support
#[derive(Clone, Debug)]
pub struct Camera3D {
    position: Vec3,
    forward: Vec3,
    up: Vec3,
    right: Vec3,
    projection_mode: ProjectionMode,
    fovy: f32,
    aspect: f32,
    near: f32,
    far: f32,
    ortho_left: f32,
    ortho_right: f32,
    ortho_bottom: f32,
    ortho_top: f32,
}

impl Camera3D {
    /// Create a perspective camera
    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            position: Vec3::ZERO,
            forward: -Vec3::Z,
            up: Vec3::Y,
            right: Vec3::X,
            projection_mode: ProjectionMode::Perspective,
            fovy,
            aspect,
            near,
            far,
            ortho_left: -1.0,
            ortho_right: 1.0,
            ortho_bottom: -1.0,
            ortho_top: 1.0,
        }
    }

    /// Create an orthographic camera
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self {
            position: Vec3::ZERO,
            forward: -Vec3::Z,
            up: Vec3::Y,
            right: Vec3::X,
            projection_mode: ProjectionMode::Orthographic,
            fovy: 1.0,
            aspect: (right - left) / (top - bottom),
            near,
            far,
            ortho_left: left,
            ortho_right: right,
            ortho_bottom: bottom,
            ortho_top: top,
        }
    }

    /// Compute view matrix (world to camera space)
    pub fn view_matrix(&self) -> Mat4 {
        // Look-at matrix: camera at position looking at position + forward
        let f = self.forward.normalize();
        let r = self.right.normalize();
        let u = self.up.normalize();

        Mat4 {
            cols: [
                [r.x, u.x, -f.x, 0.0],
                [r.y, u.y, -f.y, 0.0],
                [r.z, u.z, -f.z, 0.0],
                [-r.dot(self.position), -u.dot(self.position), f.dot(self.position), 1.0],
            ],
        }
    }

    /// Compute projection matrix (camera to clip space)
    pub fn projection_matrix(&self) -> Mat4 {
        match self.projection_mode {
            ProjectionMode::Perspective => {
                let f = 1.0 / (self.fovy / 2.0).tan();
                Mat4 {
                    cols: [
                        [f / self.aspect, 0.0, 0.0, 0.0],
                        [0.0, f, 0.0, 0.0],
                        [0.0, 0.0, (self.far + self.near) / (self.near - self.far), -1.0],
                        [0.0, 0.0, (2.0 * self.far * self.near) / (self.near - self.far), 0.0],
                    ],
                }
            }
            ProjectionMode::Orthographic => {
                let rl = self.ortho_right - self.ortho_left;
                let tb = self.ortho_top - self.ortho_bottom;
                let far_near = self.far - self.near;

                Mat4 {
                    cols: [
                        [2.0 / rl, 0.0, 0.0, 0.0],
                        [0.0, 2.0 / tb, 0.0, 0.0],
                        [0.0, 0.0, -2.0 / far_near, 0.0],
                        [
                            -(self.ortho_right + self.ortho_left) / rl,
                            -(self.ortho_top + self.ortho_bottom) / tb,
                            -(self.far + self.near) / far_near,
                            1.0,
                        ],
                    ],
                }
            }
        }
    }

    /// Compute view-projection matrix
    #[inline]
    pub fn view_projection(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Compute inverse view matrix
    pub fn inverse_view(&self) -> Mat4 {
        let r = self.right;
        let u = self.up;
        let f = self.forward;

        Mat4 {
            cols: [
                [r.x, r.y, r.z, 0.0],
                [u.x, u.y, u.z, 0.0],
                [-f.x, -f.y, -f.z, 0.0],
                [self.position.x, self.position.y, self.position.z, 1.0],
            ],
        }
    }

    /// Compute inverse projection matrix
    pub fn inverse_projection(&self) -> Option<Mat4> {
        self.projection_matrix().inverse()
    }

    /// Compute inverse view-projection matrix
    pub fn inverse_view_projection(&self) -> Option<Mat4> {
        self.view_projection().inverse()
    }

    #[inline]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    #[inline]
    pub fn forward(&self) -> Vec3 {
        self.forward
    }

    #[inline]
    pub fn right(&self) -> Vec3 {
        self.right
    }

    #[inline]
    pub fn up(&self) -> Vec3 {
        self.up
    }

    #[inline]
    pub fn fovy(&self) -> f32 {
        self.fovy
    }

    #[inline]
    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    #[inline]
    pub fn near(&self) -> f32 {
        self.near
    }

    #[inline]
    pub fn far(&self) -> f32 {
        self.far
    }

    #[inline]
    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }

    #[inline]
    pub fn set_fovy(&mut self, f: f32) {
        self.fovy = f;
    }

    #[inline]
    pub fn set_aspect(&mut self, a: f32) {
        self.aspect = a;
    }

    #[inline]
    pub fn set_near(&mut self, n: f32) {
        self.near = n;
    }

    #[inline]
    pub fn set_far(&mut self, f: f32) {
        self.far = f;
    }

    /// Orient camera to look at target point
    pub fn look_at(&mut self, target: Vec3) {
        self.forward = (target - self.position).normalize();
        self.right = Vec3::Y.cross(self.forward).normalize();
        self.up = self.forward.cross(self.right).normalize();
    }

    /// Orient camera to look in direction
    pub fn look_to(&mut self, dir: Vec3, up: Vec3) {
        self.forward = dir.normalize();
        self.right = up.cross(self.forward).normalize();
        self.up = self.forward.cross(self.right).normalize();
    }

    /// Convert screen coordinates to world ray
    pub fn screen_to_world_ray(&self, screen_pos: Vec2, screen_size: Vec2) -> Ray3 {
        // Convert to normalized device coordinates [-1, 1]
        let ndc_x = (2.0 * screen_pos.x / screen_size.x) - 1.0;
        let ndc_y = 1.0 - (2.0 * screen_pos.y / screen_size.y);

        // Clip space coordinates
        let clip = Vec4::new(ndc_x, ndc_y, -1.0, 1.0);

        // Transform to view space
        let inv_proj = self.inverse_projection().unwrap_or(Mat4::IDENTITY);
        let view = inv_proj.mul_vec4(clip);
        let view_dir = Vec3::new(view.x, view.y, -1.0).normalize();

        // Transform to world space
        let inv_view = self.inverse_view();
        let world_dir = Vec3::new(
            inv_view.cols[0][0] * view_dir.x + inv_view.cols[1][0] * view_dir.y + inv_view.cols[2][0] * view_dir.z,
            inv_view.cols[0][1] * view_dir.x + inv_view.cols[1][1] * view_dir.y + inv_view.cols[2][1] * view_dir.z,
            inv_view.cols[0][2] * view_dir.x + inv_view.cols[1][2] * view_dir.y + inv_view.cols[2][2] * view_dir.z,
        ).normalize();

        Ray3::new(self.position, world_dir)
    }

    /// Convert world position to screen coordinates
    pub fn world_to_screen(&self, world_pos: Vec3, screen_size: Vec2) -> Vec2 {
        let vp = self.view_projection();
        let clip = vp.mul_vec4(Vec4::new(world_pos.x, world_pos.y, world_pos.z, 1.0));

        if clip.w.abs() < 1e-6 {
            return Vec2::ZERO;
        }

        let ndc_x = clip.x / clip.w;
        let ndc_y = clip.y / clip.w;

        Vec2::new(
            (ndc_x + 1.0) * 0.5 * screen_size.x,
            (1.0 - ndc_y) * 0.5 * screen_size.y,
        )
    }
}

impl Default for Camera3D {
    fn default() -> Self {
        Self::perspective(45.0, 1.0, 0.1, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perspective_camera() {
        let cam = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        assert_eq!(cam.projection_mode, ProjectionMode::Perspective);
        assert_eq!(cam.fovy(), 45.0);
    }

    #[test]
    fn test_orthographic_camera() {
        let cam = Camera3D::orthographic(-10.0, 10.0, -10.0, 10.0, 0.1, 100.0);
        assert_eq!(cam.projection_mode, ProjectionMode::Orthographic);
    }

    #[test]
    fn test_look_at() {
        let mut cam = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        cam.set_position(Vec3::new(0.0, 0.0, 5.0));
        cam.look_at(Vec3::ZERO);

        // Forward should point towards target
        let expected_forward = Vec3::new(0.0, 0.0, -1.0);
        assert!((cam.forward().x - expected_forward.x).abs() < 1e-5);
        assert!((cam.forward().y - expected_forward.y).abs() < 1e-5);
        assert!((cam.forward().z - expected_forward.z).abs() < 1e-5);
    }

    #[test]
    fn test_view_projection_matrix() {
        let cam = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        let vp = cam.view_projection();
        let proj = cam.projection_matrix();
        let view = cam.view_matrix();
        let vp_composed = proj * view;

        // Check matrices match
        for i in 0..4 {
            for j in 0..4 {
                assert!((vp.cols[i][j] - vp_composed.cols[i][j]).abs() < 1e-5);
            }
        }
    }
}