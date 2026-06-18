# Module 02 — AnimationClip / Curve

> 上游 sprint: [Sprint 21](../sprint-21-animation-particle-postfx.md)
> 文件位置: `engine-anim/src/clip.rs`, `engine-anim/src/curve.rs`

## 1. AnimationClip

```rust
pub struct AnimationClip {
    pub name: String,
    pub duration: f32,                // 秒
    pub fps: f32,
    pub tracks: Vec<AnimationTrack>,
    pub root_motion: Option<RootMotion>,
}

pub struct AnimationTrack {
    pub bone_name: String,
    pub property: BoneProperty,       // Position / Rotation / Scale
    pub keys: Vec<Keyframe>,
}

pub enum BoneProperty {
    Position,
    Rotation,
    Scale,
}

pub struct Keyframe {
    pub time: f32,
    pub value: KeyValue,
}

pub enum KeyValue {
    Vec3(Vec3),
    Quat(Quat),
    Float(f32),
}
```

## 2. Curve（关键帧插值）

```rust
pub struct Curve<T: Lerp> {
    pub keys: Vec<CurveKey<T>>,
    pub interpolation: Interpolation,
    pub extrapolation: Extrapolation,
}

pub struct CurveKey<T> {
    pub time: f32,
    pub value: T,
    pub in_tangent: Option<T>,    // 仅 float
    pub out_tangent: Option<T>,
}

pub enum Interpolation {
    Linear,
    Step,
    Cubic,   // 三次贝塞尔
    Hermite,
}

pub enum Extrapolation {
    Constant,    // 超出范围保持端点
    Cycle,       // 循环
    PingPong,    // 来回
}

impl<T: Lerp + Copy> Curve<T> {
    pub fn sample(&self, time: f32) -> T {
        if self.keys.is_empty() { panic!(); }
        if time <= self.keys[0].time { return self.keys[0].value; }
        if time >= self.keys.last().unwrap().time { return self.keys.last().unwrap().value; }
        
        // 二分查找
        let (i0, i1) = self.find_segment(time);
        let k0 = &self.keys[i0];
        let k1 = &self.keys[i1];
        let t = (time - k0.time) / (k1.time - k0.time);
        
        match self.interpolation {
            Interpolation::Linear => T::lerp(k0.value, k1.value, t),
            Interpolation::Step => k0.value,
            Interpolation::Cubic => {
                // 三次贝塞尔
                let p1 = k0.value.lerp(k0.out_tangent.unwrap(), 1.0 / 3.0);
                let p2 = k1.value.lerp(k1.in_tangent.unwrap(), 1.0 / 3.0);
                // de Casteljau
                let a = p0.lerp(p1, t);
                let b = p1.lerp(p2, t);
                let c = p2.lerp(p3, t);
                let d = a.lerp(b, t);
                let e = b.lerp(c, t);
                d.lerp(e, t)
            }
            Interpolation::Hermite => {
                // 4 点 Hermite
                T::hermite(p0, p1, p2, p3, t)
            }
        }
    }
}
```

## 3. Pose Sampling

```rust
pub struct Pose {
    pub local_transforms: Vec<Transform>,
    pub duration: f32,
}

pub struct AnimationClip {
    pub fn sample(&self, time: f32) -> Pose {
        let mut pose = Pose {
            local_transforms: vec![Transform::IDENTITY; self.skeleton.bones.len()],
            duration: self.duration,
        };
        
        for track in &self.tracks {
            let bone_index = self.skeleton.find_bone(&track.bone_name).unwrap();
            let value = self.sample_track(track, time);
            match track.property {
                BoneProperty::Position => pose.local_transforms[bone_index].position = value.as_vec3(),
                BoneProperty::Rotation => pose.local_transforms[bone_index].rotation = value.as_quat(),
                BoneProperty::Scale => pose.local_transforms[bone_index].scale = value.as_vec3(),
            }
        }
        pose
    }
}
```

## 4. 验收

- [ ] 100 骨骼 60 FPS 关键帧采样 < 0.1 ms CPU
- [ ] 曲线插值：贝塞尔与 Unity 动画曲线对比一致
- [ ] 动画导入 GLTF / FBX
- [ ] Hermite 4 点精度
- [ ] Wrap / ping-pong / clamp 行为
