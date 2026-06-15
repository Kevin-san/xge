#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, point: crate::Vec2) -> bool {
        point.x >= self.x &&
        point.x <= self.x + self.w &&
        point.y >= self.y &&
        point.y <= self.y + self.h
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.w &&
        self.x + self.w > other.x &&
        self.y < other.y + other.h &&
        self.y + self.h > other.y
    }

    pub fn min(&self) -> crate::Vec2 {
        crate::Vec2::new(self.x, self.y)
    }

    pub fn max(&self) -> crate::Vec2 {
        crate::Vec2::new(self.x + self.w, self.y + self.h)
    }

    pub fn center(&self) -> crate::Vec2 {
        crate::Vec2::new(self.x + self.w * 0.5, self.y + self.h * 0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_new() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert_eq!(r.x, 0.0);
        assert_eq!(r.y, 0.0);
        assert_eq!(r.w, 100.0);
        assert_eq!(r.h, 100.0);
    }

    #[test]
    fn rect_contains() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(r.contains(crate::Vec2::new(50.0, 50.0)));
        assert!(!r.contains(crate::Vec2::new(150.0, 50.0)));
        assert!(!r.contains(crate::Vec2::new(50.0, 150.0)));
    }

    #[test]
    fn rect_intersects() {
        let r1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let r2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        let r3 = Rect::new(200.0, 200.0, 100.0, 100.0);
        
        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }
}
