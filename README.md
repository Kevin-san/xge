
# MyLang - 新一代编程语言

一个整合了所有主流编程语言优点的新型编程语言，使用 Rust 开发。

## 特性

- **Python 风格的语法**: 简洁的缩进式语法
- **Java 风格的类型系统**: 静态强类型，支持泛型和多态
- **Rust 级别的安全性**: 所有权系统和内存安全保证
- **高性能**: C/C++ 级别的性能
- **并发最佳**: 异步/await + 轻量级线程
- **跨平台**: 支持 Windows, macOS, Linux, WebAssembly
- **AI 辅助**: 内置代码补全、错误修复等 AI 功能
- **DSL 支持**: 内置 SQL、正则表达式等领域特定语言

## 项目结构

```
/workspace
├── crates/
│   ├── compiler/    # 编译器主模块
│   ├── lexer/       # 词法分析器
│   ├── parser/      # 语法分析器
│   ├── hir/         # 高级中间表示
│   ├── mir/         # 中级中间表示
│   ├── codegen/     # 代码生成
│   ├── runtime/     # 运行时库
│   ├── stdlib/      # 标准库
│   ├── ai/          # AI 辅助功能
│   ├── dsl/         # DSL 支持
│   └── cli/         # 命令行工具
├── examples/        # 示例代码
└── docs/            # 文档
```

## 快速开始

### 构建

```bash
cargo build
```

### 运行示例

```bash
cargo run --bin mylang compile examples/hello.mylang
```

## 语法示例

```python
fn add(a: i32, b: i32) -&gt; i32:
    return a + b

fn main():
    let result = add(1, 2)
    println("结果: {}", result)

class Person:
    let name: String
    let age: i32
    
    fn new(name: String, age: i32) -&gt; Person:
        let p = Person()
        p.name = name
        p.age = age
        return p

# 内置 SQL DSL
let users = sql! {
    SELECT id, name 
    FROM users 
    WHERE age &gt; 18
}

# 内置正则表达式 DSL
let phone = regex! { r"\d{3}-\d{3}-\d{4}" }
```

## 文档

- [PRD 文档](./PRD.md)
- [技术架构文档](./ARCHITECTURE.md)

## 许可证

MIT License

