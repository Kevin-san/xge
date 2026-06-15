# 模块五：沙盒安全（Script Sandbox）

## 1. 模块概述

沙盒安全模块为脚本虚拟机提供安全隔离能力，防止恶意或错误的脚本访问系统资源。每个 VM 可配置独立的安全策略，控制内存、CPU 时间、文件 IO、网络访问。

核心组件：
- `ScriptSandboxPolicy`：安全策略容器
- `ScriptLimits`：资源限制参数
- `IOWhitelist`：文件系统访问白名单
- `NetworkWhitelist`：网络访问白名单
- `InstructionCounter`：指令计数（用于超时检测）
- `TimeoutGuard`：超时守护
- `MemoryGuard`：内存守护
- `FileIoHook` / `NetworkHook`：系统钩子

---

## 2. 需求清单

### 2.1 ScriptLimits（需求 100-101, 150-151, 304-306）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-SB-001 | `ScriptLimits::max_memory_bytes` 最大内存（默认 64MB） | P0 |
| REQ-SB-002 | `ScriptLimits::max_instructions_per_tick` 最大指令数（默认 1M/tick） | P0 |
| REQ-SB-003 | `ScriptLimits::max_execution_time_ms` 最大执行时间（默认 5ms） | P0 |
| REQ-SB-004 | `ScriptLimits::max_stack_depth` 最大栈深度 | P1 |
| REQ-SB-005 | `ScriptLimits::max_string_length` 最大字符串长度 | P1 |
| REQ-SB-006 | `ScriptLimits::max_array_length` 最大数组长度 | P1 |
| REQ-SB-007 | `ScriptLimits::default()` 返回默认配置 | P0 |
| REQ-SB-008 | `ScriptLimits::with_memory(mb) -> Self` 链式配置 | P1 |
| REQ-SB-009 | `ScriptLimits::with_instructions(n) -> Self` 链式配置 | P1 |
| REQ-SB-010 | `ScriptLimits::with_time_ms(ms) -> Self` 链式配置 | P1 |

### 2.2 IOWhitelist（需求 102, 152, 307-309）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-SB-011 | `IOWhitelist::allow_path_prefix(&mut self, path)` 添加允许的路径前缀 | P0 |
| REQ-SB-012 | `IOWhitelist::is_allowed(&self, path) -> bool` 检查路径是否允许 | P0 |
| REQ-SB-013 | 默认全部禁止（需显式添加白名单） | P0 |

### 2.3 NetworkWhitelist（需求 103, 153, 309-312）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-SB-014 | `NetworkWhitelist::allow_host(&mut self, host, port)` 添加允许的 host:port | P0 |
| REQ-SB-015 | `NetworkWhitelist::is_allowed(&self, host, port) -> bool` 检查是否允许 | P0 |
| REQ-SB-016 | `NetworkWhitelist::block_all(&mut self)` 阻止所有网络（默认状态） | P0 |

### 2.4 ScriptSandboxPolicy（需求 99, 149, 307, 387）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-SB-017 | `ScriptSandboxPolicy::new(limits, io, net)` 创建策略 | P0 |
| REQ-SB-018 | `ScriptSandbox::enforce(&self, vm)` 每次调用前检查策略 | P0 |
| REQ-SB-019 | 沙盒默认禁用一切 IO/网络；必须在 `ScriptSandboxPolicy` 显式开启 | P0 |

### 2.5 指令计数与超时（需求 393-396）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-SB-020 | `InstructionCounter` 在 Js/Lua/Py 解释器中以 hook 或 bytecode patch 方式计数 | P0 |
| REQ-SB-021 | `InstructionCounter::inc(&mut self, n)` 计数增加，超出上限触发 `ScriptError::InstructionLimit` | P0 |
| REQ-SB-022 | `TimeoutGuard` 跨线程定时器，到时主动中断 VM 执行 | P0 |
| REQ-SB-023 | `MemoryGuard` 以分配器 hook 统计当前 VM 字节数（WASM 可用线性内存限制） | P1 |

### 2.6 文件 IO 与网络钩子（需求 316-318）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-SB-024 | `FileIoHook` 脚本侧文件接口，强制走 `IOWhitelist` | P0 |
| REQ-SB-025 | `NetworkHook` 脚本侧网络 API，强制走 `NetworkWhitelist` | P0 |

### 2.7 沙盒审计（需求 404）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-SB-026 | 每次被拦截的调用在日志输出 | P1 |
| REQ-SB-027 | 可订阅事件 `SandboxViolationEvent` | P2 |

---

## 3. API 签名

### 3.1 ScriptLimits

```rust
#[derive(Clone)]
pub struct ScriptLimits {
    pub max_memory_bytes: usize,
    pub max_instructions_per_tick: u64,
    pub max_execution_time_ms: u64,
    pub max_stack_depth: usize,
    pub max_string_length: usize,
    pub max_array_length: usize,
}

impl ScriptLimits {
    pub fn default() -> Self {
        Self {
            max_memory_bytes: 64 * 1024 * 1024,
            max_instructions_per_tick: 1_000_000,
            max_execution_time_ms: 5,
            max_stack_depth: 1024,
            max_string_length: 100_000,
            max_array_length: 10_000,
        }
    }

    pub fn with_memory(mut self, mb: usize) -> Self {
        self.max_memory_bytes = mb * 1024 * 1024;
        self
    }

    pub fn with_instructions(mut self, n: u64) -> Self {
        self.max_instructions_per_tick = n;
        self
    }

    pub fn with_time_ms(mut self, ms: u64) -> Self {
        self.max_execution_time_ms = ms;
        self
    }
}
```

### 3.2 IOWhitelist

```rust
pub struct IOWhitelist {
    allowed_prefixes: Vec<PathBuf>,
}

impl IOWhitelist {
    pub fn new() -> Self;
    pub fn allow_path_prefix(&mut self, path: impl AsRef<Path>);
    pub fn is_allowed(&self, path: &Path) -> bool {
        self.allowed_prefixes.iter().any(|p| path.starts_with(p))
    }
}
```

### 3.3 NetworkWhitelist

```rust
pub struct NetworkWhitelist {
    allowed: HashSet<(String, u16)>,
}

impl NetworkWhitelist {
    pub fn new() -> Self;
    pub fn allow_host(&mut self, host: &str, port: u16);
    pub fn is_allowed(&self, host: &str, port: u16) -> bool;
    pub fn block_all(&mut self);
}
```

### 3.4 ScriptSandboxPolicy

```rust
pub struct ScriptSandboxPolicy {
    limits: ScriptLimits,
    io_whitelist: IOWhitelist,
    net_whitelist: NetworkWhitelist,
}

impl ScriptSandboxPolicy {
    pub fn new(limits: ScriptLimits, io: IOWhitelist, net: NetworkWhitelist) -> Self;
}

pub struct ScriptSandbox<'a> {
    policy: &'a ScriptSandboxPolicy,
    instruction_counter: InstructionCounter,
    timeout_guard: TimeoutGuard,
    memory_guard: MemoryGuard,
}

impl<'a> ScriptSandbox<'a> {
    pub fn enforce(&self, vm: &mut dyn ScriptVM) -> Result<(), ScriptError>;
}
```

### 3.5 InstructionCounter

```rust
pub struct InstructionCounter {
    count: u64,
    limit: u64,
}

impl InstructionCounter {
    pub fn new(limit: u64) -> Self;
    pub fn inc(&mut self, n: u64) -> Result<(), ScriptError>;
    pub fn reset(&mut self);
}
```

### 3.6 TimeoutGuard

```rust
pub struct TimeoutGuard {
    deadline: Instant,
}

impl TimeoutGuard {
    pub fn new(timeout_ms: u64) -> Self;
    pub fn is_expired(&self) -> bool;
    pub fn check(&self) -> Result<(), ScriptError>;
}
```

### 3.7 MemoryGuard

```rust
pub struct MemoryGuard {
    current_bytes: usize,
    limit_bytes: usize,
}

impl MemoryGuard {
    pub fn new(limit_bytes: usize) -> Self;
    pub fn alloc(&mut self, bytes: usize) -> Result<(), ScriptError>;
    pub fn dealloc(&mut self, bytes: usize);
    pub fn current(&self) -> usize;
}
```

---

## 4. 输入与输出

| 组件 | 输入 | 输出 |
|------|------|------|
| `IOWhitelist::is_allowed` | `&Path` | `bool` |
| `NetworkWhitelist::is_allowed` | `&str, u16` | `bool` |
| `InstructionCounter::inc` | `u64` | `Result<(), ScriptError>` |
| `TimeoutGuard::check` | - | `Result<(), ScriptError>` |
| `MemoryGuard::alloc` | `usize` | `Result<(), ScriptError>` |
| `ScriptSandbox::enforce` | `&mut dyn ScriptVM` | `Result<(), ScriptError>` |

---

## 5. 验收标准

- [ ] `ScriptLimits` + `InstructionCounter` 超出上限返回 `ScriptError::InstructionLimit`
- [ ] `IOWhitelist` 对不在白名单的路径返回 `false`
- [ ] `NetworkWhitelist` 对不在白名单的 host 返回 `false`
- [ ] 无限循环脚本在 `max_instructions` 后被终止
- [ ] 尝试打开 `/etc/passwd` 失败
- [ ] 尝试连接 `8.8.8.8:53` 失败
- [ ] `SandboxViolationEvent` 正确发布被拦截的调用

---

## 6. 依赖关系

- 依赖 `04-script-vm` 模块的 `ScriptVM`、`ScriptError`
- 被 `RustScriptVM`、`JsScriptVM`、`PyScriptVM`、`LuaScriptVM`、`WasmScriptVM` 使用
- 通过 `#[cfg(feature = "sandbox")]` 条件编译可选启用

---

## 7. 优先级汇总

| 优先级 | 需求数量 | 说明 |
|-------|---------|------|
| P0 | 17 | 核心安全功能，必须完成 |
| P1 | 8 | 重要功能，应完成 |
| P2 | 2 | 审计功能增强 |
