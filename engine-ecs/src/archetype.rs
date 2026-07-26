//! Archetype 原型分组存储
//! Archetype 原型分组存储
//!
//! 核心思想：
//! - 每个 **Archetype** 代表一种"组件集合"（如 {Position, Velocity}）。
//! - 所有具备相同组件集合的实体被放入同一个 Archetype。
//! - 每个 Archetype 内部，组件以 **列为单位** 连续存储（ColumnVec<C>），提供极佳缓存局部性。
//! - 组件的移动/删除是通过 `Column::move_row_to` 的类型擦除 trait 方法完成的。

use crate::component::{Component, ComponentSet};
use crate::entity::Entity;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;

// ============================================================
// Column traits & ColumnVec
// ============================================================

/// 类型擦除的组件列 —— World 通过它对 Archetype 进行无类型操作
pub trait Column: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    /// 删除 row 行（swap_remove 语义）
    fn swap_remove_row(&mut self, row: usize);
    /// 把 row 位置的值 *移动* 到 other 列末尾（不触发 on_remove/on_add）
    fn move_row_to(&mut self, row: usize, other: &mut dyn Column) -> bool;
    /// 创建一个空的、相同类型的列
    fn clone_empty(&self) -> Box<dyn Column>;
    /// 清空（对每个元素调用 on_remove）
    fn clear(&mut self);
    /// 元素数量
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// 本列的 TypeId
    fn type_id(&self) -> std::any::TypeId;
}

/// 具体类型 C 的稠密列
pub struct ColumnVec<C: Component> {
    data: Vec<C>,
    _marker: PhantomData<fn() -> C>,
}

impl<C: Component> ColumnVec<C> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, mut value: C) {
        value.on_add();
        self.data.push(value);
    }

    pub fn push_raw(&mut self, value: C) {
        self.data.push(value);
    }

    pub fn get(&self, row: usize) -> Option<&C> {
        self.data.get(row)
    }

    pub fn get_mut(&mut self, row: usize) -> Option<&mut C> {
        self.data.get_mut(row)
    }

    pub fn as_slice(&self) -> &[C] {
        &self.data
    }

    pub fn as_slice_mut(&mut self) -> &mut [C] {
        &mut self.data
    }

    pub fn replace(&mut self, row: usize, value: C) -> Option<C> {
        if row < self.data.len() {
            let old = std::mem::replace(&mut self.data[row], value);
            let mut old = old;
            old.on_remove();
            Some(old)
        } else {
            None
        }
    }

    /// 取出行中值（不触发 on_remove —— 调用方决定是否调用）
    pub fn swap_take_raw(&mut self, row: usize) -> Option<C> {
        if row >= self.data.len() {
            return None;
        }
        let last = self.data.len() - 1;
        if row != last {
            self.data.swap(row, last);
        }
        self.data.pop()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<C: Component> Default for ColumnVec<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Component> Column for ColumnVec<C> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn swap_remove_row(&mut self, row: usize) {
        if row >= self.data.len() {
            return;
        }
        let last = self.data.len() - 1;
        if row != last {
            self.data.swap(row, last);
        }
        if let Some(mut v) = self.data.pop() {
            v.on_remove();
        }
    }

    fn move_row_to(&mut self, row: usize, other: &mut dyn Column) -> bool {
        if row >= self.data.len() {
            return false;
        }
        if let Some(other_typed) = <dyn Column>::as_any_mut(other).downcast_mut::<Self>() {
            if let Some(value) = self.swap_take_raw(row) {
                other_typed.push_raw(value);
                return true;
            }
            false
        } else {
            false
        }
    }

    fn clone_empty(&self) -> Box<dyn Column> {
        Box::new(ColumnVec::<C>::new())
    }

    fn clear(&mut self) {
        for v in self.data.iter_mut() {
            v.on_remove();
        }
        self.data.clear();
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<C>()
    }
}

// ============================================================
// Archetype & ArchetypeStorage
// ============================================================

/// 一个 Archetype：组件集合 + 每个组件类型的稠密列 + entity 行映射
pub struct Archetype {
    pub id: u32,
    pub components: ComponentSet,
    pub columns: HashMap<std::any::TypeId, Box<dyn Column>>,
    pub entities: Vec<Entity>,
}

impl Archetype {
    pub fn new(id: u32, components: ComponentSet) -> Self {
        Self {
            id,
            components,
            columns: HashMap::new(),
            entities: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn has_component(&self, type_id: std::any::TypeId) -> bool {
        self.components.contains(type_id)
    }
}

pub struct ArchetypeStorage {
    archetypes: Vec<Archetype>,
    /// 组件集合 → archetype id（快速查找）
    index: HashMap<ComponentSet, u32>,
    next_id: u32,
}

impl ArchetypeStorage {
    pub fn new() -> Self {
        let mut s = Self {
            archetypes: Vec::new(),
            index: HashMap::new(),
            next_id: 0,
        };
        // id = 0：空组件集合的 archetype
        let empty = ComponentSet::empty();
        s.get_or_create(empty);
        s
    }

    pub fn archetype_count(&self) -> usize {
        self.archetypes.len()
    }

    pub fn entity_count(&self) -> usize {
        self.archetypes.iter().map(|a| a.entities.len()).sum()
    }

    pub fn get(&self, id: u32) -> Option<&Archetype> {
        self.archetypes.iter().find(|a| a.id == id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Archetype> {
        self.archetypes.iter_mut().find(|a| a.id == id)
    }

    /// 获取或创建指定组件集合的 archetype
    pub fn get_or_create(&mut self, components: ComponentSet) -> u32 {
        if let Some(&id) = self.index.get(&components) {
            return id;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.archetypes.push(Archetype::new(id, components.clone()));
        self.index.insert(components, id);
        id
    }

    /// push entity + 一个组件值 C（用于 spawn 或 insert 场景的填充）
    pub fn push_entity_with_component<C: Component>(
        &mut self,
        arch_id: u32,
        _entity: Entity,
        value: C,
    ) {
        let type_id = std::any::TypeId::of::<C>();
        let arch = self
            .archetypes
            .iter_mut()
            .find(|a| a.id == arch_id)
            .expect("archetype not found");
        // 确保 C 列存在
        arch.columns
            .entry(type_id)
            .or_insert_with(|| Box::new(ColumnVec::<C>::new()));
        // 对 *每个* 已有列：要么 push C（如果类型匹配），要么 push 一个空值？
        // —— 不，这里我们只保证列类型匹配的列 push 值，
        //    其他列的 push 应由调用方在 *同一循环里* 再调用本方法。
        //    但这样需要多次 find ，所以我们改为：
        //    *本方法只负责 push C 到 C 列*，不 push entity。
        //    调用方在所有组件 push 完后，再调用 finalize_push_entity(arch_id, entity)。
        let col = arch.columns.get_mut(&type_id).expect("column should exist");
        if let Some(cv) = <dyn Column>::as_any_mut(&mut **col).downcast_mut::<ColumnVec<C>>() {
            cv.push(value);
        } else {
            let mut cv = ColumnVec::<C>::new();
            cv.push(value);
            arch.columns.insert(type_id, Box::new(cv));
        }
        // 注意：entity 不在此处 push，调用方需最后调用 finalize_push_entity
    }

    /// 完成一次 push：把 entity id 加入该 archetype 的 entities 列表
    pub fn finalize_push_entity(&mut self, arch_id: u32, entity: Entity) -> u32 {
        let arch = self
            .archetypes
            .iter_mut()
            .find(|a| a.id == arch_id)
            .expect("archetype not found");
        let row = arch.entities.len();
        arch.entities.push(entity);
        row as u32
    }

    /// 向指定 archetype 的 C 列 push 值（不触发生命周期 hook，用于 move 场景）
    pub fn push_raw_component<C: Component>(&mut self, arch_id: u32, value: C) {
        let type_id = std::any::TypeId::of::<C>();
        let arch = self
            .archetypes
            .iter_mut()
            .find(|a| a.id == arch_id)
            .expect("archetype not found");
        arch.columns
            .entry(type_id)
            .or_insert_with(|| Box::new(ColumnVec::<C>::new()));
        let col = arch.columns.get_mut(&type_id).unwrap();
        if let Some(cv) = <dyn Column>::as_any_mut(&mut **col).downcast_mut::<ColumnVec<C>>() {
            cv.push_raw(value);
        } else {
            let mut cv = ColumnVec::<C>::new();
            cv.push_raw(value);
            arch.columns.insert(type_id, Box::new(cv));
        }
    }

    /// 获取只读组件引用
    pub fn get_component<C: Component>(&self, arch_id: u32, row: u32) -> Option<&C> {
        let type_id = std::any::TypeId::of::<C>();
        let arch = self.archetypes.iter().find(|a| a.id == arch_id)?;
        let col = arch.columns.get(&type_id)?;
        let typed = <dyn Column>::as_any(&**col).downcast_ref::<ColumnVec<C>>()?;
        typed.get(row as usize)
    }

    pub fn get_component_mut<C: Component>(&mut self, arch_id: u32, row: u32) -> Option<&mut C> {
        let type_id = std::any::TypeId::of::<C>();
        let arch = self.archetypes.iter_mut().find(|a| a.id == arch_id)?;
        let col = arch.columns.get_mut(&type_id)?;
        let typed = <dyn Column>::as_any_mut(&mut **col).downcast_mut::<ColumnVec<C>>()?;
        typed.get_mut(row as usize)
    }

    pub fn replace_component<C: Component>(
        &mut self,
        arch_id: u32,
        row: u32,
        value: C,
    ) -> Option<C> {
        let type_id = std::any::TypeId::of::<C>();
        let arch = self.archetypes.iter_mut().find(|a| a.id == arch_id)?;
        let col = arch.columns.get_mut(&type_id)?;
        let typed = <dyn Column>::as_any_mut(&mut **col).downcast_mut::<ColumnVec<C>>()?;
        typed.replace(row as usize, value)
    }

    pub fn take_component<C: Component>(&mut self, arch_id: u32, row: u32) -> Option<C> {
        let type_id = std::any::TypeId::of::<C>();
        let arch = self.archetypes.iter_mut().find(|a| a.id == arch_id)?;
        let col = arch.columns.get_mut(&type_id)?;
        let typed = <dyn Column>::as_any_mut(&mut **col).downcast_mut::<ColumnVec<C>>()?;
        typed.swap_take_raw(row as usize)
    }

    /// 删除 arch_id 的 row 行（对每列 swap_remove_row + entities.swap_remove(row)）
    /// 返回被 swap 到 row 位置的 entity（如果有），以便 World 层更新 location
    pub fn remove_row(&mut self, arch_id: u32, row: usize) -> Option<Entity> {
        let arch = self.archetypes.iter_mut().find(|a| a.id == arch_id)?;
        if row >= arch.entities.len() {
            return None;
        }
        // entities swap_remove
        let last = arch.entities.len() - 1;
        if row != last {
            arch.entities.swap(row, last);
        }
        arch.entities.pop();
        // 各列 swap_remove_row
        for col in arch.columns.values_mut() {
            col.swap_remove_row(row);
        }
        if row < arch.entities.len() {
            // 有别的 entity 被移到 row 位置了
            Some(arch.entities[row])
        } else {
            None
        }
    }

    /// 迁移 src_arch.row 的 *所有* 组件列到 dst_arch 的末尾（不触发生命周期）
    /// 返回：被 swap 到 row 位置的 entity（如果有）
    pub fn move_all_columns_row(
        &mut self,
        src_arch_id: u32,
        row: usize,
        dst_arch_id: u32,
    ) -> Option<Entity> {
        let except: &[TypeId] = &[];
        self.move_all_columns_row_except(src_arch_id, row, dst_arch_id, except)
    }

    /// 迁移 src_arch.row 的组件列到 dst_arch 的末尾（不触发生命周期），排除 except 中的类型
    /// 返回：被 swap 到 row 位置的 entity（如果有）
    pub fn move_all_columns_row_except(
        &mut self,
        src_arch_id: u32,
        row: usize,
        dst_arch_id: u32,
        except: &[TypeId],
    ) -> Option<Entity> {
        let src_idx = self.archetypes.iter().position(|a| a.id == src_arch_id)?;
        let dst_idx = self.archetypes.iter().position(|a| a.id == dst_arch_id)?;
        if src_idx == dst_idx {
            return None;
        }

        // 收集 src 中除 except 外的列类型
        let type_ids: Vec<TypeId> = {
            let src_arch = &self.archetypes[src_idx];
            src_arch
                .columns
                .keys()
                .copied()
                .filter(|t| !except.contains(t))
                .collect()
        };

        // 对每个列：移动 row 到 dst 列末尾
        for type_id in &type_ids {
            // 先确保 dst 有对应列
            let need_new_col = {
                let dst = &self.archetypes[dst_idx];
                !dst.columns.contains_key(type_id)
            };
            if need_new_col {
                let empty_col = {
                    let src_arch = &self.archetypes[src_idx];
                    src_arch.columns.get(type_id).map(|c| c.clone_empty())
                };
                if let Some(col) = empty_col {
                    let dst = &mut self.archetypes[dst_idx];
                    dst.columns.insert(*type_id, col);
                }
            }
            let (src_parts, dst_parts) =
                split_archetypes_mut(&mut self.archetypes, src_idx, dst_idx);
            if let (Some(src_arch), Some(dst_arch)) = (src_parts, dst_parts) {
                if let (Some(src_col), Some(dst_col)) = (
                    src_arch.columns.get_mut(type_id),
                    dst_arch.columns.get_mut(type_id),
                ) {
                    src_col.move_row_to(row, dst_col.as_mut());
                }
            }
        }

        // 对 entities 列表：swap_remove(row)
        let src_arch = &mut self.archetypes[src_idx];
        if row >= src_arch.entities.len() {
            return None;
        }

        // 注意：except 列已经由调用方处理（例如 take_component 已从中删除 row）
        // 因此此处不要对 except 列再做任何修改，以免误删其他 entity 的值

        let last = src_arch.entities.len() - 1;
        if row != last {
            src_arch.entities.swap(row, last);
        }
        let _removed = src_arch.entities.pop();
        if row < src_arch.entities.len() {
            Some(src_arch.entities[row])
        } else {
            None
        }
    }

    pub fn iter_archetypes(&self) -> impl Iterator<Item = &Archetype> {
        self.archetypes.iter()
    }

    /// 遍历匹配 required 且不包含 excluded 中任何类型的 archetype
    pub fn iter_matching<'a>(
        &'a self,
        required: &'a ComponentSet,
        excluded: &'a ComponentSet,
    ) -> impl Iterator<Item = &'a Archetype> + 'a {
        self.archetypes.iter().filter(move |a| {
            if a.is_empty() {
                return false;
            }
            if !a.components.contains_all(required) {
                return false;
            }
            if !a.components.is_disjoint(excluded) {
                return false;
            }
            true
        })
    }

    pub fn column_slice<C: Component>(&self, arch_id: u32) -> Option<&[C]> {
        let type_id = std::any::TypeId::of::<C>();
        let arch = self.archetypes.iter().find(|a| a.id == arch_id)?;
        let col = arch.columns.get(&type_id)?;
        let typed = <dyn Column>::as_any(&**col).downcast_ref::<ColumnVec<C>>()?;
        Some(typed.as_slice())
    }

    pub fn column_slice_mut<C: Component>(&mut self, arch_id: u32) -> Option<&mut [C]> {
        let type_id = std::any::TypeId::of::<C>();
        let arch = self.archetypes.iter_mut().find(|a| a.id == arch_id)?;
        let col = arch.columns.get_mut(&type_id)?;
        let typed = <dyn Column>::as_any_mut(&mut **col).downcast_mut::<ColumnVec<C>>()?;
        Some(typed.as_slice_mut())
    }

    pub fn clear(&mut self) {
        for a in self.archetypes.iter_mut() {
            a.entities.clear();
            for col in a.columns.values_mut() {
                col.clear();
            }
        }
    }
}

impl Default for ArchetypeStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// 辅助函数：从 archetype Vec 中同时拿到两个 &mut Archetype
fn split_archetypes_mut(
    slice: &mut [Archetype],
    i: usize,
    j: usize,
) -> (Option<&mut Archetype>, Option<&mut Archetype>) {
    if i == j {
        return (slice.get_mut(i), None);
    }
    if i < j {
        let (left, right) = slice.split_at_mut(j);
        (left.get_mut(i), right.get_mut(0))
    } else {
        let (left, right) = slice.split_at_mut(i);
        (right.get_mut(0), left.get_mut(j))
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Position {
        x: f32,
    }
    impl Component for Position {}

    #[derive(Debug, Clone, PartialEq)]
    struct Velocity {
        dx: f32,
    }
    impl Component for Velocity {}

    #[derive(Debug, Clone, PartialEq)]
    struct Tag;
    impl Component for Tag {}

    #[test]
    fn test_storage_creation() {
        let s = ArchetypeStorage::new();
        assert_eq!(s.archetype_count(), 1); // 空 archetype
        assert_eq!(s.entity_count(), 0);
    }

    #[test]
    fn test_get_or_create_same_key() {
        let mut s = ArchetypeStorage::new();
        let key = ComponentSet::new(vec![std::any::TypeId::of::<Position>()]);
        let id1 = s.get_or_create(key.clone());
        let id2 = s.get_or_create(key);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_push_and_get_component() {
        let mut s = ArchetypeStorage::new();
        let key = ComponentSet::new(vec![
            std::any::TypeId::of::<Position>(),
            std::any::TypeId::of::<Velocity>(),
        ]);
        let arch = s.get_or_create(key);
        let entity = Entity::new(42, 0);

        s.push_entity_with_component::<Position>(arch, entity, Position { x: 1.0 });
        s.push_entity_with_component::<Velocity>(arch, entity, Velocity { dx: 2.0 });
        let row = s.finalize_push_entity(arch, entity);

        assert_eq!(s.get_component::<Position>(arch, row).unwrap().x, 1.0);
        assert_eq!(s.get_component::<Velocity>(arch, row).unwrap().dx, 2.0);
    }

    #[test]
    fn test_move_all_columns_row_and_remove() {
        // 模拟 World::insert 给 entity 增加新组件类型
        let mut s = ArchetypeStorage::new();
        let pos_only_key = ComponentSet::new(vec![std::any::TypeId::of::<Position>()]);
        let pos_vel_key = ComponentSet::new(vec![
            std::any::TypeId::of::<Position>(),
            std::any::TypeId::of::<Velocity>(),
        ]);
        let arch_pos = s.get_or_create(pos_only_key);
        let arch_pv = s.get_or_create(pos_vel_key);

        let e = Entity::new(7, 0);
        s.push_entity_with_component::<Position>(arch_pos, e, Position { x: 5.0 });
        let row = s.finalize_push_entity(arch_pos, e);

        // 现在模拟 "insert Velocity"：
        // 1. 从 arch_pos row 位置移动 Position 到 arch_pv
        s.move_all_columns_row(arch_pos, row as usize, arch_pv);
        // 2. push Velocity 新值到 arch_pv
        s.push_entity_with_component::<Velocity>(arch_pv, e, Velocity { dx: 9.0 });
        // 3. push entity 到 arch_pv entities
        s.finalize_push_entity(arch_pv, e);
        // 4. 从 arch_pos 移除 row（此时 arch_pos 的 Position 列 row 位置被 swap_take_raw 后，
        //    那列已经少了一个元素；remove_row 会清理 entities vec 并让其他列也 swap_remove_row）
        let swapped = s.remove_row(arch_pos, row as usize);
        // 这里 arch_pos 只有一个 entity，移除后 swapped = None
        assert!(swapped.is_none());

        // 验证：entity 现在在 arch_pv 中
        let arch = s.get(arch_pv).unwrap();
        assert_eq!(arch.entities.len(), 1);
        assert_eq!(arch.entities[0], e);
        // 其 Position = 5.0, Velocity = 9.0
        let pos = s.get_component::<Position>(arch_pv, 0).unwrap();
        let vel = s.get_component::<Velocity>(arch_pv, 0).unwrap();
        assert_eq!(pos.x, 5.0);
        assert_eq!(vel.dx, 9.0);
    }

    #[test]
    fn test_remove_row_swap_updates_entity_location() {
        let mut s = ArchetypeStorage::new();
        let key = ComponentSet::new(vec![std::any::TypeId::of::<Position>()]);
        let arch = s.get_or_create(key);

        // 填入 3 个 entities
        let entities: Vec<_> = (0..3u32).map(|i| Entity::new(i, 0)).collect();
        for (i, &e) in entities.iter().enumerate() {
            s.push_entity_with_component::<Position>(arch, e, Position { x: i as f32 });
            s.finalize_push_entity(arch, e);
        }
        // 删除 row=1（entity id=1）。末尾 entity(id=2) 被移到 row=1 位置
        let swapped = s.remove_row(arch, 1);
        assert!(swapped.is_some());
        assert_eq!(swapped.unwrap().id(), 2);
        // 此时 row=1 位置的 Position 应该是原来 entity 2 的 Position（x = 2.0）
        let p = s.get_component::<Position>(arch, 1).unwrap();
        assert_eq!(p.x, 2.0);
        // 总实体数 = 2
        assert_eq!(s.entity_count(), 2);
    }

    #[test]
    fn test_iter_matching() {
        let mut s = ArchetypeStorage::new();
        let pos_key = ComponentSet::new(vec![std::any::TypeId::of::<Position>()]);
        let pv_key = ComponentSet::new(vec![
            std::any::TypeId::of::<Position>(),
            std::any::TypeId::of::<Velocity>(),
        ]);
        let tag_key = ComponentSet::new(vec![std::any::TypeId::of::<Tag>()]);
        let arch_pos = s.get_or_create(pos_key);
        let arch_pv = s.get_or_create(pv_key);
        let arch_tag = s.get_or_create(tag_key);

        s.push_entity_with_component::<Position>(arch_pos, Entity::new(0, 0), Position { x: 1.0 });
        s.finalize_push_entity(arch_pos, Entity::new(0, 0));
        s.push_entity_with_component::<Position>(arch_pv, Entity::new(1, 0), Position { x: 2.0 });
        s.push_entity_with_component::<Velocity>(arch_pv, Entity::new(1, 0), Velocity { dx: 3.0 });
        s.finalize_push_entity(arch_pv, Entity::new(1, 0));
        s.push_entity_with_component::<Tag>(arch_tag, Entity::new(2, 0), Tag);
        s.finalize_push_entity(arch_tag, Entity::new(2, 0));

        // 要求 Position，排除 Velocity → 应该只有 arch_pos
        let req = ComponentSet::new(vec![std::any::TypeId::of::<Position>()]);
        let exc = ComponentSet::new(vec![std::any::TypeId::of::<Velocity>()]);
        let matches: Vec<_> = s.iter_matching(&req, &exc).collect();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].id, arch_pos);

        // 要求 Position + Velocity → 只有 arch_pv
        let req2 = ComponentSet::new(vec![
            std::any::TypeId::of::<Position>(),
            std::any::TypeId::of::<Velocity>(),
        ]);
        let empty = ComponentSet::empty();
        let matches2: Vec<_> = s.iter_matching(&req2, &empty).collect();
        assert_eq!(matches2.len(), 1);
        assert_eq!(matches2[0].id, arch_pv);
    }

    // 生命周期测试
    #[test]
    fn test_lifecycle_hooks() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        thread_local! {
            static ADD: RefCell<Arc<AtomicUsize>> = RefCell::new(Arc::new(AtomicUsize::new(0)));
            static REM: RefCell<Arc<AtomicUsize>> = RefCell::new(Arc::new(AtomicUsize::new(0)));
        }
        use std::cell::RefCell;
        use std::sync::Arc;

        struct LifecycleComp;
        impl Component for LifecycleComp {
            fn on_add(&mut self) {
                ADD.with(|c| c.borrow().fetch_add(1, Ordering::SeqCst));
            }
            fn on_remove(&mut self) {
                REM.with(|c| c.borrow().fetch_add(1, Ordering::SeqCst));
            }
        }

        let mut s = ArchetypeStorage::new();
        let key = ComponentSet::new(vec![std::any::TypeId::of::<LifecycleComp>()]);
        let arch = s.get_or_create(key);
        s.push_entity_with_component::<LifecycleComp>(arch, Entity::new(0, 0), LifecycleComp);
        s.finalize_push_entity(arch, Entity::new(0, 0));

        let add1 = ADD.with(|c| c.borrow().load(Ordering::SeqCst));
        assert_eq!(add1, 1);

        // remove_row 应触发 on_remove
        s.remove_row(arch, 0);
        let rem1 = REM.with(|c| c.borrow().load(Ordering::SeqCst));
        assert_eq!(rem1, 1);
    }

    #[test]
    fn test_columnvec_swap_take_raw_empty() {
        let mut col: ColumnVec<Position> = ColumnVec::new();
        assert!(col.swap_take_raw(0).is_none());
        assert!(col.swap_take_raw(100).is_none());
    }

    #[test]
    fn test_columnvec_swap_take_raw_out_of_bounds() {
        let mut col: ColumnVec<Position> = ColumnVec::new();
        col.push(Position { x: 1.0 });
        col.push(Position { x: 2.0 });
        assert!(col.swap_take_raw(2).is_none());
        assert!(col.swap_take_raw(100).is_none());
        assert_eq!(col.len(), 2);
    }

    #[test]
    fn test_columnvec_swap_take_raw_valid() {
        let mut col: ColumnVec<Position> = ColumnVec::new();
        col.push(Position { x: 1.0 });
        col.push(Position { x: 2.0 });
        col.push(Position { x: 3.0 });

        let val = col.swap_take_raw(1);
        assert!(val.is_some());
        assert_eq!(val.unwrap().x, 2.0);
        assert_eq!(col.len(), 2);
        assert_eq!(col.get(0).unwrap().x, 1.0);
        assert_eq!(col.get(1).unwrap().x, 3.0);
    }

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn test_archetype_storage_send_sync() {
        assert_send::<ArchetypeStorage>();
        assert_sync::<ArchetypeStorage>();
    }
}
