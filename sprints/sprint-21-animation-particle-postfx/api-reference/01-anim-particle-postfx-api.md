# Sprint 21 · API 参考

> 上游 sprint: [Sprint 21](../sprint-21-animation-particle-postfx.md)

## Animation

```rust
// engine-anim
pub use skeleton::{Skeleton, Bone};
pub use clip::{AnimationClip, AnimationTrack, Keyframe, KeyValue};
pub use curve::{Curve, CurveKey, Interpolation, Extrapolation};
pub use state_machine::{StateMachine, State, Transition, Condition};
pub use blend_tree::{BlendTree, BlendNode};
pub use ik::{FabrikSolver, FabrikChain, CcdSolver, TwoBoneIk, JointConstraint};
pub use skinning::{Skinning, SkinningMode, LinearSkinning, DualQuatSkinning};
```

## Particle

```rust
// engine-particle
pub use system::{ParticleSystem, Particle};
pub use emitter::{Emitter, EmitterShape, EmitterRate, Range};
pub use module::{ParticleModule, ColorOverLife, SizeOverLife, ForceField, Gravity, Wind, Vortex, Turbulence};
pub use force::ForceField;
pub use render::ParticleRenderMode;
```

## PostFX

```rust
// engine-postfx
pub use bloom::BloomPass;
pub use dof::DofPass;
pub use ssao::{SsaoPass, SsaoAlgorithm};
pub use ssr::{SsrPass, HiZPyramid};
pub use color_grading::{ColorGrading, Lut};
```

## 关键 API

```rust
impl Skeleton {
    pub fn world_transform(&self, bone_index: usize) -> Mat4;
    pub fn set_local_pose(&mut self, bone_index: usize, transform: Transform);
}

impl AnimationClip {
    pub fn sample(&self, time: f32) -> Pose;
    pub fn duration(&self) -> f32;
}

impl StateMachine {
    pub fn update(&mut self, dt: f32) -> Option<Pose>;
    pub fn set_parameter(&mut self, name: &str, value: f32);
    pub fn fire_trigger(&mut self, name: &str);
}

impl ParticleSystem {
    pub fn play(&mut self);
    pub fn stop(&mut self);
    pub fn emit(&mut self, count: u32);
    pub fn simulate(&mut self, dt: f32, ctx: &mut RenderContext);
}

impl BloomPass {
    pub fn apply(&self, ctx: &mut RenderContext, input: &GpuTexture, output: &mut GpuTexture);
}
```
