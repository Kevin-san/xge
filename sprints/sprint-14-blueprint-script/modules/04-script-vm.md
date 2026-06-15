# 模块四：脚本虚拟机（Script VM）

## 1. 模块概述

Script VM 模块提供统一的多语言脚本运行时抽象，支持 Rust dylib、JavaScript、TypeScript、Python、Lua、WASM 等脚本语言的嵌入与执行。

核心组件：
- `ScriptVM` trait：统一虚拟机抽象
- `ScriptValue` / `ScriptInstance` / `ScriptHandle`：值、实例、句柄类型
- `RustScriptVM` / `JsScriptVM` / `TsCompiler` / `PyScriptVM` / `LuaScriptVM` / `WasmScriptVM`：各语言实现
- `ScriptReflect` / `Bindgen`：反射与绑定生成
- `EntityRef` / `ComponentRef` / `ResourceRef`：脚本侧 ECS 访问
- `ScriptSystem` / `ScriptEventBus`：脚本系统集成

---

## 2. 需求清单

### 2.1 ScriptVM Trait（需求 82-87, 132, 257-264）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-001 | `ScriptVM` trait 定义：`load / call / set_global / get_global / update / gc` | P0 |
| REQ-VM-002 | `ScriptVM::load(&mut self, source: ScriptSource) -> Result<ScriptHandle, ScriptError>` 加载脚本 | P0 |
| REQ-VM-003 | `ScriptVM::call(&mut self, handle, fn_name: &str, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError>` 调用函数 | P0 |
| REQ-VM-004 | `ScriptVM::call_mut(&mut self, handle, fn_name, args) -> Result<ScriptValue, ScriptError>` 可变调用 | P1 |
| REQ-VM-005 | `ScriptVM::set_global(&mut self, name: &str, value: ScriptValue)` 设置全局变量 | P0 |
| REQ-VM-006 | `ScriptVM::get_global(&self, name: &str) -> Option<ScriptValue>` 获取全局变量 | P0 |
| REQ-VM-007 | `ScriptVM::update(&mut self, dt: f32)` 每帧更新 | P1 |
| REQ-VM-008 | `ScriptVM::gc(&mut self)` 垃圾回收 | P1 |

### 2.2 ScriptValue（需求 88, 138, 266-275）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-009 | `ScriptValue::Null / Bool(bool) / Int(i64) / Float(f64) / String(String)` | P0 |
| REQ-VM-010 | `ScriptValue::Array(Vec<ScriptValue>)` 数组类型 | P0 |
| REQ-VM-011 | `ScriptValue::Map(HashMap<String, ScriptValue>)` 映射类型 | P0 |
| REQ-VM-012 | `ScriptValue::Entity(EntityRef)` 实体引用 | P0 |
| REQ-VM-013 | `ScriptValue::UserData(TypeId, Arc<dyn Any>)` 用户数据 | P1 |
| REQ-VM-014 | `ScriptValue::type_name(&self) -> &str` 获取类型名称 | P1 |
| REQ-VM-015 | `ScriptValue::to_bool(&self) -> Option<bool>` 类型转换 | P1 |
| REQ-VM-016 | `ScriptValue::to_int(&self) -> Option<i64>` 类型转换 | P1 |
| REQ-VM-017 | `ScriptValue::to_float(&self) -> Option<f64>` 类型转换 | P1 |
| REQ-VM-018 | `ScriptValue::to_string_lossy(&self) -> String` 转为字符串 | P1 |

### 2.3 ScriptSource 与 ScriptHandle（需求 265, 278-282）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-019 | `ScriptSource::Path(PathBuf) / Code(String) / Bytes(Vec<u8>)` 脚本源类型 | P0 |
| REQ-VM-020 | `ScriptHandle = u64` 强类型 newtype | P0 |
| REQ-VM-021 | `ScriptInstance`：`vm_handle + fn_table + state + last_error` | P0 |
| REQ-VM-022 | `ScriptInstance::new(handle) -> Self` 创建实例 | P0 |
| REQ-VM-023 | `ScriptInstance::has(&self, name) -> bool` 检查函数是否存在 | P1 |
| REQ-VM-024 | `ScriptInstance::last_error(&self) -> Option<&ScriptError>` 获取最后错误 | P1 |

### 2.4 ScriptError（需求 276-277）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-025 | `ScriptError::CompileError(String) / RuntimeError(String)` | P0 |
| REQ-VM-026 | `ScriptError::Timeout / MemoryLimit` | P0 |
| REQ-VM-027 | `ScriptError::NotFound(String) / InvalidArg(String) / IoError(io::Error)` | P0 |

### 2.5 RustScriptVM（需求 90-92, 141-142, 283-287）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-028 | `RustScriptVM::new() -> Self` | P0 |
| REQ-VM-029 | `RustScriptVM::load_dylib(&mut self, path) -> Result<ScriptHandle, ScriptError>` 加载 dylib | P0 |
| REQ-VM-030 | `RustScriptVM::reload(&mut self, handle)` 热替换 | P1 |
| REQ-VM-031 | `RustScriptVM::export(&self, handle, name) -> Option<unsafe extern "C" fn(...)>` 导出函数 | P1 |

### 2.6 JsScriptVM（需求 93-94, 143-144, 288-291）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-032 | `JsScriptVM::new() -> Self`（默认 quickjs，可选 RustyV8） | P0 |
| REQ-VM-033 | `JsScriptVM::evaluate(&mut self, code: &str) -> Result<ScriptValue, ScriptError>` 执行 JS | P0 |
| REQ-VM-034 | `JsScriptVM::register_fn(&mut self, name, f)` 注入 Rust 闭包为 JS 函数 | P1 |
| REQ-VM-035 | `JsScriptVM::register_class(&mut self, name, methods)` 注入 Rust struct 为 JS 对象 | P1 |

### 2.7 TsCompiler（需求 95, 145, 292-293）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-036 | `TsCompiler::compile(&self, source) -> Result<JsCode, String>` 编译 TS 到 JS | P1 |
| REQ-VM-037 | `TsCompiler::emit_declarations(&self, source) -> Result<String, String>` 生成 `.d.ts` | P1 |

### 2.8 PyScriptVM（需求 96, 146, 294-296）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-038 | `PyScriptVM::new() -> Self`（`pyo3::Python::with_gil`） | P0 |
| REQ-VM-039 | `PyScriptVM::evaluate(&mut self, code: &str) -> Result<ScriptValue, ScriptError>` 执行 Python | P0 |
| REQ-VM-040 | `PyScriptVM::register_module(&mut self, name, module_fn)` 导出模块 | P1 |

### 2.9 LuaScriptVM（需求 97, 147, 297-299）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-041 | `LuaScriptVM::new() -> Self`（`mlua::Lua::new()`） | P0 |
| REQ-VM-042 | `LuaScriptVM::evaluate(&mut self, code: &str) -> Result<ScriptValue, ScriptError>` 执行 Lua | P0 |
| REQ-VM-043 | `LuaScriptVM::create_table(&mut self) -> ScriptValue` 创建表 | P1 |

### 2.10 WasmScriptVM（需求 98, 148, 300-302）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-044 | `WasmScriptVM::new() -> Self`（`wasmtime::Engine` + `Store`） | P0 |
| REQ-VM-045 | `WasmScriptVM::load_bytes(&mut self, bytes: &[u8]) -> Result<ScriptHandle, ScriptError>` 加载 WASM | P0 |
| REQ-VM-046 | `WasmScriptVM::call_export(&mut self, handle, name, args) -> Result<ScriptValue, ScriptError>` 调用导出 | P0 |
| REQ-VM-047 | `WasmScriptVM::link_host_fn(&mut self, module, name, f)` 链接 host 函数 | P1 |

### 2.11 ScriptReflect 与 Bindgen（需求 105-107, 156-157, 321-337）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-048 | `ScriptReflect` 对脚本暴露的 Rust 类型/函数的描述 | P1 |
| REQ-VM-049 | `ScriptReflect::type_name(&self) -> &str` | P1 |
| REQ-VM-050 | `ScriptReflect::fields(&self) -> &[FieldDesc]` | P1 |
| REQ-VM-051 | `ScriptReflect::methods(&self) -> &[MethodDesc]` | P1 |
| REQ-VM-052 | `FieldDesc::new(name, ty, readonly)` | P1 |
| REQ-VM-053 | `MethodDesc::new(name, params, ret)` | P1 |
| REQ-VM-054 | `ScriptType::Null / Bool / Int / Float / String / Array / Map / Entity / UserType(String)` | P1 |
| REQ-VM-055 | `Bindgen::new() -> Self` | P2 |
| REQ-VM-056 | `Bindgen::add_type(&mut self, reflect)` | P2 |
| REQ-VM-057 | `Bindgen::add_function(&mut self, name, signature)` | P2 |
| REQ-VM-058 | `Bindgen::emit_d_ts(&self) -> String` TypeScript 声明 | P2 |
| REQ-VM-059 | `Bindgen::emit_pyi(&self) -> String` Python stub | P2 |
| REQ-VM-060 | `Bindgen::emit_lua_defs(&self) -> String` Lua 注释 | P2 |
| REQ-VM-061 | `Bindgen::emit_wasm_js(&self) -> String` WASM JS 粘合层 | P2 |
| REQ-VM-062 | `#[script_expose]` 过程宏（标记函数/struct 为脚本可见） | P1 |
| REQ-VM-063 | `#[script_expose]` 对 `struct` 生成 `ScriptReflect` 实现 | P1 |

### 2.12 Entity/Component/Resource 访问（需求 158-160, 338-343）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-064 | `EntityRef`：`Entity(index, generation)`，脚本不可直接修改字段 | P0 |
| REQ-VM-065 | `EntityRef::id(&self) -> u64` | P0 |
| REQ-VM-066 | `EntityRef::is_alive(&self, world) -> bool` | P1 |
| REQ-VM-067 | `ComponentRef::get(&self, entity) -> Option<ScriptValue>` | P0 |
| REQ-VM-068 | `ComponentRef::set(&self, entity, value) -> Result<(), ScriptError>` | P0 |
| REQ-VM-069 | `ResourceRef::get(&self) -> ScriptValue` | P1 |
| REQ-VM-070 | `ResourceRef::set(&self, value)` | P1 |

### 2.13 ScriptSystem 与 ScriptEventBus（需求 111-112, 162, 344-346）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-VM-071 | `ScriptSystem::new(world) -> Self` | P0 |
| REQ-VM-072 | `ScriptSystem::update(&mut self, world, dt)` 每个实例调用 `OnUpdate(dt)` | P0 |
| REQ-VM-073 | `ScriptEventBus::emit(&mut self, event)` 事件派发到脚本回调 | P1 |

---

## 3. API 签名

### 3.1 ScriptVM Trait

```rust
pub trait ScriptVM {
    fn load(&mut self, source: ScriptSource) -> Result<ScriptHandle, ScriptError>;
    fn call(&mut self, handle: ScriptHandle, fn_name: &str, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError>;
    fn call_mut(&mut self, handle: ScriptHandle, fn_name: &str, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError>;
    fn set_global(&mut self, name: &str, value: ScriptValue);
    fn get_global(&self, name: &str) -> Option<ScriptValue>;
    fn update(&mut self, dt: f32);
    fn gc(&mut self);
}

pub enum ScriptSource {
    Path(PathBuf),
    Code(String),
    Bytes(Vec<u8>),
}

#[derive(Clone, Copy)]
pub struct ScriptHandle(pub u64);
```

### 3.2 ScriptValue

```rust
pub enum ScriptValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<ScriptValue>),
    Map(HashMap<String, ScriptValue>),
    Entity(EntityRef),
    UserData(TypeId, Arc<dyn Any>),
}

impl ScriptValue {
    pub fn type_name(&self) -> &str;
    pub fn to_bool(&self) -> Option<bool>;
    pub fn to_int(&self) -> Option<i64>;
    pub fn to_float(&self) -> Option<f64>;
    pub fn to_string_lossy(&self) -> String;
}
```

### 3.3 各语言 VM

```rust
// RustScriptVM
pub struct RustScriptVM { /* ... */ }
impl RustScriptVM {
    pub fn new() -> Self;
    pub fn load_dylib(&mut self, path: &Path) -> Result<ScriptHandle, ScriptError>;
    pub fn reload(&mut self, handle: ScriptHandle);
    pub fn export(&self, handle: ScriptHandle, name: &str) -> Option<unsafe extern "C" fn(...)>;
}

// JsScriptVM
pub struct JsScriptVM { /* ... */ }
impl JsScriptVM {
    pub fn new() -> Self;
    pub fn evaluate(&mut self, code: &str) -> Result<ScriptValue, ScriptError>;
    pub fn register_fn<F: 'static>(&mut self, name: &str, f: F)
        where F: Fn(&[ScriptValue]) -> ScriptValue;
    pub fn register_class(&mut self, name: &str, methods: &[(&str, ...));
}

// TsCompiler
pub struct TsCompiler;
impl TsCompiler {
    pub fn compile(&self, source: &str) -> Result<String, String>;
    pub fn emit_declarations(&self, source: &str) -> Result<String, String>;
}

// PyScriptVM
pub struct PyScriptVM { /* ... */ }
impl PyScriptVM {
    pub fn new() -> Self;
    pub fn evaluate(&mut self, code: &str) -> Result<ScriptValue, ScriptError>;
    pub fn register_module(&mut self, name: &str, module_fn: Py<PyModule>);
}

// LuaScriptVM
pub struct LuaScriptVM { /* ... */ }
impl LuaScriptVM {
    pub fn new() -> Self;
    pub fn evaluate(&mut self, code: &str) -> Result<ScriptValue, ScriptError>;
    pub fn create_table(&mut self) -> ScriptValue;
}

// WasmScriptVM
pub struct WasmScriptVM { /* ... */ }
impl WasmScriptVM {
    pub fn new() -> Self;
    pub fn load_bytes(&mut self, bytes: &[u8]) -> Result<ScriptHandle, ScriptError>;
    pub fn call_export(&mut self, handle: ScriptHandle, name: &str, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError>;
    pub fn link_host_fn<F: 'static>(&mut self, module: &str, name: &str, f: F);
}
```

---

## 4. 输入与输出

| 组件 | 输入 | 输出 |
|------|------|------|
| `ScriptVM::load` | `ScriptSource` | `Result<ScriptHandle, ScriptError>` |
| `ScriptVM::call` | `ScriptHandle, &str, &[ScriptValue]` | `Result<ScriptValue, ScriptError>` |
| `ScriptVM::set_global` | `&str, ScriptValue` | `()` |
| `ScriptVM::get_global` | `&str` | `Option<ScriptValue>` |
| `ScriptValue::to_bool/int/float` | - | `Option<T>` |
| `RustScriptVM::load_dylib` | `&Path` | `Result<ScriptHandle, ScriptError>` |
| `JsScriptVM::evaluate` | `&str` | `Result<ScriptValue, ScriptError>` |
| `PyScriptVM::evaluate` | `&str` | `Result<ScriptValue, ScriptError>` |
| `LuaScriptVM::evaluate` | `&str` | `Result<ScriptValue, ScriptError>` |
| `WasmScriptVM::call_export` | `ScriptHandle, &str, &[ScriptValue]` | `Result<ScriptValue, ScriptError>` |

---

## 5. 验收标准

- [ ] `JsScriptVM::evaluate` 对 `1+2` 返回 `ScriptValue::Float(3.0)`
- [ ] `TsCompiler::compile` 对 TS 源码生成可被 JS 运行代码
- [ ] `PyScriptVM` 对 `x = 1; x + 2` 返回 3
- [ ] `LuaScriptVM` 对 `return 1 + 2` 返回 3
- [ ] `WasmScriptVM` 对 `add(i32, i32) -> i32` 模块调用正确
- [ ] `RustScriptVM::load_dylib` 对 mock `.so` 成功加载
- [ ] `ScriptValue::from/to` 各种类型转换正确
- [ ] 脚本可访问 Entity/Component/Resource

---

## 6. 依赖关系

- 依赖 `engine-ecs` crate 的 `World`、`Entity`、`Component`
- 依赖 `libloading`（Rust dylib）、`rquickjs`/`rusty_v8`（JS）、`pyo3`（Python）、`mlua`（Lua）、`wasmtime`（WASM）
- 被 `ScriptSystem` 聚合，每帧调用 `update`
- 被 `examples/script_*` 示例使用

---

## 7. 优先级汇总

| 优先级 | 需求数量 | 说明 |
|-------|---------|------|
| P0 | 38 | 核心 VM 功能，必须完成 |
| P1 | 26 | 重要功能，应完成 |
| P2 | 6 | 绑定生成等增强功能 |
