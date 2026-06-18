# Module 04 — IK 系统

> 上游 sprint: [Sprint 21](../sprint-21-animation-particle-postfx.md)
> 文件位置: `engine-anim/src/ik/`

## 1. FABRIK（Forward-And-Backward-Reaching IK）

```rust
pub struct FabrikSolver {
    pub max_iterations: u32,    // 默认 10
    pub tolerance: f32,         // 默认 0.01
}

pub struct FabrikChain {
    pub joints: Vec<Entity>,    // 关节实体
    pub lengths: Vec<f32>,      // 段长度
    pub pole_vector: Option<Vec3>,  // 朝向约束
    pub constraints: Vec<JointConstraint>,
}

pub struct JointConstraint {
    pub min_angle: f32,  // 弧度
    pub max_angle: f32,
    pub axis: Vec3,      // 旋转轴（局部）
}

impl FabrikSolver {
    pub fn solve(
        &self,
        chain: &mut FabrikChain,
        target: Vec3,
        global_transforms: &mut Vec<Mat4>,
    ) -> bool {
        let n = chain.joints.len();
        let total_length: f32 = chain.lengths.iter().sum();
        
        // 目标不可达
        let root_pos = extract_translation(&global_transforms[chain.joints[0].index()]);
        if (target - root_pos).length() > total_length {
            // 沿直线伸展
            for i in 0..n-1 {
                let r = (target - root_pos).normalize();
                let new_pos = root_pos + r * chain.lengths[0..i+1].iter().sum::<f32>();
                set_translation(&mut global_transforms[chain.joints[i].index()], new_pos);
            }
            return false;
        }
        
        for iter in 0..self.max_iterations {
            // Backward：从末端向根
            set_translation(&mut global_transforms[chain.joints[n-1].index()], target);
            for i in (0..n-1).rev() {
                let next = extract_translation(&global_transforms[chain.joints[i+1].index()]);
                let curr = extract_translation(&global_transforms[chain.joints[i].index()]);
                let dir = (curr - next).normalize();
                let new_pos = next + dir * chain.lengths[i];
                set_translation(&mut global_transforms[chain.joints[i].index()], new_pos);
                // 应用关节限制
                self.apply_constraint(chain, i, &mut global_transforms);
            }
            
            // Forward：从根到末端
            set_translation(&mut global_transforms[chain.joints[0].index()], root_pos);
            for i in 0..n-1 {
                let prev = extract_translation(&global_transforms[chain.joints[i].index()]);
                let next = extract_translation(&global_transforms[chain.joints[i+1].index()]);
                let dir = (next - prev).normalize();
                let new_pos = prev + dir * chain.lengths[i];
                set_translation(&mut global_transforms[chain.joints[i+1].index()], new_pos);
            }
            
            // 检查收敛
            let end = extract_translation(&global_transforms[chain.joints[n-1].index()]);
            if (end - target).length() < self.tolerance {
                return true;
            }
        }
        false
    }
}
```

## 2. Two-Bone IK（解析解）

```rust
pub struct TwoBoneIk {
    pub upper: Entity,
    pub lower: Entity,
    pub end: Entity,
}

impl TwoBoneIk {
    pub fn solve(&self, target: Vec3, pole: Vec3, global_transforms: &mut Vec<Mat4>) {
        let a = extract_translation(&global_transforms[self.upper.index()]);
        let b = extract_translation(&global_transforms[self.lower.index()]);
        let c = extract_translation(&global_transforms[self.end.index()]);
        
        let upper_len = (b - a).length();
        let lower_len = (c - b).length();
        let target_len = (target - a).length().min(upper_len + lower_len - 0.001);
        
        // 1. 计算上臂与 target 的夹角（余弦定理）
        let ab = (b - a).normalize();
        let at = (target - a).normalize();
        let ac = ab.dot(at).clamp(-1.0, 1.0);
        let acos_ab = ac.acos();
        
        // 2. 计算上臂角度（lower 与 upper 之间）
        let bc = (target_len.powi(2) - upper_len.powi(2) - lower_len.powi(2)) / (2.0 * upper_len * lower_len);
        let angle_lower = bc.clamp(-1.0, 1.0).acos();
        
        // 3. 朝向
        let bend_dir = (pole - a).reject_from(ab).normalize();
        
        // 4. 应用旋转
        let upper_quat = Quat::from_axis_angle(bend_dir, acos_ab) * extract_rotation(&global_transforms[self.upper.index()]);
        let lower_quat = Quat::from_axis_angle(bend_dir.cross(ab), std::f32::consts::PI - angle_lower) * upper_quat;
        
        set_rotation(&mut global_transforms[self.upper.index()], upper_quat);
        set_rotation(&mut global_transforms[self.lower.index()], lower_quat);
    }
}
```

## 3. CCD（Cyclic Coordinate Descent）

```rust
pub struct CcdSolver {
    pub max_iterations: u32,
    pub tolerance: f32,
}

impl CcdSolver {
    pub fn solve(&self, chain: &mut FabrikChain, target: Vec3, global_transforms: &mut Vec<Mat4>) -> bool {
        for _ in 0..self.max_iterations {
            for i in (0..chain.joints.len() - 1).rev() {
                let joint_pos = extract_translation(&global_transforms[chain.joints[i].index()]);
                let end_pos = extract_translation(&global_transforms[chain.joints.last().unwrap().index()]);
                
                let to_end = (end_pos - joint_pos).normalize();
                let to_target = (target - joint_pos).normalize();
                let rotation = Quat::from_rotation_to(to_end, to_target);
                
                // 应用旋转
                let current_rot = extract_rotation(&global_transforms[chain.joints[i].index()]);
                set_rotation(&mut global_transforms[chain.joints[i].index()], rotation * current_rot);
                // 更新后续关节位置
                self.update_chain_forward(chain, i, global_transforms);
            }
            
            let end = extract_translation(&global_transforms[chain.joints.last().unwrap().index()]);
            if (end - target).length() < self.tolerance { return true; }
        }
        false
    }
}
```

## 4. Look-At

```rust
pub fn look_at(
    global_transform: &mut Mat4,
    target: Vec3,
    up: Vec3,
) {
    let pos = extract_translation(global_transform);
    let forward = (target - pos).normalize();
    let right = forward.cross(up).normalize();
    let up_corrected = right.cross(forward).normalize();
    
    *global_transform = Mat4::from_cols(
        right.extend(0.0),
        up_corrected.extend(0.0),
        forward.extend(0.0),
        pos.extend(1.0),
    );
}
```

## 5. 验收

- [ ] FABRIK 30 关节链 5 迭代 < 0.2 ms
- [ ] Two-bone IK 解析解 < 1 µs
- [ ] 关节限制 + IK 兼容
- [ ] 收敛容差 0.01 m
- [ ] 测试：手臂抓取物体
- [ ] 测试：脚部踩地（look-at）
