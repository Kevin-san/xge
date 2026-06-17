# Sprint 21 · 验收测试计划

> 上游 sprint: [Sprint 21](../sprint-21-animation-particle-postfx.md)

## 1. 单元测试清单

| 模块 | 测试数 | 覆盖 |
|------|--------|------|
| Skeleton | 20+ | 父子 / World Transform |
| Skinned Mesh | 15+ | LBS / DLB |
| AnimationClip | 30+ | 4 种插值 / 时间 wrap |
| Curve | 25+ | Hermite / 贝塞尔 |
| StateMachine | 30+ | 转换 / Trigger / Blend |
| BlendTree | 20+ | 1D / 2D |
| FABRIK | 20+ | 收敛 / 关节限制 |
| Two-bone IK | 10+ | 解析解 |
| Particle | 40+ | 发射 / 力场 / 模块 |
| PostFX | 30+ | Bloom / SSAO / SSR |

**总计：** 250+ 单元测试

## 2. 关键测试

### 2.1 DLB 蒙皮 vs LBS

```rust
#[test]
fn test_dlb_vs_lbs() {
    // 测试极端权重（单骨骼权重 = 1.0）
    let weights = vec![
        BoneWeight { bone_index: 0, weight: 1.0 },
        BoneWeight { bone_index: 1, weight: 0.0 },
        BoneWeight { bone_index: 2, weight: 0.0 },
        BoneWeight { bone_index: 3, weight: 0.0 },
    ];
    
    // DLB 应退化为 LBS
    let pos = Vec3::new(1.0, 0.0, 0.0);
    let dlb_result = dlb_skin(pos, &weights, &bone_dqs);
    let lbs_result = lbs_skin(pos, &weights, &bone_matrices);
    
    let diff = (dlb_result - lbs_result).length();
    assert!(diff < 1e-4);
}
```

### 2.2 State Machine Transition

```rust
#[test]
fn test_state_machine_transition() {
    let mut sm = StateMachine::new();
    sm.add_state("idle", clip_idle);
    sm.add_state("run", clip_run);
    sm.add_transition("idle", "run", Condition::Greater("speed", 0.1), 0.2);
    
    sm.set_parameter("speed", 0.0);
    sm.update(0.016);
    assert_eq!(sm.current_state_name(), "idle");
    
    sm.set_parameter("speed", 1.0);
    sm.update(0.016);
    assert_eq!(sm.current_state_name(), "run");
}
```

### 2.3 FABRIK 收敛

```rust
#[test]
fn test_fabrik_convergence() {
    let mut chain = FabrikChain::new(/* 30 关节 */);
    let target = Vec3::new(5.0, 0.0, 0.0);
    let solver = FabrikSolver { max_iterations: 10, tolerance: 0.01 };
    
    let mut transforms = /* ... */;
    let success = solver.solve(&mut chain, target, &mut transforms);
    
    let end_pos = extract_translation(&transforms[chain.joints.last().unwrap().index()]);
    assert!(success);
    assert!((end_pos - target).length() < 0.01);
}
```

### 2.4 Particle 模块组合

```rust
#[test]
fn test_particle_modules() {
    let mut system = ParticleSystem::new();
    system.add_module(Box::new(Gravity { acceleration: Vec3::new(0.0, -9.8, 0.0) }));
    system.add_module(Box::new(Wind { direction: Vec3::X, strength: 1.0 }));
    system.add_module(Box::new(ColorOverLife::new(red_to_blue)));
    system.add_module(Box::new(SizeOverLife::new(start_to_end)));
    
    system.emit(1000);
    for _ in 0..60 {
        system.simulate(1.0 / 60.0);
    }
    
    // 验证粒子位置 / 颜色 / 大小
}
```

## 3. 性能基准

| 基准 | 目标 |
|------|------|
| 100 骨骼蒙皮 | < 0.5 ms GPU |
| 100 骨骼 60 FPS 采样 | < 0.1 ms CPU |
| 100 状态切换 | < 1 ms |
| FABRIK 30 关节 5 iter | < 0.2 ms |
| 10000 GPU 粒子 | 60 FPS |
| Bloom 5 级 | < 0.5 ms |
| SSAO 16 采样 | < 0.5 ms |
| SSR 64 步 | < 1 ms |
| LUT 256³ | < 0.1 ms |

## 4. 视觉

- [ ] 角色动画：idle / walk / run 混合
- [ ] DLB 球套球动画无 candy-wrapper
- [ ] FABRIK 手臂抓取
- [ ] 粒子：火焰 / 烟雾 / 爆炸
- [ ] Bloom：HDR 高光溢色
- [ ] SSAO：墙角 / 缝隙 AO
- [ ] SSR：金属反射
