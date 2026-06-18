# 文档与社区模块（engine-docs）

## 模块概述

文档与社区模块建立完善的文档系统与社区生态，包括自动 API 文档、教程文档、最佳实践指南，以及 Discord/Matrix/微信/QQ/知乎/B站/Reddit 等多渠道社区。

**Crate**: `engine-docs`
**周期**: 4 周
**优先级**: P0

---

## 需求清单

### 1. 文档生成（需求 4, 151-233, 638-776）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 4 | `engine-docs` crate / 工作目录建立 | P0 |
| 151 | `engine-docs` 自动 API 文档（基于 cargo doc） | P0 |
| 152 | `engine-docs` 主题（自定义 CSS / 侧边栏 / 搜索） | P0 |
| 153-158 | docs/src/ 目录结构建立 | P0 |
| 198 | 文档版本切换（v0.9 / v1.0 / v1.1 / latest） | P1 |
| 199 | 离线文档（docs.tar.gz / PDF 导出） | P1 |
| 200 | docs 站点使用 mdbook 构建 | P0 |
| 201 | docs 站点备用 zola 构建 | P2 |
| 220 | 最佳实践指南（Best Practices） | P1 |
| 221 | 性能调优指南（Performance Tuning Guide） | P1 |
| 222 | 安全指南（Safety Guide） | P1 |
| 223 | 迁移指南（Migration Guide）——从旧版本迁移 | P1 |

#### 目录结构

```
docs/
├── book.toml           # mdbook 配置
├── src/
│   ├── SUMMARY.md      # 目录
│   ├── introduction.md # 引言
│   ├── getting_started/
│   │   ├── installation.md
│   │   ├── quick_start.md
│   │   └── first_project.md
│   ├── concepts/
│   │   ├── ecs_overview.md
│   │   ├── entities_components_systems.md
│   │   ├── asset_system.md
│   │   ├── scene_graph.md
│   │   ├── rendering_pipeline.md
│   │   ├── physics_engine.md
│   │   └── event_system.md
│   ├── guides/
│   │   ├── rendering_2d.md
│   │   ├── rendering_3d.md
│   │   ├── physics.md
│   │   ├── ui_guide.md
│   │   ├── audio_guide.md
│   │   ├── animation_guide.md
│   │   ├── networking_guide.md
│   │   ├── scripting_guide.md
│   │   └── deployment_guide.md
│   ├── examples/
│   │   ├── hello_world.md
│   │   ├── platformer_2d.md
│   │   ├── mini_3d.md
│   │   ├── ui_demo.md
│   │   ├── physics_demo.md
│   │   ├── animation_demo.md
│   │   ├── particles_demo.md
│   │   ├── network_demo.md
│   │   └── blueprint_demo.md
│   ├── api_reference/
│   │   ├── index.md
│   │   ├── engine_asset_store.md
│   │   ├── engine_template.md
│   │   ├── engine_profiler.md
│   │   └── core_modules.md
│   ├── faq/
│   │   ├── general.md
│   │   ├── build_issues.md
│   │   └── performance.md
│   ├── best_practices.md
│   ├── performance_tuning.md
│   ├── safety_guide.md
│   ├── migration_guide.md
│   ├── migration/
│   │   ├── from_0_9_to_1_0.md
│   │   └── from_0_8_to_0_9.md
│   ├── CHANGELOG.md
│   ├── ROADMAP.md
│   ├── CONTRIBUTING.md
│   ├── CODE_OF_CONDUCT.md
│   └── SECURITY.md
├── theme/
│   ├── index.hbs
│   ├── css/
│   │   ├── general.css
│   │   ├── chrome.css
│   │   └── variables.css
│   ├── favicon.svg
│   └── logo.svg
├── zh/                  # 中文文档
│   └── src/
│       └── ...
├── en/                  # 英文文档
│   └── src/
│       └── ...
├── zola.toml            # 备用 zola 配置
├── templates/           # zola 模板
├── static/              # 静态资源
└── scripts/
    ├── build.sh
    ├── watch.sh
    ├── serve.sh
    ├── export_pdf.sh
    └── export_tarball.sh
```

---

### 2. mdbook 配置（需求 638-640, 679-682）

```toml
# docs/book.toml
[book]
title = "Engine Documentation"
author = "Engine Team"
description = "Official documentation for Engine game engine"
language = "en"

[build]
build-dir = "book"
create-missing = false

[output.html]
theme = "theme"
default-hide-levels = [2, 3]
no-section-label = false
git-repository-url = "https://github.com/engine/engine"
edit-url-template = "https://github.com/engine/engine/edit/main/docs/src/{path}"

[output.html.search]
enable = true
limit-results = 30
use-typeahead = true
prebuilt-search-langs = ["en", "zh"]

[output.html.navigation-replacements]
# 语言选择
additional-js = ["language_selector.js"]
additional-css = ["css/language.css"]

[output.html.fold]
enable = true
level = 1

[preprocessor.search]
renderers = ["html", "json"]

[preprocessor.links]
endpoints = ["https://api.example.com/docs"]
```

---

### 3. 中英文双语（需求 222, 686-700）

```markdown
# docs/zh/src/SUMMARY.md
# 中文文档目录

- [简介](introduction.md)
- [入门](getting_started/)
  - [安装指南](getting_started/installation.md)
  - [快速开始](getting_started/quick_start.md)
  - [第一个项目](getting_started/first_project.md)
- [概念](concepts/)
  - [ECS 概览](concepts/ecs_overview.md)
- [指南](guides/)
  - [2D 渲染](guides/rendering_2d.md)
  - [部署指南](guides/deployment_guide.md)
```

---

### 4. 文档工具脚本（需求 713-720）

```bash
#!/bin/bash
# docs/build.sh

set -e

echo "Building documentation..."
cd "$(dirname "$0")/.."

# mdbook build
cargo install mdbook --quiet
mdbook build docs/

# 生成搜索索引
mdbook search build docs/

echo "Documentation built successfully!"
```

---

### 5. 文档覆盖率要求（需求 765-771）

| 需求ID | 描述 | 目标 |
|--------|------|------|
| 765 | `cargo doc --workspace --no-deps --document-private-items` 成功 | 100% |
| 766 | `cargo rustdoc -- --html-in-header custom.html` 自定义头部 | 成功 |
| 768 | 所有公开函数文档注释覆盖率 | >= 95% |
| 769 | 所有公开结构体文档注释覆盖率 | 100% |
| 770 | 所有公开枚举文档注释覆盖率 | 100% |
| 771 | 文档示例代码块均通过 `cargo test --doc` | 100% |

---

### 6. 教程视频生态（需求 168-218, 731-808）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 168 | 教学视频脚本大纲至少 20 讲 | P0 |
| 169-190 | 教学视频第 01-22 讲脚本 | P0 |
| 754-756 | 每讲脚本包含：title/duration/prerequisites/outline/key_points/commands/demo_steps/summary/homework | P0 |
| 757 | 每讲时长约 20-40 分钟 | P1 |
| 758 | `videos/README.md` 视频总索引 | P1 |
| 759-760 | 中英文字幕脚本目录 | P1 |

#### 视频大纲结构

```markdown
# videos/
├── outline.md                    # 视频大纲文档
├── episode_01_setup.md           # 第 01 讲：引擎介绍与环境搭建
├── episode_02_hello_world.md    # 第 02 讲：创建项目 + 运行空窗口
├── episode_03_ecs_concepts.md    # 第 03 讲：ECS 介绍
├── episode_04_ecs_api.md         # 第 04 讲：Entity/Component/System API
├── episode_05_assets.md          # 第 05 讲：资源系统
├── episode_06_render_2d.md       # 第 06 讲：2D 渲染
├── episode_07_physics_2d.md      # 第 07 讲：2D 物理
├── episode_08_render_3d.md       # 第 08 讲：3D 渲染
├── episode_09_pbr.md             # 第 09 讲：PBR 材质
├── episode_10_physics_3d.md      # 第 10 讲：3D 物理
├── episode_11_ui.md             # 第 11 讲：UI 系统
├── episode_12_input.md          # 第 12 讲：输入系统
├── episode_13_animation.md      # 第 13 讲：动画系统
├── episode_14_state_machine.md   # 第 14 讲：状态机
├── episode_15_particles.md       # 第 15 讲：粒子系统
├── episode_16_audio.md           # 第 16 讲：音频系统
├── episode_17_scripting.md       # 第 17 讲：脚本与蓝图
├── episode_18_networking.md      # 第 18 讲：多人联网
├── episode_19_profiler.md       # 第 19 讲：Profiler 使用
├── episode_20_deploy.md          # 第 20 讲：打包发布
├── episode_21_asset_store.md    # 第 21 讲：资源商店
├── episode_22_extension.md      # 第 22 讲：编辑器扩展
├── assets/
│   ├── slides/                   # 幻灯片素材
│   └── code/                     # 示例代码
├── zh/                           # 中文字幕
└── en/                           # 英文字幕
```

#### 单讲脚本格式

```markdown
# 第 01 讲：引擎介绍与环境搭建

## 基本信息
- **标题**：引擎介绍与环境搭建
- **时长**：30 分钟
- **先决条件**：无
- **难度**：入门

## 大纲
1. 引擎简介（5 分钟）
2. 系统要求（3 分钟）
3. 安装步骤（10 分钟）
4. IDE 配置（7 分钟）
5. 创建第一个项目（5 分钟）

## 关键要点
- 引擎架构概述
- 跨平台支持
- 最低系统要求

## 命令
```bash
cargo new my_project
cd my_project
cargo run
```

## 演示步骤
1. 下载安装包
2. 运行安装程序
3. 配置环境变量
4. 验证安装

## 总结
- 引擎安装完成
- 下一步：创建 Hello World 项目

## 课后作业
- 安装引擎
- 运行 examples/hello_world
```

---

### 7. 示例项目（需求 159-167, 809-826）

| 示例 | 描述 | 优先级 |
|------|------|--------|
| `hello_world` | 引擎初始化 + 空窗口 | P0 |
| `2d_platformer` | 2D 平台跳跃（玩家 + 平台 + 跳跃 + 敌人 + 瓦片地图 + 分数 HUD） | P0 |
| `3d_mini` | 3D 小场景（PBR 材质立方体 + 方向光 + 摄像机轨道控制 + 天空盒 + 阴影） | P0 |
| `ui_demo` | UI 组件演示（按钮/文本框/滑块/下拉框/弹窗 + 布局） | P0 |
| `physics_demo` | 物理碰撞演示（碰撞物体 + 交互投放） | P0 |
| `animation_demo` | 动画系统演示（行走循环 + IK 瞄准） | P0 |
| `particles_demo` | 粒子系统演示（火焰/烟雾/爆炸粒子效果） | P0 |
| `network_demo` | 多人联网演示（客户端/服务器双向通信 + 玩家位置同步） | P0 |
| `blueprint_demo` | 蓝图/可视化脚本演示（蓝图节点图 + 执行） | P0 |

---

### 8. 社区工具（需求 232-278, 854-900）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 232 | Discord 社区服务器 | P0 |
| 233 | Matrix 社区频道 | P1 |
| 234 | 微信社区群 | P1 |
| 235 | QQ 社区群 | P1 |
| 236 | 知乎专栏 | P1 |
| 237 | B 站官方账号 | P0 |
| 238 | Reddit 社区 | P1 |
| 239 | 邮件列表 | P1 |
| 240 | 官方博客（blog.engine.example.com） | P0 |
| 241 | 更新日志发布（每 Sprint 一次） | P0 |
| 242 | 路线图公开（每季更新） | P1 |

#### Discord 频道结构

```
engine/
├── #announcements     # 公告
├── #general           # 通用讨论
├── #help              # 帮助求助
├── #development       # 开发讨论
├── #showcase          # 作品展示
├── #resources         # 资源共享
├── #jobs              # 招聘求职
├── zh/                # 中文频道
│   ├── #general-zh
│   └── #help-zh
└── dev/               # 开发者频道
    ├── #core
    ├── #render
    ├── #physics
    └── #ai
```

---

### 9. GitHub Issue/PR 模板（需求 256-260, 854-858）

```markdown
# .github/ISSUE_TEMPLATE/bug_report.md
---
name: Bug Report
about: 报告一个 bug
title: "[Bug] "
labels: bug
assignees: ''
---

## 描述
简要描述问题

## 重现步骤
1. Go to '...'
2. Run '....'
3. See error

## 预期行为
描述预期行为

## 实际行为
描述实际行为

## 环境
- OS: [e.g. Windows 11]
- Engine Version: [e.g. 1.0.0]
- Rust Version: [e.g. 1.70.0]

## 日志
```
日志内容
```

## 截图
如果有截图
```

```markdown
# .github/ISSUE_TEMPLATE/feature_request.md
---
name: Feature Request
about: 请求新功能
title: "[Feature] "
labels: enhancement
assignees: ''
---

## 功能描述
清晰描述所需功能

## 使用场景
描述使用场景

## 建议的解决方案
描述建议的解决方案

## 替代方案
描述替代方案

## 其他
其他信息
```

```markdown
# .github/PULL_REQUEST_TEMPLATE.md
## 描述
<!-- 简要描述 PR -->

## 动机
<!-- 解释为什么需要这个 PR -->

## 变更
<!-- 详细描述变更内容 -->

## 测试
<!-- 描述测试方式 -->

## Checklist
- [ ] `cargo test --workspace` 通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --all` 通过
- [ ] `cargo doc --workspace --no-deps` 通过
- [ ] examples 构建成功

## 截图
如有 UI 变更
```

---

### 10. 自动标签与机器人（需求 261-265, 861-865）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 261 | Label bot：根据路径自动打标签 | P1 |
| 262 | Label bot：根据 PR size 打标签 | P1 |
| 263 | Label bot：根据 title 前缀打标签 | P1 |
| 264 | Stale bot：60 天无活动 issue → stale，再 7 天关闭 | P1 |
| 265 | Stale bot：PR 45 天无活动 → stale-pr | P1 |
| 266 | Dependabot：每周检查依赖更新 | P1 |

#### 标签规则

| 标签前缀 | 含义 | 示例 |
|----------|------|------|
| A- | 领域 | A-asset-store, A-template, A-profiler, A-docs |
| C- | 类型 | C-bug, C-feature, C-refactor, C-perf |
| S- | 大小 | S-small, S-medium, S-large |
| bug/feat/docs/ci | 类型 | bug, feat, docs, ci, refactor, perf |

---

### 11. 安全披露（需求 273-278, 885-898）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 273 | 安全披露政策（Security Disclosure Policy） | P0 |
| 274 | CVE 追踪（security@engine.example.com + GitHub Security Advisory） | P0 |

#### SECURITY.md

```markdown
# 安全披露政策

## 报告漏洞

如果您发现安全漏洞，请发送邮件至 security@engine.example.com

我们承诺：
- 48 小时内确认收到报告
- 90 天内提供修复计划
- 公开前进行 embargo

## 支持的版本

| 版本 | 支持状态 |
|------|----------|
| 1.0.x | 当前 stable |
| 0.9.x | 安全补丁 |
| < 0.9 | 不支持 |

## 安全更新

安全更新将发布到 GitHub Security Advisories
```

---

### 12. 品牌资产（需求 276-280, 947-960）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 276 | 官方 Logo（矢量 SVG） | P1 |
| 277 | 视觉识别系统（色板/字体/logo 使用规范） | P1 |
| 278 | 品牌指南（Brand Guide） | P1 |

#### Logo 文件

```
assets/
├── logo/
│   ├── engine-logo.svg           # 主 Logo
│   ├── engine-logo-black.svg     # 黑底 Logo
│   ├── engine-logo-white.svg     # 白底 Logo
│   ├── favicon.svg               # Favicon
│   ├── engine-logo-128.png       # 128px
│   ├── engine-logo-256.png       # 256px
│   └── engine-logo-512.png       # 512px
├── brand/
│   ├── palette.md                # 色板规范
│   ├── typography.md              # 字体规范
│   ├── guide.md                   # 品牌使用指南
│   └── social-preview.png        # 社交分享预览
```

---

### 13. 迁移工具（需求 246-248, 273, 881-884）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 246 | 迁移工具（cargo rge-migrate） | P1 |
| 273 | 弃用周期：deprecate → 2 个 minor 版本后移除 | P0 |
| 881 | `cargo rge-migrate --from 0.9 --to 1.0` | P1 |
| 882 | `cargo rge-migrate --dry-run` | P1 |

#### 迁移命令

```bash
# 迁移项目
cargo rge-migrate --from 0.9 --to 1.0 --path ./my_project

# 预览迁移
cargo rge-migrate --from 0.9 --to 1.0 --dry-run --path ./my_project
```

---

### 14. CHANGELOG 与路线图（需求 228-231, 705-714）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 228 | `CHANGELOG.md` 按 Keep a Changelog 格式 | P0 |
| 229 | `ROADMAP.md` 路线图文档 | P0 |
| 230 | `CONTRIBUTING.md` 贡献指南 | P0 |
| 231 | `CODE_OF_CONDUCT.md` 行为准则 | P0 |
| 756 | `docs/src/CHANGELOG.md` 变更日志 | P0 |
| 757 | `docs/src/ROADMAP.md` 路线图 | P0 |
| 758 | `docs/src/CONTRIBUTING.md` 贡献指南 | P0 |
| 759 | `docs/src/CODE_OF_CONDUCT.md` 行为准则 | P0 |
| 760 | `docs/src/SECURITY.md` 安全披露政策 | P0 |

#### CHANGELOG 格式

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [1.0.0] - 2024-01-01

### Added
- Feature A
- Feature B

### Changed
- Changed behavior of X

### Deprecated
- Deprecated Y

### Removed
- Removed Z

### Fixed
- Fixed bug in W

### Security
- Security improvement V
```

---

## 验收标准

### 文档验收

| ID | 验收条件 | 测试方式 |
|----|----------|----------|
| AC-1 | mdbook 构建成功 | 本地构建 |
| AC-2 | `cargo doc --workspace` 生成文档 | CI 执行 |
| AC-3 | 中英文文档完整 | 人工审查 |
| AC-4 | 文档示例代码可运行 | `cargo test --doc` |
| AC-5 | 文档版本切换正常 | 人工测试 |

### 社区验收

| 平台 | 验收条件 |
|------|----------|
| Discord | 服务器可访问，频道正常 |
| 知乎 | 专栏可发布文章 |
| B 站 | 视频可上传 |
| GitHub | Issue/PR 模板正常工作 |

---

## 依赖关系

### 内部依赖

- `engine-core`: 核心库文档
- `engine-asset-store`: 资源商店文档
- `engine-template`: 模板管理文档
- `engine-profiler`: 性能分析文档

### 外部依赖

- mdbook
- zola（备用）
- wkhtmltopdf（PDF 导出）
- tantivy/lunr（搜索）

---

## 优先级说明

- **P0**: 核心功能，MVP 必须包含
- **P1**: 重要功能，下一迭代包含
- **P2**: 增强功能，后续迭代包含
