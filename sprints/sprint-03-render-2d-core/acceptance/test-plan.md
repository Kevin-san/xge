# 测试计划

## 1. 概述

本文档定义 Sprint 03 `engine-render` crate 的测试策略，包括单元测试、集成测试、性能测试和验收标准。

**目标**：确保 2D 渲染核心功能稳定可靠，满足性能要求。

---

## 2. 测试目标

| 指标 | 目标 |
|------|------|
| 单元测试覆盖率 | >= 80% |
| 单元测试数量 | >= 20 条（需求 111） |
| 示例运行 | 11+ 个示例全部通过 |
| 性能测试 | 10k/100k 精灵帧率基线 |
| clippy | 无 warning |
| fmt | 检查通过 |
| cargo doc | 生成成功 |

---

## 3. 单元测试（需求 362-370, 417-424）

### 3.1 Image 模块测试

| 测试项 | 需求ID | 测试内容 |
|--------|--------|----------|
| `Image::from_bytes` | 362 | 从字节数据加载图像，验证尺寸和像素 |
| `Image::flip_horizontal` | 221 | 水平翻转后像素位置正确 |
| `Image::flip_vertical` | 222 | 垂直翻转后像素位置正确 |
| `Image::crop` | 220 | 裁剪区域边界正确 |

### 3.2 Texture 模块测试

| 测试项 | 需求ID | 测试内容 |
|--------|--------|----------|
| `Texture::update` | 363 | 更新纹理区域，数据正确上传 |
| `Texture::format` | 168 | 格式返回正确 |

### 3.3 SpriteBatch 模块测试

| 测试项 | 需求ID | 测试内容 |
|--------|--------|----------|
| `SpriteBatch::add` | 363 | 添加精灵返回正确索引 |
| `SpriteBatch::draw` | 363 | 批量绘制无 panic |
| `SpriteBatch::clear` | 281 | 清空后 len == 0 |

### 3.4 Camera 模块测试

| 测试项 | 需求ID | 测试内容 |
|--------|--------|----------|
| `Camera::screen_to_world` | 365 | 屏幕坐标转世界坐标正确 |
| `OrthographicCamera::projection` | 368 | 投影矩阵计算正确 |

### 3.5 Color 模块测试

| 测试项 | 需求ID | 测试内容 |
|--------|--------|----------|
| `Color::from_hex` | 366 | "#FF0000" -> RED |
| `Color::to_hex` | 366 | RED -> "#FF0000FF" |
| `Color::from_hex / to_hex` 往返 | 366 | 往返一致 |
| `Color::lerp` | 303 | 插值结果正确 |

### 3.6 TextureAtlas 模块测试

| 测试项 | 需求ID | 测试内容 |
|--------|--------|----------|
| 打包不越界 | 367 | 所有子图在图集边界内 |
| `get_uv` | 267 | UV 坐标范围 [0,1] |
| `get_rect` | 266 | 矩形无重叠 |

### 3.7 OrthographicCamera 模块测试

| 测试项 | 需求ID | 测试内容 |
|--------|--------|----------|
| 投影矩阵 | 368 | 矩阵元素正确 |
| `view_projection` | 336 | 视图投影正确 |

### 3.8 VertexLayout 模块测试

| 测试项 | 需求ID | 测试内容 |
|--------|--------|----------|
| `VertexLayout::stride` | 369 | stride = sum of attributes |
| `push::<Vec2>` | 377-378 | 正确添加属性 |

### 3.9 BlendMode 模块测试

| 测试项 | 需求ID | 测试内容 |
|--------|--------|----------|
| `BlendMode::Alpha::to_gl_enum` | 370 | 返回正确 GL 枚举 |

---

## 4. 集成测试

### 4.1 渲染管线集成测试

| 测试项 | 测试内容 |
|--------|----------|
| 纹理加载与绑定 | 加载纹理 → 创建 Sprite → 绘制 |
| 批处理集成 | SpriteBatch → 多批次 → flush |
| 图集动画 | TextureAtlas → AnimatedSprite → update/draw |
| 相机渲染 | OrthographicCamera → screen_to_world → 绘制 |

### 4.2 示例集成测试

| 示例 | 验证点 |
|------|--------|
| sprite_draw | 精灵显示，无错误 |
| multi_sprite | 1000 精灵，FPS >= 30 |
| batch_draw | 10k 精灵，draw_calls <= 5 |
| atlas_animation | 动画播放流畅 |
| camera_follow | 跟随无抖动 |
| shape_draw | 所有图形正确显示 |
| debug_draw | 调试图形正确 |
| blend_mode | 混合效果可见 |
| scissor | 裁剪正确 |
| transform_stack | 变换累积正确 |

---

## 5. 性能测试

### 5.1 性能基线（criterion）

| 场景 | 目标 |
|------|------|
| 10k 精灵渲染 | >= 60 FPS |
| 100k 精灵渲染 | >= 30 FPS |
| TextureAtlas 构建 | <= 100ms (100 张图) |

### 5.2 性能测试代码

```rust
#[cfg(test)]
mod benchmarks {
    use criterion::*;

    fn sprite_batch_10k(c: &mut Criterion) {
        c.bench_function("sprite_batch_10k", |b| {
            let mut batch = SpriteBatch::new(texture.clone());
            for i in 0..10000 {
                batch.add(&sprite, Vec2::new(i as f32, 0.0));
            }

            b.iter(|| {
                batch.draw(&mut ctx);
            });
        });
    }

    fn sprite_batch_100k(c: &mut Criterion) {
        c.bench_function("sprite_batch_100k", |b| {
            let mut batch = SpriteBatch::new(texture.clone());
            for i in 0..100000 {
                batch.add(&sprite, Vec2::new(i as f32, 0.0));
            }

            b.iter(|| {
                batch.draw(&mut ctx);
            });
        });
    }
}
```

---

## 6. 代码质量测试

### 6.1 clippy 检查

```bash
cargo clippy -- -D warnings
```

| 检查项 | 要求 |
|--------|------|
| 警告数 | 0 |
| 错误数 | 0 |
| 安全问题 | 0 |

### 6.2 fmt 检查

```bash
cargo fmt --check
```

| 检查项 | 要求 |
|--------|------|
| 格式差异 | 0 |

### 6.3 doc 检查

```bash
cargo doc --workspace --no-deps
```

| 检查项 | 要求 |
|--------|------|
| 文档生成 | 成功 |
| 链接错误 | 0 |

---

## 7. CI 测试矩阵

| 平台 | 操作系统 | 工具链 |
|------|----------|--------|
| Linux | Ubuntu 20.04 | stable |
| macOS | macOS 11 | stable |
| Windows | Windows 2019 | stable |

| 测试项 | Linux | macOS | Windows |
|--------|-------|-------|---------|
| cargo build | ✓ | ✓ | ✓ |
| cargo test | ✓ | ✓ | ✓ |
| cargo clippy | ✓ | ✓ | ✓ |
| cargo fmt --check | ✓ | ✓ | ✓ |
| cargo doc | ✓ | ✓ | ✓ |
| Examples | ✓ | ✓ | ✓ |

---

## 8. 验收标准清单（需求 447-457）

### 8.1 功能验收

- [ ] `cargo run --example sprite_draw` 显示精灵
- [ ] `cargo run --example batch_draw` 批量绘制无闪烁
- [ ] `cargo run --example atlas_animation` 可播放动画
- [ ] `cargo run --example camera_follow` 相机可跟随对象移动

### 8.2 测试验收

- [ ] `cargo test -p engine-render` 全部通过

### 8.3 代码质量验收

- [ ] clippy 无 warning
- [ ] fmt check 通过
- [ ] cargo doc 成功

### 8.4 CI 验收

- [ ] 三平台 CI green

### 8.5 文档验收

- [ ] README 至少 5 章节

---

## 9. 测试时间安排

| 阶段 | 活动 | 时长 |
|------|------|------|
| Week 1 | 单元测试编写（Image, Texture, Sprite） | 2 days |
| Week 2 | 单元测试编写（Camera, Color, Atlas） + 集成测试 | 3 days |
| Week 3 | 性能测试 + CI 配置 | 2 days |
| Week 4 | 验收测试 + bug 修复 | 3 days |

---

## 10. 测试环境要求

### 10.1 硬件要求

| 指标 | 最低要求 | 推荐要求 |
|------|----------|----------|
| CPU | 4 核 | 8 核 |
| 内存 | 8 GB | 16 GB |
| GPU | OpenGL 3.0 | OpenGL 4.5 |

### 10.2 软件要求

| 软件 | 版本 |
|------|------|
| Rust | 1.60+ |
| cargo | 1.60+ |
| GLFW | 3.3+ |
| OpenGL | 3.0+ |

---

## 11. 测试报告模板

```markdown
# Sprint 03 测试报告

## 测试概要
- 测试日期：YYYY-MM-DD
- 测试人员：XXX
- 测试环境：Linux/macOS/Windows

## 测试结果

### 单元测试
| 模块 | 用例数 | 通过数 | 失败数 | 覆盖率 |
|------|--------|--------|--------|--------|
| Image | 5 | 5 | 0 | 85% |
| Texture | 2 | 2 | 0 | 80% |
| ... | ... | ... | ... | ... |

### 性能测试
| 场景 | 结果 | 目标 | 状态 |
|------|------|------|------|
| 10k 精灵 | 65 FPS | >= 60 FPS | ✓ |
| 100k 精灵 | 35 FPS | >= 30 FPS | ✓ |

### CI 状态
- Linux: ✓
- macOS: ✓
- Windows: ✓

## 问题列表

| ID | 描述 | 严重度 | 状态 |
|----|------|--------|------|
| 1 | ... | 高 | 已修复 |

## 结论
[通过/不通过] - 原因
```