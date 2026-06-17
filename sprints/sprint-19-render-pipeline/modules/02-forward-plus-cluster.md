# Module 02 — Forward+ 集群光照

> 上游 sprint: [Sprint 19](../sprint-19-render-pipeline.md)

## 1. 目标

**Forward+ 集群光照**：
- 视锥分割为 3D 网格 Cluster
- CPU 端分配光源到 Cluster
- GPU 着色器按 Cluster 评估光照

## 2. Cluster Grid

```rust
pub struct ClusterGrid {
    pub screen_x: u32,  // 默认 16
    pub screen_y: u32,  // 默认 16
    pub depth_z: u32,   // 默认 32
    pub near: f32,
    pub far: f32,
}

pub struct Cluster {
    pub min: Vec3,        // 世界空间 AABB
    pub max: Vec3,
    pub light_indices: Vec<u32>,
}

pub struct ClusterBuffer {
    pub clusters: Vec<Cluster>,
    pub light_indices: Vec<u32>,
    pub grid: ClusterGrid,
}

impl ClusterBuffer {
    pub fn build(
        &mut self,
        frustum: &Frustum,
        lights: &[Light],
        depth_pyramid: &DepthPyramid,
    );
}
```

## 3. Light Culling

```rust
pub fn assign_lights_to_cluster(
    cluster: &Cluster,
    lights: &[Light],
) -> Vec<u32> {
    let mut result = Vec::new();
    for (i, light) in lights.iter().enumerate() {
        if light_aabb_intersects_cluster(light, cluster) {
            result.push(i as u32);
        }
    }
    result
}
```

## 4. 光源 AABB

```rust
pub fn light_aabb(light: &Light) -> AABB {
    match light {
        Light::Directional(_) => AABB::infinite(),
        Light::Point(p) => {
            let r = p.radius;
            AABB::from_center_half_extents(p.position, Vec3::splat(r))
        }
        Light::Spot(s) => {
            // 锥体 AABB
            // 简化：bounding sphere
            let r = s.range;
            AABB::from_center_half_extents(s.position, Vec3::splat(r))
        }
    }
}
```

## 5. GPU Shader

```glsl
// fragment shader
layout(set = 0, binding = 0) uniform Globals {
    mat4 view;
    mat4 projection;
    vec3 camera_pos;
    float z_near;
    float z_far;
    uvec3 cluster_grid;
};

layout(set = 0, binding = 1) buffer ClusterBuffer {
    uint light_indices[];
};

layout(set = 0, binding = 2) buffer Lights {
    LightData lights[];
};

uint get_cluster_index(vec3 frag_pos) {
    vec2 screen = gl_FragCoord.xy;
    uvec3 idx;
    idx.x = uint(screen.x * cluster_grid.x / screen_width);
    idx.y = uint(screen.y * cluster_grid.y / screen_height);
    // depth 分段：对数分布
    float linear_depth = length(frag_pos - camera_pos);
    float z = (log(linear_depth / z_near) / log(z_far / z_near));
    idx.z = uint(z * cluster_grid.z);
    return idx.x + idx.y * cluster_grid.x + idx.z * cluster_grid.x * cluster_grid.y;
}
```

## 6. 验收

- [ ] 1024 光源（512 动态） 60 FPS @ 1080p
- [ ] Cluster 16×16×32 = 8192，光源归属 < 1 ms CPU
- [ ] GPU cluster 评估 < 0.5 ms
- [ ] 与 Forward 单光源对比 100x speedup
- [ ] 集成到 PBR shader

## 7. 性能

| 操作 | 目标 |
|------|------|
| CPU cluster 构建 (1024 光源) | < 1 ms |
| GPU cluster lookup | < 5 ns |
| GPU light 评估（每像素） | < 0.5 ms (1024 候选) |
