
# 新编程语言技术架构

## 1. 架构设计

### 1.1 整体架构图

```
┌─────────────────────────────────────────────────────────────────┐
│                          工具层 (Tools)                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   编译器CLI  │  │  LSP 服务器  │  │  AI 辅助工具  │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                        编译器驱动 (Driver)                        │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              编译管道协调与错误处理                        │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                       编译器前端 (Frontend)                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
│  │  Lexer   │→ │ Parser   │→ │ Semantic │→ │  Typeck  │      │
│  │ (词法分析)│  │ (语法分析)│  │ (语义分析)│  │ (类型检查)│      │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘      │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                    中间表示层 (Middle End)                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
│  │  HIR Gen │→ │  MIR Gen │→ │ Optimize │→ │  IR Pass │      │
│  │ (高级IR) │  │ (中级IR) │  │  (优化)   │  │ (IR 处理)│      │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘      │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                     编译器后端 (Backend)                          │
│  ┌──────────────────┐              ┌──────────────────┐        │
│  │  Native Backend  │              │  Wasm Backend    │        │
│  │  ┌────────────┐  │              │  ┌────────────┐  │        │
│  │  │ x86_64 Gen │  │              │  │ Wasm32 Gen │  │        │
│  │  ├────────────┤  │              │  ├────────────┤  │        │
│  │  │ ARM64 Gen  │  │              │  │ Wasm Opt   │  │        │
│  │  ├────────────┤  │              │  └────────────┘  │        │
│  │  │ Linker     │  │              └──────────────────┘        │
│  │  └────────────┘  │                                          │
│  └──────────────────┘                                          │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                        运行时系统 (Runtime)                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
│  │  Memory  │  │  Scheduler│  │  GC/ARC  │  │  Exceptions│    │
│  │  (内存管理)│  │ (调度器) │  │ (垃圾回收)│  │  (异常处理)│     │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘      │
└─────────────────────────────────────────────────────────────────┘
```

## 2. 技术选型

### 2.1 核心技术栈
- **开发语言**: Rust (stable 最新版本)
- **词法分析**: Logos (高性能词法分析器生成库)
- **语法分析**: Lalrpop (LR 解析器生成器) + 手工编写递归下降
- **中间表示**: 自定义 SSA (静态单赋值) 形式 IR
- **代码生成**: 
  - 原生后端: Cranelift (代码生成库)
  - Wasm 后端: wasmtime / walrus
- **AI 集成**: 
  - 本地模型: candle (Rust 深度学习框架)
  - 远程 API: reqwest (HTTP 客户端)

### 2.2 依赖库
| 库名 | 用途 | 版本 |
|------|------|------|
| logos | 词法分析 | 0.14 |
| lalrpop | 语法分析 | 0.20 |
| cranelift | 代码生成 | 0.108 |
| wasmtime | Wasm 运行时 | 20.0 |
| candle | 深度学习 | 0.6 |
| anyhow | 错误处理 | 1.0 |
| thiserror | 错误定义 | 1.0 |
| clap | CLI 解析 | 4.5 |
| tokio | 异步运行时 | 1.36 |
| parking_lot | 并发原语 | 0.12 |
| indexmap | 有序映射 | 2.2 |

## 3. 项目结构

```
/workspace
├── Cargo.toml              # 工作空间配置
├── PRD.md                  # 需求文档
├── ARCHITECTURE.md         # 架构文档
├── README.md               # 项目说明
├── crates/
│   ├── compiler/           # 编译器主 crate
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── driver.rs   # 编译驱动
│   │   │   ├── lexer/      # 词法分析
│   │   │   ├── parser/     # 语法分析
│   │   │   ├── semantic/   # 语义分析
│   │   │   ├── hir/        # 高级中间表示
│   │   │   ├── mir/        # 中级中间表示
│   │   │   ├── optimize/   # 优化通道
│   │   │   └── backend/    # 代码生成后端
│   ├── lexer/              # 词法分析器 crate
│   ├── parser/             # 语法分析器 crate
│   ├── hir/                # HIR crate
│   ├── mir/                # MIR crate
│   ├── codegen/            # 代码生成 crate
│   ├── runtime/            # 运行时库
│   ├── stdlib/             # 标准库
│   ├── ai/                 # AI 辅助功能
│   ├── dsl/                # DSL 支持
│   ├── cli/                # 命令行工具
│   └── tests/              # 集成测试
├── examples/               # 示例代码
└── docs/                   # 文档
```

## 4. 核心数据结构

### 4.1 Token 类型 (词法分析输出)
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // 关键字
    Let,
    If,
    Else,
    For,
    While,
    Fn,
    Class,
    Return,
    Async,
    Await,
    Try,
    Except,
    
    // 字面量
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
    
    // 标识符
    Ident(String),
    
    // 运算符
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    Not,
    
    // 分隔符
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Semicolon,
    Arrow,
    FatArrow,
    
    // 其他
    Indent,
    Dedent,
    Newline,
    Eof,
}
```

### 4.2 AST 节点 (语法分析输出)
```rust
#[derive(Debug, Clone)]
pub struct Module {
    pub items: Vec&lt;Item&gt;,
}

#[derive(Debug, Clone)]
pub enum Item {
    FnDef(FnDef),
    ClassDef(ClassDef),
    LetDecl(LetDecl),
    Import(Import),
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: Ident,
    pub params: Vec&lt;Param&gt;,
    pub return_type: Option&lt;Type&gt;,
    pub body: Block,
    pub is_async: bool,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(PrimitiveType),
    Named(Ident),
    Generic(Box&lt;Type&gt;, Vec&lt;Type&gt;),
    Function(Vec&lt;Type&gt;, Box&lt;Type&gt;),
    Option(Box&lt;Type&gt;),
    Result(Box&lt;Type&gt;, Box&lt;Type&gt;),
}

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    F32, F64,
    Bool, Char, String,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec&lt;Stmt&gt;,
    pub expr: Option&lt;Expr&gt;,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(LetDecl),
    Expr(Expr),
    If(IfStmt),
    For(ForStmt),
    While(WhileStmt),
    Return(Option&lt;Expr&gt;),
    Try(TryStmt),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(Ident),
    Binary(Box&lt;Expr&gt;, BinOp, Box&lt;Expr&gt;),
    Unary(UnOp, Box&lt;Expr&gt;),
    Call(Box&lt;Expr&gt;, Vec&lt;Expr&gt;),
    Method(Box&lt;Expr&gt;, Ident, Vec&lt;Expr&gt;),
    Field(Box&lt;Expr&gt;, Ident),
    Index(Box&lt;Expr&gt;, Box&lt;Expr&gt;),
    Await(Box&lt;Expr&gt;),
    Block(Block),
    If(Box&lt;Expr&gt;, Block, Option&lt;Block&gt;),
}
```

### 4.3 HIR (高级中间表示)
```rust
#[derive(Debug, Clone)]
pub struct HirModule {
    pub items: Vec&lt;HirItem&gt;,
}

#[derive(Debug, Clone)]
pub enum HirItem {
    Function(HirFunction),
    Struct(HirStruct),
    Enum(HirEnum),
    Const(HirConst),
}

#[derive(Debug, Clone)]
pub struct HirFunction {
    pub name: Symbol,
    pub sig: HirSignature,
    pub body: HirBody,
    pub is_async: bool,
}

#[derive(Debug, Clone)]
pub struct HirBody {
    pub locals: Vec&lt;HirLocal&gt;,
    pub blocks: Vec&lt;HirBasicBlock&gt;,
}

#[derive(Debug, Clone)]
pub struct HirBasicBlock {
    pub stmts: Vec&lt;HirStmt&gt;,
    pub terminator: HirTerminator,
}

#[derive(Debug, Clone)]
pub enum HirStmt {
    Assign(HirPlace, HirRvalue),
    Nop,
}

#[derive(Debug, Clone)]
pub enum HirTerminator {
    Goto(BasicBlockId),
    Return(Option&lt;HirOperand&gt;),
    If(HirOperand, BasicBlockId, BasicBlockId),
    Unreachable,
}
```

## 5. 编译管道

### 5.1 编译流程
1. **词法分析 (Lexing)**: 将源代码转换为 Token 流
2. **语法分析 (Parsing)**: 将 Token 流转换为 AST
3. **语义分析 (Semantic Analysis)**: 
   - 名称解析
   - 类型检查
   - 借用检查
4. **HIR 生成**: 将 AST 转换为高级中间表示
5. **MIR 生成**: 将 HIR 转换为中级中间表示 (SSA 形式)
6. **优化 (Optimization)**: 
   - 常量折叠
   - 死代码消除
   - 内联
   - 循环优化
7. **代码生成 (Code Generation)**:
   - 原生机器码生成 (x86_64, ARM64)
   - WebAssembly 生成

### 5.2 编译驱动
```rust
pub struct Compiler {
    options: CompilerOptions,
    diagnostics: Diagnostics,
}

impl Compiler {
    pub fn new(options: CompilerOptions) -&gt; Self {
        Compiler {
            options,
            diagnostics: Diagnostics::new(),
        }
    }
    
    pub fn compile(&amp;mut self, source: &amp;str) -&gt; Result&lt;CompileOutput, CompileError&gt; {
        let tokens = self.lex(source)?;
        let ast = self.parse(tokens)?;
        let hir = self.lower_to_hir(ast)?;
        let mir = self.lower_to_mir(hir)?;
        let optimized_mir = self.optimize(mir)?;
        let output = self.codegen(optimized_mir)?;
        
        Ok(output)
    }
}
```

## 6. 优化算法

### 6.1 常量折叠 (Constant Folding)
- **算法**: 在编译时计算常量表达式
- **复杂度**: O(n)，n 为表达式数量
- **实现**: 遍历 MIR，对纯常量表达式进行计算

### 6.2 死代码消除 (Dead Code Elimination)
- **算法**: 基于控制流图的可达性分析
- **复杂度**: O(n + e)，n 为基本块数，e 为边数
- **实现**: 
  1. 标记从入口块可达的所有块
  2. 删除不可达块
  3. 删除未使用的变量定义

### 6.3 内联优化 (Inlining)
- **算法**: 启发式内联，基于函数大小和调用频率
- **复杂度**: O(n * m)，n 为调用点数，m 为函数大小
- **启发式规则**:
  - 小函数 (≤ 10 条指令) 总是内联
  - 只调用一次的函数内联
  - 热路径上的函数优先内联

### 6.4 循环优化 (Loop Optimization)
- **循环不变量外提 (Loop Invariant Code Motion)**: O(n)
- **循环展开 (Loop Unrolling)**: 基于目标架构的优化
- **循环向量化 (Loop Vectorization)**: SIMD 优化

## 7. 后端代码生成

### 7.1 Cranelift 后端 (原生代码)
```
MIR → Cranelift CLIF → 机器码 → 链接 → 可执行文件
```

### 7.2 Wasm 后端
```
MIR → Wasm 二进制 → Wasm 优化 → Wasm 模块
```

## 8. AI 辅助模块架构

### 8.1 模块结构
```
ai/
├── src/
│   ├── lib.rs
│   ├── code_completion.rs    # 代码补全
│   ├── error_fix.rs          # 错误修复
│   ├── refactoring.rs        # 重构建议
│   ├── doc_gen.rs            # 文档生成
│   └── model/                # 模型集成
│       ├── local.rs          # 本地模型
│       └── remote.rs         # 远程 API
```

### 8.2 代码补全算法
- **算法**: N-gram + 神经网络混合模型
- **特征**: 
  - 前缀 Token 序列
  - AST 上下文
  - 类型信息
  - 导入的库

## 9. DSL 支持架构

### 9.1 SQL DSL
- 使用 proc macro 实现 `sql!` 宏
- 编译时 SQL 语法检查
- 类型安全的查询生成

### 9.2 正则表达式 DSL
- 编译时正则表达式编译
- 优化的自动机生成
- 零成本抽象

## 10. 并发模型

### 10.1 异步任务调度
- 工作窃取 (Work-stealing) 调度器
- 无锁队列
- M:N 线程映射

### 10.2 线程安全
- 所有权系统
- Send/Sync trait
- 原子操作优先
- 无锁数据结构

## 11. 测试策略

### 11.1 测试金字塔
- **单元测试**: 70%，覆盖各模块
- **集成测试**: 20%，覆盖编译管道
- **端到端测试**: 10%，完整程序测试

### 11.2 测试框架
- Rust 内置测试框架
- 自定义测试运行器
- 模糊测试 (cargo-fuzz)
- 基准测试 (criterion)

