# Module 02 — Mat4 inverse 重构（修复列主序错误）

> 上游 sprint: [Sprint 17](../sprint-17-math-simd.md)
> 文件位置: `engine-math/src/mat4.rs#inverse`

---

## 1. 目标

**修复 `engine-math/src/mat4.rs#L175` 手写伴随矩阵法的列主序/符号错误，引入 LU 分解后备方案。**

**当前 Bug：**
- 现有实现 `m[0][0]` 假设行主序，但 `Mat4.cols[col][row]` 实际是列主序
- 导致行列式符号错误
- 影响：3D 相机视图逆矩阵、screen_to_world_ray 计算（已用 workaround）

## 2. 正确实现（伴随矩阵法 + 列主序）

```rust
impl Mat4 {
    /// 4x4 矩阵求逆（伴随矩阵法，列主序）
    /// 
    /// 布局约定：cols[col][row]
    /// 例如 cols[0] = [m00, m10, m20, m30]（第一列）
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        // 提取 16 个分量到局部变量（避免重复索引）
        let m00 = self.cols[0][0];
        let m01 = self.cols[0][1];
        let m02 = self.cols[0][2];
        let m03 = self.cols[0][3];
        let m10 = self.cols[1][0];
        let m11 = self.cols[1][1];
        let m12 = self.cols[1][2];
        let m13 = self.cols[1][3];
        let m20 = self.cols[2][0];
        let m21 = self.cols[2][1];
        let m22 = self.cols[2][2];
        let m23 = self.cols[2][3];
        let m30 = self.cols[3][0];
        let m31 = self.cols[3][1];
        let m32 = self.cols[3][2];
        let m33 = self.cols[3][3];

        // 2x2 子行列式（用于加速 3x3 / 4x4）
        let b00 = m00 * m11 - m01 * m10;
        let b01 = m00 * m12 - m02 * m10;
        let b02 = m00 * m13 - m03 * m10;
        let b03 = m01 * m12 - m02 * m11;
        let b04 = m01 * m13 - m03 * m11;
        let b05 = m02 * m13 - m03 * m12;
        let b06 = m20 * m31 - m21 * m30;
        let b07 = m20 * m32 - m22 * m30;
        let b08 = m20 * m33 - m23 * m30;
        let b09 = m21 * m32 - m22 * m31;
        let b10 = m21 * m33 - m23 * m31;
        let b11 = m22 * m33 - m23 * m32;

        // 4x4 行列式
        let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

        // 奇异矩阵
        if det.abs() < 1e-8 {
            return None;
        }

        let inv_det = det.recip();

        // 伴随矩阵 × 1/det（列主序输出）
        Some(Self {
            cols: [
                // 第 0 列
                [
                    (m11 * b11 - m12 * b10 + m13 * b09) * inv_det,
                    (m02 * b10 - m01 * b11 - m03 * b09) * inv_det,
                    (m31 * b05 - m32 * b04 + m33 * b03) * inv_det,
                    (m22 * b04 - m21 * b05 - m23 * b03) * inv_det,
                ],
                // 第 1 列
                [
                    (m12 * b08 - m10 * b11 - m13 * b07) * inv_det,
                    (m00 * b11 - m02 * b08 + m03 * b07) * inv_det,
                    (m32 * b02 - m30 * b05 - m33 * b01) * inv_det,
                    (m20 * b05 - m22 * b02 + m23 * b01) * inv_det,
                ],
                // 第 2 列
                [
                    (m10 * b10 - m11 * b08 + m13 * b06) * inv_det,
                    (m01 * b08 - m00 * b10 - m03 * b06) * inv_det,
                    (m30 * b04 - m31 * b02 + m33 * b00) * inv_det,
                    (m21 * b02 - m20 * b04 - m23 * b00) * inv_det,
                ],
                // 第 3 列
                [
                    (m11 * b07 - m10 * b09 - m12 * b06) * inv_det,
                    (m00 * b09 - m01 * b07 + m02 * b06) * inv_det,
                    (m31 * b01 - m30 * b03 - m32 * b00) * inv_det,
                    (m20 * b03 - m21 * b01 + m22 * b00) * inv_det,
                ],
            ],
        })
    }
}
```

## 3. 测试用例

```rust
#[test]
fn test_inverse_identity() {
    let m = Mat4::IDENTITY;
    let inv = m.inverse().unwrap();
    assert_mat4_eq(inv, Mat4::IDENTITY, 1e-6);
}

#[test]
fn test_inverse_translation() {
    let m = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
    let inv = m.inverse().unwrap();
    let product = m * inv;
    assert_mat4_eq(product, Mat4::IDENTITY, 1e-5);
}

#[test]
fn test_inverse_singular() {
    // 退化矩阵（最后一行全 0）
    let m = Mat4 {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],  // 奇异
        ],
    };
    assert!(m.inverse().is_none());
}

#[test]
fn test_inverse_random_100() {
    // 100 个随机矩阵 vs nalgebra 对比
    for _ in 0..100 {
        let m = random_mat4();
        let inv_ours = m.inverse().unwrap();
        let inv_ref = nalgebra_reference(m);
        assert_mat4_eq(inv_ours, inv_ref, 1e-4);
    }
}
```

## 4. 集成

**修复 `engine-render-3d/src/camera.rs#screen_to_world_ray`：**

```rust
// 替换 workaround 版本
pub fn screen_to_world_ray(&self, screen_pos: Vec2, screen_size: Vec2) -> Ray3 {
    let ndc_x = 2.0 * screen_pos.x / screen_size.x - 1.0;
    let ndc_y = 2.0 * screen_pos.y / screen_size.y - 1.0;
    let inv_vp = self.view_projection().inverse()
        .expect("Camera VP should be invertible");
    // 解除 workaround，改用 inv_vp
    ...
}
```

## 5. 验收标准

- [ ] 100 个随机矩阵与 nalgebra 误差 < 1e-4
- [ ] 退化矩阵返回 `None`
- [ ] 4x4 inverse 性能 < 50 ns (AVX2)
- [ ] `engine-render-3d` 3d_picker 示例：ray 拾取数值与参考匹配
- [ ] `screen_to_world_ray` 移除 workaround
