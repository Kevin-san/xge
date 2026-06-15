# Sprint 13 测试计划

## 测试范围

本测试计划覆盖 Sprint 13 两大核心模块：
- `engine-particles`：粒子系统
- `engine-postfx`：后期特效栈

---

## 一、单元测试（Unit Tests）

### 1.1 engine-particles 单元测试

#### 1.1.1 Particle 数据结构测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| P-UT-001 | `Particle::new` 创建粒子 | position=(0,0,0), lifetime=2.0 | age=0, is_alive=true | 181, 51 |
| P-UT-002 | `Particle::is_alive` 超龄死亡 | age=3.0, lifetime=2.0 | false | 51 |
| P-UT-003 | `Particle::is_alive` 存活 | age=1.0, lifetime=2.0 | true | 51 |
| P-UT-004 | `Particle::normalized_age` | age=1.0, lifetime=2.0 | 0.5 | 189 |
| P-UT-005 | `Particle::update` 推进 age | dt=0.016, age=0 | age=0.016 | 190 |

#### 1.1.2 ParticlePool 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| P-UT-010 | `spawn` 成功 | max=10, spawn 1 particle | alive_count=1 | 193 |
| P-UT-011 | `spawn` 池满 | max=1, spawn 2 particles | 第2次返回false | 193 |
| P-UT-012 | `kill` 标记死亡 | index=0 | alive_count=0 | 194 |
| P-UT-013 | `dead_count` 正确 | alive=3, total=10 | dead_count=7 | 196 |
| P-UT-014 | `swap_remove` 交换删除 | remove index=0 | 最后粒子移至index=0 | 197 |

#### 1.1.3 EmitShape 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| P-UT-020 | `Point` sample | rng | 返回(origin, forward) | 241, 314 |
| P-UT-021 | `Sphere` sample 长度 | rng | 接近 radius | 243, 315 |
| P-UT-022 | `Box` sample 范围 | rng | 在 AABB 内部 | 242, 316 |
| P-UT-023 | `Sphere(Volume)` sample 均匀 | 10000 samples | 半径分布均匀 | 243 |
| P-UT-024 | `Sphere(Shell)` sample 表面 | rng | 长度 ≈ radius | 243 |

#### 1.1.4 BurstConfig 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| P-UT-030 | `should_fire` 触发 | time=1.0, current_time=1.0 | true | 229 |
| P-UT-031 | `should_fire` 未触发 | time=2.0, current_time=1.0 | false | 229 |
| P-UT-032 | `should_fire` 循环触发 | cycles=3, current_time=2.5, interval=1.0 | true（第3次） | 229 |
| P-UT-033 | `reset` 重置 | 已触发1次 | time_offset 重置 | 230 |

#### 1.1.5 Module 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| P-UT-040 | `InitialVelocityModule` | velocity区间(-1,1), (1,2) | 输出在区间内 | 262, 322 |
| P-UT-041 | `GravityModule` | gravity=(0,-9.8,0), dt=0.1 | vy 减少 0.98 | 273, 323 |
| P-UT-042 | `DragModule` | drag=0.5, velocity=(10,0,0), dt=1.0 | 速度衰减至0 | 275, 324 |
| P-UT-043 | `ColorGradient::sample` | t=0.5, stops=[(0,RED),(1,BLUE)] | 混合色 | 267, 325 |
| P-UT-044 | `ColorGradient::sample` 边界 | t=-0.5 | clamp 到 RED | 267, 326 |
| P-UT-045 | `ColorGradient::sample` 边界 | t=1.5 | clamp 到 BLUE | 267, 326 |
| P-UT-046 | `AttractorModule` 中心 | position=center | 吸引=0（稳定点） | 281, 327 |
| P-UT-047 | `CollisionModule::Sphere` 法向量 | 碰撞速度(0,-1,0), normal=(0,1,0) | 反射向量单位化 | 290, 328 |
| P-UT-048 | `CollisionModule::Plane` | 碰撞 normal=(0,1,0) | 正确反弹 | 291, 329 |
| P-UT-049 | `KillModule` 超龄 | age > lifetime | particle.is_alive = false | 296, 330 |

---

### 1.2 engine-postfx 单元测试

#### 1.2.1 PostProcessStack 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| FX-UT-001 | `add_pass` | add FXAA | passes().len() = 1 | 105 |
| FX-UT-002 | `remove_pass` | remove index=0 | passes().len() = 0 | 107 |
| FX-UT-003 | `reorder` | 交换顺序 | order() 返回新顺序 | 108, 372 |
| FX-UT-004 | `apply` 顺序 | 3个Pass | 按顺序调用每个Pass.apply | 373, 542 |
| FX-UT-005 | `enabled=false` 时 | apply | Pass.apply 不执行 | 367, 543 |

#### 1.2.2 FXAA 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| FX-UT-010 | `enabled=false` | apply | no-op | 367, 543 |

#### 1.2.3 Bloom 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| FX-UT-020 | `extract` threshold=0 | 全白输入 | 输出=输入 | 544 |
| FX-UT-021 | `extract` threshold=1.0 | 亮度0.5输入 | 输出全黑 | 400 |
| FX-UT-022 | `mip_count` | 默认配置 | mip_count=6 | 399 |

#### 1.2.4 SSAO 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| FX-UT-030 | `generate_kernel` | size=32 | 生成32个向量 | 422, 545 |
| FX-UT-031 | `blur` 保边 | 棋盘格输入 | 边缘保持 | 425 |

#### 1.2.5 ToneMapping 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| FX-UT-040 | `Reinhard` L=1 | Reinhard, L=1 | 0.5 | 436, 546 |
| FX-UT-041 | `ACES` 曲线 | 任意输入 | S-curve形状 | 438 |
| FX-UT-042 | `exposure` | exposure=2.0, color=0.5 | 输出=1.0 | 441 |

#### 1.2.6 ColorGrading 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| FX-UT-050 | `white_balance` | 纯灰 (0.5,0.5,0.5) | 不变 | 447, 547 |
| FX-UT-051 | `saturation=0` | 任意颜色 | 灰度输出 | 448 |
| FX-UT-052 | `lut` 加载 | 32x32x32 LUT | 正确采样 | 445 |

#### 1.2.7 Vignette 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| FX-UT-060 | 中心像素 | center=(0.5,0.5) | 输出=输入 | 463, 548 |

#### 1.2.8 MotionVectorTexture 测试

| 测试用例 ID | 描述 | 输入 | 预期结果 | 需求 ID |
|------------|------|------|----------|---------|
| FX-UT-070 | 前向单位向量 | 前向运动 | motion_vector ≈ (0,0) | 474, 549 |

---

## 二、集成测试（Integration Tests）

### 2.1 engine-particles 集成测试

| 测试用例 ID | 描述 | 场景 | 预期结果 |
|------------|------|------|----------|
| P-IT-001 | 完整粒子生命周期 | spawn → update → kill | particle_count 正确 |
| P-IT-002 | 多个 Emitter | system.add_emitter x3 | 3个emitter均工作 |
| P-IT-003 | Burst 发射 | BurstConfig(time=1.0, count=100) | 1.0秒时发射100个 |
| P-IT-004 | Continuous 发射 | rate=60, dt=1/60 | 每帧发射1个 |
| P-IT-005 | Mixed 发射 | rate=10 + Burst | 持续+爆发同时 |
| P-IT-006 | SubEmitter 级联 | fire→smoke→spark | 三级粒子共存 |
| P-IT-007 | LOD 降级 | 距离>100m | particle_count 降低 |

### 2.2 engine-postfx 集成测试

| 测试用例 ID | 描述 | 场景 | 预期结果 |
|------------|------|------|----------|
| FX-IT-001 | 完整 Pass 链 | FXAA→Bloom→ToneMap | 按顺序执行 |
| FX-IT-002 | Ping-Pong RT | 2帧循环 | RT 正确复用 |
| FX-IT-003 | HDR/LDR 双路径 | hdr=true/false | 正确分支 |
| FX-IT-004 | Pass 跳过 | pass.enabled=false | 跳过该 Pass |

---

## 三、性能基准测试（Benchmark Tests）

### 3.1 engine-particles 性能测试

| 测试用例 ID | 描述 | 目标 | 需求 ID |
|------------|------|------|---------|
| P-BM-001 | 100,000 CPU 粒子 update | <= 8ms/帧（release） | 206, 558 |
| P-BM-002 | GPU 粒子 100,000 | <= 2ms/帧 | 331 |
| P-BM-003 | 100 Emitters 同步 | 无帧率下降 | - |

### 3.2 engine-postfx 性能测试

| 测试用例 ID | 描述 | 目标 | 需求 ID |
|------------|------|------|---------|
| FX-BM-001 | 默认后期链 | <= 6ms（1080p / integrated GPU） | 565 |
| FX-BM-002 | Bloom only | <= 2ms | - |
| FX-BM-003 | SSAO 半分辨率 | <= 2ms | 518 |

---

## 四、示例工程测试（Example Tests）

### 4.1 particles_* 示例

| 测试用例 ID | 命令 | 预期结果 | 需求 ID |
|------------|------|----------|---------|
| P-EX-001 | `cargo run --example particles_2d` | 成功运行，无 panic | 143 |
| P-EX-002 | `cargo run --example particles_3d` | 成功运行，无 panic | 144, 571 |
| P-EX-003 | `cargo run --example particles_fire` | 火焰+烟雾效果可见 | 145 |
| P-EX-004 | `cargo run --example particles_smoke` | 烟雾湍流可见 | 146 |
| P-EX-005 | `cargo run --example particles_rain` | 雨滴拉伸效果可见 | 147 |
| P-EX-006 | `cargo run --example particles_snow` | 雪花飘落可见 | 148 |
| P-EX-007 | `cargo run --example particles_collision` | 碰撞反弹可见 | 149 |
| P-EX-008 | `cargo run --example particles_gpu` | 10万粒子流畅 | 102, 572 |

### 4.2 postfx_* 示例

| 测试用例 ID | 命令 | 预期结果 | 需求 ID |
|------------|------|----------|---------|
| FX-EX-001 | `cargo run --example postfx_stack` | 成功运行，可切换 Pass | 150, 572 |
| FX-EX-002 | `cargo run --example postfx_bloom` | HDR 发光效果可见 | 151 |
| FX-EX-003 | `cargo run --example postfx_dof` | 散景效果可见 | 152 |
| FX-EX-004 | `cargo run --example postfx_ssao` | AO 增加深度感 | 153 |
| FX-EX-005 | `cargo run --example postfx_color` | LUT 正确应用 | 154 |

---

## 五、CI 测试矩阵

| 平台 | particles | postfx | clippy | fmt | doc | WASM |
|------|-----------|--------|--------|-----|-----|------|
| Windows | ✅ | ✅ | ✅ | ✅ | ✅ | - |
| macOS | ✅ | ✅ | ✅ | ✅ | ✅ | - |
| Linux | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

---

## 六、测试执行命令

### 6.1 单元测试

```bash
# particles 单元测试
cargo test -p engine-particles

# postfx 单元测试
cargo test -p engine-postfx

# 所有单元测试
cargo test --workspace
```

### 6.2 性能基准测试

```bash
# particles 基准测试
cargo bench -p engine-particles

# 手动计时（示例）
time cargo run --release --example particles_3d
```

### 6.3 Clippy / Fmt / Doc

```bash
# clippy
cargo clippy --workspace -- -D warnings

# fmt
cargo fmt --check --workspace

# doc
cargo doc --workspace --no-deps
```

### 6.4 示例运行

```bash
# 粒子示例
cargo run --example particles_2d
cargo run --example particles_3d
cargo run --example particles_fire
cargo run --example particles_smoke
cargo run --example particles_rain
cargo run --example particles_snow
cargo run --example particles_collision
cargo run --example particles_gpu

# 后期示例
cargo run --example postfx_stack
cargo run --example postfx_bloom
cargo run --example postfx_dof
cargo run --example postfx_ssao
cargo run --example postfx_color
```

---

## 七、验收标准

| 类别 | 验收项 | 标准 |
|------|--------|------|
| 单元测试 | particles 通过率 | 100% |
| 单元测试 | postfx 通过率 | 100% |
| 性能 | 100k CPU 粒子 | <= 8ms/帧 |
| 性能 | 后期栈默认链 | <= 6ms |
| 示例 | particles_3d | cargo run 成功 |
| 示例 | postfx_stack | cargo run 成功，可切换 Pass |
| CI | 三平台 | Windows/macOS/Linux green |
| WASM | 编译 | wasm32-unknown-unknown 编译通过 |
| 代码质量 | clippy | 无 warnings |
| 代码质量 | fmt | 符合格式 |
| 文档 | doc comment | 公开 API >= 95% 覆盖 |
| 文档 | CHANGELOG | 记录 0.13.0 |
| 文档 | README | 包含粒子系统和后期栈章节 |

---

## 八、测试日程

| 阶段 | 时间 | 内容 |
|------|------|------|
| 单元测试开发 | Sprint 第 1 周 | 完成 P-UT-*/FX-UT-* 测试 |
| 集成测试开发 | Sprint 第 2 周 | 完成 P-IT-*/FX-IT-* 测试 |
| 性能调优 | Sprint 第 3 周 | 基准测试 + 性能优化 |
| CI 验证 | Sprint 第 4 周 | 三平台 CI green |
| 示例完善 | Sprint 第 4 周 | UI 参数调整 |
