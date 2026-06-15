# Sprint 10 测试计划

## 概述

本文档定义 Sprint 10 PBR 材质与着色器系统的完整测试计划，包括单元测试、集成测试、CI 验证和验收标准。

**对应原需求编号**：147-195, 429-511

---

## 1. 单元测试

### 1.1 PbrMaterial 测试

| 测试编号 | 需求编号 | 测试内容 | 验证方法 |
|----------|----------|----------|----------|
| UT-001 | 147, 429 | PbrMaterial JSON 往返 | 序列化 -> 反序列化 -> 比较字段 |
| UT-002 | 440 | PbrMaterialFlags 位运算 | HAS_ALBEDO_MAP / HAS_NORMAL_MAP 等位与/或操作 |
| UT-003 | 486 | PbrMaterial::default() | 全 1.0 白色，无贴图 |
| UT-004 | 487 | PbrMaterial::from_albedo() | 正确设置 albedo 颜色 |

### 1.2 ShaderGraph 测试

| 测试编号 | 需求编号 | 测试内容 | 验证方法 |
|----------|----------|----------|----------|
| UT-005 | 148, 430 | 拓扑排序 | DAG 无环图拓扑排序正确 |
| UT-006 | 149, 431 | 简单图编译 | Constant -> Output 生成正确代码 |
| UT-007 | 150, 432 | PBR Master 代码生成 | 生成完整 BRDF 函数 |

### 1.3 BRDF 数学函数测试

| 测试编号 | 需求编号 | 测试内容 | 验证方法 |
|----------|----------|----------|----------|
| UT-008 | 151, 433 | fresnel_schlick 值范围 [0,1] | 极端输入 0/1 保持范围 |
| UT-009 | 152, 434 | cook_torrance 不 NaN | 常见输入无 NaN/Inf |
| UT-010 | 153, 435 | Tonemapper::aces 有限输出 | 正输入输出有限 |

### 1.4 IBL 测试

| 测试编号 | 需求编号 | 测试内容 | 验证方法 |
|----------|----------|----------|----------|
| UT-011 | 154, 436 | bake_brdf_lut 非空 | 生成贴图非全黑/全白 |

### 1.5 TextureCompiler 测试

| 测试编号 | 需求编号 | 测试内容 | 验证方法 |
|----------|----------|----------|----------|
| UT-012 | 155, 437 | Mipmap 级数正确 | log2(max(w,h)) + 1 |

### 1.6 Pipeline 测试

| 测试编号 | 需求编号 | 测试内容 | 验证方法 |
|----------|----------|----------|----------|
| UT-013 | 156, 438 | PbrPipeline::new 构建成功 | 无 panic，返回 Ok |

### 1.7 ShaderKey 测试

| 测试编号 | 需求编号 | 测试内容 | 验证方法 |
|----------|----------|----------|----------|
| UT-014 | 157, 439 | 不同 feature 不同 hash | 模拟 permutation 变化 |

### 1.8 单元测试代码模板

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbr_material_json_roundtrip() {
        let original = PbrMaterial::from_albedo(Color::RED)
            .with_metallic(0.5)
            .with_roughness(0.3);

        let json = original.to_json();
        let restored = PbrMaterial::from_json(&json).unwrap();

        assert_eq!(original.albedo(), restored.albedo());
        assert_eq!(original.metallic(), restored.metallic());
        assert_eq!(original.roughness(), restored.roughness());
    }

    #[test]
    fn test_fresnel_schlick_bounds() {
        // 极端输入
        let f0 = 0.04;
        assert!(fresnel_schlick(0.0, f0) >= 0.0 && fresnel_schlick(0.0, f0) <= 1.0);
        assert!(fresnel_schlick(1.0, f0) >= 0.0 && fresnel_schlick(1.0, f0) <= 1.0);
    }

    #[test]
    fn test_cook_torrance_no_nan() {
        let result = cook_torrance(0.5, 0.5, 0.5, 0.4, 0.5, 0.04);
        assert!(!result.is_nan() && !result.is_infinite());
    }

    #[test]
    fn test_tonemapper_aces_bounded() {
        let tonemapper = Tonemapper::Aces;
        let input = Vec3::splat(1000.0); // HDR input
        let output = tonemapper.apply(input);
        assert!(output.x.is_finite() && output.y.is_finite() && output.z.is_finite());
    }
}
```

---

## 2. 集成测试

### 2.1 示例程序测试

| 测试编号 | 需求编号 | 测试内容 | 验证方法 |
|----------|----------|----------|----------|
| IT-001 | 160, 186, 441 | pbr_materials 运行 10 帧无错误 | 启动 -> 渲染 10 帧 -> 无 panic/error |
| IT-002 | 416 | pbr_materials 显示材质球 | 至少 12 种材质球可见 |
| IT-003 | 417 | pbr_materials 鼠标旋转 | 拖拽更新相机角度 |
| IT-004 | 418 | pbr_materials 切换环境贴图 | 至少 2 个环境贴图可切换 |
| IT-005 | 419 | pbr_editor 节点编辑 | 可添加/连接/删除节点 |
| IT-006 | 420 | pbr_ibl IBL 效果 | 环境光照影响材质 |
| IT-007 | 421 | pbr_tonemap 色调映射切换 | 4 种模式可切换 |
| IT-008 | 422 | pbr_shadow 阴影显示 | 方向光阴影可见 |
| IT-009 | 423 | pbr_normal_map 法线贴图 | 法线效果明显 |
| IT-010 | 424 | pbr_emissive 自发光 | 发光区域可见 |
| IT-011 | 425 | pbr_parallax 视差效果 | 视差效果明显 |
| IT-012 | 426 | pbr_clear_coat 清漆 | 清漆层反射可见 |
| IT-013 | 427 | pbr_subsurface SSS | 半透明区域散射可见 |
| IT-014 | 428 | pbr_anisotropy 各向异性 | 拉丝效果可见 |

### 2.2 集成测试代码模板

```rust
#[test]
fn test_pbr_materials_runs_without_error() {
    let renderer = Renderer::new(&Config::default()).unwrap();
    let scene = PbrMaterialsScene::new(&renderer).unwrap();

    for _ in 0..10 {
        scene.update(1.0 / 60.0);
        scene.render(&renderer);
    }

    // 无 panic 即通过
}
```

---

## 3. 代码质量检查

### 3.1 Clippy 检查

| 测试编号 | 需求编号 | 检查内容 |
|----------|----------|----------|
| CQ-001 | 162, 188, 443 | `cargo clippy --workspace -- -D warnings` 通过 |

### 3.2 Format 检查

| 测试编号 | 需求编号 | 检查内容 |
|----------|----------|----------|
| CQ-002 | 163, 189, 444 | `cargo fmt --check --workspace` 通过 |

### 3.3 Documentation 检查

| 测试编号 | 需求编号 | 检查内容 |
|----------|----------|----------|
| CQ-003 | 164, 191, 445 | `cargo doc --workspace --no-deps` 成功 |
| CQ-004 | 171, 197, 452 | 公开 API doc comment 覆盖率 100% |

### 3.4 Unsafe 代码检查

| 测试编号 | 需求编号 | 检查内容 |
|----------|----------|----------|
| CQ-005 | 172, 198, 453 | `unsafe` 块 <= 10 个 |

---

## 4. CI 验证

### 4.1 三平台构建

| 测试编号 | 需求编号 | 平台 | 验证内容 |
|----------|----------|------|----------|
| CI-001 | 165, 192, 446 | Linux | `cargo build --release` 成功 |
| CI-002 | 165, 192, 446 | macOS | `cargo build --release` 成功 |
| CI-003 | 165, 192, 446 | Windows | `cargo build --release` 成功 |

### 4.2 全量测试

| 测试编号 | 需求编号 | 验证内容 |
|----------|----------|----------|
| CI-004 | 161, 187, 442 | `cargo test -p engine-pbr` 全部通过 |

---

## 5. 文档与发布

### 5.1 CHANGELOG

| 测试编号 | 需求编号 | 验证内容 |
|----------|----------|----------|
| DOC-001 | 166, 193, 447 | CHANGELOG 记录版本 0.10.0 |

### 5.2 README 更新

| 测试编号 | 需求编号 | 验证内容 |
|----------|----------|----------|
| DOC-002 | 167, 194, 448 | README.md 加入「PBR 材质系统」章节 |
| DOC-003 | 168, 195, 449 | README.md 加入「着色器系统与 ShaderGraph」章节 |
| DOC-004 | 169, 196, 450 | README.md 加入「IBL 与环境光照」章节 |
| DOC-005 | 170, 451 | README.md 加入「色调映射与颜色分级」章节 |

---

## 6. 示例工程统计

| 测试编号 | 需求编号 | 验证内容 | 要求 |
|----------|----------|----------|------|
| EX-001 | 173, 199, 454 | 新增 example 工程 >= 10 个 | 实际数量 |

示例工程清单：
1. `examples/pbr_materials`
2. `examples/pbr_editor`
3. `examples/pbr_ibl`
4. `examples/pbr_tonemap`
5. `examples/pbr_shadow`
6. `examples/pbr_normal_map`
7. `examples/pbr_emissive`
8. `examples/pbr_parallax`
9. `examples/pbr_clear_coat`
10. `examples/pbr_subsurface`
11. `examples/pbr_anisotropy`

---

## 7. 验收标准清单

| 需求编号 | 验收条件 | 测试方法 | 状态 |
|----------|----------|----------|------|
| 517 | `cargo run --example pbr_materials` 展示多种材质球 | 手动验证 | ☐ |
| 518 | `cargo run --example pbr_editor` 节点编辑器可用 | 手动验证 | ☐ |
| 519 | `cargo run --example pbr_ibl` 展示 HDR 环境光照 | 手动验证 | ☐ |
| 520 | `cargo run --example pbr_shadow` 方向光阴影正常 | 手动验证 | ☐ |
| 521 | `cargo run --example pbr_normal_map` 法线贴图效果明显 | 手动验证 | ☐ |
| 522 | `cargo run --example pbr_parallax` 视差效果明显 | 手动验证 | ☐ |
| 523 | `cargo test -p engine-pbr` 全部通过 | CI | ☐ |
| 524 | `cargo clippy --workspace -- -D warnings` 通过 | CI | ☐ |
| 525 | `cargo fmt --check --workspace` 通过 | CI | ☐ |
| 526 | 三平台 CI green | CI | ☐ |
| 527 | CHANGELOG 记录 0.10.0 | 代码审查 | ☐ |

---

## 8. 性能基准

| 需求编号 | 验证内容 | 指标 |
|----------|----------|------|
| 175, 455 | 典型场景 60fps | 30 个 PBR mesh, GTX 1660 |

---

## 9. 编辑器集成

| 需求编号 | 验证内容 |
|----------|----------|
| 176, 203, 456 | 材质 inspector 预览可用 |

---

## 10. 测试执行计划

### Phase 1: 单元测试（第 1-2 周）
- BRDF 数学函数测试
- PbrMaterial 序列化测试
- ShaderGraph 编译测试
- TextureCompiler 测试

### Phase 2: 集成测试（第 2-3 周）
- 各 example 程序运行测试
- 渲染结果视觉验证
- 性能基准测试

### Phase 3: CI 验证（第 3-4 周）
- 三平台构建验证
- Clippy/Format/Doc 检查
- 全量测试套件执行

### Phase 4: 验收（第 4 周）
- 手动验收测试
- 文档审查
- CHANGELOG 更新

---

## 11. 测试覆盖矩阵

| 模块 | 单元测试 | 集成测试 | CI |
|------|----------|----------|-----|
| PbrMaterial | UT-001~004 | - | ✓ |
| ShaderCompiler | - | - | ✓ |
| ShaderGraph | UT-005~007 | - | ✓ |
| BRDF Functions | UT-008~010 | - | ✓ |
| IBL | UT-011 | IT-006 | ✓ |
| TextureCompiler | UT-012 | - | ✓ |
| PbrPipeline | UT-013 | - | ✓ |
| ShaderKey | UT-014 | - | ✓ |
| Examples | - | IT-001~014 | ✓ |
| Code Quality | - | - | CQ-001~005 |
| Documentation | - | - | DOC-001~005 |
