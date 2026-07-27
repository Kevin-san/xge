use crate::{Quat, Vec3};

#[derive(Clone)]
pub struct SoaVec3 {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

impl SoaVec3 {
    #[inline]
    pub fn new() -> Self {
        Self {
            x: Vec::new(),
            y: Vec::new(),
            z: Vec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            x: Vec::with_capacity(cap),
            y: Vec::with_capacity(cap),
            z: Vec::with_capacity(cap),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.x.len()
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.x.capacity()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }

    #[inline]
    pub fn push(&mut self, v: Vec3) {
        self.x.push(v.x);
        self.y.push(v.y);
        self.z.push(v.z);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.x.clear();
        self.y.clear();
        self.z.clear();
    }

    #[inline]
    pub fn iter(&self) -> SoaVec3Iter<'_> {
        SoaVec3Iter {
            soa: self,
            index: 0,
        }
    }

    #[inline]
    pub fn get_vec3(&self, i: usize) -> Vec3 {
        Vec3::new(self.x[i], self.y[i], self.z[i])
    }
}

impl Default for SoaVec3 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

pub struct SoaVec3Iter<'a> {
    soa: &'a SoaVec3,
    index: usize,
}

impl<'a> Iterator for SoaVec3Iter<'a> {
    type Item = Vec3;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.soa.len() {
            let v = self.soa.get_vec3(self.index);
            self.index += 1;
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.soa.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for SoaVec3Iter<'_> {}

#[derive(Clone)]
pub struct SoaQuat {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
    w: Vec<f32>,
}

impl SoaQuat {
    #[inline]
    pub fn new() -> Self {
        Self {
            x: Vec::new(),
            y: Vec::new(),
            z: Vec::new(),
            w: Vec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            x: Vec::with_capacity(cap),
            y: Vec::with_capacity(cap),
            z: Vec::with_capacity(cap),
            w: Vec::with_capacity(cap),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.x.len()
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.x.capacity()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }

    #[inline]
    pub fn push(&mut self, q: Quat) {
        self.x.push(q.x);
        self.y.push(q.y);
        self.z.push(q.z);
        self.w.push(q.w);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.x.clear();
        self.y.clear();
        self.z.clear();
        self.w.clear();
    }

    #[inline]
    pub fn iter(&self) -> SoaQuatIter<'_> {
        SoaQuatIter {
            soa: self,
            index: 0,
        }
    }

    #[inline]
    pub fn get_quat(&self, i: usize) -> Quat {
        Quat {
            x: self.x[i],
            y: self.y[i],
            z: self.z[i],
            w: self.w[i],
        }
    }
}

impl Default for SoaQuat {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

pub struct SoaQuatIter<'a> {
    soa: &'a SoaQuat,
    index: usize,
}

impl<'a> Iterator for SoaQuatIter<'a> {
    type Item = Quat;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.soa.len() {
            let v = self.soa.get_quat(self.index);
            self.index += 1;
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.soa.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for SoaQuatIter<'_> {}

#[inline]
pub fn aos_to_soa_vec3(aos: &[Vec3]) -> SoaVec3 {
    let mut soa = SoaVec3::with_capacity(aos.len());
    for v in aos {
        soa.push(*v);
    }
    soa
}

#[inline]
pub fn soa_to_aos_vec3(soa: &SoaVec3) -> Vec<Vec3> {
    let mut aos = Vec::with_capacity(soa.len());
    for v in soa.iter() {
        aos.push(v);
    }
    aos
}

#[inline]
pub fn aos_to_soa_quat(aos: &[Quat]) -> SoaQuat {
    let mut soa = SoaQuat::with_capacity(aos.len());
    for q in aos {
        soa.push(*q);
    }
    soa
}

#[inline]
pub fn soa_to_aos_quat(soa: &SoaQuat) -> Vec<Quat> {
    let mut aos = Vec::with_capacity(soa.len());
    for q in soa.iter() {
        aos.push(q);
    }
    aos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soa_vec3_new() {
        let soa = SoaVec3::new();
        assert_eq!(soa.len(), 0);
        assert!(soa.is_empty());
    }

    #[test]
    fn test_soa_vec3_push() {
        let mut soa = SoaVec3::new();
        soa.push(Vec3::new(1.0, 2.0, 3.0));
        soa.push(Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(soa.len(), 2);
        assert!(!soa.is_empty());
        assert_eq!(soa.get_vec3(0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(soa.get_vec3(1), Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_soa_vec3_clear() {
        let mut soa = SoaVec3::new();
        soa.push(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(soa.len(), 1);
        soa.clear();
        assert_eq!(soa.len(), 0);
        assert!(soa.is_empty());
    }

    #[test]
    fn test_soa_vec3_iter() {
        let mut soa = SoaVec3::new();
        soa.push(Vec3::new(1.0, 2.0, 3.0));
        soa.push(Vec3::new(4.0, 5.0, 6.0));
        soa.push(Vec3::new(7.0, 8.0, 9.0));

        let collected: Vec<Vec3> = soa.iter().collect();
        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0], Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(collected[1], Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(collected[2], Vec3::new(7.0, 8.0, 9.0));
    }

    #[test]
    fn test_soa_vec3_capacity() {
        let soa = SoaVec3::with_capacity(10);
        assert!(soa.cap() >= 10);
    }

    #[test]
    fn test_soa_vec3_aos_roundtrip() {
        let aos = vec![
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(7.0, 8.0, 9.0),
        ];
        let soa = aos_to_soa_vec3(&aos);
        assert_eq!(soa.len(), 3);
        let back = soa_to_aos_vec3(&soa);
        assert_eq!(back, aos);
    }

    #[test]
    fn test_soa_vec3_empty_roundtrip() {
        let aos: Vec<Vec3> = vec![];
        let soa = aos_to_soa_vec3(&aos);
        assert_eq!(soa.len(), 0);
        let back = soa_to_aos_vec3(&soa);
        assert_eq!(back, aos);
    }

    #[test]
    fn test_soa_quat_new() {
        let soa = SoaQuat::new();
        assert_eq!(soa.len(), 0);
        assert!(soa.is_empty());
    }

    #[test]
    fn test_soa_quat_push() {
        let mut soa = SoaQuat::new();
        soa.push(Quat::IDENTITY);
        soa.push(Quat::from_rotation_x(0.5));
        assert_eq!(soa.len(), 2);
        assert!(!soa.is_empty());
        assert_eq!(soa.get_quat(0), Quat::IDENTITY);
    }

    #[test]
    fn test_soa_quat_clear() {
        let mut soa = SoaQuat::new();
        soa.push(Quat::IDENTITY);
        assert_eq!(soa.len(), 1);
        soa.clear();
        assert_eq!(soa.len(), 0);
        assert!(soa.is_empty());
    }

    #[test]
    fn test_soa_quat_iter() {
        let mut soa = SoaQuat::new();
        soa.push(Quat::IDENTITY);
        soa.push(Quat::from_rotation_x(0.5));

        let collected: Vec<Quat> = soa.iter().collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0], Quat::IDENTITY);
    }

    #[test]
    fn test_soa_quat_capacity() {
        let soa = SoaQuat::with_capacity(10);
        assert!(soa.cap() >= 10);
    }

    #[test]
    fn test_soa_quat_aos_roundtrip() {
        let aos = vec![
            Quat::IDENTITY,
            Quat::from_rotation_x(0.5),
            Quat::from_rotation_y(1.0),
        ];
        let soa = aos_to_soa_quat(&aos);
        assert_eq!(soa.len(), 3);
        let back = soa_to_aos_quat(&soa);
        assert_eq!(back, aos);
    }

    #[test]
    fn test_soa_quat_empty_roundtrip() {
        let aos: Vec<Quat> = vec![];
        let soa = aos_to_soa_quat(&aos);
        assert_eq!(soa.len(), 0);
        let back = soa_to_aos_quat(&soa);
        assert_eq!(back, aos);
    }

    #[test]
    fn test_soa_vec3_clone() {
        let mut soa = SoaVec3::new();
        soa.push(Vec3::new(1.0, 2.0, 3.0));
        let cloned = soa.clone();
        assert_eq!(cloned.len(), 1);
        assert_eq!(cloned.get_vec3(0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_soa_quat_clone() {
        let mut soa = SoaQuat::new();
        soa.push(Quat::IDENTITY);
        let cloned = soa.clone();
        assert_eq!(cloned.len(), 1);
        assert_eq!(cloned.get_quat(0), Quat::IDENTITY);
    }

    #[test]
    fn test_soa_vec3_default() {
        let soa: SoaVec3 = Default::default();
        assert_eq!(soa.len(), 0);
    }

    #[test]
    fn test_soa_quat_default() {
        let soa: SoaQuat = Default::default();
        assert_eq!(soa.len(), 0);
    }

    #[test]
    fn test_soa_vec3_iter_exact_size() {
        let mut soa = SoaVec3::new();
        for i in 0..5 {
            soa.push(Vec3::new(i as f32, 0.0, 0.0));
        }
        let iter = soa.iter();
        assert_eq!(iter.len(), 5);
    }

    #[test]
    fn test_soa_quat_iter_exact_size() {
        let mut soa = SoaQuat::new();
        for _ in 0..5 {
            soa.push(Quat::IDENTITY);
        }
        let iter = soa.iter();
        assert_eq!(iter.len(), 5);
    }

    #[test]
    fn test_soa_vec3_large_scale() {
        let aos: Vec<Vec3> = (0..1000).map(|i| Vec3::new(i as f32, (i as f32) * 2.0, (i as f32) * 3.0)).collect();
        let soa = aos_to_soa_vec3(&aos);
        assert_eq!(soa.len(), 1000);
        let back = soa_to_aos_vec3(&soa);
        assert_eq!(back.len(), 1000);
        for i in 0..1000 {
            assert_eq!(back[i], aos[i]);
        }
    }

    #[test]
    fn test_soa_quat_large_scale() {
        let aos: Vec<Quat> = (0..500).map(|i| Quat::from_axis_angle(Vec3::X, i as f32 * 0.01)).collect();
        let soa = aos_to_soa_quat(&aos);
        assert_eq!(soa.len(), 500);
        let back = soa_to_aos_quat(&soa);
        assert_eq!(back.len(), 500);
        for i in 0..500 {
            let diff = (back[i].x - aos[i].x).abs()
                + (back[i].y - aos[i].y).abs()
                + (back[i].z - aos[i].z).abs()
                + (back[i].w - aos[i].w).abs();
            assert!(diff < 1e-6, "Quat mismatch at index {}", i);
        }
    }

    #[test]
    fn test_soa_vec3_push_individual() {
        let mut soa = SoaVec3::new();
        soa.push(Vec3::X);
        soa.push(Vec3::Y);
        soa.push(Vec3::Z);
        assert_eq!(soa.len(), 3);
        assert_eq!(soa.get_vec3(0), Vec3::X);
        assert_eq!(soa.get_vec3(1), Vec3::Y);
        assert_eq!(soa.get_vec3(2), Vec3::Z);
    }
}