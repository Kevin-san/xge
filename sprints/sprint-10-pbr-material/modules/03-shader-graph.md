# 模块三：ShaderGraph 可视化编辑器需求

## 3.1 模块概述

ShaderGraph 是节点式着色器编辑器，允许用户通过可视化方式构建复杂着色器逻辑。编辑器提供内置 PBR 主节点、常用 math/tex/uv 节点，支持撤销/重做、实时预览、JSON 序列化导出。

**对应原需求编号**：46-58, 281-365

---

## 3.2 核心类型定义

### 3.2.1 NodeId 与 EdgeId

```rust
pub struct NodeId(u32);

pub struct EdgeId(u32);
```

### 3.2.2 NodeKind 枚举

```rust
pub enum NodeKind {
    // 输入输出
    Input(String, Type),           // 输入端口
    Output(String, Type),          // 输出端口
    
    // 基础节点
    Constant(ConstantValue),      // 常量值
    TextureSample(Handle<Texture>, UVCoord),  // 纹理采样
    VertexData(VertexField),      // 顶点数据
    FragmentData(FragmentField),  // 片段数据
    
    // 数学节点
    MathBinary(MathOp, NodeId, NodeId),  // 二元运算
    MathUnary(MathOp, NodeId),           // 一元运算
    
    // 颜色节点
    Color(ColorOp),                // 颜色操作
    
    // UV 节点
    UV(UVOp),                      // UV 操作
    
    // 时间节点
    Time(TimeOp),                  // 时间相关
    
    // 特效节点
    NormalMap(Handle<Texture>, NodeId, f32),  // 法线贴图
    PbrMaster(PbrMasterInputs),    // PBR 主节点
    
    // 控制流
    If(NodeId, Vec<NodeId>, Vec<NodeId>),   // 条件分支
    Switch(NodeId, Vec<(Value, Vec<NodeId>)>),  // 开关
    
    // 自定义
    Custom(String, String),        // 自定义节点
}
```

### 3.2.3 MathOp 枚举

```rust
pub enum MathOp {
    // 二元运算
    Add, Sub, Mul, Div, Pow, Min, Max, Dot, Cross, Distance,
    // 一元运算
    Negate, Abs, Sign, Sqrt, Log, Exp, Sin, Cos, Tan,
    Floor, Ceil, Round, Normalize, Length,
}
```

### 3.2.4 ColorOp 枚举

```rust
pub enum ColorOp {
    Swizzle(String),
    Mix(NodeId, NodeId),
    ToSrgb,
    ToLinear,
    Gamma(f32),
}
```

### 3.2.5 UVOp 枚举

```rust
pub enum UVOp {
    Tiling(f32, f32),
    Offset(f32, f32),
    Rotate(f32),
    Pan(f32, f32),
}
```

### 3.2.6 TimeOp 枚举

```rust
pub enum TimeOp {
    Time,
    SinTime,
    CosTime,
}
```

### 3.2.7 PbrMasterInputs 结构体

```rust
pub struct PbrMasterInputs {
    pub albedo: NodeId,
    pub normal: Option<NodeId>,
    pub metallic: NodeId,
    pub roughness: NodeId,
    pub ao: Option<NodeId>,
    pub emissive: Option<NodeId>,
    pub alpha: Option<NodeId>,
    pub clear_coat: Option<NodeId>,
    pub clear_coat_rough: Option<NodeId>,
    pub sheen: Option<NodeId>,
    pub sheen_rough: Option<NodeId>,
    pub subsurface: Option<NodeId>,
    pub anisotropy: Option<NodeId>,
}
```

### 3.2.8 ShaderGraphNode 编辑器属性

```rust
pub struct ShaderGraphNode {
    pub id: NodeId,
    pub kind: NodeKind,
    pub position: Vec2,      // 编辑器中的位置
    pub comment: String,     // 节点注释
    pub color: Color,         // 节点颜色
}
```

---

## 3.3 ShaderGraph API

### 3.3.1 生命周期

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 47, 281 | `ShaderGraph::new() -> Self` | 创建空图 |
| 323 | `ShaderGraph::name(&self) -> &str` | 获取图名称 |
| 324 | `ShaderGraph::set_name(&mut self, name: String)` | 设置图名称 |

### 3.3.2 图结构

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 325 | `ShaderGraph::nodes(&self) -> &[ShaderGraphNode]` | 获取所有节点 |
| 326 | `ShaderGraph::edges(&self) -> &[Edge]` | 获取所有边 |
| 327 | `ShaderGraph::add_node(&mut self, kind: NodeKind) -> NodeId` | 添加节点 |
| 328 | `ShaderGraph::remove_node(&mut self, id: NodeId)` | 删除节点 |
| 329 | `ShaderGraph::add_edge(&mut self, from: NodeId, to: NodeId) -> EdgeId` | 添加边 |
| 330 | `ShaderGraph::remove_edge(&mut self, id: EdgeId)` | 删除边 |

### 3.3.3 编译与验证

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 331 | `ShaderGraph::topological_order(&self) -> Result<Vec<NodeId>, CycleError>` | 拓扑排序 |
| 332 | `ShaderGraph::compile(&self) -> Result<ShaderSource>` | 编译为 ShaderSource |
| 333 | `ShaderGraph::validate(&self) -> Result<()>` | 验证图结构 |
| 334 | `ShaderGraph::to_json(&self) -> String` | 序列化为 JSON |
| 335 | `ShaderGraph::from_json(json: &str) -> Result<Self>` | 从 JSON 反序列化 |
| 357 | `ShaderGraph::generate_wgsl(&self) -> String` | 生成 WGSL 代码 |
| 358 | `ShaderGraph::generate_glsl(&self) -> String` | 生成 GLSL 代码 |

---

## 3.4 ShaderGraphEditor

编辑器 UI 集成。

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 359 | `ShaderGraphEditor::open(&mut self, graph: Handle<ShaderGraph>)` | 打开图 |
| 361 | `ShaderGraphEditor::close(&mut self)` | 关闭编辑器 |
| 362 | `ShaderGraphEditor::select(&mut self, node_id: NodeId)` | 选择节点 |
| 363 | `ShaderGraphEditor::draw(&mut self, ui: &mut Ui)` | 绘制编辑器 UI |
| 364 | `ShaderGraphEditor::preview(&mut self, renderer: &mut Renderer)` | 预览材质球 |

---

## 3.5 节点分类

| 需求编号 | 分类 | 包含节点 |
|----------|------|----------|
| 59 | Input | Input, VertexData, FragmentData |
| 59 | Constant | Constant |
| 59 | Math | MathBinary, MathUnary |
| 59 | Color | Color |
| 59 | Texture | TextureSample, NormalMap |
| 59 | UV | UV |
| 59 | Utility | If, Switch, Time |
| 59 | Advanced | Custom |

---

## 3.6 序列化格式

ShaderGraph JSON 格式：

```json
{
  "version": "1.0",
  "name": "MyShader",
  "nodes": [
    {
      "id": 0,
      "kind": "Input",
      "position": [100, 100],
      "comment": "",
      "color": "#ffffff"
    }
  ],
  "edges": [
    {
      "id": 0,
      "from": 0,
      "to": 1
    }
  ]
}
```

---

## 3.7 输入与输出

### 输入
- 用户通过编辑器 UI 添加节点和边
- 从 JSON 文件加载图结构

### 输出
- `ShaderSource`: 生成的着色器代码
- `Handle<ShaderGraph>`: 图资源句柄

---

## 3.8 验收标准

| 编号 | 标准 |
|----------|------|
| 47 | `ShaderGraph::new()` 创建空图 |
| 51 | `compile()` 正确生成 ShaderSource |
| 57 | 图可保存为 JSON 格式并版本化 |
| 58 | 编辑器支持撤销/重做 |
| 59 | 节点属性面板（Inspector 扩展）|
| 78 | 按拓扑排序生成 WGSL 片段 |
| 79 | 循环/分支 DAG 展开 |
| 80 | 内置节点代码生成 |
| 81 | PBR Master 节点生成完整 BRDF |
| 330 | `from_json` / `to_json` 往返一致 |
| 430 | 单测 `ShaderGraph` 拓扑排序 |
| 431 | 单测 `ShaderGraph` 代码生成（简单样例）|
| 432 | 单测 `ShaderGraph` PBR master 代码生成 |

---

## 3.9 依赖关系

### 依赖模块
- `ShaderCompiler`: 代码编译
- `PbrShader`: PBR 主节点实现

### 被依赖模块
- `engine-editor`: 编辑器 UI 集成

---

## 3.10 优先级

| 优先级 | 需求编号 | 说明 |
|--------|----------|------|
| P0 | 47, 51, 78-81, 281-295, 323-335 | 核心图结构与编译 |
| P1 | 57-59, 357-358, 359-364 | 编辑器功能与序列化 |
| P2 | 307-314, 330 | 高级节点与编辑器 UI |
