# 测试计划（Test Plan）

## 概述

本文档定义 Sprint 04 的测试策略、测试用例、验收标准，确保 `engine-physics-2d` 和 `engine-scene` 两个 crate 的质量。

---

## 测试策略

### 1. 测试分层

| 层级 | 范围 | 测试方式 |
|------|------|----------|
| 单元测试 | 独立模块/API | `#[test]` |
| 集成测试 | 跨模块协作 | `tests/` 目录 |
| 示例测试 | 示例工程可运行 | `cargo run --example` |
| 性能测试 | 性能指标 | 手动验证 / benchmark |
| CI 测试 | 三平台构建 | GitHub Actions |

### 2. 代码质量

| 检查项 | 命令 | 目标 |
|--------|------|------|
| 格式化 | `cargo fmt --check --workspace` | 通过 |
| 代码风格 | `cargo clippy --workspace -- -D warnings` | 无 warning |
| 文档 | `cargo doc --workspace --no-deps` | 成功生成 |
| 公开 API 数量 | 手动统计 | ≤ 120 |
| unsafe 代码 | 手动统计 | ≤ 5 处 |
| doc comment 覆盖 | 手动检查 | 100% |

---

## 单元测试用例

### 1. RigidBody2DBuilder 测试

| 编号 | 用例 | 验证点 |
|------|------|--------|
| 169 | `RigidBody2DBuilder::dynamic().build()` | 构建成功，类型为 Dynamic |
| 169 | `RigidBody2DBuilder::static_().build()` | 构建成功，类型为 Static |
| 169 | Builder 链式调用 | 所有配置项正确应用 |

### 2. 物理仿真测试

| 编号 | 用例 | 验证点 |
|------|------|--------|
| 170 | 重力下球体下落 | 1秒后 y = -4.905（自由落体） |
| 171 | 两圆碰撞反弹 | 弹性碰撞速度守恒 |
| 197 | 双球碰撞速度守恒 | 碰撞前后总动能守恒 |
| 172 | Circle vs AABB 碰撞 | 碰撞点坐标正确 |

### 3. 空间查询测试

| 编号 | 用例 | 验证点 |
|------|------|--------|
| 173 | RayCast 命中坐标 | 命中点与预期偏差 < 0.001 |
| 200 | RayCastHit2D 命中点坐标 | 坐标正确 |
| 419 | RayCast 过滤 sensor | `include_sensors: false` 不返回传感器 |
| 212 | point_overlap 返回传感器 | 包含传感器的碰撞体 |

### 4. 碰撞分组测试

| 编号 | 用例 | 验证点 |
|------|------|--------|
| 78 | `CollisionGroup::can_interact_with` | 正确判断可交互性 |
| 206 | CollisionGroup 分组过滤 | 不同组不产生碰撞 |
| 420 | CollisionGroup 互相过滤 | membership + filter 组合正确 |

### 5. SceneTree 测试

| 编号 | 用例 | 验证点 |
|------|------|--------|
| 174 | SceneTree 遍历顺序 | 先序 update，后序 draw |
| 201 | SceneTree 遍历顺序正确 | 子节点在父节点之后 update |
| 421 | SceneTree 层级遍历 | 父子关系正确 |

### 6. Prefab 测试

| 编号 | 用例 | 验证点 |
|------|------|--------|
| 175 | Prefab 实例化不修改模板 | 修改实例后模板不变 |
| 202 | Prefab 实例化不修改模板 | 同上 |
| 355 | Prefab 实例化与原模板独立 | 数据隔离 |

### 7. SceneLoader 测试

| 编号 | 用例 | 验证点 |
|------|------|--------|
| 176 | SceneLoader JSON 往返 | 序列化/反序列化数据一致 |
| 203 | SceneLoader JSON 往返 | 同上 |
| 366 | SceneLoader JSON 往返 | 同上 |

### 8. Tween 测试

| 编号 | 用例 | 验证点 |
|------|------|--------|
| 177 | Tween ease_in_out 时间曲线 | 线性时间对应正确值 |
| 204 | Tween ease_in_out 时间曲线 | 同上 |
| 367 | Tween 线性缓动 | `progress()` 在 0.0~1.0 范围 |
| 368 | Ease::InOutCubic 在 t=0 / 0.5 / 1 处输出 | 分别为 0.0 / 0.5 / 1.0 |
| 424 | Tween ease_in_out 时间曲线 | 同上 |

### 9. Timer 测试

| 编号 | 用例 | 验证点 |
|------|------|--------|
| 178 | Timer 重复模式下 tick N 次后 finished 次数正确 | Once 模式只触发一次 |
| 205 | Timer 重复模式下 tick N 次后 finished 次数正确 | 同上 |
| 369 | Timer Once 模式的 finished 行为 | tick 后 finished 为 true |

### 10. Signal 测试

| 编号 | 用例 | 验证点 |
|------|------|--------|
| 370 | Signal emit 被所有 handler 接收 | 多 handler 全部调用 |
| 427 | Signal emit 被所有 handler 接收 | 同上 |

---

## 集成测试

| 编号 | 测试名称 | 验证点 |
|------|----------|--------|
| 371 | `cargo test --workspace` | 所有测试通过 |
| 372 | `cargo fmt --check --workspace` | 格式化通过 |
| 373 | `cargo clippy --workspace -- -D warnings` | Clippy 通过 |
| 374 | `cargo doc --workspace --no-deps` | 文档生成成功 |

---

## 平台测试

| 编号 | 平台 | 验证点 |
|------|------|--------|
| 375 | Windows | CI green |
| 375 | macOS | CI green |
| 375 | Linux | CI green |

---

## 性能测试用例

| 编号 | 用例 | 指标 | 目标 |
|------|------|------|------|
| 223 | 100 球 1680x720 | FPS | ≥ 60fps |
| 224 | 1000 球 1680x720 | FPS | ≥ 30fps |
| 196 | 性能 Bench | FPS | 100球 60fps |
| 197 | 性能 Bench | FPS | 1000球 30fps |

---

## 示例工程验收

| 编号 | 示例 | 验证命令 | 预期结果 |
|------|------|----------|----------|
| 158 | pixel_platformer | `cargo run --example pixel_platformer` | 可玩（跳跃、碰撞、得分） |
| 159 | ball_pit | `cargo run --example ball_pit` | 1000球稳定 30fps |
| 160 | dominoes | `cargo run --example dominoes` | 多米诺骨牌倒下 |
| 161 | ray_cast | `cargo run --example ray_cast` | 射线检测正常 |
| 162 | joints | `cargo run --example joints` | 关节演示正常 |
| 163 | scene_tree | `cargo run --example scene_tree` | 节点层级演示正常 |
| 164 | prefab_basic | `cargo run --example prefab_basic` | 实例化无崩溃 |
| 165 | scene_switch | `cargo run --example scene_switch` | 多场景切换正常 |
| 166 | signals | `cargo run --example signals` | 信号派发正常 |
| 167 | tween | `cargo run --example tween` | 补间正常 |
| 168 | hello_engine | `cargo run --example hello_engine` | 最小可运行 |
| 413 | timer | `cargo run --example timer` | 定时器演示正常 |
| 360 | physics_perf | `cargo run --example physics_perf` | 性能达标 |

---

## Debug 功能验证

| 编号 | 功能 | 验证方法 |
|------|------|----------|
| 225 | 按 `` ` `` + B 显示/隐藏碰撞体线框 | 切换后线框显隐变化 |
| 226 | 按 `` ` `` + P 暂停/继续物理 | 切换后物理状态变化 |
| 227 | 按 `` ` `` + F 显示/隐藏 FPS | 切换后 FPS 显示变化 |

---

## pixel_platformer 详细测试

| 编号 | 功能 | 验证点 |
|------|------|--------|
| 191 | 至少 3 个关卡 | 场景文件中定义 3 个关卡节点 |
| 192 | 至少 1 种敌人 | 敌人 Prefab，简单的左右巡逻 AI |
| 193 | 至少 1 种可收集金币 | 金币 Prefab，收集后加分 |
| 194 | HUD 显示分数与生命 | UI 界面显示 |
| 195 | 支持空格跳跃 + 左右方向键移动 | 输入响应正确 |
| 343 | Title 场景 | 标题画面 |
| 344 | Game 场景 | 游戏主场景 |
| 345 | GameOver 场景 | 游戏结束画面 |
| 346 | 使用 Prefab 创建玩家、敌人、金币 | Prefab 实例化 |
| 347 | 使用信号连接点击事件 | Signal connect/emit |
| 348 | 使用 Tween 做过渡动画 | Tween 应用 |
| 349 | HUD 分数使用 UI 系统 | Sprint 06 后升级 |

---

## 文档验收

| 编号 | 文档 | 验证点 |
|------|------|--------|
| 185 | CHANGELOG | 记录版本 0.4.0（阶段一完成） |
| 186 | README.md | 加入「物理世界」章节 |
| 187 | README.md | 加入「场景与节点」章节 |
| 188 | README.md | 加入「预制体与场景切换」章节 |
| 189 | README.md | 加入「信号与 Tween」章节 |
| 218 | README.md | 记录玩法与操作 |
| 376 | CHANGELOG | 记录版本 0.4.0「阶段一完成」 |
| 377-381 | README.md | 各章节齐全 |
| 434-438 | README.md 各章节 | 同上 |

---

## 公开 API 质量

| 编号 | 检查项 | 目标 |
|------|------|------|
| 442 | 公开 API 数量 | ≤ 120 |
| 443 | doc comment 覆盖率 | 100% |
| 441 | unsafe 代码数量 | ≤ 5 |

---

## 测试执行计划

### 本地开发阶段
1. 每次 PR 需要通过所有单元测试
2. `cargo fmt --check` 和 `cargo clippy` 无 warning
3. 示例工程可独立运行

### CI 阶段
1. 三平台（Windows/macOS/Linux）并行构建
2. `cargo test --workspace` 全量测试
3. 性能测试手动验证（CI 不强制）

### 验收阶段
1. Sprint Review 前完成所有测试用例
2. 性能指标手动验证并记录
3. 示例工程全部可运行

---

## 验收标准汇总

### 必须通过（P0）

| 类别 | 标准 |
|------|------|
| 单元测试 | `cargo test --workspace` 全部通过 |
| 代码质量 | `cargo clippy --workspace -- -D warnings` 通过 |
| 格式化 | `cargo fmt --check --workspace` 通过 |
| 文档 | `cargo doc --workspace --no-deps` 成功 |
| CI | Windows / macOS / Linux green |
| 示例 | pixel_platformer 可玩 |
| 示例 | joints 关节演示正常 |
| 示例 | scene_switch 多场景切换正常 |
| 示例 | signals 信号派发正常 |
| 示例 | tween 补间正常 |
| 示例 | timer 定时器正常 |
| 示例 | physics_perf 性能达标 |
| Debug | B/P/F 快捷键功能正常 |
| 文档 | CHANGELOG 0.4.0 |
| 文档 | README 5 章节齐全 |

### 建议通过（P1）

| 类别 | 标准 |
|------|------|
| 示例 | ball_pit 1000 球 30fps |
| 示例 | dominoes 多米诺正常 |
| 示例 | ray_cast 射线检测正常 |
| 示例 | scene_tree 节点演示正常 |
| 示例 | prefab_basic 实例化正常 |
| pixel_platformer | 3 关卡 + 敌人 + 金币 + HUD |

---

## 测试报告模板

```markdown
## Sprint 04 测试报告

### 执行时间
YYYY-MM-DD

### 测试环境
- OS: 
- Rust version: 
- Cargo version: 

### 测试结果

| 测试类型 | 通过 | 失败 | 总计 |
|----------|------|------|------|
| 单元测试 | X | Y | Z |
| 集成测试 | X | Y | Z |
| 示例测试 | X | Y | Z |

### 性能测试

| 场景 | 目标 FPS | 实际 FPS | 结果 |
|------|----------|----------|------|
| 100 球 | 60 | X | PASS/FAIL |
| 1000 球 | 30 | X | PASS/FAIL |

### CI 状态
- [ ] Windows
- [ ] macOS
- [ ] Linux

### 发现的问题
1. 
2. 

### 验收结论
[ ] 通过  [ ] 有条件通过  [ ] 不通过
```
