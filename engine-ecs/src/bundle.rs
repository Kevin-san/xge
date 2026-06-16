//! Bundle 模块
//!
//! 定义 Bundle trait 用于批量插入/移除组件。

use super::{Component, World};

/// Bundle trait
///
/// 实现此 trait 的类型可以批量插入/移除组件。
pub trait Bundle: Send + Sync + 'static {
    /// 批量插入组件到世界
    fn insert(self, world: &mut World, entity: crate::Entity)
    where
        Self: Sized;
}

/// Bundle for 单个组件
impl<C: Component + Send + Sync + 'static> Bundle for C {
    fn insert(self, world: &mut World, entity: crate::Entity) {
        world.insert(entity, self);
    }
}

/// Bundle for 两个组件
impl<C1: Component + Send + Sync + 'static, C2: Component + Send + Sync + 'static> Bundle for (C1, C2) {
    fn insert(self, world: &mut World, entity: crate::Entity) {
        world.insert(entity, self.0);
        world.insert(entity, self.1);
    }
}

/// Bundle for 三个组件
impl<C1: Component + Send + Sync + 'static, C2: Component + Send + Sync + 'static, C3: Component + Send + Sync + 'static> Bundle for (C1, C2, C3) {
    fn insert(self, world: &mut World, entity: crate::Entity) {
        world.insert(entity, self.0);
        world.insert(entity, self.1);
        world.insert(entity, self.2);
    }
}

/// Bundle for 四个组件
impl<C1: Component + Send + Sync + 'static, C2: Component + Send + Sync + 'static, C3: Component + Send + Sync + 'static, C4: Component + Send + Sync + 'static> Bundle for (C1, C2, C3, C4) {
    fn insert(self, world: &mut World, entity: crate::Entity) {
        world.insert(entity, self.0);
        world.insert(entity, self.1);
        world.insert(entity, self.2);
        world.insert(entity, self.3);
    }
}

/// Bundle for 五个组件
impl<C1: Component + Send + Sync + 'static, C2: Component + Send + Sync + 'static, C3: Component + Send + Sync + 'static, C4: Component + Send + Sync + 'static, C5: Component + Send + Sync + 'static> Bundle for (C1, C2, C3, C4, C5) {
    fn insert(self, world: &mut World, entity: crate::Entity) {
        world.insert(entity, self.0);
        world.insert(entity, self.1);
        world.insert(entity, self.2);
        world.insert(entity, self.3);
        world.insert(entity, self.4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as engine_ecs;
    use crate::Entity;

    #[derive(Debug, Clone, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    impl Component for Position {}

    #[derive(Debug, Clone, PartialEq)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    impl Component for Velocity {}

    #[test]
    fn test_bundle_single() {
        let mut world = World::new();
        let entity = world.spawn();

        let pos = Position { x: 1.0, y: 2.0 };
        pos.insert(&mut world, entity);

        assert!(world.get_component::<Position>(entity).is_some());
    }

    #[test]
    fn test_bundle_tuple() {
        let mut world = World::new();
        let entity = world.spawn();

        let bundle = (Position { x: 1.0, y: 2.0 }, Velocity { x: 0.5, y: 0.5 });
        bundle.insert(&mut world, entity);

        assert!(world.get_component::<Position>(entity).is_some());
        assert!(world.get_component::<Velocity>(entity).is_some());
    }
}
