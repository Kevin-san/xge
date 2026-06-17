# Sprint 19 · 验收测试计划

> 上游 sprint: [Sprint 19](../sprint-19-render-pipeline.md)

## 1. 单元测试清单

| 模块 | 测试数 | 覆盖 |
|------|--------|------|
| RenderGraph | 30+ | 拓扑排序 / 资源依赖 / 循环检测 |
| CommandList | 25+ | Draw/Dispatch/Barrier/Blit |
| GPU Device | 20+ | 资源创建/释放/重置 |
| ClusterBuffer | 20+ | 构建 / 光源归属 / 边界 |
| PbrMaterial | 20+ | 默认 / 错误 / 切换 |
| CSM | 25+ | Splits / 拟合 / 稳定化 |
| HdrTarget | 15+ | Resize / 格式 |
| FXAA/TAA | 15+ | 抖动 / 历史 |

**总计：** 170+ 单元测试

## 2. 关键测试用例

### 2.1 RenderGraph 拓扑排序

```rust
#[test]
fn test_graph_topology() {
    let mut graph = RenderGraph::new();
    let p1 = graph.add_pass("p1", Box::new(PipelineA));
    let p2 = graph.add_pass("p2", Box::new(PipelineB));
    let p3 = graph.add_pass("p3", Box::new(PipelineC));
    
    let tex = graph.create_texture("tex", 1920, 1080);
    graph.connect(p1, ResourceSlot::Output(0), tex);
    graph.connect(p2, ResourceSlot::Input(0), tex);
    graph.connect(p3, ResourceSlot::Input(0), tex);
    
    graph.compile();
    // p1 必须先于 p2, p3
    let order = graph.execution_order();
    assert!(order[0] == p1);
}
```

### 2.2 PBR BRDF 精度

```rust
#[test]
fn test_pbr_consistency() {
    // 与参考实现（参考 Unity 标准 PBR）对比
    let result = pbr_evaluate(albedo, metallic, roughness, normal, view, light);
    let reference = reference_pbr(/* ... */);
    assert!((result - reference).length() < 0.05);
}
```

### 2.3 CSM 稳定化

```rust
#[test]
fn test_csm_stabilization() {
    let mut csm = CascadedShadowMap::new();
    csm.compute_splits(&camera1);
    let proj1 = csm.cascades[0].view_proj;
    
    // 移动相机 1 texel
    let mut camera2 = camera1.clone();
    camera2.position += vec3(0.01, 0, 0);  // < 1 texel
    csm.compute_splits(&camera2);
    let proj2 = csm.cascades[0].view_proj;
    
    // 投影矩阵的平移分量应该按 texel 对齐，所以几乎不变
    let diff = (proj1.cols[3] - proj2.cols[3]).length();
    assert!(diff < 0.001, "Shadow swimming: {}", diff);
}
```

### 2.4 Forward+ Cluster 边界

```rust
#[test]
fn test_cluster_boundary() {
    let mut buffer = ClusterBuffer::new();
    buffer.grid = ClusterGrid { screen_x: 16, screen_y: 16, depth_z: 32, near: 0.1, far: 1000.0 };
    buffer.build(&frustum, &lights, &depth_pyramid);
    
    // 边界 cluster 不应丢失光源
    let center_cluster = buffer.get(8, 8, 16);
    let boundary_cluster = buffer.get(15, 15, 31);
    assert!(!center_cluster.light_indices.is_empty() || !boundary_cluster.light_indices.is_empty());
}
```

## 3. 性能基准

| 基准 | 目标 |
|------|------|
| RenderGraph 编译（10 pass） | < 5 ms |
| CommandList 1000 draw | < 5 ms CPU |
| Cluster 构建（1024 光源） | < 1 ms |
| PBR shader 编译 | < 50 ms |
| CSM 4 级联更新 | < 1 ms |
| FXAA 1080p | < 0.5 ms GPU |
| TAA 1080p | < 1 ms GPU |

## 4. 视觉对比

- [ ] 与 Unity URP / UE Forward 截图对比
- [ ] BRDF 渲染球体与参考实现差异 < 5%
- [ ] 阴影 PBR 截图无明显瑕疵
- [ ] HDR 室内外场景色调自然

## 5. 跨后端

- [ ] OpenGL 4.5 后端
- [ ] Vulkan 1.3 后端（可选）
- [ ] WebGPU 后端（可选）
- [ ] CI 矩阵编译通过
