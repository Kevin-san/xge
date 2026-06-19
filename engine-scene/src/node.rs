//! 节点模块
//!
//! 定义 Node trait 和 NodeHandle 类型。

use std::any::Any;

/// 节点句柄
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NodeHandle(u32);

impl NodeHandle {
    /// 创建新的句柄
    pub fn new(index: u32) -> Self {
        Self(index)
    }

    /// 获取索引
    pub fn index(&self) -> u32 {
        self.0
    }

    /// 空句柄
    pub fn null() -> Self {
        Self(u32::MAX)
    }

    /// 是否是空句柄
    pub fn is_null(&self) -> bool {
        self.0 == u32::MAX
    }
}

/// 节点 trait
///
/// 所有场景节点必须实现此 trait。
pub trait Node {
    /// 获取节点名称
    fn name(&self) -> &str;

    /// 获取父节点句柄
    fn parent(&self) -> Option<NodeHandle>;

    /// 获取子节点列表
    fn children(&self) -> &[NodeHandle];

    /// 获取是否暂停
    fn paused(&self) -> bool;

    /// 获取可见性
    fn visible(&self) -> bool;

    /// 首次创建后调用
    fn on_ready(&mut self);

    /// 每帧更新
    fn on_update(&mut self, dt: f32);

    /// 绘制回调
    fn on_draw(&self);

    /// 销毁回调
    fn on_destroy(&mut self);

    /// 添加子节点
    fn add_child(&mut self, child: NodeHandle);

    /// 移除子节点
    fn remove_child(&mut self, child: NodeHandle);

    /// 设置父节点
    fn set_parent(&mut self, parent: Option<NodeHandle>);

    /// 从父节点分离
    fn detach(&mut self);

    /// 设置暂停状态
    fn set_paused(&mut self, paused: bool);

    /// 设置可见性
    fn set_visible(&mut self, visible: bool);

    /// 设置名称
    fn set_name(&mut self, name: String);

    /// 节点类型标识符（用于序列化）
    fn node_type(&self) -> &'static str {
        "Node"
    }

    /// 返回 Any 引用（用于 downcast）
    fn as_any(&self) -> &dyn Any;

    /// 返回 Any 可变引用（用于 downcast）
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 为 dyn Node 提供类型转换方法
impl dyn Node {
    /// 转换为具体类型引用
    pub fn downcast_ref<T: Node + 'static>(&self) -> Option<&T> {
        if self.name() == std::any::type_name::<T>() {
            // 这是一个简化的检查，实际实现需要更复杂的类型追踪
            None
        } else {
            None
        }
    }

    /// 转换为具体类型可变引用
    pub fn downcast_mut<T: Node + 'static>(&mut self) -> Option<&mut T> {
        // 由于 Rust 不提供运行时类型检查，我们需要其他方法
        // 这里返回一个 None，实际实现需要存储类型信息
        let _ = std::any::type_name::<T>();
        None
    }
}
