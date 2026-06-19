//! 层级系统 - Parent/Children 组件实现
//!
//! 提供父子层级关系的 ECS 组件和系统，支持层级变换传播。

use crate::{Component, Entity, World};

/// 父实体组件
///
/// 标记一个实体是谁的子实体。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parent(pub Entity);

impl Component for Parent {}

/// 子实体列表组件
///
/// 存储一个实体的所有直接子实体。
#[derive(Debug, Clone)]
pub struct Children {
    /// 直接子实体
    pub entities: Vec<Entity>,
}

impl Component for Children {}

impl Children {
    /// 创建空的 Children
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    /// 使用已有的子实体列表创建 Children
    pub fn with_children(entities: impl IntoIterator<Item = Entity>) -> Self {
        Self {
            entities: entities.into_iter().collect(),
        }
    }

    /// 返回子实体数量
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// 检查是否有子实体
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::new()
    }
}

/// 层级命令扩展
///
/// 提供便捷的层级操作命令。
pub trait HierarchyCommandsExt {
    /// 设置实体的父实体
    fn set_parent(&mut self, entity: Entity, parent: Option<Entity>);

    /// 添加子实体到父实体
    fn push_children(&mut self, parent: Entity, children: impl IntoIterator<Item = Entity>);

    /// 移除最后一个子实体并返回
    fn pop_children(&mut self, parent: Entity) -> Vec<Entity>;

    /// 移除所有子实体
    fn remove_children(&mut self, parent: Entity);
}

/// World 层级扩展
///
/// 为 World 提供层级相关操作。
pub trait WorldHierarchyExt {
    /// 设置实体的父实体
    fn set_parent(&mut self, entity: Entity, parent: Option<Entity>);

    /// 获取实体的父实体
    fn get_parent(&self, entity: Entity) -> Option<Entity>;

    /// 获取实体的直接子实体
    fn get_children(&self, entity: Entity) -> Option<&Children>;

    /// 获取实体的直接子实体（可变）
    fn get_children_mut(&mut self, entity: Entity) -> Option<&mut Children>;

    /// 获取实体的所有祖先（父、父的父...）
    fn get_ancestors(&self, entity: Entity) -> Vec<Entity>;

    /// 获取实体的所有后代（子、子的子...）
    fn get_descendants(&self, entity: Entity) -> Vec<Entity>;

    /// 从父实体断开连接
    fn detach_from_parent(&mut self, entity: Entity);

    /// 检查两个实体是否是父子关系
    fn is_parent_of(&self, parent: Entity, child: Entity) -> bool;

    /// 检查实体是否是根实体（没有父实体）
    fn is_root(&self, entity: Entity) -> bool;
}

impl WorldHierarchyExt for World {
    fn set_parent(&mut self, entity: Entity, parent: Option<Entity>) {
        // 移除旧的父子关系
        if let Some(old_parent) = self.get_parent(entity) {
            if let Some(children) = self.get_children_mut(old_parent) {
                children.entities.retain(|&e| e != entity);
            }
        }

        // 建立新的父子关系
        if let Some(new_parent) = parent {
            // 如果实体是它自己的父（自环），则忽略
            if new_parent == entity {
                return;
            }

            self.insert(entity, Parent(new_parent));

            // 确保父实体有 Children 组件
            if self.get_component::<Children>(new_parent).is_none() {
                self.insert(new_parent, Children::new());
            }

            if let Some(children) = self.get_children_mut(new_parent) {
                // 避免重复添加
                if !children.entities.contains(&entity) {
                    children.entities.push(entity);
                }
            }
        } else {
            // 断开与父实体的连接
            self.remove::<Parent>(entity);
        }
    }

    fn get_parent(&self, entity: Entity) -> Option<Entity> {
        self.get_component::<Parent>(entity).map(|p| p.0)
    }

    fn get_children(&self, entity: Entity) -> Option<&Children> {
        self.get_component::<Children>(entity)
    }

    fn get_children_mut(&mut self, entity: Entity) -> Option<&mut Children> {
        self.get_component_mut::<Children>(entity)
    }

    fn get_ancestors(&self, entity: Entity) -> Vec<Entity> {
        let mut ancestors = Vec::new();
        let mut current = self.get_parent(entity);
        while let Some(parent) = current {
            ancestors.push(parent);
            current = self.get_parent(parent);
        }
        ancestors
    }

    fn get_descendants(&self, entity: Entity) -> Vec<Entity> {
        let mut descendants = Vec::new();
        let mut stack: Vec<Entity> = self
            .get_children(entity)
            .map(|c| c.entities.clone())
            .unwrap_or_default();

        while let Some(child) = stack.pop() {
            descendants.push(child);
            if let Some(children) = self.get_children(child) {
                stack.extend_from_slice(&children.entities);
            }
        }
        descendants
    }

    fn detach_from_parent(&mut self, entity: Entity) {
        self.set_parent(entity, None);
    }

    fn is_parent_of(&self, parent: Entity, child: Entity) -> bool {
        self.get_parent(child) == Some(parent)
    }

    fn is_root(&self, entity: Entity) -> bool {
        self.get_parent(entity).is_none()
    }
}

/// 变换传播系统
///
/// 将父级变换应用到子级。需要与 Transform 组件配合使用。
///
/// 注意：此系统需要 Transform 组件存在才能正常工作。
/// 目前是简化版本，实际使用需要结合具体的变换组件。
pub fn propagate_transforms_system(_world: &mut World) {
    // TODO: 实现完整的变换传播
    // 需要与 Transform 组件配合：
    // 1. 获取所有根实体（没有 Parent 组件的实体）
    // 2. 自上而下递归传播变换矩阵
    // 3. 子实体的最终变换 = 父实体变换 * 子实体局部变换
}

/// 销毁实体及其所有后代
pub fn despawn_with_children_system(world: &mut World, entity: Entity) -> bool {
    // 获取所有后代
    let descendants = world.get_descendants(entity);

    // 先销毁所有后代（从叶节点到根）
    for descendant in descendants.iter().rev() {
        world.despawn(*descendant);
    }

    // 最后销毁自身
    world.despawn(entity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent_child_relationship() {
        let mut world = World::new();

        let parent = world.spawn();
        let child = world.spawn();

        world.set_parent(child, Some(parent));

        // 检查子实体的父实体
        assert_eq!(world.get_parent(child), Some(parent));

        // 检查父实体的子实体
        let children = world.get_children(parent);
        assert!(children.is_some());
        assert_eq!(children.unwrap().entities.len(), 1);
        assert_eq!(children.unwrap().entities[0], child);
    }

    #[test]
    fn test_ancestors() {
        let mut world = World::new();

        let root = world.spawn();
        let parent = world.spawn();
        let child = world.spawn();

        world.set_parent(parent, Some(root));
        world.set_parent(child, Some(parent));

        let ancestors = world.get_ancestors(child);
        assert_eq!(ancestors.len(), 2);
        assert_eq!(ancestors[0], parent);
        assert_eq!(ancestors[1], root);
    }

    #[test]
    fn test_descendants() {
        let mut world = World::new();

        let root = world.spawn();
        let child1 = world.spawn();
        let child2 = world.spawn();
        let grandchild = world.spawn();

        world.set_parent(child1, Some(root));
        world.set_parent(child2, Some(root));
        world.set_parent(grandchild, Some(child1));

        let descendants = world.get_descendants(root);
        assert_eq!(descendants.len(), 3);
        assert!(descendants.contains(&child1));
        assert!(descendants.contains(&child2));
        assert!(descendants.contains(&grandchild));
    }

    #[test]
    fn test_detach_from_parent() {
        let mut world = World::new();

        let parent = world.spawn();
        let child = world.spawn();

        world.set_parent(child, Some(parent));
        world.detach_from_parent(child);

        assert!(world.get_parent(child).is_none());
        assert!(
            world.get_children(parent).is_none() || world.get_children(parent).unwrap().is_empty()
        );
    }

    #[test]
    fn test_self_parent_rejection() {
        let mut world = World::new();

        let entity = world.spawn();

        // 设置自己为父应该被忽略
        world.set_parent(entity, Some(entity));

        // 实体不应该有父实体
        assert!(world.get_parent(entity).is_none());
    }

    #[test]
    fn test_remove_parent_via_set_parent_none() {
        let mut world = World::new();

        let parent = world.spawn();
        let child = world.spawn();

        world.set_parent(child, Some(parent));
        world.set_parent(child, None);

        assert!(world.get_parent(child).is_none());
    }

    #[test]
    fn test_parent_child_multiple_children() {
        let mut world = World::new();
        let parent = world.spawn();
        let child1 = world.spawn();
        let child2 = world.spawn();
        let child3 = world.spawn();
        world.set_parent(child1, Some(parent));
        world.set_parent(child2, Some(parent));
        world.set_parent(child3, Some(parent));
        let children = world.get_children(parent).unwrap();
        assert_eq!(children.entities.len(), 3);
    }

    #[test]
    fn test_three_level_hierarchy() {
        let mut world = World::new();
        let root = world.spawn();
        let mid = world.spawn();
        let leaf = world.spawn();
        world.set_parent(mid, Some(root));
        world.set_parent(leaf, Some(mid));
        let ancestors = world.get_ancestors(leaf);
        assert!(ancestors.contains(&root));
    }

    #[test]
    fn test_get_parent_none() {
        let mut world = World::new();
        let e = world.spawn();
        assert!(world.get_parent(e).is_none());
    }

    #[test]
    fn test_children_is_owned() {
        let mut world = World::new();
        let parent = world.spawn();
        let child = world.spawn();
        world.set_parent(child, Some(parent));
        // children 字段存在验证
        assert_eq!(world.get_children(parent).iter().count(), 1);
    }
}
