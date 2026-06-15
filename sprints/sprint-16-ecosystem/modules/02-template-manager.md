# 模板管理器模块（engine-template）

## 模块概述

模板管理器模块提供游戏工程模板的创建、管理与分发功能，支持多种游戏类型（2D/3D/VR/AR/FPS/RPG等），实现模板市场与自定义模板导出，构建完善的工程模板生态。

**Crate**: `engine-template`
**周期**: 4 周
**优先级**: P0

---

## 需求清单

### 1. 核心管理器（需求 2, 72-98, 401-458）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 2 | 建立 `engine-template` crate | P0 |
| 72 | `TemplateManager::new() -> Self` | P0 |
| 73 | `TemplateManager::list_templates(&self) -> Vec<Template>` | P0 |
| 74 | `TemplateManager::list_templates_by_category(&self, cat) -> Vec<Template>` | P0 |
| 75 | `TemplateManager::get_template(&self, id) -> Option<&Template>` | P0 |
| 76 | `TemplateManager::create_project(&self, template_id, output_dir, project_name) -> Result<Project>` | P0 |
| 99 | `TemplateManager::register_template(template) -> TemplateId` | P1 |
| 100 | `TemplateManager::unregister_template(id) -> Result<()>` | P1 |
| 101 | `TemplateManager::template_count(&self) -> usize` | P1 |
| 102 | `TemplateManager::reload() -> Result<()>` | P1 |
| 113 | `TemplateManager::filter(filter) -> Vec<Template>` | P1 |
| 114 | `TemplateManager::search(keyword) -> Vec<Template>` | P1 |
| 115 | `TemplateManager::featured() -> Vec<Template>` | P1 |
| 116 | `TemplateManager::recent() -> Vec<Template>` | P1 |

#### API 签名详情

```rust
pub struct TemplateManager;

impl TemplateManager {
    pub fn new() -> Self
    pub fn list_templates(&self) -> Vec<Template>
    pub fn list_templates_by_category(&self, cat: TemplateType) -> Vec<Template>
    pub fn get_template(&self, id: &TemplateId) -> Option<&Template>
    pub fn create_project(
        &self,
        template_id: &TemplateId,
        output_dir: &Path,
        project_name: &str,
    ) -> Result<Project>
    pub fn create_project_with_options(
        &self,
        id: &TemplateId,
        options: CreateProjectOptions,
    ) -> Result<Project>
    pub fn register_template(&mut self, template: Template) -> TemplateId
    pub fn unregister_template(&self, id: &TemplateId) -> Result<()>
    pub fn template_count(&self) -> usize
    pub fn reload(&mut self) -> Result<()>
    pub fn filter(&self, filter: TemplateFilter) -> Vec<Template>
    pub fn search(&self, keyword: &str) -> Vec<Template>
    pub fn featured(&self) -> Vec<Template>
    pub fn recent(&self) -> Vec<Template>
}
```

---

### 2. 模板类型（需求 77-79, 104-108）

```rust
pub enum TemplateType {
    Template2D,
    Template3D,
    TemplateVR,
    TemplateAR,
    TemplateEmpty,
    TemplateTutorial,
}

pub enum TemplateGameType {
    FPS,
    TPS,
    RPG,
    RTS,
    MOBA,
    Racing,
    Platformer,
    Puzzle,
    Card,
    Roguelike,
    VisualNovel,
    TowerDefense,
}

pub struct Template {
    pub id: TemplateId,
    pub name: String,
    pub description: String,
    pub category: TemplateType,
    pub game_type: TemplateGameType,
    pub thumbnail: Option<PathBuf>,
    pub engine_version: String,
    pub files: Vec<TemplateFile>,
}

impl Template {
    pub fn version(&self) -> &str
    pub fn engine_version_required(&self) -> &str
    pub fn is_compatible(&self, engine_version: &str) -> bool
    pub fn files_count(&self) -> usize
    pub fn thumbnail_path(&self) -> Option<&Path>
    pub fn readme_content(&self) -> Option<&str>
    pub fn tags(&self) -> &[String]
    pub fn from_zip(path: &Path) -> Result<Self>
    pub fn to_zip(&self, output_path: &Path) -> Result<()>
    pub fn save_zip(&self, path: &Path) -> Result<()>
    pub fn load_zip(path: &Path) -> Result<Self>
    pub fn validate(&self) -> Result<()>
    pub fn validate_required_files(&self) -> Result<()>
    pub fn validate_engine_version(&self) -> Result<()>
    pub fn validate_manifest(&self) -> Result<()>
}
```

---

### 3. 模板标识符（需求 103, 405-406）

```rust
pub struct TemplateId(uuid::Uuid);

impl TemplateId {
    pub fn new(uuid: Uuid) -> Self
    pub fn parse(s: &str) -> Result<Self>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct uuid::Uuid;
```

---

### 4. 项目结构（需求 82-84, 109-112, 418-432）

```rust
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub cargo_toml_path: PathBuf,
    pub main_scene_path: PathBuf,
}

impl Project {
    pub fn open(path: &Path) -> Result<Self>
    pub fn name(&self) -> &str
    pub fn path(&self) -> &Path
    pub fn cargo_toml(&self) -> &Path
    pub fn main_scene(&self) -> &Path
    pub fn exists(&self) -> bool
    pub fn is_initialized(&self) -> bool
    pub fn build(&self) -> Result<Output>
    pub fn run(&self) -> Result<Output>
    pub fn test(&self) -> Result<Output>
    pub fn run_cargo(&self, args: &[String]) -> Result<Output>
    pub fn read_cargo_toml(&self) -> Result<CargoToml>
}

pub struct CargoToml {
    pub package_name: String,
    pub version: String,
    pub edition: String,
    pub authors: Vec<String>,
    pub description: String,
    pub dependencies: Vec<CargoTomlDependency>,
}

pub struct CargoTomlDependency {
    pub name: String,
    pub version: String,
    pub path: Option<String>,
    pub git: Option<String>,
    pub features: Vec<String>,
}
```

---

### 5. 模板内容（需求 85, 112, 472-474）

```rust
pub enum TemplateContent {
    CargoToml,
    MainScene,
    MainScript,
    README,
    EngineConfig,
    Gitignore,
}

pub struct TemplateFile {
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub is_binary: bool,
    pub content_hash: String,
}

impl TemplateContent {
    pub fn files(&self) -> Vec<TemplateFile>
}
```

---

### 6. 模板过滤器（需求 113, 413）

```rust
pub struct TemplateFilter {
    pub category: Option<TemplateType>,
    pub game_type: Option<TemplateGameType>,
    pub engine_version: Option<String>,
    pub tags: Vec<String>,
}
```

---

### 7. 项目创建选项（需求 117, 458-459）

```rust
pub struct CreateProjectOptions {
    pub project_name: String,
    pub output_dir: PathBuf,
    pub overwrite: bool,
    pub init_git: bool,
    pub run_cargo_check: bool,
}
```

---

### 8. 模板构建器（需求 118, 475-482）

```rust
pub struct TemplateBuilder {
    pub name: String,
}

impl TemplateBuilder {
    pub fn new(name: &str) -> Self
    pub fn category(&mut self, cat: TemplateType) -> &mut Self
    pub fn game_type(&mut self, gt: TemplateGameType) -> &mut Self
    pub fn description(&mut self, s: &str) -> &mut Self
    pub fn add_file(&mut self, source: &Path, target: &Path) -> &mut Self
    pub fn add_directory(&mut self, dir: &Path) -> &mut Self
    pub fn thumbnail(&mut self, path: &Path) -> &mut Self
    pub fn build(&self) -> Result<Template>
}
```

---

### 9. 模板缓存（需求 119, 485-488）

```rust
pub struct TemplateCache;

impl TemplateCache {
    pub fn get(&self, id: &TemplateId) -> Option<&Template>
    pub fn insert(&mut self, template: Template) -> ()
    pub fn invalidate(&mut self, id: &TemplateId) -> ()
    pub fn clear(&mut self) -> ()
}
```

---

### 10. 项目初始化（需求 121, 449-452）

```rust
pub struct ProjectInitializer;

impl ProjectInitializer {
    pub fn init_git_repo(project: &Project) -> Result<()>
    pub fn write_default_config(project: &Project) -> Result<()>
    pub fn write_readme(project: &Project) -> Result<()>
    pub fn write_gitignore(project: &Project) -> Result<()>
}
```

---

### 11. 模板变量（需求 120, 456-459）

```rust
pub enum TemplateVariable {
    PROJECT_NAME,
    AUTHOR,
    ENGINE_VERSION,
    CREATION_DATE,
}

impl TemplateVariable {
    pub fn replace_in(content: &str, context: &TemplateVariableContext) -> String
}

pub struct TemplateVariableContext {
    pub project_name: String,
    pub author: String,
    pub engine_version: String,
    pub date: String,
}
```

---

### 12. 模板显示信息（需求 118, 460-465）

```rust
pub struct TemplateTypeDisplay;

impl TemplateTypeDisplay {
    pub fn label(&self) -> &str
    pub fn icon(&self) -> &str
    pub fn description(&self) -> &str
}

pub struct TemplateGameTypeDisplay;

impl TemplateGameTypeDisplay {
    pub fn label(&self) -> &str
    pub fn icon(&self) -> &str
}
```

---

### 13. 内置模板（需求 121, 464-481）

```rust
pub struct BuiltInTemplates;

impl BuiltInTemplates {
    pub fn all() -> Vec<Template>
    pub fn empty_2d() -> Template
    pub fn empty_3d() -> Template
    pub fn empty_vr() -> Template
    pub fn empty_ar() -> Template
    pub fn fps() -> Template
    pub fn tps() -> Template
    pub fn rpg() -> Template
    pub fn racing() -> Template
    pub fn platformer_2d() -> Template
    pub fn puzzle() -> Template
    pub fn card_game() -> Template
    pub fn roguelike() -> Template
    pub fn visual_novel() -> Template
    pub fn tower_defense() -> Template
    pub fn tutorial_first_project() -> Template
}
```

---

### 14. 模板市场（需求 86-89, 113-116）

```rust
pub struct TemplateMarketplace;

impl TemplateMarketplace {
    pub fn publish(template: Template) -> Result<TemplateId>
    pub fn install(template_id: &TemplateId) -> Result<Template>
    pub fn uninstall(template_id: &TemplateId) -> Result<()>
    pub fn search(keyword: &str) -> Vec<Template>
}
```

---

## 验收标准

### 功能验收

| ID | 验收条件 | 测试方式 |
|----|----------|----------|
| AC-1 | `TemplateManager::new()` 成功创建实例 | 单元测试 |
| AC-2 | `list_templates()` 返回所有可用模板 | 单元测试 |
| AC-3 | `create_project()` 正确生成项目结构 | 集成测试 |
| AC-4 | `Template::from_zip()` 正确加载模板 | 单元测试 |
| AC-5 | `Template::to_zip()` 正确保存模板 | 单元测试 |
| AC-6 | 模板变量替换正确执行 | 单元测试 |
| AC-7 | 所有内置模板可正常使用 | 集成测试 |

### 示例验收

| 示例 | 验收条件 |
|------|----------|
| `template_new` | 从 BuiltInTemplates::fps() 创建项目并编译运行 |
| `template_new` | 验证 Cargo.toml 正确生成 |
| `template_new` | 验证主场景文件存在 |
| `template_custom` | 构造自定义模板 → 保存 zip → 加载 zip → 从 zip 创建项目 |
| `template_custom` | 验证模板变量替换正确 |

---

## 依赖关系

### 内部依赖

- `engine-core`: 基础类型定义
- `engine-asset`: 资源系统

### 外部依赖

- `uuid`: 模板 ID 生成
- `walkdir`: 目录遍历
- `serde`: 序列化

---

## 优先级说明

- **P0**: 核心功能，MVP 必须包含
- **P1**: 重要功能，下一迭代包含
- **P2**: 增强功能，后续迭代包含
