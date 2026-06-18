# Module 01 — 骨骼与 Skinned Mesh

> 上游 sprint: [Sprint 21](../sprint-21-animation-particle-postfx.md)
> 文件位置: `engine-anim/src/skeleton.rs`, `engine-anim/src/skinning/`

## 1. Skeleton

```rust
pub struct Skeleton {
    pub bones: Vec<Bone>,
    pub inverse_bind_poses: Vec<Mat4>,
    pub root_index: usize,
}

pub struct Bone {
    pub name: String,
    pub parent_index: Option<u16>,
    pub local_transform: Transform,  // 相对于父级
}

impl Skeleton {
    pub fn new() -> Self;
    pub fn from_bones(bones: Vec<Bone>) -> Self;
    pub fn bone_count(&self) -> usize;
    pub fn find_bone(&self, name: &str) -> Option<usize>;
    pub fn world_transform(&self, bone_index: usize) -> Mat4;
    pub fn set_local_pose(&mut self, bone_index: usize, transform: Transform);
    
    /// 累积父级变换
    fn compute_world_transforms(&self) -> Vec<Mat4> {
        let mut world = vec![Mat4::IDENTITY; self.bones.len()];
        for i in 0..self.bones.len() {
            if let Some(parent) = self.bones[i].parent_index {
                world[i] = world[parent as usize] * self.bones[i].local_transform.to_matrix();
            } else {
                world[i] = self.bones[i].local_transform.to_matrix();
            }
        }
        world
    }
}
```

## 2. Skinning Modes

### 2.1 LBS（Linear Blend Skinning）

```rust
pub struct LinearSkinning {
    pub bones_per_vertex: u32,  // 通常 4
}

impl Skinning for LinearSkinning {
    fn skin_vertex(&self, pos: Vec3, weights: &[BoneWeight], bone_matrices: &[Mat4]) -> Vec3 {
        let mut result = Vec3::ZERO;
        let mut total_weight = 0.0;
        for w in weights {
            if w.weight > 0.0 {
                let transformed = bone_matrices[w.bone_index].transform_point(pos);
                result += transformed * w.weight;
                total_weight += w.weight;
            }
        }
        if total_weight > 0.0 {
            result / total_weight
        } else {
            pos
        }
    }
}
```

### 2.2 DLB（Dual Quat Linear Blending）

```rust
pub struct DualQuatSkinning;

impl Skinning for DualQuatSkinning {
    fn skin_vertex(&self, pos: Vec3, weights: &[BoneWeight], bone_dqs: &[DualQuat]) -> Vec3 {
        // 累加对偶四元数
        let mut accum = DualQuat::ZERO;
        for w in weights {
            if w.weight > 0.0 {
                // sign flip 避免长路径
                let dq = if accum.real.dot(bone_dqs[w.bone_index].real) < 0.0 {
                    -bone_dqs[w.bone_index] * w.weight
                } else {
                    bone_dqs[w.bone_index] * w.weight
                };
                accum = accum + dq;
            }
        }
        // 归一化
        let norm = accum.real.length();
        if norm > 0.0 {
            accum = accum * (1.0 / norm);
        }
        
        // 变换点
        accum.transform_point(pos)
    }
}
```

## 3. Skinned Mesh

```rust
pub struct SkinnedMesh {
    pub mesh: Mesh3D,                            // 静态网格
    pub skeleton: Skeleton,
    pub vertex_weights: Vec<[BoneWeight; 4]>,   // 4 骨骼 / 顶点
    pub skinning_mode: SkinningMode,
    pub bone_matrices: Vec<Mat4>,               // 蒙皮后矩阵缓冲
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkinningMode {
    Linear,              // LBS
    DualQuaternion,      // DLB
}

pub struct BoneWeight {
    pub bone_index: u32,
    pub weight: f32,
}
```

## 4. GPU 蒙皮（Compute Shader）

```rust
pub fn gpu_skin(
    device: &GpuDevice,
    input_mesh: &Mesh3D,
    output_mesh: &mut Mesh3D,
    bone_matrices: &GpuBuffer,
    skinning_mode: SkinningMode,
) {
    // 1. 上传 input 顶点 + 权重 + bone_matrices
    // 2. 调度 compute shader
    // 3. 读取结果到 output_mesh
    
    let shader = match skinning_mode {
        SkinningMode::Linear => &SKIN_LBS_SHADER,
        SkinningMode::DualQuaternion => &SKIN_DLB_SHADER,
    };
    
    device.dispatch(shader, (vertex_count / 64 + 1, 1, 1));
}
```

## 5. 验收

- [ ] 100 骨骼蒙皮 < 0.5 ms GPU
- [ ] DLB vs LBS 视觉差异 < 1%（无 candy-wrapper 形变）
- [ ] 蒙皮精度测试：球套球动画
- [ ] GPU 蒙皮 CPU 端验证（结果与 CPU 一致）
- [ ] 骨骼权重 4 限制
- [ ] GLTF 蒙皮数据导入
