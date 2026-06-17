//! Game engine animation system
//!
//! Provides keyframe animation, skeleton, pose, state machine, blend tree and IK.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use engine_math::{Mat4, Quat, Vec2, Vec3};

// ============================================================================
// Constants
// ============================================================================

/// Default tolerance for IK convergence
const IK_TOLERANCE: f32 = 0.001;
/// Default max iterations for IK
const IK_MAX_ITERATIONS: u32 = 10;
/// Small epsilon for float comparisons
const EPSILON: f32 = 1e-6;

// ============================================================================
// Interpolation Types
// ============================================================================

/// Interpolation mode for keyframes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interpolation {
    /// Constant value (no interpolation)
    Step,
    /// Linear interpolation
    Linear,
    /// Cubic spline interpolation with tangents
    CubicSpline,
}

/// Wrap mode for animation playback
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrapMode {
    /// Play once and stop
    Once,
    /// Loop forever
    Loop,
    /// Ping-pong between start and end
    PingPong,
    /// Clamp at the last frame
    ClampForever,
}

/// Blend mode for transitions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendMode {
    /// Linear blend
    Linear,
    /// Additive blend
    Additive,
    /// Crossfade blend
    Crossfade,
}

/// Compare operation for conditions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompareOp {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

// ============================================================================
// Keyframe
// ============================================================================

/// A single keyframe with time, value and interpolation
#[derive(Clone, Debug)]
pub struct Keyframe<T> {
    /// Time in seconds
    time: f32,
    /// The value at this keyframe
    value: T,
    /// Interpolation mode to next keyframe
    interpolation: Interpolation,
    /// In tangent for cubic spline (optional)
    in_tangent: Option<T>,
    /// Out tangent for cubic spline (optional)
    out_tangent: Option<T>,
}

impl<T: Clone> Keyframe<T> {
    /// Create a new keyframe
    pub fn new(time: f32, value: T) -> Self {
        Self {
            time,
            value,
            interpolation: Interpolation::Linear,
            in_tangent: None,
            out_tangent: None,
        }
    }

    /// Create keyframe with specific interpolation
    pub fn with_interpolation(time: f32, value: T, interpolation: Interpolation) -> Self {
        Self {
            time,
            value,
            interpolation,
            in_tangent: None,
            out_tangent: None,
        }
    }

    /// Create cubic spline keyframe with tangents
    pub fn with_tangents(time: f32, value: T, in_tangent: T, out_tangent: T) -> Self {
        Self {
            time,
            value,
            interpolation: Interpolation::CubicSpline,
            in_tangent: Some(in_tangent),
            out_tangent: Some(out_tangent),
        }
    }

    /// Get the time of this keyframe
    pub fn time(&self) -> f32 {
        self.time
    }

    /// Get the value of this keyframe
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get interpolation mode
    pub fn interpolation(&self) -> Interpolation {
        self.interpolation
    }
}

// ============================================================================
// Curve - Interpolation traits
// ============================================================================

/// Trait for types that can be interpolated
pub trait Interpolate: Clone + Sized {
    /// Linear interpolation between self and other at t
    fn lerp(&self, other: &Self, t: f32) -> Self;
    /// Cubic spline interpolation
    fn cubic_spline(
        &self,
        other: &Self,
        t: f32,
        t0: f32,
        t1: f32,
        in_t: &Self,
        out_t: &Self,
    ) -> Self;
}

impl Interpolate for f32 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
    fn cubic_spline(
        &self,
        other: &Self,
        t: f32,
        t0: f32,
        t1: f32,
        in_t: &Self,
        out_t: &Self,
    ) -> Self {
        let dt = t1 - t0;
        let t2 = t * t;
        let t3 = t2 * t;
        let p0 = *self;
        let m0 = *out_t * dt;
        let p1 = *other;
        let m1 = *in_t * dt;
        (2.0 * t3 - 3.0 * t2 + 1.0) * p0
            + (t3 - 2.0 * t2 + t) * m0
            + (-2.0 * t3 + 3.0 * t2) * p1
            + (t3 - t2) * m1
    }
}

impl Interpolate for Vec3 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Vec3::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
            self.z + (other.z - self.z) * t,
        )
    }
    fn cubic_spline(
        &self,
        other: &Self,
        t: f32,
        t0: f32,
        t1: f32,
        in_t: &Self,
        out_t: &Self,
    ) -> Self {
        Vec3::new(
            self.x.cubic_spline(&other.x, t, t0, t1, &in_t.x, &out_t.x),
            self.y.cubic_spline(&other.y, t, t0, t1, &in_t.y, &out_t.y),
            self.z.cubic_spline(&other.z, t, t0, t1, &in_t.z, &out_t.z),
        )
    }
}

impl Interpolate for Quat {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        self.nlerp(*other, t)
    }
    fn cubic_spline(
        &self,
        other: &Self,
        t: f32,
        _t0: f32,
        _t1: f32,
        _in_t: &Self,
        _out_t: &Self,
    ) -> Self {
        // Simplified: use nlerp for quaternion spline approximation
        self.lerp(other, t)
    }
}

// ============================================================================
// Curve
// ============================================================================

/// A curve of keyframes for animation
#[derive(Clone, Debug)]
pub struct Curve<T: Interpolate> {
    keyframes: Vec<Keyframe<T>>,
    interpolation: Interpolation,
    wrap_mode: WrapMode,
}

impl<T: Interpolate> Curve<T> {
    /// Create a new empty curve
    pub fn new() -> Self {
        Self {
            keyframes: vec![],
            interpolation: Interpolation::Linear,
            wrap_mode: WrapMode::Loop,
        }
    }

    /// Create curve with default interpolation
    pub fn with_interpolation(interpolation: Interpolation) -> Self {
        Self {
            keyframes: vec![],
            interpolation,
            wrap_mode: WrapMode::Loop,
        }
    }

    /// Push a keyframe (assumes sorted by time)
    pub fn push(&mut self, keyframe: Keyframe<T>) {
        self.keyframes.push(keyframe);
    }

    /// Insert a keyframe, keeping sorted order
    pub fn insert_sorted(&mut self, keyframe: Keyframe<T>) {
        let time = keyframe.time;
        let pos = self.keyframes.iter().position(|k| k.time > time);
        match pos {
            Some(idx) => self.keyframes.insert(idx, keyframe),
            None => self.keyframes.push(keyframe),
        }
    }

    /// Remove a keyframe by index
    pub fn remove(&mut self, idx: usize) {
        if idx < self.keyframes.len() {
            self.keyframes.remove(idx);
        }
    }

    /// Get number of keyframes
    pub fn len(&self) -> usize {
        self.keyframes.len()
    }

    /// Check if curve is empty
    pub fn is_empty(&self) -> bool {
        self.keyframes.is_empty()
    }

    /// Get keyframes slice
    pub fn keyframes(&self) -> &[Keyframe<T>] {
        &self.keyframes
    }

    /// Get mutable keyframes slice
    pub fn keyframes_mut(&mut self) -> &mut [Keyframe<T>] {
        &mut self.keyframes
    }

    /// Get duration of curve
    pub fn duration(&self) -> f32 {
        if self.keyframes.is_empty() {
            0.0
        } else {
            self.keyframes.last().unwrap().time
        }
    }

    /// Set wrap mode
    pub fn set_wrap_mode(&mut self, mode: WrapMode) {
        self.wrap_mode = mode;
    }

    /// Get wrap mode
    pub fn wrap_mode(&self) -> WrapMode {
        self.wrap_mode
    }

    /// Sample the curve at a given time
    pub fn sample(&self, time: f32) -> T {
        self.sample_with_wrap(time, self.wrap_mode)
    }

    /// Sample with specific wrap mode
    pub fn sample_with_wrap(&self, time: f32, wrap: WrapMode) -> T {
        if self.keyframes.is_empty() {
            return self.default_value();
        }

        let duration = self.duration();
        let wrapped_time = match wrap {
            WrapMode::Once => time.min(duration),
            WrapMode::Loop => {
                if duration > 0.0 {
                    time % duration
                } else {
                    0.0
                }
            }
            WrapMode::PingPong => {
                if duration <= 0.0 {
                    return self.default_value();
                }
                let cycle = (time / duration) as i32;
                let t = time % duration;
                if cycle % 2 == 0 {
                    t
                } else {
                    duration - t
                }
            }
            WrapMode::ClampForever => time.min(duration),
        };

        // Find surrounding keyframes
        if wrapped_time <= self.keyframes[0].time {
            return self.keyframes[0].value.clone();
        }
        if wrapped_time >= self.keyframes.last().unwrap().time {
            return self.keyframes.last().unwrap().value.clone();
        }

        // Binary search for the right keyframe
        let mut lo = 0;
        let mut hi = self.keyframes.len() - 1;
        while lo < hi - 1 {
            let mid = (lo + hi) / 2;
            if self.keyframes[mid].time <= wrapped_time {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        let k0 = &self.keyframes[lo];
        let k1 = &self.keyframes[lo + 1];
        let t0 = k0.time;
        let t1 = k1.time;
        let dt = t1 - t0;
        let t = if dt > 0.0 {
            (wrapped_time - t0) / dt
        } else {
            0.0
        };

        match k0.interpolation {
            Interpolation::Step => k0.value.clone(),
            Interpolation::Linear => k0.value.lerp(&k1.value, t),
            Interpolation::CubicSpline => {
                let in_t = k1.in_tangent.as_ref().unwrap_or(&k1.value);
                let out_t = k0.out_tangent.as_ref().unwrap_or(&k0.value);
                k0.value.cubic_spline(&k1.value, t, t0, t1, in_t, out_t)
            }
        }
    }

    /// Get default value for type
    fn default_value(&self) -> T {
        // This is a workaround; proper implementation would need type-specific defaults
        // For now, we rely on the keyframes being non-empty
        unimplemented!("Curve must have at least one keyframe")
    }

    /// Optimize curve by removing redundant keyframes
    pub fn optimize(&mut self, _max_error: f32) {
        // Simplified optimization placeholder
        // Full implementation would need type-specific error metrics
    }
}

impl<T: Interpolate> Default for Curve<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Track
// ============================================================================

/// Target property for a track
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackTarget {
    /// Translation (position)
    Translation,
    /// Rotation
    Rotation,
    /// Scale
    Scale,
    /// Custom float property by name
    Float(String),
}

/// A track binds a curve to a bone/property
#[derive(Clone, Debug)]
pub struct Track {
    /// Bone index this track affects
    bone_index: usize,
    /// Translation curve
    translation: Curve<Vec3>,
    /// Rotation curve
    rotation: Curve<Quat>,
    /// Scale curve
    scale: Curve<Vec3>,
    /// Custom float curves by name
    custom_curves: BTreeMap<String, Curve<f32>>,
}

impl Track {
    /// Create a new track for a bone
    pub fn new(bone_index: usize) -> Self {
        Self {
            bone_index,
            translation: Curve::new(),
            rotation: Curve::new(),
            scale: Curve::new(),
            custom_curves: BTreeMap::new(),
        }
    }

    /// Create track with curves
    pub fn with_curves(
        bone_index: usize,
        translation: Curve<Vec3>,
        rotation: Curve<Quat>,
        scale: Curve<Vec3>,
    ) -> Self {
        Self {
            bone_index,
            translation,
            rotation,
            scale,
            custom_curves: BTreeMap::new(),
        }
    }

    /// Get bone index
    pub fn bone_index(&self) -> usize {
        self.bone_index
    }

    /// Get translation curve
    pub fn translation(&self) -> &Curve<Vec3> {
        &self.translation
    }

    /// Get rotation curve
    pub fn rotation(&self) -> &Curve<Quat> {
        &self.rotation
    }

    /// Get scale curve
    pub fn scale(&self) -> &Curve<Vec3> {
        &self.scale
    }

    /// Get custom curves
    pub fn custom_curves(&self) -> &BTreeMap<String, Curve<f32>> {
        &self.custom_curves
    }

    /// Sample local pose at time
    pub fn sample_local_pose(&self, time: f32) -> (Vec3, Quat, Vec3) {
        let pos = if self.translation.is_empty() {
            Vec3::ZERO
        } else {
            self.translation.sample(time)
        };
        let rot = if self.rotation.is_empty() {
            Quat::IDENTITY
        } else {
            self.rotation.sample(time)
        };
        let scale = if self.scale.is_empty() {
            Vec3::ONE
        } else {
            self.scale.sample(time)
        };
        (pos, rot, scale)
    }
}

// ============================================================================
// Animation Event
// ============================================================================

/// An event triggered at a specific time in animation
#[derive(Clone, Debug)]
pub struct AnimationEvent {
    /// Event name
    name: String,
    /// Trigger time in seconds
    time: f32,
    /// Optional payload data
    payload: Option<String>,
}

impl AnimationEvent {
    /// Create a new event
    pub fn new(name: String, time: f32) -> Self {
        Self {
            name,
            time,
            payload: None,
        }
    }

    /// Create event with payload
    pub fn with_payload(name: String, time: f32, payload: String) -> Self {
        Self {
            name,
            time,
            payload: Some(payload),
        }
    }

    /// Get event name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get trigger time
    pub fn time(&self) -> f32 {
        self.time
    }

    /// Get payload
    pub fn payload(&self) -> Option<&str> {
        self.payload.as_deref()
    }
}

// ============================================================================
// AnimationClip
// ============================================================================

/// An animation clip containing multiple tracks
#[derive(Clone, Debug)]
pub struct AnimationClip {
    /// Clip name
    name: String,
    /// Duration in seconds
    duration: f32,
    /// Tracks for each bone
    tracks: Vec<Track>,
    /// Events triggered during playback
    events: Vec<AnimationEvent>,
    /// Wrap mode
    wrap_mode: WrapMode,
}

impl AnimationClip {
    /// Create a new animation clip
    pub fn new(name: String, duration: f32) -> Self {
        Self {
            name,
            duration,
            tracks: vec![],
            events: vec![],
            wrap_mode: WrapMode::Loop,
        }
    }

    /// Create clip with wrap mode
    pub fn with_wrap_mode(name: String, duration: f32, wrap_mode: WrapMode) -> Self {
        Self {
            name,
            duration,
            tracks: vec![],
            events: vec![],
            wrap_mode,
        }
    }

    /// Get clip name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get duration
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// Get tracks
    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    /// Get mutable tracks
    pub fn tracks_mut(&mut self) -> &mut Vec<Track> {
        &mut self.tracks
    }

    /// Add a track
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Get events
    pub fn events(&self) -> &[AnimationEvent] {
        &self.events
    }

    /// Add an event
    pub fn add_event(&mut self, event: AnimationEvent) {
        self.events.push(event);
    }

    /// Get wrap mode
    pub fn wrap_mode(&self) -> WrapMode {
        self.wrap_mode
    }

    /// Set wrap mode
    pub fn set_wrap_mode(&mut self, mode: WrapMode) {
        self.wrap_mode = mode;
    }

    /// Check if looping
    pub fn is_looping(&self) -> bool {
        self.wrap_mode == WrapMode::Loop || self.wrap_mode == WrapMode::PingPong
    }

    /// Wrap time according to wrap mode
    fn wrap_time(&self, time: f32) -> f32 {
        match self.wrap_mode {
            WrapMode::Once => time.min(self.duration),
            WrapMode::Loop => {
                if self.duration > 0.0 {
                    time % self.duration
                } else {
                    0.0
                }
            }
            WrapMode::PingPong => {
                if self.duration <= 0.0 {
                    return 0.0;
                }
                let cycle = (time / self.duration) as i32;
                let t = time % self.duration;
                if cycle % 2 == 0 {
                    t
                } else {
                    self.duration - t
                }
            }
            WrapMode::ClampForever => time.min(self.duration),
        }
    }

    /// Sample pose at time (returns pose with bone count from tracks)
    pub fn sample(&self, time: f32) -> Pose {
        let wrapped_time = self.wrap_time(time);

        // Find max bone index to determine pose size
        let max_bone = self
            .tracks
            .iter()
            .map(|t| t.bone_index())
            .max()
            .unwrap_or(0);
        let mut pose = Pose::new(max_bone + 1);

        for track in &self.tracks {
            let (pos, rot, scale) = track.sample_local_pose(wrapped_time);
            pose.set_bone(track.bone_index(), pos, rot, scale);
        }

        pose
    }

    /// Sample into existing pose
    pub fn sample_into(&self, time: f32, pose: &mut Pose) {
        let wrapped_time = self.wrap_time(time);

        for track in &self.tracks {
            let (pos, rot, scale) = track.sample_local_pose(wrapped_time);
            pose.set_bone(track.bone_index(), pos, rot, scale);
        }
    }

    /// Get events triggered between two times
    pub fn events_in_range(&self, start_time: f32, end_time: f32) -> Vec<&AnimationEvent> {
        self.events
            .iter()
            .filter(|e| e.time >= start_time && e.time < end_time)
            .collect()
    }
}

// ============================================================================
// Pose
// ============================================================================

/// Bone transform: position, rotation, scale
#[derive(Clone, Copy, Debug, Default)]
pub struct BoneTransform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl BoneTransform {
    pub const IDENTITY: Self = Self {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    /// Convert to matrix
    pub fn to_matrix(&self) -> Mat4 {
        let rot_mat = Mat4::from_quat(self.rotation);
        let scale_mat = Mat4::from_scale(self.scale);
        let trans_mat = Mat4::from_translation(self.position);
        trans_mat * rot_mat * scale_mat
    }

    /// Blend with another transform
    pub fn blend(&self, other: &Self, alpha: f32) -> Self {
        Self {
            position: self.position.lerp(other.position, alpha),
            rotation: self.rotation.nlerp(other.rotation, alpha),
            scale: self.scale.lerp(other.scale, alpha),
        }
    }

    /// Additive blend
    pub fn additive_blend(&self, additive: &Self, alpha: f32) -> Self {
        Self {
            position: self.position + additive.position * alpha,
            rotation: (self.rotation * additive.rotation).normalize(),
            scale: self.scale * (Vec3::ONE + (additive.scale - Vec3::ONE) * alpha),
        }
    }
}

/// A pose is a collection of bone transforms at a specific time
#[derive(Clone, Debug)]
pub struct Pose {
    bones: Vec<BoneTransform>,
}

impl Pose {
    /// Create a new pose with given bone count
    pub fn new(num_bones: usize) -> Self {
        Self {
            bones: vec![BoneTransform::IDENTITY; num_bones],
        }
    }

    /// Create pose with default transforms
    pub fn with_default(num_bones: usize) -> Self {
        Self::new(num_bones)
    }

    /// Get number of bones
    pub fn len(&self) -> usize {
        self.bones.len()
    }

    /// Check if pose is empty
    pub fn is_empty(&self) -> bool {
        self.bones.is_empty()
    }

    /// Get bone transforms
    pub fn bones(&self) -> &[BoneTransform] {
        &self.bones
    }

    /// Get mutable bone transforms
    pub fn bones_mut(&mut self) -> &mut [BoneTransform] {
        &mut self.bones
    }

    /// Set a bone transform
    pub fn set_bone(&mut self, idx: usize, position: Vec3, rotation: Quat, scale: Vec3) {
        if idx < self.bones.len() {
            self.bones[idx] = BoneTransform::new(position, rotation, scale);
        }
    }

    /// Set a bone transform directly
    pub fn set_bone_transform(&mut self, idx: usize, transform: BoneTransform) {
        if idx < self.bones.len() {
            self.bones[idx] = transform;
        }
    }

    /// Get a bone transform
    pub fn get_bone(&self, idx: usize) -> BoneTransform {
        if idx < self.bones.len() {
            self.bones[idx]
        } else {
            BoneTransform::IDENTITY
        }
    }

    /// Get a bone transform as tuple
    pub fn get_bone_tuple(&self, idx: usize) -> (Vec3, Quat, Vec3) {
        let b = self.get_bone(idx);
        (b.position, b.rotation, b.scale)
    }

    /// Blend two poses
    pub fn blend(a: &Pose, b: &Pose, alpha: f32) -> Pose {
        let len = a.len().max(b.len());
        let mut result = Pose::new(len);
        for i in 0..len {
            let ta = a.get_bone(i);
            let tb = b.get_bone(i);
            result.set_bone_transform(i, ta.blend(&tb, alpha));
        }
        result
    }

    /// Blend into this pose from another
    pub fn blend_into(&mut self, other: &Pose, alpha: f32) {
        for i in 0..self.len().min(other.len()) {
            let ta = self.bones[i];
            let tb = other.get_bone(i);
            self.bones[i] = ta.blend(&tb, alpha);
        }
    }

    /// Additive blend
    pub fn additive_blend(base: &Pose, additive: &Pose, alpha: f32) -> Pose {
        let len = base.len().max(additive.len());
        let mut result = Pose::new(len);
        for i in 0..len {
            let tb = base.get_bone(i);
            let ta = additive.get_bone(i);
            result.set_bone_transform(i, tb.additive_blend(&ta, alpha));
        }
        result
    }

    /// Create identity pose
    pub fn identity(num_bones: usize) -> Pose {
        Pose::new(num_bones)
    }

    /// Clone into another pose
    pub fn clone_into(&self, other: &mut Pose) {
        other.bones.resize(self.len(), BoneTransform::IDENTITY);
        for (i, b) in self.bones.iter().enumerate() {
            other.bones[i] = *b;
        }
    }

    /// Compute world matrices from local pose given skeleton
    pub fn local_to_world(&self, skeleton: &Skeleton) -> Vec<Mat4> {
        let mut world_matrices = vec![Mat4::IDENTITY; self.len()];

        for i in 0..self.len() {
            let local = self.bones[i].to_matrix();
            let parent_idx = skeleton.bone(i).parent();
            if let Some(parent) = parent_idx {
                if parent < world_matrices.len() {
                    world_matrices[i] = world_matrices[parent] * local;
                } else {
                    world_matrices[i] = local;
                }
            } else {
                world_matrices[i] = local;
            }
        }

        world_matrices
    }

    /// Compute skinning matrix palette
    pub fn compute_skin_matrices(&self, skeleton: &Skeleton) -> Vec<Mat4> {
        let world_matrices = self.local_to_world(skeleton);
        let inverse_bind = skeleton.get_inverse_bind_matrices();

        let mut skin_matrices = vec![Mat4::IDENTITY; self.len()];
        for i in 0..self.len().min(inverse_bind.len()) {
            skin_matrices[i] = world_matrices[i] * inverse_bind[i];
        }
        skin_matrices
    }
}

// ============================================================================
// Bone
// ============================================================================

/// A bone in the skeleton hierarchy
#[derive(Clone, Debug)]
pub struct Bone {
    /// Bone name
    name: String,
    /// Parent bone index (None for root)
    parent: Option<usize>,
    /// Local bind pose transform
    local_bind_pose: BoneTransform,
    /// Inverse bind pose matrix
    inverse_bind_pose: Mat4,
}

impl Bone {
    /// Create a new bone
    pub fn new(
        name: String,
        parent: Option<usize>,
        local_bind_pose: BoneTransform,
        inverse_bind_pose: Mat4,
    ) -> Self {
        Self {
            name,
            parent,
            local_bind_pose,
            inverse_bind_pose,
        }
    }

    /// Create bone with simple transform
    pub fn simple(
        name: String,
        parent: Option<usize>,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Self {
        let local_bind = BoneTransform::new(position, rotation, scale);
        let inverse_bind = local_bind.to_matrix().inverse().unwrap_or(Mat4::IDENTITY);
        Self::new(name, parent, local_bind, inverse_bind)
    }

    /// Get bone name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get parent index
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    /// Get local bind pose
    pub fn local_bind_pose(&self) -> BoneTransform {
        self.local_bind_pose
    }

    /// Get inverse bind pose matrix
    pub fn inverse_bind_pose(&self) -> Mat4 {
        self.inverse_bind_pose
    }
}

// ============================================================================
// Skeleton
// ============================================================================

/// A skeleton is a hierarchy of bones
#[derive(Clone, Debug)]
pub struct Skeleton {
    bones: Vec<Bone>,
    bind_pose: Pose,
    inverse_bind_matrices: Vec<Mat4>,
    root_index: usize,
}

impl Skeleton {
    /// Create a new skeleton from bones
    pub fn new(bones: Vec<Bone>) -> Self {
        let bind_pose = Pose::new(bones.len());
        let inverse_bind_matrices: Vec<Mat4> =
            bones.iter().map(|b| b.inverse_bind_pose()).collect();

        // Find root bone (no parent)
        let root_index = bones.iter().position(|b| b.parent.is_none()).unwrap_or(0);

        Self {
            bones,
            bind_pose,
            inverse_bind_matrices,
            root_index,
        }
    }

    /// Create skeleton with computed bind pose
    pub fn with_bind_pose(bones: Vec<Bone>) -> Self {
        let num_bones = bones.len();
        let mut bind_pose = Pose::new(num_bones);

        for (i, bone) in bones.iter().enumerate() {
            bind_pose.set_bone_transform(i, bone.local_bind_pose());
        }

        // Compute world bind pose matrices
        let mut world_bind = vec![Mat4::IDENTITY; num_bones];
        for i in 0..num_bones {
            let local = bones[i].local_bind_pose.to_matrix();
            if let Some(parent) = bones[i].parent {
                if parent < world_bind.len() {
                    world_bind[i] = world_bind[parent] * local;
                } else {
                    world_bind[i] = local;
                }
            } else {
                world_bind[i] = local;
            }
        }

        // Compute inverse bind matrices
        let inverse_bind_matrices: Vec<Mat4> = world_bind
            .iter()
            .map(|m| m.inverse().unwrap_or(Mat4::IDENTITY))
            .collect();

        let root_index = bones.iter().position(|b| b.parent.is_none()).unwrap_or(0);

        Self {
            bones,
            bind_pose,
            inverse_bind_matrices,
            root_index,
        }
    }

    /// Get bones slice
    pub fn bones(&self) -> &[Bone] {
        &self.bones
    }

    /// Get a bone by index
    pub fn bone(&self, idx: usize) -> &Bone {
        if idx < self.bones.len() {
            &self.bones[idx]
        } else {
            // Return a reference to a static dummy bone
            static DUMMY: Bone = Bone {
                name: String::new(),
                parent: None,
                local_bind_pose: BoneTransform::IDENTITY,
                inverse_bind_pose: Mat4::IDENTITY,
            };
            // SAFETY: This is a workaround for returning a reference
            // In practice, callers should check bounds first
            &DUMMY
        }
    }

    /// Get bone count
    pub fn bone_count(&self) -> usize {
        self.bones.len()
    }

    /// Get root bone index
    pub fn root(&self) -> usize {
        self.root_index
    }

    /// Get children of a bone
    pub fn children(&self, parent: usize) -> Vec<usize> {
        self.bones
            .iter()
            .enumerate()
            .filter_map(|(i, b)| {
                if b.parent == Some(parent) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get bind pose
    pub fn bind_pose(&self) -> &Pose {
        &self.bind_pose
    }

    /// Get inverse bind matrix by index
    pub fn inverse_bind_matrix(&self, idx: usize) -> Mat4 {
        if idx < self.inverse_bind_matrices.len() {
            self.inverse_bind_matrices[idx]
        } else {
            Mat4::IDENTITY
        }
    }

    /// Get all inverse bind matrices
    pub fn get_inverse_bind_matrices(&self) -> &[Mat4] {
        &self.inverse_bind_matrices
    }

    /// Find bone by name
    pub fn find_bone_by_name(&self, name: &str) -> Option<usize> {
        self.bones.iter().position(|b| b.name == name)
    }
}

// ============================================================================
// Skin
// ============================================================================

/// Vertex weight for skinning
#[derive(Clone, Copy, Debug, Default)]
pub struct VertexWeight {
    /// Bone index
    bone: u32,
    /// Weight value
    weight: f32,
}

impl VertexWeight {
    pub fn new(bone: u32, weight: f32) -> Self {
        Self { bone, weight }
    }

    pub fn bone(&self) -> u32 {
        self.bone
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }
}

/// Skin data for skinned mesh
#[derive(Clone, Debug)]
pub struct Skin {
    /// Bone names
    bone_names: Vec<String>,
    /// Inverse bind matrices
    inverse_bind_matrices: Vec<Mat4>,
}

impl Skin {
    pub fn new(bone_names: Vec<String>, inverse_bind_matrices: Vec<Mat4>) -> Self {
        Self {
            bone_names,
            inverse_bind_matrices,
        }
    }

    pub fn bone_count(&self) -> usize {
        self.bone_names.len()
    }

    pub fn bone_names(&self) -> &[String] {
        &self.bone_names
    }

    pub fn inverse_bind_matrices(&self) -> &[Mat4] {
        &self.inverse_bind_matrices
    }
}

// ============================================================================
// ParameterValue
// ============================================================================

/// Parameter value for animation controller
#[derive(Clone, Debug)]
pub enum ParameterValue {
    Bool(bool),
    Float(f32),
    Int(i32),
    Vec2(Vec2),
    Vec3(Vec3),
    Trigger, // One-shot event
}

impl ParameterValue {
    pub fn as_float(&self) -> Option<f32> {
        match self {
            ParameterValue::Float(v) => Some(*v),
            ParameterValue::Int(v) => Some(*v as f32),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ParameterValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self {
            ParameterValue::Int(v) => Some(*v),
            ParameterValue::Float(v) => Some(*v as i32),
            _ => None,
        }
    }

    pub fn as_vec2(&self) -> Option<Vec2> {
        match self {
            ParameterValue::Vec2(v) => Some(*v),
            _ => None,
        }
    }
}

/// Parameter map for animation controller
#[derive(Clone, Debug, Default)]
pub struct ParameterMap {
    params: BTreeMap<String, ParameterValue>,
    /// Pending triggers to consume
    pending_triggers: Vec<String>,
}

impl ParameterMap {
    pub fn new() -> Self {
        Self {
            params: BTreeMap::new(),
            pending_triggers: vec![],
        }
    }

    pub fn get(&self, name: &str) -> Option<&ParameterValue> {
        self.params.get(name)
    }

    pub fn set(&mut self, name: String, value: ParameterValue) {
        self.params.insert(name, value);
    }

    pub fn trigger(&mut self, name: &str) {
        self.pending_triggers.push(name.to_string());
    }

    pub fn has_trigger(&self, name: &str) -> bool {
        self.pending_triggers.iter().any(|t| t == name)
    }

    pub fn consume_trigger(&mut self, name: &str) -> bool {
        if let Some(pos) = self.pending_triggers.iter().position(|t| t == name) {
            self.pending_triggers.remove(pos);
            true
        } else {
            false
        }
    }
}

// ============================================================================
// Condition
// ============================================================================

/// Condition for state transitions
#[derive(Clone, Debug)]
pub enum Condition {
    /// Always true
    True,
    /// Always false
    False,
    /// Parameter comparison
    Parameter(String, CompareOp, ParameterValue),
    /// Logical AND
    And(Box<Condition>, Box<Condition>),
    /// Logical OR
    Or(Box<Condition>, Box<Condition>),
    /// Logical NOT
    Not(Box<Condition>),
    /// Time elapsed in current state
    TimeElapsed(f32),
    /// Event triggered
    EventTriggered(String),
}

impl Condition {
    /// Create parameter condition
    pub fn param(name: &str, op: CompareOp, value: ParameterValue) -> Self {
        Condition::Parameter(name.to_string(), op, value)
    }

    /// Create AND condition
    pub fn and(a: Condition, b: Condition) -> Self {
        Condition::And(Box::new(a), Box::new(b))
    }

    /// Create OR condition
    pub fn or(a: Condition, b: Condition) -> Self {
        Condition::Or(Box::new(a), Box::new(b))
    }

    /// Create NOT condition
    pub fn not(c: Condition) -> Self {
        Condition::Not(Box::new(c))
    }

    /// Evaluate condition against parameters
    pub fn evaluate(&self, params: &ParameterMap, state_time: f32) -> bool {
        match self {
            Condition::True => true,
            Condition::False => false,
            Condition::Parameter(name, op, value) => {
                let param = params.get(name);
                match (param, value) {
                    (Some(ParameterValue::Bool(p)), ParameterValue::Bool(v)) => match op {
                        CompareOp::Equal => *p == *v,
                        CompareOp::NotEqual => *p != *v,
                        _ => false,
                    },
                    (Some(ParameterValue::Float(p)), ParameterValue::Float(v)) => match op {
                        CompareOp::Equal => (*p - *v).abs() < EPSILON,
                        CompareOp::NotEqual => (*p - *v).abs() >= EPSILON,
                        CompareOp::Less => *p < *v,
                        CompareOp::LessEqual => *p <= *v,
                        CompareOp::Greater => *p > *v,
                        CompareOp::GreaterEqual => *p >= *v,
                    },
                    (Some(ParameterValue::Int(p)), ParameterValue::Int(v)) => match op {
                        CompareOp::Equal => *p == *v,
                        CompareOp::NotEqual => *p != *v,
                        CompareOp::Less => *p < *v,
                        CompareOp::LessEqual => *p <= *v,
                        CompareOp::Greater => *p > *v,
                        CompareOp::GreaterEqual => *p >= *v,
                    },
                    _ => false,
                }
            }
            Condition::And(a, b) => {
                a.evaluate(params, state_time) && b.evaluate(params, state_time)
            }
            Condition::Or(a, b) => a.evaluate(params, state_time) || b.evaluate(params, state_time),
            Condition::Not(c) => !c.evaluate(params, state_time),
            Condition::TimeElapsed(t) => state_time >= *t,
            Condition::EventTriggered(name) => params.has_trigger(name),
        }
    }
}

// ============================================================================
// Transition
// ============================================================================

/// Transition between states
#[derive(Clone, Debug)]
pub struct Transition {
    /// Source state name
    from: String,
    /// Target state name
    to: String,
    /// Blend duration
    duration: f32,
    /// Blend mode
    blend_mode: BlendMode,
    /// Condition for transition
    condition: Condition,
    /// Exit time (normalized, 0-1)
    exit_time: Option<f32>,
    /// Has exit time requirement
    has_exit_time: bool,
}

impl Transition {
    /// Create a new transition
    pub fn new(from: String, to: String, duration: f32, condition: Condition) -> Self {
        Self {
            from,
            to,
            duration,
            blend_mode: BlendMode::Linear,
            condition,
            exit_time: None,
            has_exit_time: false,
        }
    }

    /// Create transition with exit time
    pub fn with_exit_time(
        from: String,
        to: String,
        duration: f32,
        exit_time: f32,
        condition: Condition,
    ) -> Self {
        Self {
            from,
            to,
            duration,
            blend_mode: BlendMode::Linear,
            condition,
            exit_time: Some(exit_time),
            has_exit_time: true,
        }
    }

    pub fn from_state(&self) -> &str {
        &self.from
    }

    pub fn to_state(&self) -> &str {
        &self.to
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }

    pub fn exit_time(&self) -> Option<f32> {
        self.exit_time
    }

    pub fn has_exit_time(&self) -> bool {
        self.has_exit_time
    }

    pub fn condition(&self) -> &Condition {
        &self.condition
    }

    /// Check if transition can trigger
    pub fn can_trigger(&self, params: &ParameterMap, state_time: f32, state_duration: f32) -> bool {
        let cond_ok = self.condition.evaluate(params, state_time);
        let time_ok = if self.has_exit_time {
            let normalized_time = if state_duration > 0.0 {
                state_time / state_duration
            } else {
                0.0
            };
            normalized_time >= self.exit_time.unwrap_or(0.0)
        } else {
            true
        };
        cond_ok && time_ok
    }
}

// ============================================================================
// BlendNode1D
// ============================================================================

/// 1D blend tree node
#[derive(Clone, Debug)]
pub struct BlendNode1D {
    /// Parameter name
    parameter: String,
    /// Blend points (value, clip index)
    points: Vec<(f32, usize)>,
}

impl BlendNode1D {
    /// Create new 1D blend node
    pub fn new(parameter: String) -> Self {
        Self {
            parameter,
            points: vec![],
        }
    }

    /// Create with points
    pub fn with_points(parameter: String, points: Vec<(f32, usize)>) -> Self {
        Self { parameter, points }
    }

    /// Add a blend point
    pub fn push(&mut self, value: f32, clip_index: usize) {
        self.points.push((value, clip_index));
        // Sort by value
        self.points
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(core::cmp::Ordering::Equal));
    }

    /// Get parameter name
    pub fn parameter(&self) -> &str {
        &self.parameter
    }

    /// Get duration (max of all clips)
    pub fn duration(&self, clips: &[AnimationClip]) -> f32 {
        self.points
            .iter()
            .filter_map(|(_, idx)| clips.get(*idx).map(|c| c.duration()))
            .fold(0.0, |a, b| a.max(b))
    }

    /// Interpolate poses based on parameter value
    pub fn interpolate(
        &self,
        time: f32,
        param_value: f32,
        clips: &[AnimationClip],
        skeleton: &Skeleton,
    ) -> Pose {
        if self.points.is_empty() {
            return Pose::new(skeleton.bone_count());
        }

        // Find surrounding points
        if param_value <= self.points[0].0 {
            let clip = clips.get(self.points[0].1);
            return clip
                .map(|c| c.sample(time))
                .unwrap_or_else(|| Pose::new(skeleton.bone_count()));
        }
        if param_value >= self.points.last().unwrap().0 {
            let clip = clips.get(self.points.last().unwrap().1);
            return clip
                .map(|c| c.sample(time))
                .unwrap_or_else(|| Pose::new(skeleton.bone_count()));
        }

        // Binary search
        let mut lo = 0;
        let mut hi = self.points.len() - 1;
        while lo < hi - 1 {
            let mid = (lo + hi) / 2;
            if self.points[mid].0 <= param_value {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        let (v0, idx0) = &self.points[lo];
        let (v1, idx1) = &self.points[lo + 1];
        let t = (param_value - v0) / (v1 - v0);

        let clip0 = clips.get(*idx0);
        let clip1 = clips.get(*idx1);

        match (clip0, clip1) {
            (Some(c0), Some(c1)) => {
                let pose0 = c0.sample(time);
                let pose1 = c1.sample(time);
                Pose::blend(&pose0, &pose1, t)
            }
            (Some(c0), None) => c0.sample(time),
            (None, Some(c1)) => c1.sample(time),
            (None, None) => Pose::new(skeleton.bone_count()),
        }
    }
}

// ============================================================================
// BlendNode2D
// ============================================================================

/// 2D blend tree node
#[derive(Clone, Debug)]
pub struct BlendNode2D {
    /// X parameter name
    x_parameter: String,
    /// Y parameter name
    y_parameter: String,
    /// Blend points (x, y, clip index)
    points: Vec<(f32, f32, usize)>,
}

impl BlendNode2D {
    /// Create new 2D blend node
    pub fn new(x_parameter: String, y_parameter: String) -> Self {
        Self {
            x_parameter,
            y_parameter,
            points: vec![],
        }
    }

    /// Create with points
    pub fn with_points(
        x_parameter: String,
        y_parameter: String,
        points: Vec<(f32, f32, usize)>,
    ) -> Self {
        Self {
            x_parameter,
            y_parameter,
            points,
        }
    }

    /// Add a blend point
    pub fn push(&mut self, x: f32, y: f32, clip_index: usize) {
        self.points.push((x, y, clip_index));
    }

    /// Get X parameter name
    pub fn x_parameter(&self) -> &str {
        &self.x_parameter
    }

    /// Get Y parameter name
    pub fn y_parameter(&self) -> &str {
        &self.y_parameter
    }

    /// Get duration (max of all clips)
    pub fn duration(&self, clips: &[AnimationClip]) -> f32 {
        self.points
            .iter()
            .filter_map(|(_, _, idx)| clips.get(*idx).map(|c| c.duration()))
            .fold(0.0, |a, b| a.max(b))
    }

    /// Interpolate poses based on (x, y) parameter values
    pub fn interpolate(
        &self,
        time: f32,
        x: f32,
        y: f32,
        clips: &[AnimationClip],
        skeleton: &Skeleton,
    ) -> Pose {
        if self.points.is_empty() {
            return Pose::new(skeleton.bone_count());
        }

        // Find nearest point (simplified)
        let nearest = self.points.iter().min_by(|a, b| {
            let da = (a.0 - x).powi(2) + (a.1 - y).powi(2);
            let db = (b.0 - x).powi(2) + (b.1 - y).powi(2);
            da.partial_cmp(&db).unwrap_or(core::cmp::Ordering::Equal)
        });

        match nearest {
            Some((_, _, idx)) => clips
                .get(*idx)
                .map(|c| c.sample(time))
                .unwrap_or_else(|| Pose::new(skeleton.bone_count())),
            None => Pose::new(skeleton.bone_count()),
        }
    }
}

// ============================================================================
// AnimationMask
// ============================================================================

/// Animation mask for selective bone blending
#[derive(Clone, Debug)]
pub struct AnimationMask {
    /// Mask bits (true = affected)
    mask: Vec<bool>,
}

impl AnimationMask {
    pub fn new(num_bones: usize) -> Self {
        Self {
            mask: vec![false; num_bones],
        }
    }

    pub fn set(&mut self, idx: usize, value: bool) {
        if idx < self.mask.len() {
            self.mask[idx] = value;
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        if idx < self.mask.len() {
            self.mask[idx]
        } else {
            false
        }
    }

    pub fn invert(&mut self) {
        for b in &mut self.mask {
            *b = !*b;
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let len = self.mask.len().max(other.mask.len());
        let mut result = Self::new(len);
        for i in 0..len {
            result.mask[i] = self.get(i) || other.get(i);
        }
        result
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let len = self.mask.len().max(other.mask.len());
        let mut result = Self::new(len);
        for i in 0..len {
            result.mask[i] = self.get(i) && other.get(i);
        }
        result
    }
}

// ============================================================================
// IK System
// ============================================================================

/// IK chain definition
#[derive(Clone, Debug)]
pub struct IKChain {
    /// Bone indices in chain (root to tip)
    bones: Vec<usize>,
}

impl IKChain {
    pub fn new() -> Self {
        Self { bones: vec![] }
    }

    pub fn with_bones(bones: Vec<usize>) -> Self {
        Self { bones }
    }

    pub fn push(&mut self, bone_idx: usize) {
        self.bones.push(bone_idx);
    }

    pub fn bones(&self) -> &[usize] {
        &self.bones
    }

    pub fn root(&self) -> usize {
        self.bones.first().copied().unwrap_or(0)
    }

    pub fn tip(&self) -> usize {
        self.bones.last().copied().unwrap_or(0)
    }

    pub fn len(&self) -> usize {
        self.bones.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bones.is_empty()
    }
}

impl Default for IKChain {
    fn default() -> Self {
        Self::new()
    }
}

/// IK solver functions
pub struct IK;

impl IK {
    /// Two-bone IK solver
    /// Returns (shoulder_rotation, elbow_rotation) to reach target
    pub fn two_bone_ik(
        shoulder_pos: Vec3,
        elbow_pos: Vec3,
        wrist_pos: Vec3,
        target_pos: Vec3,
        elbow_dir: Vec3,
    ) -> (Quat, Quat) {
        // Calculate bone lengths
        let upper_len = (elbow_pos - shoulder_pos).length();
        let lower_len = (wrist_pos - elbow_pos).length();
        let total_len = upper_len + lower_len;

        // Distance to target
        let target_dist = (target_pos - shoulder_pos).length();

        // Clamp target to reachable range
        let clamped_dist = target_dist
            .min(total_len - EPSILON)
            .max((upper_len - lower_len).abs() + EPSILON);
        let clamped_target = shoulder_pos + (target_pos - shoulder_pos).normalize() * clamped_dist;

        // Calculate desired elbow angle using law of cosines
        let cos_angle = (upper_len * upper_len + lower_len * lower_len
            - clamped_dist * clamped_dist)
            / (2.0 * upper_len * lower_len);
        let elbow_angle = cos_angle.clamp(-1.0, 1.0).acos();

        // Calculate shoulder rotation to point towards target
        let to_target = (clamped_target - shoulder_pos).normalize_or_zero();
        let to_elbow = (elbow_pos - shoulder_pos).normalize_or_zero();

        // Rotation from current elbow direction to target direction
        let shoulder_rot = rotation_from_to(to_elbow, to_target, elbow_dir);

        // Apply elbow bend
        let elbow_axis = elbow_dir.normalize_or_zero();
        let elbow_rot = quat_from_axis_angle(elbow_axis, -elbow_angle);

        (shoulder_rot, elbow_rot)
    }

    /// CCD IK solver (Cyclic Coordinate Descent)
    pub fn ccd_ik(
        chain_positions: &[Vec3],
        target_pos: Vec3,
        tolerance: f32,
        max_iterations: u32,
    ) -> Vec<Quat> {
        let n = chain_positions.len();
        if n < 2 {
            return vec![Quat::IDENTITY; n];
        }

        let mut positions = chain_positions.to_vec();
        let mut rotations = vec![Quat::IDENTITY; n];

        let tip_idx = n - 1;
        let tolerance_sq = tolerance * tolerance;

        for _ in 0..max_iterations {
            // Check if we've reached target
            let tip_to_target_sq = (positions[tip_idx] - target_pos).length_squared();
            if tip_to_target_sq < tolerance_sq {
                break;
            }

            // Iterate from tip to root
            for i in (0..tip_idx).rev() {
                // Current tip position
                let tip_pos = positions[tip_idx];

                // Joint position
                let joint_pos = positions[i];

                // Vector from joint to tip
                let to_tip = (tip_pos - joint_pos).normalize_or_zero();

                // Vector from joint to target
                let to_target = (target_pos - joint_pos).normalize_or_zero();

                // Rotation to align tip with target
                let rot = rotation_from_to_simple(to_tip, to_target);

                // Apply rotation
                rotations[i] = rot * rotations[i];

                // Update positions of all bones after this joint
                for j in (i + 1)..n {
                    let rel = positions[j] - positions[i];
                    positions[j] = positions[i] + rot * rel;
                }
            }
        }

        rotations
    }

    /// FABRIK IK solver (Forward And Backward Reaching Inverse Kinematics)
    pub fn fabrik(
        chain_positions: &[Vec3],
        target_pos: Vec3,
        tolerance: f32,
        max_iterations: u32,
    ) -> Vec<Vec3> {
        let n = chain_positions.len();
        if n < 2 {
            return chain_positions.to_vec();
        }

        // Calculate bone lengths
        let bone_lengths: Vec<f32> = chain_positions
            .iter()
            .skip(1)
            .enumerate()
            .map(|(i, p)| (*p - chain_positions[i]).length())
            .collect();

        let mut positions = chain_positions.to_vec();
        let root_pos = chain_positions[0];
        let tolerance_sq = tolerance * tolerance;

        for _ in 0..max_iterations {
            // Check convergence
            if (positions[n - 1] - target_pos).length_squared() < tolerance_sq {
                break;
            }

            // Forward reaching: from tip to root
            positions[n - 1] = target_pos;
            for i in (0..n - 1).rev() {
                let dir = (positions[i] - positions[i + 1]).normalize_or_zero();
                positions[i] = positions[i + 1] + dir * bone_lengths[i];
            }

            // Backward reaching: from root to tip
            positions[0] = root_pos;
            for i in 0..n - 1 {
                let dir = (positions[i + 1] - positions[i]).normalize_or_zero();
                positions[i + 1] = positions[i] + dir * bone_lengths[i];
            }
        }

        positions
    }
}

/// Helper: create quaternion from axis-angle
fn quat_from_axis_angle(axis: Vec3, angle: f32) -> Quat {
    let half = angle / 2.0;
    let s = half.sin();
    let c = half.cos();
    let axis = axis.normalize_or_zero();
    Quat {
        x: axis.x * s,
        y: axis.y * s,
        z: axis.z * s,
        w: c,
    }
}

/// Helper: create quaternion rotation from one direction to another
fn rotation_from_to(from: Vec3, to: Vec3, up_hint: Vec3) -> Quat {
    let from = from.normalize_or_zero();
    let to = to.normalize_or_zero();

    if (from - to).length_squared() < EPSILON {
        return Quat::IDENTITY;
    }

    if (from + to).length_squared() < EPSILON {
        // Directions are opposite, use up hint
        let axis = up_hint.normalize_or_zero();
        return quat_from_axis_angle(axis, core::f32::consts::PI);
    }

    let half = (from + to).normalize_or_zero();
    let cross = from.cross(half);

    Quat {
        x: cross.x,
        y: cross.y,
        z: cross.z,
        w: from.dot(half),
    }
    .normalize()
}

/// Helper: simple rotation from one direction to another
fn rotation_from_to_simple(from: Vec3, to: Vec3) -> Quat {
    let from = from.normalize_or_zero();
    let to = to.normalize_or_zero();

    let dot = from.dot(to);

    if dot > 1.0 - EPSILON {
        return Quat::IDENTITY;
    }

    if dot < -1.0 + EPSILON {
        // Find perpendicular axis
        let axis = if from.x.abs() > from.y.abs() {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let cross = from.cross(axis).normalize_or_zero();
        return quat_from_axis_angle(cross, core::f32::consts::PI);
    }

    let cross = from.cross(to);
    let s = (1.0 + dot).sqrt();

    Quat {
        x: cross.x / s,
        y: cross.y / s,
        z: cross.z / s,
        w: s / 2.0,
    }
    .normalize()
}

/// Aim IK: make a bone aim at target
pub struct AimIK {
    bone_index: usize,
    aim_axis: Vec3,
}

impl AimIK {
    pub fn new(bone_index: usize, aim_axis: Vec3) -> Self {
        Self {
            bone_index,
            aim_axis: aim_axis.normalize_or_zero(),
        }
    }

    pub fn apply(&self, pose: &Pose, target_pos: Vec3, skeleton: &Skeleton) -> Pose {
        let mut result = pose.clone();

        if self.bone_index >= pose.len() {
            return result;
        }

        // Get bone world position
        let world_matrices = pose.local_to_world(skeleton);
        if self.bone_index >= world_matrices.len() {
            return result;
        }

        let bone_world_pos = Vec3::new(
            world_matrices[self.bone_index].cols[3][0],
            world_matrices[self.bone_index].cols[3][1],
            world_matrices[self.bone_index].cols[3][2],
        );

        // Target direction
        let target_dir = (target_pos - bone_world_pos).normalize_or_zero();

        // Rotation to aim
        let rot = rotation_from_to_simple(self.aim_axis, target_dir);

        // Apply to bone
        let bone = pose.get_bone(self.bone_index);
        result.set_bone(
            self.bone_index,
            bone.position,
            rot * bone.rotation,
            bone.scale,
        );

        result
    }
}

/// Look-at IK: make a bone look at target
pub struct LookAtIK {
    bone_index: usize,
    up: Vec3,
}

impl LookAtIK {
    pub fn new(bone_index: usize, up: Vec3) -> Self {
        Self {
            bone_index,
            up: up.normalize_or_zero(),
        }
    }

    pub fn apply(&self, pose: &Pose, target_pos: Vec3, skeleton: &Skeleton) -> Pose {
        let mut result = pose.clone();

        if self.bone_index >= pose.len() {
            return result;
        }

        // Get bone world position
        let world_matrices = pose.local_to_world(skeleton);
        if self.bone_index >= world_matrices.len() {
            return result;
        }

        let bone_world_pos = Vec3::new(
            world_matrices[self.bone_index].cols[3][0],
            world_matrices[self.bone_index].cols[3][1],
            world_matrices[self.bone_index].cols[3][2],
        );

        // Look direction
        let forward = (target_pos - bone_world_pos).normalize_or_zero();

        // Build look-at rotation (simplified)
        let rot = rotation_from_to_simple(Vec3::Z, forward);

        let bone = pose.get_bone(self.bone_index);
        result.set_bone(
            self.bone_index,
            bone.position,
            rot * bone.rotation,
            bone.scale,
        );

        result
    }
}

// ============================================================================
// Animator Component
// ============================================================================

/// Animator component for simple animation playback
#[derive(Clone, Debug)]
pub struct Animator {
    /// Current clip index
    clip_index: Option<usize>,
    /// Current playback time
    time: f32,
    /// Playback speed
    speed: f32,
    /// Is playing
    is_playing: bool,
    /// Wrap mode override
    wrap_mode: WrapMode,
    /// Current pose
    pose: Pose,
    /// Triggered events (pending)
    triggered_events: Vec<AnimationEvent>,
    /// Last sample time (for event detection)
    last_time: f32,
}

impl Animator {
    pub fn new(num_bones: usize) -> Self {
        Self {
            clip_index: None,
            time: 0.0,
            speed: 1.0,
            is_playing: false,
            wrap_mode: WrapMode::Loop,
            pose: Pose::new(num_bones),
            triggered_events: vec![],
            last_time: 0.0,
        }
    }

    pub fn play(&mut self, clip_index: usize) {
        self.clip_index = Some(clip_index);
        self.time = 0.0;
        self.is_playing = true;
        self.last_time = 0.0;
    }

    pub fn play_with_speed(&mut self, clip_index: usize, speed: f32) {
        self.play(clip_index);
        self.speed = speed;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn set_time(&mut self, t: f32) {
        self.time = t;
        self.last_time = t;
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn wrap_mode(&self) -> WrapMode {
        self.wrap_mode
    }

    pub fn set_wrap_mode(&mut self, mode: WrapMode) {
        self.wrap_mode = mode;
    }

    pub fn current_clip(&self) -> Option<usize> {
        self.clip_index
    }

    pub fn pose(&self) -> &Pose {
        &self.pose
    }

    pub fn events_triggered(&self) -> &[AnimationEvent] {
        &self.triggered_events
    }

    /// Update animator with clips
    pub fn update(&mut self, dt: f32, clips: &[AnimationClip]) {
        if !self.is_playing {
            return;
        }

        self.time += dt * self.speed;
        self.triggered_events.clear();

        if let Some(clip_idx) = self.clip_index {
            if let Some(clip) = clips.get(clip_idx) {
                // Detect events in time range
                let start = self.last_time;
                let end = self.time;
                for event in clip.events_in_range(start, end) {
                    self.triggered_events.push(event.clone());
                }

                // Sample pose
                self.pose = clip.sample(self.time);
                self.last_time = self.time;

                // Handle wrap
                if !clip.is_looping() && self.time >= clip.duration() {
                    self.is_playing = false;
                    self.time = clip.duration();
                }
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_f32_linear() {
        let mut curve: Curve<f32> = Curve::new();
        curve.push(Keyframe::new(0.0, 0.0));
        curve.push(Keyframe::new(1.0, 10.0));

        assert_eq!(curve.sample_with_wrap(0.0, WrapMode::Once), 0.0);
        assert_eq!(curve.sample_with_wrap(0.5, WrapMode::Once), 5.0);
        assert_eq!(curve.sample_with_wrap(1.0, WrapMode::Once), 10.0);
    }

    #[test]
    fn test_curve_vec3_linear() {
        let mut curve: Curve<Vec3> = Curve::new();
        curve.push(Keyframe::new(0.0, Vec3::ZERO));
        curve.push(Keyframe::new(1.0, Vec3::ONE));

        let mid = curve.sample_with_wrap(0.5, WrapMode::Once);
        assert!((mid.x - 0.5).abs() < EPSILON);
        assert!((mid.y - 0.5).abs() < EPSILON);
        assert!((mid.z - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_curve_quat_slerp() {
        let mut curve: Curve<Quat> = Curve::new();
        curve.push(Keyframe::new(0.0, Quat::IDENTITY));
        curve.push(Keyframe::new(
            1.0,
            Quat::from_rotation_y(core::f32::consts::PI),
        ));

        let mid = curve.sample_with_wrap(0.5, WrapMode::Once);
        // Should be rotation by PI/2
        let rotated = mid * Vec3::X;
        assert!((rotated.x - 0.0).abs() < 0.1);
        assert!((rotated.y - 0.0).abs() < EPSILON);
        assert!((rotated.z + 1.0).abs() < 0.1);
    }

    #[test]
    fn test_curve_wrap_loop() {
        let mut curve: Curve<f32> = Curve::new();
        curve.push(Keyframe::new(0.0, 0.0));
        curve.push(Keyframe::new(1.0, 10.0));

        assert_eq!(curve.sample_with_wrap(1.5, WrapMode::Loop), 5.0);
        assert_eq!(curve.sample_with_wrap(2.0, WrapMode::Loop), 0.0);
    }

    #[test]
    fn test_curve_wrap_pingpong() {
        let mut curve: Curve<f32> = Curve::new();
        curve.push(Keyframe::new(0.0, 0.0));
        curve.push(Keyframe::new(1.0, 10.0));

        assert_eq!(curve.sample_with_wrap(0.5, WrapMode::PingPong), 5.0);
        assert_eq!(curve.sample_with_wrap(1.5, WrapMode::PingPong), 5.0); // Going back
        assert_eq!(curve.sample_with_wrap(2.0, WrapMode::PingPong), 0.0); // Back to start
    }

    #[test]
    fn test_pose_blend() {
        let mut pose_a = Pose::new(1);
        pose_a.set_bone(0, Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);

        let mut pose_b = Pose::new(1);
        pose_b.set_bone(
            0,
            Vec3::ONE,
            Quat::from_rotation_y(core::f32::consts::PI),
            Vec3::splat(2.0),
        );

        let blended = Pose::blend(&pose_a, &pose_b, 0.5);
        let bone = blended.get_bone(0);

        assert!((bone.position.x - 0.5).abs() < EPSILON);
        assert!((bone.scale.x - 1.5).abs() < EPSILON);
    }

    #[test]
    fn test_pose_additive_blend() {
        let mut base = Pose::new(1);
        base.set_bone(0, Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);

        let mut additive = Pose::new(1);
        additive.set_bone(
            0,
            Vec3::new(0.1, 0.0, 0.0),
            Quat::IDENTITY,
            Vec3::splat(1.1),
        );

        let result = Pose::additive_blend(&base, &additive, 0.5);
        let bone = result.get_bone(0);

        assert!((bone.position.x - 0.05).abs() < EPSILON);
        assert!((bone.scale.x - 1.05).abs() < EPSILON);
    }

    #[test]
    fn test_skeleton_find_bone() {
        let bones = vec![
            Bone::simple(
                "root".to_string(),
                None,
                Vec3::ZERO,
                Quat::IDENTITY,
                Vec3::ONE,
            ),
            Bone::simple(
                "child".to_string(),
                Some(0),
                Vec3::Y,
                Quat::IDENTITY,
                Vec3::ONE,
            ),
        ];
        let skeleton = Skeleton::new(bones);

        assert_eq!(skeleton.find_bone_by_name("root"), Some(0));
        assert_eq!(skeleton.find_bone_by_name("child"), Some(1));
        assert_eq!(skeleton.find_bone_by_name("missing"), None);
    }

    #[test]
    fn test_skeleton_children() {
        let bones = vec![
            Bone::simple(
                "root".to_string(),
                None,
                Vec3::ZERO,
                Quat::IDENTITY,
                Vec3::ONE,
            ),
            Bone::simple(
                "child1".to_string(),
                Some(0),
                Vec3::Y,
                Quat::IDENTITY,
                Vec3::ONE,
            ),
            Bone::simple(
                "child2".to_string(),
                Some(0),
                Vec3::new(1.0, 0.0, 0.0),
                Quat::IDENTITY,
                Vec3::ONE,
            ),
        ];
        let skeleton = Skeleton::new(bones);

        let children = skeleton.children(0);
        assert_eq!(children.len(), 2);
        assert!(children.contains(&1));
        assert!(children.contains(&2));
    }

    #[test]
    fn test_condition_parameter() {
        let mut params = ParameterMap::new();
        params.set("speed".to_string(), ParameterValue::Float(5.0));

        let cond = Condition::param("speed", CompareOp::Greater, ParameterValue::Float(3.0));
        assert!(cond.evaluate(&params, 0.0));

        let cond2 = Condition::param("speed", CompareOp::Less, ParameterValue::Float(3.0));
        assert!(!cond2.evaluate(&params, 0.0));
    }

    #[test]
    fn test_condition_and_or() {
        let mut params = ParameterMap::new();
        params.set("a".to_string(), ParameterValue::Bool(true));
        params.set("b".to_string(), ParameterValue::Bool(false));

        let cond_a = Condition::param("a", CompareOp::Equal, ParameterValue::Bool(true));
        let cond_b = Condition::param("b", CompareOp::Equal, ParameterValue::Bool(true));

        let and_cond = Condition::and(cond_a.clone(), cond_b.clone());
        assert!(!and_cond.evaluate(&params, 0.0));

        let or_cond = Condition::or(cond_a, cond_b);
        assert!(or_cond.evaluate(&params, 0.0));
    }

    #[test]
    fn test_condition_time_elapsed() {
        let params = ParameterMap::new();

        let cond = Condition::TimeElapsed(1.0);
        assert!(!cond.evaluate(&params, 0.5));
        assert!(cond.evaluate(&params, 1.0));
        assert!(cond.evaluate(&params, 2.0));
    }

    #[test]
    fn test_animation_mask_union() {
        let mut mask_a = AnimationMask::new(4);
        mask_a.set(0, true);
        mask_a.set(1, true);

        let mut mask_b = AnimationMask::new(4);
        mask_b.set(2, true);
        mask_b.set(3, true);

        let union = mask_a.union(&mask_b);
        assert!(union.get(0));
        assert!(union.get(1));
        assert!(union.get(2));
        assert!(union.get(3));
    }

    #[test]
    fn test_animation_mask_intersection() {
        let mut mask_a = AnimationMask::new(4);
        mask_a.set(0, true);
        mask_a.set(1, true);

        let mut mask_b = AnimationMask::new(4);
        mask_b.set(0, true);
        mask_b.set(2, true);

        let intersection = mask_a.intersection(&mask_b);
        assert!(intersection.get(0));
        assert!(!intersection.get(1));
        assert!(!intersection.get(2));
    }

    #[test]
    fn test_ik_two_bone() {
        // Simple test: verify IK produces valid rotations
        let shoulder = Vec3::ZERO;
        let elbow = Vec3::Y;
        let wrist = Vec3::new(0.0, 2.0, 0.0);
        let target = Vec3::new(0.0, 1.0, 0.0); // Target at elbow position
        let elbow_dir = Vec3::Z;

        let (shoulder_rot, elbow_rot) = IK::two_bone_ik(shoulder, elbow, wrist, target, elbow_dir);

        // Verify rotations are valid (normalized)
        assert!(
            (shoulder_rot.x * shoulder_rot.x
                + shoulder_rot.y * shoulder_rot.y
                + shoulder_rot.z * shoulder_rot.z
                + shoulder_rot.w * shoulder_rot.w
                - 1.0)
                .abs()
                < 0.1
        );
        assert!(
            (elbow_rot.x * elbow_rot.x
                + elbow_rot.y * elbow_rot.y
                + elbow_rot.z * elbow_rot.z
                + elbow_rot.w * elbow_rot.w
                - 1.0)
                .abs()
                < 0.1
        );
    }

    #[test]
    fn test_ik_ccd() {
        // Simple test: verify CCD produces valid rotations
        let positions = vec![Vec3::ZERO, Vec3::Y, Vec3::new(0.0, 2.0, 0.0)];
        let target = Vec3::new(0.0, 1.0, 0.0);

        let rotations = IK::ccd_ik(&positions, target, 0.5, 5);

        // Verify all rotations are valid quaternions
        for rot in &rotations {
            let len_sq = rot.x * rot.x + rot.y * rot.y + rot.z * rot.z + rot.w * rot.w;
            assert!(
                (len_sq - 1.0).abs() < 0.1,
                "Invalid quaternion length: {}",
                len_sq
            );
        }
    }

    #[test]
    fn test_ik_fabrik() {
        let positions = vec![
            Vec3::ZERO,
            Vec3::Y,
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(0.0, 3.0, 0.0),
        ];
        let target = Vec3::new(0.0, 2.0, 1.0);

        let result = IK::fabrik(&positions, target, 0.01, 10);

        let tip = result[result.len() - 1];
        let dist = (tip - target).length();
        assert!(dist < 0.1, "FABRIK distance to target: {}", dist);

        // Root should stay fixed
        assert!((result[0] - positions[0]).length() < EPSILON);
    }

    #[test]
    fn test_animation_clip_sample() {
        let mut clip = AnimationClip::new("test".to_string(), 1.0);

        let mut track = Track::new(0);
        track.translation.push(Keyframe::new(0.0, Vec3::ZERO));
        track.translation.push(Keyframe::new(1.0, Vec3::ONE));
        clip.add_track(track);

        let pose = clip.sample(0.5);
        let bone = pose.get_bone(0);

        assert!((bone.position.x - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_animation_clip_events() {
        let mut clip = AnimationClip::new("test".to_string(), 2.0);
        clip.add_event(AnimationEvent::new("step".to_string(), 0.5));
        clip.add_event(AnimationEvent::new("step".to_string(), 1.5));

        let events = clip.events_in_range(0.0, 1.0);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name(), "step");

        let events2 = clip.events_in_range(1.0, 2.0);
        assert_eq!(events2.len(), 1);
    }

    #[test]
    fn test_blend_node_1d() {
        let clips = vec![
            AnimationClip::new("idle".to_string(), 1.0),
            AnimationClip::new("walk".to_string(), 1.0),
            AnimationClip::new("run".to_string(), 1.0),
        ];

        let mut blend = BlendNode1D::new("speed".to_string());
        blend.push(0.0, 0);
        blend.push(0.5, 1);
        blend.push(1.0, 2);

        let skeleton = Skeleton::new(vec![Bone::simple(
            "root".to_string(),
            None,
            Vec3::ZERO,
            Quat::IDENTITY,
            Vec3::ONE,
        )]);

        // Test interpolation at boundaries
        let pose_0 = blend.interpolate(0.0, 0.0, &clips, &skeleton);
        let pose_1 = blend.interpolate(0.0, 1.0, &clips, &skeleton);

        // Should sample valid poses
        assert_eq!(pose_0.len(), skeleton.bone_count());
        assert_eq!(pose_1.len(), skeleton.bone_count());
    }

    #[test]
    fn test_bone_transform_to_matrix() {
        let transform = BoneTransform::new(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::ONE);

        let mat = transform.to_matrix();

        // Check translation
        assert!((mat.cols[3][0] - 1.0).abs() < EPSILON);
        assert!((mat.cols[3][1] - 2.0).abs() < EPSILON);
        assert!((mat.cols[3][2] - 3.0).abs() < EPSILON);
    }

    #[test]
    fn test_pose_local_to_world() {
        // Create a simple skeleton with root at origin and child at Y=1
        let bones = vec![
            Bone::simple(
                "root".to_string(),
                None,
                Vec3::ZERO,
                Quat::IDENTITY,
                Vec3::ONE,
            ),
            Bone::simple(
                "child".to_string(),
                Some(0),
                Vec3::new(0.0, 1.0, 0.0),
                Quat::IDENTITY,
                Vec3::ONE,
            ),
        ];
        let skeleton = Skeleton::with_bind_pose(bones);

        // Use bind pose as the pose
        let pose = skeleton.bind_pose().clone();
        let world_matrices = pose.local_to_world(&skeleton);

        // Root should be at origin
        assert!((world_matrices[0].cols[3][0] - 0.0).abs() < EPSILON);
        assert!((world_matrices[0].cols[3][1] - 0.0).abs() < EPSILON);

        // Child should be at Y=1 (root at origin + child local Y=1)
        assert!((world_matrices[1].cols[3][1] - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_keyframe_insert_sorted() {
        let mut curve: Curve<f32> = Curve::new();
        curve.insert_sorted(Keyframe::new(1.0, 10.0));
        curve.insert_sorted(Keyframe::new(0.0, 0.0));
        curve.insert_sorted(Keyframe::new(2.0, 20.0));
        curve.insert_sorted(Keyframe::new(0.5, 5.0));

        assert_eq!(curve.keyframes()[0].time(), 0.0);
        assert_eq!(curve.keyframes()[1].time(), 0.5);
        assert_eq!(curve.keyframes()[2].time(), 1.0);
        assert_eq!(curve.keyframes()[3].time(), 2.0);
    }

    #[test]
    fn test_curve_step_interpolation() {
        let mut curve: Curve<f32> = Curve::new();
        curve.push(Keyframe::with_interpolation(0.0, 0.0, Interpolation::Step));
        curve.push(Keyframe::with_interpolation(1.0, 10.0, Interpolation::Step));

        // Step interpolation should hold previous value
        assert_eq!(curve.sample_with_wrap(0.0, WrapMode::Once), 0.0);
        assert_eq!(curve.sample_with_wrap(0.5, WrapMode::Once), 0.0); // Still 0 until we hit 1.0
        assert_eq!(curve.sample_with_wrap(1.0, WrapMode::Once), 10.0);
    }
}
