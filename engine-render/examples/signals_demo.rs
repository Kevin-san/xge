//! signals_demo.rs - 信号系统演示
//!
//! 本示例演示 Signal（信号）系统的设计用法。
//! 注意：这是一个 API 设计演示，Signal 系统尚未实现。

fn main() {
    println!("=== Signals Demo (API Design) ===");
    println!();

    // Signal API 基于 sprint-04 文档设计

    // 1. Signal 创建
    println!("1. Signal creation:");
    println!("   let signal = Signal::new(\"clicked\");");
    println!("   // 创建一个名为 \"clicked\" 的信号");
    println!();

    // 2. HandlerId
    println!("2. HandlerId (for disconnection):");
    println!("   struct HandlerId(u64);");
    println!("   // 每个连接的 handler 都有一个唯一 ID");
    println!();

    // 3. connect 方法
    println!("3. Connecting handlers:");
    println!("   let handler_id = signal.connect(|args| {{");
    println!("       println!(\"Signal received with {{}} args\", args.len());");
    println!("   }});");
    println!();

    // 4. disconnect 方法
    println!("4. Disconnecting handlers:");
    println!("   signal.disconnect(handler_id);");
    println!("   // 使用保存的 HandlerId 断开连接");
    println!();

    // 5. emit 方法
    println!("5. Emitting signals:");
    println!("   signal.emit(&[&any1, &any2]);");
    println!("   // 派发信号，所有已连接的 handler 都会被调用");
    println!();

    // 6. NodeSignal 概念
    println!("6. NodeSignal (integrated with Node):");
    println!("   // Node trait 扩展方法:");
    println!("   - node.connect(signal_name, handler) -> HandlerId");
    println!("   - node.emit(signal_name, args...)");
    println!("   - node.get_signal(signal_name) -> &Signal");
    println!("   - node.signal_mut(signal_name) -> &mut Signal");
    println!();

    // 7. 模拟信号派发
    println!("7. Simulated signal flow:");

    let mut handler_called = 0;
    let mut last_arg_value = 0;

    // 模拟信号连接
    println!("   // 模拟: 连接一个 handler");
    println!("   let handler = || {{ handler_called += 1; }};");

    // 模拟多次 emit
    println!();
    println!("   Simulating 3 emits:");
    for i in 1..=3 {
        println!("   - Emit #{}: calling handler", i);
        handler_called += 1;
        last_arg_value = i;
    }
    println!("   - Total handler calls: {}", handler_called);
    println!();

    // 8. 多个 Handler
    println!("8. Multiple handlers per signal:");
    println!("   let signal = Signal::new(\"event\");");
    println!();
    println!("   let id1 = signal.connect(handler1);");
    println!("   let id2 = signal.connect(handler2);");
    println!("   let id3 = signal.connect(handler3);");
    println!();
    println!("   signal.emit(&[]);  // 所有 3 个 handler 都被调用");
    println!();

    // 9. 断开连接示例
    println!("9. Disconnection example:");
    println!("   let signal = Signal::new(\"click\");");
    println!();
    println!("   let id1 = signal.connect(handler_a);");
    println!("   let id2 = signal.connect(handler_b);");
    println!("   let id3 = signal.connect(handler_c);");
    println!();
    println!("   signal.emit();  // A, B, C 都被调用");
    println!();
    println!("   signal.disconnect(id2);  // 断开 B");
    println!();
    println!("   signal.emit();  // 只有 A, C 被调用");
    println!();

    // 10. 实际使用场景
    println!("10. Practical usage scenarios:");
    println!();
    println!("    // 按钮点击信号");
    println!("    button.connect(\"clicked\", |args| {{");
    println!("        println!(\"Button {{}} clicked!\", args[0]);");
    println!("    }});");
    println!();
    println!("    // 碰撞信号");
    println!("    player.connect(\"hit\", |args| {{");
    println!("        let damage: i32 = args[0].downcast_ref().unwrap();");
    println!("        player.take_damage(damage);");
    println!("    }});");
    println!();
    println!("    // 自定义事件");
    println!("    emitter.connect(\"game_over\", |_| {{");
    println!("        show_game_over_screen();");
    println!("    }});");
    println!();
    println!("    emitter.emit(\"game_over\");");
    println!();

    // 11. Any 类型参数
    println!("11. Signal arguments (using Any):");
    println!("    // Signal::emit takes &[&dyn Any]");
    println!("    // 可传递任意类型的数据");
    println!();
    println!("    let value = 42i32;");
    println!("    signal.emit(&[&value]);");
    println!();
    println!("    // Handler 中可以 downcast 获取原始值");
    println!("    handler.connect(|args| {{");
    println!("        if let Some(num) = args[0].downcast_ref::<i32>() {{");
    println!("            println!(\"Got value: {{}}\", num);");
    println!("        }}");
    println!("    }});");
    println!();

    // 12. 性能考虑
    println!("12. Performance considerations:");
    println!("    - Signal dispatch is O(n) where n = number of handlers");
    println!("    - Handlers are called in connection order");
    println!("    - Disconnect is O(1) with stored HandlerId");
    println!("    - Consider using direct method calls for performance-critical paths");
    println!();

    // 13. 常见模式
    println!("13. Common patterns:");
    println!();
    println!("    // One-shot handler (disconnects after first emit)");
    println!("    let id = signal.connect(handler);");
    println!("    // ... later ...");
    println!("    signal.disconnect(id);");
    println!();
    println!("    // Scoped signals (dropped with owner)");
    println!("    struct MyNode {{");
    println!("        signal: Signal,");
    println!("    }}");
    println!();

    println!("Signals demo completed (API design demonstration)!");
}
