//! 物理调试渲染器 - 基于 DebugRenderer 绘制碰撞体线框

use crate::{Collider2D, ColliderShape, RigidBody2D};
use engine_math::{Rect, Vec2};

/// 调试渲染线段
#[derive(Debug, Clone)]
pub struct DebugLine {
    /// 起点
    pub start: Vec2,
    /// 终点
    pub end: Vec2,
    /// 颜色 (RGBA)
    pub color: [f32; 4],
}

/// 调试渲染圆
#[derive(Debug, Clone)]
pub struct DebugCircle {
    /// 圆心
    pub center: Vec2,
    /// 半径
    pub radius: f32,
    /// 颜色 (RGBA)
    pub color: [f32; 4],
}

/// 调试渲染矩形
#[derive(Debug, Clone)]
pub struct DebugRect {
    /// 矩形
    pub rect: Rect,
    /// 颜色 (RGBA)
    pub color: [f32; 4],
}

/// 物理调试渲染器
///
/// 用于绘制物理实体的调试线框。
pub struct PhysicsDebugRenderer {
    /// 线段列表
    lines: Vec<DebugLine>,
    /// 圆列表
    circles: Vec<DebugCircle>,
    /// 矩形列表
    rects: Vec<DebugRect>,
    /// 默认颜色
    default_color: [f32; 4],
}

impl PhysicsDebugRenderer {
    /// 创建新的调试渲染器
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            circles: Vec::new(),
            rects: Vec::new(),
            default_color: [0.0, 1.0, 0.0, 1.0], // 绿色
        }
    }

    /// 绘制刚体和碰撞体
    ///
    /// 根据碰撞体类型绘制对应的几何形状。
    pub fn draw_body(&mut self, body: &RigidBody2D, collider: &Collider2D) {
        let position = body.position();
        let rotation = body.rotation();

        match collider.shape() {
            ColliderShape::Circle { radius } => {
                self.draw_circle(position, *radius, self.default_color);
            }
            ColliderShape::Aabb { half_extents } => {
                let rect = Rect::new(
                    position.x - half_extents.x,
                    position.y - half_extents.y,
                    half_extents.x * 2.0,
                    half_extents.y * 2.0,
                );
                self.draw_rect(rect, self.default_color);
            }
            ColliderShape::Rectangle { half_extents } => {
                self.draw_rotated_rect(position, rotation, *half_extents, self.default_color);
            }
            ColliderShape::Polygon { vertices } => {
                self.draw_polygon(position, rotation, vertices, self.default_color);
            }
            ColliderShape::Capsule {
                top,
                bottom,
                radius,
            } => {
                self.draw_capsule(
                    position,
                    rotation,
                    *top,
                    *bottom,
                    *radius,
                    self.default_color,
                );
            }
        }
    }

    /// 绘制圆形
    pub fn draw_circle(&mut self, center: Vec2, radius: f32, color: [f32; 4]) {
        self.circles.push(DebugCircle {
            center,
            radius,
            color,
        });

        // 绘制圆的轮廓线段
        let segments = 16;
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
            let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::PI * 2.0;

            let start = Vec2::new(
                center.x + radius * angle1.cos(),
                center.y + radius * angle1.sin(),
            );
            let end = Vec2::new(
                center.x + radius * angle2.cos(),
                center.y + radius * angle2.sin(),
            );

            self.lines.push(DebugLine { start, end, color });
        }
    }

    /// 绘制矩形（轴对齐）
    pub fn draw_rect(&mut self, rect: Rect, color: [f32; 4]) {
        self.rects.push(DebugRect { rect, color });

        // 绘制矩形轮廓线段
        let p1 = Vec2::new(rect.x, rect.y);
        let p2 = Vec2::new(rect.x + rect.w, rect.y);
        let p3 = Vec2::new(rect.x + rect.w, rect.y + rect.h);
        let p4 = Vec2::new(rect.x, rect.y + rect.h);

        self.lines.push(DebugLine {
            start: p1,
            end: p2,
            color,
        });
        self.lines.push(DebugLine {
            start: p2,
            end: p3,
            color,
        });
        self.lines.push(DebugLine {
            start: p3,
            end: p4,
            color,
        });
        self.lines.push(DebugLine {
            start: p4,
            end: p1,
            color,
        });
    }

    /// 绘制旋转矩形
    pub fn draw_rotated_rect(
        &mut self,
        center: Vec2,
        rotation: f32,
        half_extents: Vec2,
        color: [f32; 4],
    ) {
        let cos = rotation.cos();
        let sin = rotation.sin();

        // 计算四个角点（局部坐标转世界坐标）
        let local_corners = [
            Vec2::new(-half_extents.x, -half_extents.y),
            Vec2::new(half_extents.x, -half_extents.y),
            Vec2::new(half_extents.x, half_extents.y),
            Vec2::new(-half_extents.x, half_extents.y),
        ];

        let world_corners: Vec<Vec2> = local_corners
            .iter()
            .map(|corner| {
                Vec2::new(
                    corner.x * cos - corner.y * sin + center.x,
                    corner.x * sin + corner.y * cos + center.y,
                )
            })
            .collect();

        // 绘制四条边
        for i in 0..4 {
            self.lines.push(DebugLine {
                start: world_corners[i],
                end: world_corners[(i + 1) % 4],
                color,
            });
        }
    }

    /// 绘制多边形
    pub fn draw_polygon(
        &mut self,
        center: Vec2,
        rotation: f32,
        vertices: &[Vec2],
        color: [f32; 4],
    ) {
        if vertices.len() < 3 {
            return;
        }

        let cos = rotation.cos();
        let sin = rotation.sin();

        // 转换顶点到世界坐标
        let world_vertices: Vec<Vec2> = vertices
            .iter()
            .map(|v| {
                Vec2::new(
                    v.x * cos - v.y * sin + center.x,
                    v.x * sin + v.y * cos + center.y,
                )
            })
            .collect();

        // 绘制边
        for i in 0..world_vertices.len() {
            self.lines.push(DebugLine {
                start: world_vertices[i],
                end: world_vertices[(i + 1) % world_vertices.len()],
                color,
            });
        }
    }

    /// 绘制胶囊形状
    pub fn draw_capsule(
        &mut self,
        center: Vec2,
        rotation: f32,
        top: Vec2,
        bottom: Vec2,
        radius: f32,
        color: [f32; 4],
    ) {
        let cos = rotation.cos();
        let sin = rotation.sin();

        // 变换端点到世界坐标
        let world_top = Vec2::new(
            top.x * cos - top.y * sin + center.x,
            top.x * sin + top.y * cos + center.y,
        );
        let world_bottom = Vec2::new(
            bottom.x * cos - bottom.y * sin + center.x,
            bottom.x * sin + bottom.y * cos + center.y,
        );

        // 绘制胶囊轮廓
        let direction = world_top - world_bottom;
        let length = direction.length();

        if length < f32::EPSILON {
            // 退化为圆形
            self.draw_circle(world_bottom, radius, color);
            return;
        }

        let normalized = direction.normalize();
        let perpendicular = Vec2::new(-normalized.y, normalized.x);

        // 胶囊两端半圆
        let top_center = world_top;
        let bottom_center = world_bottom;

        // 绘制上下半圆（各 8 段）
        let segments = 8;
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * std::f32::consts::PI;
            let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::PI;

            // 上半圆
            let top_start = top_center
                + Vec2::new(
                    radius * (normalized.x * angle1.cos() - normalized.y * angle1.sin()),
                    radius * (normalized.x * angle1.sin() + normalized.y * angle1.cos()),
                );
            let top_end = top_center
                + Vec2::new(
                    radius * (normalized.x * angle2.cos() - normalized.y * angle2.sin()),
                    radius * (normalized.x * angle2.sin() + normalized.y * angle2.cos()),
                );

            // 下半圆
            let bottom_start = bottom_center
                + Vec2::new(
                    radius * (normalized.x * (-angle1.cos()) - normalized.y * (-angle1.sin())),
                    radius * (normalized.x * (-angle1.sin()) + normalized.y * (-angle1.cos())),
                );
            let bottom_end = bottom_center
                + Vec2::new(
                    radius * (normalized.x * (-angle2.cos()) - normalized.y * (-angle2.sin())),
                    radius * (normalized.x * (-angle2.sin()) + normalized.y * (-angle2.cos())),
                );

            self.lines.push(DebugLine {
                start: top_start,
                end: top_end,
                color,
            });
            self.lines.push(DebugLine {
                start: bottom_start,
                end: bottom_end,
                color,
            });
        }

        // 绘制连接线
        let top_left = top_center - perpendicular * radius;
        let top_right = top_center + perpendicular * radius;
        let bottom_left = bottom_center - perpendicular * radius;
        let bottom_right = bottom_center + perpendicular * radius;

        self.lines.push(DebugLine {
            start: top_left,
            end: bottom_left,
            color,
        });
        self.lines.push(DebugLine {
            start: top_right,
            end: bottom_right,
            color,
        });
    }

    /// 清空所有调试数据
    pub fn clear(&mut self) {
        self.lines.clear();
        self.circles.clear();
        self.rects.clear();
    }

    /// 获取线段列表
    pub fn lines(&self) -> &[DebugLine] {
        &self.lines
    }

    /// 获取圆列表
    pub fn circles(&self) -> &[DebugCircle] {
        &self.circles
    }

    /// 获取矩形列表
    pub fn rects(&self) -> &[DebugRect] {
        &self.rects
    }

    /// 设置默认颜色
    pub fn set_default_color(&mut self, color: [f32; 4]) {
        self.default_color = color;
    }
}

impl Default for PhysicsDebugRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_renderer_creation() {
        let renderer = PhysicsDebugRenderer::new();
        assert!(renderer.lines().is_empty());
        assert!(renderer.circles().is_empty());
        assert!(renderer.rects().is_empty());
    }

    #[test]
    fn test_clear() {
        let mut renderer = PhysicsDebugRenderer::new();
        renderer.draw_circle(Vec2::new(0.0, 0.0), 1.0, [1.0, 0.0, 0.0, 1.0]);
        renderer.clear();
        assert!(renderer.lines().is_empty());
        assert!(renderer.circles().is_empty());
    }

    #[test]
    fn test_draw_rect() {
        let mut renderer = PhysicsDebugRenderer::new();
        let rect = Rect::new(0.0, 0.0, 10.0, 20.0);
        renderer.draw_rect(rect, [1.0, 0.0, 0.0, 1.0]);
        // 矩形有 4 条边
        assert_eq!(renderer.lines().len(), 4);
    }

    #[test]
    fn test_draw_rotated_rect() {
        use crate::RigidBody2DBuilder;

        let mut renderer = PhysicsDebugRenderer::new();
        let body = RigidBody2DBuilder::dynamic()
            .with_position(Vec2::new(0.0, 0.0))
            .with_rotation(std::f32::consts::FRAC_PI_4)
            .build();

        let collider = Collider2D::new(ColliderShape::rectangle(2.0, 4.0));
        renderer.draw_body(&body, &collider);

        // 旋转矩形有 4 条边
        assert_eq!(renderer.lines().len(), 4);
    }

    // ============= PhysicsDebugRenderer 更多测试 =============

    #[test]
    fn test_draw_circle_has_16_segments() {
        let mut renderer = PhysicsDebugRenderer::new();
        renderer.draw_circle(Vec2::ZERO, 5.0, [1.0, 0.0, 0.0, 1.0]);
        // 默认 16 段
        assert_eq!(renderer.lines().len(), 16);
        assert_eq!(renderer.circles().len(), 1);
    }

    #[test]
    fn test_draw_multiple_shapes() {
        let mut renderer = PhysicsDebugRenderer::new();
        renderer.draw_circle(Vec2::ZERO, 1.0, [1.0, 0.0, 0.0, 1.0]);
        renderer.draw_rect(
            engine_math::Rect::new(0.0, 0.0, 5.0, 5.0),
            [0.0, 1.0, 0.0, 1.0],
        );
        assert_eq!(renderer.circles().len(), 1);
        assert_eq!(renderer.rects().len(), 1);
        // 圆 16 段 + 矩形 4 段
        assert_eq!(renderer.lines().len(), 20);
    }

    #[test]
    fn test_clear_empties_all() {
        let mut renderer = PhysicsDebugRenderer::new();
        renderer.draw_circle(Vec2::ZERO, 1.0, [1.0, 0.0, 0.0, 1.0]);
        renderer.draw_rect(
            engine_math::Rect::new(0.0, 0.0, 1.0, 1.0),
            [0.0, 1.0, 0.0, 1.0],
        );
        renderer.clear();
        assert!(renderer.lines().is_empty());
        assert!(renderer.circles().is_empty());
        assert!(renderer.rects().is_empty());
    }

    #[test]
    fn test_set_default_color() {
        let mut renderer = PhysicsDebugRenderer::new();
        renderer.set_default_color([0.5, 0.5, 0.0, 1.0]);
        renderer.draw_circle(Vec2::ZERO, 1.0, [0.5, 0.5, 0.0, 1.0]);
        assert_eq!(renderer.circles().first().unwrap().color[0], 0.5);
    }

    #[test]
    fn test_draw_body_with_circle_shape() {
        use crate::RigidBody2DBuilder;
        let mut renderer = PhysicsDebugRenderer::new();
        let body = RigidBody2DBuilder::dynamic()
            .with_position(Vec2::new(3.0, 3.0))
            .build();
        let collider = Collider2D::new(ColliderShape::circle(2.0));
        renderer.draw_body(&body, &collider);
        assert_eq!(renderer.circles().len(), 1);
        assert_eq!(
            renderer.circles().first().unwrap().center,
            Vec2::new(3.0, 3.0)
        );
    }
}
