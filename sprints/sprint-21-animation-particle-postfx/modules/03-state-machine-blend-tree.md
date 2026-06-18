# Module 03 — 状态机 + 混合树

> 上游 sprint: [Sprint 21](../sprint-21-animation-particle-postfx.md)
> 文件位置: `engine-anim/src/state_machine.rs`, `engine-anim/src/blend_tree.rs`

## 1. 状态机

```rust
pub struct StateMachine {
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
    pub parameters: AnimationParameters,
    pub initial_state: StateId,
    pub current_state: StateId,
    pub previous_state: Option<StateId>,
    pub transition_time: f32,  // 当前过渡时间
    pub active_transition: Option<TransitionId>,
}

pub struct State {
    pub name: String,
    pub clip: Option<AnimationClipHandle>,
    pub speed: f32,
    pub loop_mode: LoopMode,
    pub blend_tree: Option<BlendTree>,
}

pub struct Transition {
    pub from: StateId,
    pub to: StateId,
    pub condition: Condition,
    pub blend_duration: f32,
    pub blend_curve: Curve<f32>,
    pub interruptible: bool,
}

pub enum Condition {
    Bool(String, bool),
    Trigger(String),     // 一次性
    Greater(String, f32),
    Less(String, f32),
    Between(String, f32, f32),
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Not(Box<Condition>),
}

pub struct AnimationParameters {
    pub floats: HashMap<String, f32>,
    pub bools: HashMap<String, bool>,
    pub triggers: HashSet<String>,
}

impl AnimationParameters {
    pub fn set(&mut self, name: &str, value: f32);
    pub fn set_bool(&mut self, name: &str, value: bool);
    pub fn fire_trigger(&mut self, name: &str);
    pub fn consume_trigger(&mut self, name: &str);
}
```

## 2. 状态更新

```rust
impl StateMachine {
    pub fn update(&mut self, dt: f32) -> Option<Pose> {
        // 1. 检查 transition
        if let Some(trans_id) = self.check_transitions() {
            self.start_transition(trans_id);
        }
        
        // 2. 应用过渡
        let pose = if let Some(transition) = self.active_transition {
            self.blend_transition(transition, dt)
        } else {
            self.current_state().evaluate(self.parameters, dt)
        };
        
        Some(pose)
    }
    
    fn check_transitions(&self) -> Option<TransitionId> {
        for (i, t) in self.transitions.iter().enumerate() {
            if t.from == self.current_state && t.condition.evaluate(&self.parameters) {
                return Some(i);
            }
        }
        None
    }
}
```

## 3. BlendTree

```rust
pub struct BlendTree {
    pub root: BlendNode,
    pub parameters: AnimationParameters,
}

pub enum BlendNode {
    Clip(AnimationClipHandle),
    Blend1D { 
        clips: Vec<AnimationClipHandle>,
        parameter: String,
        threshold: Vec<f32>,
    },
    Blend2D {
        clips: Vec<AnimationClipHandle>,
        x_parameter: String,
        y_parameter: String,
        positions: Vec<Vec2>,  // 在 (x, y) 平面上的位置
    },
    Additive {
        base: Box<BlendNode>,
        additive: Box<BlendNode>,
    },
    Slot {
        default: Box<BlendNode>,
    },
}

impl BlendNode {
    pub fn evaluate(&self, params: &AnimationParameters, time: f32) -> Pose {
        match self {
            BlendNode::Clip(c) => c.sample(time),
            BlendNode::Blend1D { clips, parameter, threshold } => {
                let v = params.get_float(parameter);
                // 找两个相邻 threshold
                let (i0, i1, t) = find_blend_indices(threshold, v);
                let p0 = clips[i0].sample(time);
                let p1 = clips[i1].sample(time);
                Pose::lerp(&p0, &p1, t)
            }
            // ...
        }
    }
}
```

## 4. 验收

- [ ] 100 状态拓扑切换 < 1 ms
- [ ] 5 clip blend < 0.1 ms
- [ ] 状态转换视觉无跳变
- [ ] Trigger 一次性消费
- [ ] 2D blend：locomotion 八方向
