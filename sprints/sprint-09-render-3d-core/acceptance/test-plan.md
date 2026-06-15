# Sprint 09 测试计划

## 概述

本文档定义 Sprint 09（3D 渲染核心）的测试计划，包括单元测试、集成测试、示例测试和验收标准。

---

## 1. 测试概览

| 测试类型 | 测试数量 | 覆盖模块 |
|----------|----------|----------|
| 单元测试 | 12+ | Camera3D, Frustum, Ray3, Transform3D, AABB, Mesh3D, MeshBuilder3D, Scene3D |
| 集成测试 | 5+ | GLTF 加载、渲染一帧 |
| 示例测试 | 7+ | 各 example 编译运行 |
| CI 测试 | 3 平台 | Linux/macOS/Windows |
| 代码质量 | 4 项 | clippy/fmt/doc/unsafe |

---

## 2. 单元测试

### 2.1 Camera3D 矩阵测试

**需求ID**：201, 227

**测试文件**：`engine-render-3d/src/camera/camera3d_test.rs`

| 测试项 | 描述 | 预期结果 |
|--------|------|----------|
| `test_view_matrix_orthogonality` | 验证 view_matrix 的列正交性 | `view * view^T ≈ I` |
| `test_projection_matrix_determinant` | 验证投影矩阵行列式非零 | `det(projection) != 0` |
| `test_view_projection_inverse` | VP 矩阵的逆等于逆 VP | `VP * inv(VP) ≈ I` |
| `test_inverse_view_projection` | 逆 VP 正确 | `IVP * VP ≈ I` |
| `test_screen_to_world_ray_roundtrip` | 射线端点 world_to_screen 回代正确 | 误差 < 0.001 |

```rust
#[test]
fn test_camera_matrix_products() {
    let camera = Camera3D::perspective(60.0_f32.to_radians(), 16.0/9.0, 0.1, 100.0);
    
    let view = camera.view_matrix();
    let proj = camera.projection_matrix();
    let vp = camera.view_projection();
    
    // view * inverse_view ≈ I
    let iv = camera.inverse_view();
    assert!(mat4_near(view * iv, Mat4::IDENTITY, 0.001));
    
    // proj * inverse_proj ≈ I
    let ip = camera.inverse_projection();
    assert!(mat4_near(proj * ip, Mat4::IDENTITY, 0.001));
    
    // VP * inverse_VP ≈ I
    let ivp = camera.inverse_view_projection();
    assert!(mat4_near(vp * ivp, Mat4::IDENTITY, 0.001));
}
```

### 2.2 Frustum 裁剪测试

**需求ID**：202, 228

**测试文件**：`engine-render-3d/src/camera/frustum_test.rs`

| 测试项 | 描述 | 预期结果 |
|--------|------|----------|
| `test_frustum_from_vp` | 从 VP 正确提取 6 平面 | 6 平面法线朝内 |
| `test_frustum_contains_point_inside` | 点在视锥内 | 返回 true |
| `test_frustum_contains_point_outside` | 点在视锥外 | 返回 false |
| `test_frustum_contains_aabb_inside` | AABB 完全在视锥内 | 返回 true |
| `test_frustum_contains_aabb_outside` | AABB 完全在视锥外 | 返回 false |
| `test_frustum_contains_aabb_partial` | AABB 部分在视锥内 | 返回 false |
| `test_frustum_contains_sphere_inside` | 球体完全在视锥内 | 返回 true |
| `test_frustum_contains_sphere_outside` | 球体完全在视锥外 | 返回 false |
| `test_frustum_intersects_aabb` | AABB 与视锥相交 | 返回 true |

```rust
#[test]
fn test_frustum_contains_aabb() {
    let camera = Camera3D::perspective(60.0_f32.to_radians(), 1.0, 0.1, 100.0);
    let frustum = Frustum::from_view_projection(camera.view_projection());
    
    // AABB 完全在视锥内
    let inside_aabb = AABB::new(vec3(-1.0, -1.0, -5.0), vec3(1.0, 1.0, -3.0));
    assert!(frustum.contains_aabb(&inside_aabb));
    
    // AABB 完全在视锥外
    let outside_aabb = AABB::new(vec3(10.0, 0.0, 0.0), vec3(20.0, 1.0, 1.0));
    assert!(!frustum.contains_aabb(&outside_aabb));
}
```

### 2.3 Ray3 命中测试

**需求ID**：203, 229

**测试文件**：`engine-render-3d/src/geometry/ray3_test.rs`

| 测试项 | 描述 | 预期结果 |
|--------|------|----------|
| `test_ray_hit_aabb_front` | 射线正面击中 AABB | 返回正确的 t 值 |
| `test_ray_hit_aabb_miss` | 射线错过 AABB | 返回 None |
| `test_ray_hit_sphere_front` | 射线正面击中球体 | 返回正确的 t 值 |
| `test_ray_hit_sphere_miss` | 射线错过球体 | 返回 None |
| `test_ray_hit_triangle` | Möller-Trumbore 算法正确 | 返回正确的 t 值 |
| `test_ray_at_parameter` | `ray.at(t)` 计算正确 | 误差 < 0.001 |

```rust
#[test]
fn test_ray_hit_triangle() {
    let ray = Ray3::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, -1.0));
    
    // 三角形位于 z = -5
    let v0 = vec3(-1.0, -1.0, -5.0);
    let v1 = vec3(1.0, -1.0, -5.0);
    let v2 = vec3(0.0, 1.0, -5.0);
    
    let result = ray.hit_triangle(v0, v1, v2);
    assert!(result.is_some());
    
    let t = result.unwrap();
    assert!((t - 5.0).abs() < 0.001);  // 距离应该是 5
}
```

### 2.4 Transform3D 测试

**需求ID**：204, 230

**测试文件**：`engine-render-3d/src/transform/transform3d_test.rs`

| 测试项 | 描述 | 预期结果 |
|--------|------|----------|
| `test_transform_identity` | 单位变换 | matrix = I, translation = 0 |
| `test_transform_matrix_inverse` | 矩阵与逆矩阵乘积 | `matrix * inverse ≈ I` |
| `test_transform_point` | 点变换正确 | 包含平移和旋转 |
| `test_transform_vector` | 向量变换正确 | 不包含平移 |
| `test_transform_direction` | 方向变换正确 | 仅旋转影响 |
| `test_transform_lerp` | 插值正确 | 边界值正确 |

```rust
#[test]
fn test_transform_matrix_inverse() {
    let transform = Transform3D::from_translation(vec3(1.0, 2.0, 3.0))
        * Transform3D::from_rotation(Quat::from_euler(glam::EulerRot::XYZ, 0.5, 0.3, 0.1))
        * Transform3D::from_scale(vec3(2.0, 2.0, 2.0));
    
    let matrix = transform.matrix();
    let inverse = transform.inverse_matrix();
    
    // M * M^-1 ≈ I
    let product = matrix * inverse;
    assert!(mat4_near(product, Mat4::IDENTITY, 0.001));
}
```

### 2.5 AABB 测试

**需求ID**：205, 231

**测试文件**：`engine-render-3d/src/geometry/aabb_test.rs`

| 测试项 | 描述 | 预期结果 |
|--------|------|----------|
| `test_aabb_from_points` | 从点集创建 | min/max 正确 |
| `test_aabb_contains_point` | 点包含检测 | 边界内 true |
| `test_aabb_transform` | 变换后包含所有顶点 | 变换正确 |

```rust
#[test]
fn test_aabb_transform() {
    let aabb = AABB::new(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0));
    
    let translation = Mat4::from_translation(vec3(5.0, 0.0, 0.0));
    let rotated = Mat4::from_rotation_y(45.0_f32.to_radians());
    let transform = translation * rotated;
    
    let transformed = aabb.transform_by(transform);
    
    // 验证变换后的 AABB 包含原始 8 个角点
    let corners = [
        vec3(-1.0, -1.0, -1.0), vec3(1.0, -1.0, -1.0),
        vec3(-1.0, 1.0, -1.0), vec3(1.0, 1.0, -1.0),
        vec3(-1.0, -1.0, 1.0), vec3(1.0, -1.0, 1.0),
        vec3(-1.0, 1.0, 1.0), vec3(1.0, 1.0, 1.0),
    ];
    
    for corner in corners {
        let transformed_corner = (transform * corner.extend(1.0)).truncate();
        assert!(transformed.contains_point(transformed_corner));
    }
}
```

### 2.6 Mesh3D 图元生成测试

**需求ID**：232

**测试文件**：`engine-render-3d/src/mesh/mesh3d_test.rs`

| 测试项 | 描述 | 预期结果 |
|--------|------|----------|
| `test_mesh_cube_triangle_count` | 立方体三角面数量 | 应为 12 |
| `test_mesh_cube_vertex_count` | 立方体顶点数量 | 应为 8 |
| `test_mesh_cube_index_count` | 立方体索引数量 | 应为 36 (12*3) |
| `test_mesh_sphere_vertex_index_count` | 球体顶点和索引匹配 | 符合公式 |

```rust
#[test]
fn test_mesh_cube_triangle_count() {
    let cube = Mesh3D::cube(1.0);
    
    // 立方体有 6 个面，每面 2 个三角，每三角 3 索引
    assert_eq!(cube.triangles(), 12);
    assert_eq!(cube.vertices(), 24);  // 每个面 4 个顶点（共享顶点分开的索引）
}
```

### 2.7 MeshBuilder3D 测试

**需求ID**：234

```rust
#[test]
fn test_mesh_builder_basic() {
    let mut builder = MeshBuilder3D::new();
    
    // 添加一个三角
    builder.vertex(Vertex::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0)));
    builder.vertex(Vertex::new(vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)));
    builder.vertex(Vertex::new(vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 0.0), vec2(0.0, 1.0)));
    
    builder.index(0);
    builder.index(1);
    builder.index(2);
    
    let mesh = builder.build();
    
    assert_eq!(mesh.vertices(), 3);
    assert_eq!(mesh.triangles(), 1);
    assert!(mesh.has_normals());
}
```

### 2.8 Scene3D 世界变换测试

**需求ID**：209, 235

```rust
#[test]
fn test_scene_update_world_transforms() {
    let mut scene = Scene3D::new();
    
    // 创建父节点
    let parent_transform = Transform3D::from_translation(vec3(1.0, 0.0, 0.0));
    let mut parent_node = Node3D::new();
    parent_node.set_transform(parent_transform);
    let parent_handle = scene.add_node(parent_node);
    
    // 创建子节点
    let child_transform = Transform3D::from_translation(vec3(2.0, 0.0, 0.0));
    let mut child_node = Node3D::new();
    child_node.set_transform(child_transform);
    let child_handle = scene.add_node(child_node);
    
    // 添加子节点到父节点
    scene.add_child(parent_handle, child_handle);
    
    // 更新世界变换
    scene.update_world_transforms();
    
    // 验证子节点世界变换 = 父 * 子
    let child_world = scene.node(child_handle).world_transform().translation();
    assert!((child_world - vec3(3.0, 0.0, 0.0)).length() < 0.001);
}
```

---

## 3. 集成测试

### 3.1 GLTF 加载测试

**需求ID**：210, 236

**测试文件**：`engine-render-3d/tests/gltf_loading.rs`

```rust
#[test]
fn test_load_cube_gltf() {
    // 假设存在 test fixtures
    let mesh = Mesh3D::from_file("fixtures/cube.gltf");
    
    assert!(mesh.is_ok());
    let mesh = mesh.unwrap();
    
    // 验证基本属性
    assert!(mesh.vertices() > 0);
    assert!(mesh.triangles() > 0);
    assert!(mesh.has_normals());
}

#[test]
fn test_load_nonexistent_gltf() {
    let result = Mesh3D::from_file("nonexistent.gltf");
    assert!(result.is_err());
}
```

### 3.2 渲染一帧测试

**需求ID**：211, 237

```rust
#[test]
fn test_render_one_frame() {
    let engine = Engine::new();
    let renderer = engine.renderer();
    
    let pipeline = RenderPipeline3D::new(renderer, config).unwrap();
    let scene = Scene3D::new();
    let camera = Camera3D::perspective(60.0_f32.to_radians(), 1.0, 0.1, 100.0);
    let lights = LightManager::new();
    
    // 应该不 panic
    pipeline.begin_frame();
    pipeline.draw_scene(&scene, &camera, &lights);
    pipeline.end_frame();
}
```

---

## 4. 示例测试

### 4.1 示例编译测试

**需求ID**：514-543

| 示例 | 编译 | 运行 | 功能验证 |
|------|------|------|----------|
| mesh_viewer | `cargo build --example mesh_viewer` | 加载 GLTF 文件 | 相机控制正常 |
| primitives_demo | `cargo build --example primitives_demo` | 显示 7 种图元 | 旋转动画正常 |
| 3d_scene_simple | `cargo build --example 3d_scene_simple` | 显示立方体+地面+光照 | 光照正确 |
| 3d_lights | `cargo build --example 3d_lights` | 显示多光源 | 光源叠加正确 |
| 3d_frustum_cull | `cargo build --example 3d_frustum_cull` | 显示裁剪统计 | 统计正确 |
| 3d_picker | `cargo build --example 3d_picker` | 点击选中物体 | 射线检测正确 |
| 3d_shader_hot_reload | `cargo build --example 3d_shader_hot_reload` | 监视着色器变化 | 热重载正常 |

---

## 5. 代码质量测试

### 5.1 Clippy 检查

**需求ID**：213, 239, 477

```bash
cargo clippy --workspace -- -D warnings
```

**验收标准**：无 warnings，无 errors

### 5.2 Format 检查

**需求ID**：214, 240, 478

```bash
cargo fmt --check --workspace
```

**验收标准**：所有文件符合 `rustfmt.toml` 规范

### 5.3 Doc 检查

**需求ID**：215, 241, 479

```bash
cargo doc --workspace --no-deps
```

**验收标准**：
- 所有公开 API 有 doc comment
- 文档编译无错误

### 5.4 Unsafe 块限制

**需求ID**：221, 247

```bash
# 统计 unsafe 块数量
rg 'unsafe\s*\{' engine-render-3d/src --type rust | wc -l
```

**验收标准**：`unsafe` 块数量 <= 8

---

## 6. CI 测试

### 6.1 三平台测试

**需求ID**：216, 242, 480

| 平台 | 工具链 | 测试命令 |
|------|--------|----------|
| Linux | stable | `cargo test -p engine-render-3d` |
| macOS | stable | `cargo test -p engine-render-3d` |
| Windows | stable | `cargo test -p engine-render-3d` |

### 6.2 CI Pipeline

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - name: Run tests
        run: cargo test -p engine-render-3d
      - name: Run clippy
        run: cargo clippy --workspace -- -D warnings
      - name: Check format
        run: cargo fmt --check --workspace
      - name: Build docs
        run: cargo doc --workspace --no-deps
```

---

## 7. 验收标准检查清单

### 7.1 功能验收

| 序号 | 验收项 | 对应需求 | 状态 |
|------|--------|----------|------|
| 1 | `cargo run --example mesh_viewer` 能加载 GLTF 并显示 | 150, 176 | ☐ |
| 2 | `cargo run --example 3d_scene_simple` 立方体 + 平面 + 方向光正确渲染 | 180, 462 | ☐ |
| 3 | `cargo run --example 3d_lights` 多光源有效 | 181, 463 | ☐ |
| 4 | `cargo run --example 3d_picker` 点击可命中实体 | 183, 465 | ☐ |
| 5 | `cargo run --example 3d_frustum_cull` 视锥裁剪生效，统计可见 | 182, 464 | ☐ |

### 7.2 质量验收

| 序号 | 验收项 | 对应需求 | 状态 |
|------|--------|----------|------|
| 6 | `cargo test -p engine-render-3d` 全部通过 | 212, 236, 476, 532 | ☐ |
| 7 | `cargo clippy --workspace -- -D warnings` 通过 | 213, 239, 477, 533 | ☐ |
| 8 | `cargo fmt --check --workspace` 通过 | 214, 240, 478, 534 | ☐ |
| 9 | 三平台 CI green | 216, 242, 480, 536 | ☐ |
| 10 | CHANGELOG 记录 0.9.0 | 217, 243, 481, 537 | ☐ |

### 7.3 文档验收

| 序号 | 验收项 | 对应需求 | 状态 |
|------|--------|----------|------|
| 11 | README.md 加入「3D 渲染」章节 | 218, 244, 482 | ☐ |
| 12 | README.md 加入「加载 3D 模型」章节 | 219, 245, 483 | ☐ |
| 13 | README.md 加入「相机与视锥裁剪」章节 | 244, 484 | ☐ |
| 14 | 公开 API doc comment 覆盖率 100% | 220, 246, 485 | ☐ |

### 7.4 其他验收

| 序号 | 验收项 | 对应需求 | 状态 |
|------|--------|----------|------|
| 15 | `unsafe` 块 <= 8 | 221, 247, 486 | ☐ |
| 16 | 新增 example 工程 >= 7 个 | 222, 248, 487 | ☐ |
| 17 | 每帧 RenderStats3D 可在调试面板显示 | 223, 249 | ☐ |
| 18 | RenderStats3D 在 `examples/mesh_viewer` 中通过 `~` 键切换显示 | 225, 251 | ☐ |

---

## 8. 测试执行计划

### 8.1 开发期测试

1. **TDD 方式**：每实现一个模块，先写测试再实现
2. **本地验证**：`cargo test -p engine-render-3d` 需全部通过
3. **示例验证**：每个 example 编译运行验证

### 8.2 提测标准

提测前必须满足：
- [ ] 所有单元测试通过
- [ ] `cargo clippy` 无警告
- [ ] `cargo fmt` 检查通过
- [ ] 所有 example 编译通过
- [ ] 至少 3 个 example 实际运行验证

### 8.3 发布标准

- [ ] CI 三平台全部 green
- [ ] CHANGELOG 已更新
- [ ] 文档已完成