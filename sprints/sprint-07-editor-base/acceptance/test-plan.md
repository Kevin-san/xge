# Sprint 07 测试计划

## 概述

本文档定义 Sprint 07（可视化编辑器基础框架）的测试计划，涵盖单元测试、集成测试、验收测试。

## 测试环境

- Rust 1.70+
- `cargo test -p engine-editor`
- 三平台 CI：Linux / macOS / Windows

---

## 一、单元测试

### 1.1 EditorActionStack 测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| UT-001 | undo/redo 正确性 | push action 后 undo 能恢复到之前状态 |
| UT-002 | redo 正确性 | undo 后 redo 能恢复操作 |
| UT-003 | 批量操作 | BatchAction 可合并多个操作 |
| UT-004 | 栈容量限制 | 超过 50 步时最旧操作被丢弃 |
| UT-005 | clear 清空 | clear 后 can_undo/can_redo 返回 false |

**测试用例代码：**

```rust
#[test]
fn test_action_stack_undo_redo() {
    let mut stack = EditorActionStack::new(50);
    let action = Box::new(SetPropertyAction { /* ... */ });
    
    stack.push(action);
    assert!(stack.can_undo());
    assert!(!stack.can_redo());
}

#[test]
fn test_action_stack_max_len() {
    let mut stack = EditorActionStack::new(50);
    for i in 0..60 {
        let action = Box::new(SetPropertyAction { /* ... */ });
        stack.push(action);
    }
    assert!(stack.len() <= 50);
}
```

### 1.2 EditorSelection 测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| UT-011 | 增删操作 | select/add/remove 正确更新选择集 |
| UT-012 | toggle 行为 | toggle 已选中 entity 会移除 |
| UT-013 | contains 检查 | contains 返回正确结果 |
| UT-014 | 迭代器 | iter() 返回所有选中实体 |
| UT-015 | first/last | first() 和 last() 返回正确实体 |
| UT-016 | is_empty | 清空后 is_empty 返回 true |

**测试用例代码：**

```rust
#[test]
fn test_selection_toggle() {
    let mut selection = EditorSelection::new();
    let entity = 1u64;
    
    selection.select(entity);
    assert!(selection.contains(entity));
    
    selection.toggle(entity);
    assert!(!selection.contains(entity));
}
```

### 1.3 SceneSaver/SceneLoader 测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| UT-021 | JSON 往返 | save_json 后 load_json 数据一致 |
| UT-022 | BIN 往返 | save_bin 后 load_bin 数据一致 |
| UT-023 | 实体属性 | 往返后 Transform 属性正确 |
| UT-024 | 父子关系 | 往返后父子关系保持 |
| UT-025 | 组件数据 | 往返后组件数据完整 |

**测试用例代码：**

```rust
#[test]
fn test_scene_save_load_json() {
    let scene = create_test_scene();
    let path = Path::new("/tmp/test.scene.json");
    
    SceneSaver::save_json(&scene, path).unwrap();
    let loaded = SceneLoader::load_json(path).unwrap();
    
    assert_eq!(scene.entity_count(), loaded.entity_count());
}
```

### 1.4 Prefab 测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| UT-031 | Prefab 保存 | PrefabSaver::save_json 成功 |
| UT-032 | Prefab 加载 | PrefabLoader::load_json 成功 |
| UT-033 | 往返一致 | 往返后 Prefab 数据一致 |

### 1.5 Gizmo 测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| UT-041 | GizmoSystem 创建 | new() 成功 |
| UT-042 | 绘制方法 | draw_gizmo_circle/rect/arrow 不 panic |

### 1.6 AssetPipeline 测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| UT-051 | 扫描目录 | scan() 正确返回 AssetInfo 列表 |
| UT-052 | 导入流程 | import_all() 可执行（mock 文件） |

### 1.7 EditorAction 测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| UT-061 | CreateNodeAction | apply 创建实体，undo 删除 |
| UT-062 | DeleteNodeAction | apply 删除实体，undo 恢复 |
| UT-063 | SetPropertyAction | apply 设置属性，undo 恢复旧值 |
| UT-064 | MoveNodesAction | apply 移动节点，undo 恢复位置 |
| UT-065 | mergeable | 相同类型操作可合并 |

---

## 二、集成测试

### 2.1 编辑器启动测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| IT-001 | 正常启动 | EditorApp::run() 不 panic |
| IT-002 | 面板创建 | 5 个面板正确创建 |
| IT-003 | 菜单创建 | 7 个菜单正确创建 |

### 2.2 场景编辑测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| IT-011 | 创建节点 | EditorApp::new_scene() 创建成功 |
| IT-012 | 保存加载 | 场景保存后加载数据一致 |
| IT-013 | 选择操作 | select/toggle 操作正常 |
| IT-014 | 删除操作 | 删除节点后从场景移除 |

### 2.3 撤销重做测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| IT-021 | Undo 操作 | Ctrl+Z 触发 undo |
| IT-022 | Redo 操作 | Ctrl+Y 触发 redo |
| IT-023 | 多步撤销 | 连续多次 undo 正确回退 |
| IT-024 | 保存清空 | 场景保存后 undo 栈清空 |

### 2.4 面板交互测试

| 测试编号 | 测试内容 | 验证点 |
|---------|---------|--------|
| IT-031 | HierarchyPanel 选择 | 点击节点触发选择 |
| IT-032 | InspectorPanel 编辑 | 修改属性值生效 |
| IT-033 | AssetPanel 拖拽 | 拖拽资源到场景创建节点 |

---

## 三、验收测试

### 3.1 功能验收

| 验收编号 | 功能 | 验证标准 |
|---------|------|---------|
| AC-001 | 启动编辑器 | `cargo run --example editor_app` 可启动 |
| AC-002 | 新建节点 | HierarchyPanel 显示新节点 |
| AC-003 | 移动节点 | Transform 属性更新 |
| AC-004 | 保存场景 | 生成 `*.scene.json` 文件 |
| AC-005 | 加载场景 | 从文件恢复场景 |
| AC-006 | 撤销/重做 | 操作可撤销和重做 |
| AC-007 | 2D 节点选择 | SceneView 点击选中 |
| AC-008 | 2D 节点移动 | W/E/R 工具切换 |
| AC-009 | Play 模式 | 场景可运行 |
| AC-010 | 面板显示 | 5 个面板可用 |

### 3.2 代码质量验收

| 验收编号 | 验证标准 | 命令 |
|---------|---------|------|
| QC-001 | 单元测试通过 | `cargo test -p engine-editor` |
| QC-002 | Clippy 无警告 | `cargo clippy --workspace -- -D warnings` |
| QC-003 | Format 通过 | `cargo fmt --check --workspace` |
| QC-004 | Doc 生成成功 | `cargo doc --workspace --no-deps` |
| QC-005 | Unsafe 块 <= 5 | 代码审查 |

### 3.3 CI 验收

| 验收编号 | 平台 | 验证标准 |
|---------|------|---------|
| CI-001 | Linux | CI green |
| CI-002 | macOS | CI green |
| CI-003 | Windows | CI green |

### 3.4 文档验收

| 验收编号 | 内容 | 验证标准 |
|---------|------|---------|
| DC-001 | CHANGELOG | 记录版本 0.7.0 |
| DC-002 | README | 加入可视化编辑器章节 |
| DC-003 | README | 加入编辑器使用指南章节 |
| DC-004 | README | 加入插件开发指南章节 |
| DC-005 | Doc Comment | 公开 API 覆盖率 100% |

---

## 四、测试执行计划

### 4.1 本地测试

```bash
# 单元测试
cargo test -p engine-editor

# Clippy
cargo clippy --workspace -- -D warnings

# Format
cargo fmt --check --workspace

# Doc
cargo doc --workspace --no-deps

# 运行示例
cargo run --example editor_app
cargo run --example editor_custom_panel
cargo run --example editor_plugin
cargo run --example editor_game
```

### 4.2 CI 测试

所有测试在 GitHub Actions 三平台（Linux/macOS/Windows）执行。

---

## 五、测试覆盖率目标

| 模块 | 覆盖率目标 |
|------|-----------|
| EditorActionStack | 90%+ |
| EditorSelection | 90%+ |
| SceneSaver/Loader | 85%+ |
| PrefabSaver/Loader | 85%+ |
| GizmoSystem | 80%+ |
| AssetPipeline | 75%+ |

---

## 六、风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| UI 测试难以自动化 | 高 | 重点单元测试，确保核心逻辑正确 |
| 跨平台差异 | 中 | CI 三平台覆盖 |
| 性能测试不稳定 | 低 | 使用平均值，容忍小幅波动 |
