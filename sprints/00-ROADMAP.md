# Rust 通用游戏引擎 · Sprint 总路线图与需求索引

> 文档版本：V1.0 | 状态：规划中  
> 依据：`rust-game-engine-prd.html`（基于 Unity / Unreal / Cocos / Godot / GameMaker 五大引擎整合）  
> 定位：以 Rust 内存安全与零成本抽象为基石，打造高性能、跨平台、全场景通用游戏引擎

---

## 一、总体目标

将 PRD 中的五层解耦架构（应用层 / 逻辑层 / 核心引擎层 / 图形接口层 / 平台适配层）与双架构模式（节点树 + ECS）拆分为 **4 大阶段 × 16 个 Sprint**，每个 Sprint 聚焦同一层级的需求聚合点，以保证每次迭代均可独立交付可用产物，并形成可验证的验收标准。

- **阶段一（Sprint 1–4）**：基础内核 MVP — 可制作基础 2D 像素游戏
- **阶段二（Sprint 5–8）**：编辑器 + 跨平台 — 完整 2D 小游戏生产能力
- **阶段三（Sprint 9–12）**：3D 渲染管线升级 — 支持中小型 3D 游戏
- **阶段四（Sprint 13–16）**：高阶能力与生态 — 全功能通用引擎

---

## 二、Sprint 总览

| Sprint | 名称 | 交付目标 | 关键产物 |
| :--- | :--- | :--- | :--- |
| 01 | 核心架构骨架与模块抽象层 | Cargo workspace 建立、L3-L5 分层契约初版 | `engine-core` crate、平台抽象 trait、模块注册机制 |
| 02 | 事件循环 / 窗口 / 输入原语 | 单窗口事件调度、键鼠输入、DeltaTime | EventLoop、Window 抽象、InputDevice trait |
| 03 | 2D 渲染核心（精灵 / 纹理 / 批处理） | Vulkan/GL 后端初版、精灵批量化绘制、图集打包 | Renderer trait、Batch2D、TextureAtlas |
| 04 | 2D 物理 + 节点树 MVP | 轻量物理引擎（重力/碰撞）、场景节点树、Prefab 初版 | RigidBody2D、Collider2D、SceneTree |
| 05 | ECS 实体组件系统内核 | 实体 / 组件 / 资源 / 系统 / 查询批处理 | World、Entity、Component、System、Query |
| 06 | UI 控件库与布局引擎 | 控件库（按钮/文本/图片/滑动条/输入框）、锚点布局、事件冒泡 | Widget、LayoutEngine、UIEvent |
| 07 | 可视化编辑器基础框架 | 七面板布局、场景视图、属性面板、资源面板、控制台 | Editor App、SceneView、Inspector |
| 08 | 跨平台打包与资源管线 | Android/iOS/Web/H5 打包、资源压缩加密、增量差分 | BuildPipeline、AssetPipeline |
| 09 | 3D 渲染核心（网格 / 相机 / 光照） | 3D 网格加载、相机变换、方向光/点光/聚光灯初版 | Mesh3D、Camera3D、Light3D |
| 10 | PBR 材质与着色器系统 | PBR 金属/粗糙度工作流、六种贴图通道、ShaderGraph 雏形 | PBRMaterial、ShaderCompiler |
| 11 | 3D 物理引擎集成 | 刚体、关节、角色控制器、射线检测、碰撞层 | RigidBody3D、CharacterController3D |
| 12 | 动画系统 / 骨骼 / 状态机 | 帧动画、骨骼动画、状态机、混合树、动画事件 | AnimationClip、Animator、BlendTree |
| 13 | 粒子系统与后期特效 | 通用粒子管线（2D/3D）、后期栈（FXAA/DOF/Bloom） | ParticleSystem、PostProcessStack |
| 14 | 蓝图可视化逻辑 + 脚本虚拟机 | 节点连线编辑、Rust/JS/TS 双调用、脚本 VM 沙盒 | BlueprintGraph、ScriptVM |
| 15 | 网络 / 热更新 / 插件系统 | TCP/UDP/WebSocket、差分热更、插件沙盒 API | NetChannel、HotfixManager、PluginHost |
| 16 | 资源商店 / 生态 / 性能调优 | Asset Store、工程模板、性能面板、文档与教程 | AssetStore Client、Profiler、TemplateManager |

---

## 三、需求拆分原则

1. **同一层级聚合**：每个 Sprint 仅围绕一个引擎层级或紧密相关的一组模块展开，确保技术栈统一、依赖可控。
2. **最小可交付**：每个 Sprint 至少交付一个可独立运行的 Demo，形成可验证闭环。
3. **先契约后实现**：每个 Sprint 以 trait/接口定义起始，再提供默认实现，允许后续替换（L4 图形 API 可切换、L5 平台可替换）。
4. **测试驱动**：每个 Sprint 产出物附带单元测试 / 集成测试 / 基准测试，保证回归能力。
5. **文档伴随**：每个功能需求点同步输出 API 文档、使用示例、设计备忘录。

---

## 四、文档目录

- [Sprint 01 · 核心架构骨架与模块抽象层](./sprint-01-core-arch.md)
- [Sprint 02 · 事件循环 / 窗口 / 输入原语](./sprint-02-event-loop.md)
- [Sprint 03 · 2D 渲染核心（精灵 / 纹理 / 批处理）](./sprint-03-render-2d-core.md)
- [Sprint 04 · 2D 物理 + 节点树 MVP](./sprint-04-physics-2d.md)
- [Sprint 05 · ECS 实体组件系统内核](./sprint-05-ecs-core.md)
- [Sprint 06 · UI 控件库与布局引擎](./sprint-06-ui-system.md)
- [Sprint 07 · 可视化编辑器基础框架](./sprint-07-editor-base.md)
- [Sprint 08 · 跨平台打包与资源管线](./sprint-08-cross-platform.md)
- [Sprint 09 · 3D 渲染核心（网格 / 相机 / 光照）](./sprint-09-render-3d-core.md)
- [Sprint 10 · PBR 材质与着色器系统](./sprint-10-pbr-material.md)
- [Sprint 11 · 3D 物理引擎集成](./sprint-11-physics-3d.md)
- [Sprint 12 · 动画系统 / 骨骼 / 状态机](./sprint-12-animation.md)
- [Sprint 13 · 粒子系统与后期特效](./sprint-13-particles-postfx.md)
- [Sprint 14 · 蓝图可视化逻辑 + 脚本虚拟机](./sprint-14-blueprint-script.md)
- [Sprint 15 · 网络 / 热更新 / 插件系统](./sprint-15-network-hotfix.md)
- [Sprint 16 · 资源商店 / 生态 / 性能调优](./sprint-16-ecosystem.md)

---

## 五、关键里程碑（Milestones）

| 里程碑 | 触发 Sprint | 验收演示 |
| :--- | :--- | :--- |
| M1 · 内核跑通 Hello World | Sprint 02 | 能在 Windows 上弹出窗口并显示三角/精灵 |
| M2 · 2D 游戏可玩 | Sprint 04 | 横版像素 Demo：角色移动 + 碰撞 + 简单关卡 |
| M3 · ECS 架构可用 | Sprint 05 | 万级粒子 Demo，CPU 批处理性能达标 |
| M4 · UI 布局完整 | Sprint 06 | 主菜单 + 设置面板 + HUD Demo |
| M5 · 编辑器可编辑场景 | Sprint 07 | 拖拽创建节点、修改属性、保存/加载场景 |
| M6 · 可出安卓包 | Sprint 08 | 一键打包到 Android，真机正常运行 |
| M7 · 3D 场景可行 | Sprint 09 | 加载 GLTF 模型 + 基础光照 + 相机漫游 |
| M8 · PBR 材质合格 | Sprint 10 | 典型材质球 Demo：金属/非金属对比 |
| M9 · 3D 游戏可玩 | Sprint 11 | 第三人称角色控制 + 物理交互 |
| M10 · 动画合格 | Sprint 12 | 角色待机/走/跑/跳混合切换 |
| M11 · 视觉特效可用 | Sprint 13 | 火焰、爆炸、雾效 Demo |
| M12 · 零代码可用 | Sprint 14 | 蓝图连线实现「敌人追踪 + 攻击」逻辑 |
| M13 · 联网可用 | Sprint 15 | 双人联机 Demo，热更新资源 |
| M14 · 生态可用 | Sprint 16 | 资源商店下载、模板一键创建、Profiler 分析 |

---

## 六、需求数量级承诺

- 本系列文档中，**每个 Sprint 至少包含 100 条项目需求**（Functional + Non-Functional + Technical）；
- **每个项目需求至少下钻 100 条细分需求 / 验收子项 / API 子条目**（以清单形式呈现，便于任务拆分与工时估算）；
- 所有需求遵循「Rust 原生、零 GC、模块化插拔、可跨平台编译」的硬性约束。
