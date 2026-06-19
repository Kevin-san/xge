//! Buffer 模块 - 通用 GPU 缓冲区抽象
//!
//! 提供 `Buffer<T>`、`VertexBuffer`、`IndexBuffer` 等后端无关的缓冲区类型。
//! 这些类型不依赖具体的图形后端，仅在 CPU 端维护缓冲区数据与元信息。
//! 实际与 GPU 的绑定由 `Renderer` trait 的实现（如 OpenGL / wgpu）完成。

use std::marker::PhantomData;

/// 缓冲区用途
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BufferUsage {
    /// 顶点缓冲区
    #[default]
    Vertex,
    /// 索引缓冲区
    Index,
    /// 统一缓冲区（Uniform）
    Uniform,
    /// 存储缓冲区（Storage）
    Storage,
}

/// 缓冲区内存使用策略
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BufferMemoryHint {
    /// 静态：上传后很少修改，GPU 高频访问
    #[default]
    Static,
    /// 动态：需要频繁修改
    Dynamic,
    /// 流：每帧更新一次
    Stream,
}

/// 通用 GPU 缓冲区（类型 T）
///
/// 内部使用字节存储，支持连续数据追加和随机访问。
/// 实际上传到 GPU 的逻辑由具体 `Renderer` 实现处理。
pub struct Buffer<T> {
    /// 原始数据（字节级存储）
    data: Vec<u8>,
    /// 当前元素数
    count: usize,
    /// 容量（元素单位）
    capacity: usize,
    /// 缓冲区用途
    usage: BufferUsage,
    /// 内存策略
    hint: BufferMemoryHint,
    /// 数据是否自上次上传后发生变化
    dirty: bool,
    /// 类型占位
    _marker: PhantomData<T>,
}

impl<T: Copy> Buffer<T> {
    /// 创建空缓冲区（默认容量 = 1024 个元素）
    pub fn new(usage: BufferUsage) -> Self {
        Self::with_capacity(usage, 1024)
    }

    /// 创建指定容量的缓冲区（元素数）
    pub fn with_capacity(usage: BufferUsage, capacity: usize) -> Self {
        let byte_size = capacity * std::mem::size_of::<T>();
        Self {
            data: Vec::with_capacity(byte_size),
            count: 0,
            capacity,
            usage,
            hint: BufferMemoryHint::default(),
            dirty: false,
            _marker: PhantomData,
        }
    }

    /// 从已有元素切片构造缓冲区
    pub fn from_slice(usage: BufferUsage, slice: &[T]) -> Self {
        let mut buf = Self::with_capacity(usage, slice.len());
        buf.extend_from_slice(slice);
        buf
    }

    /// 从 Vec<T> 构造缓冲区（零额外拷贝）
    pub fn from_vec(usage: BufferUsage, vec: Vec<T>) -> Self {
        let count = vec.len();
        let capacity = vec.capacity();
        let byte_size = count * std::mem::size_of::<T>();
        let mut data: Vec<u8> = Vec::with_capacity(byte_size);
        unsafe {
            std::ptr::copy_nonoverlapping(vec.as_ptr() as *const u8, data.as_mut_ptr(), byte_size);
            data.set_len(byte_size);
            std::mem::forget(vec);
        }
        Self {
            data,
            count,
            capacity,
            usage,
            hint: BufferMemoryHint::Static,
            dirty: true,
            _marker: PhantomData,
        }
    }

    /// 设置内存使用策略
    pub fn set_memory_hint(&mut self, hint: BufferMemoryHint) {
        self.hint = hint;
    }

    /// 获取内存使用策略
    pub fn memory_hint(&self) -> BufferMemoryHint {
        self.hint
    }

    /// 获取缓冲区用途
    pub fn usage(&self) -> BufferUsage {
        self.usage
    }

    /// 当前元素数
    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }

    /// 当前容量（元素数）
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// 是否需要重新上传到 GPU
    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// 标记已上传（清除 dirty 标志）
    #[inline]
    pub fn mark_uploaded(&mut self) {
        self.dirty = false;
    }

    /// 以元素为单位追加数据
    pub fn push(&mut self, value: T) {
        if self.count >= self.capacity {
            self.grow(1);
        }
        let byte_offset = self.count * std::mem::size_of::<T>();
        let end = byte_offset + std::mem::size_of::<T>();
        if self.data.len() < end {
            self.data.resize(end, 0);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value as *const T as *const u8,
                self.data.as_mut_ptr().add(byte_offset),
                std::mem::size_of::<T>(),
            );
        }
        self.count += 1;
        self.dirty = true;
    }

    /// 批量追加数据
    pub fn extend_from_slice(&mut self, slice: &[T]) {
        let needed = self.count + slice.len();
        if needed > self.capacity {
            self.grow(slice.len());
        }
        let byte_offset = self.count * std::mem::size_of::<T>();
        let byte_len = slice.len() * std::mem::size_of::<T>();
        let end = byte_offset + byte_len;
        if self.data.len() < end {
            self.data.resize(end, 0);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                slice.as_ptr() as *const u8,
                self.data.as_mut_ptr().add(byte_offset),
                byte_len,
            );
        }
        self.count += slice.len();
        self.dirty = true;
    }

    /// 保留至少 `additional` 个额外元素的空间
    pub fn reserve(&mut self, additional: usize) {
        if self.count + additional > self.capacity {
            self.grow(additional);
        }
    }

    /// 清空缓冲区数据（保留分配的内存）
    pub fn clear(&mut self) {
        self.count = 0;
        self.data.clear();
        self.dirty = true;
    }

    /// 获取数据的字节视图（只读）
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.count * std::mem::size_of::<T>()]
    }

    /// 获取元素数相关的字节大小
    pub fn byte_len(&self) -> usize {
        self.count * std::mem::size_of::<T>()
    }

    /// 获取单个元素的字节大小
    pub fn element_size(&self) -> usize {
        std::mem::size_of::<T>()
    }

    /// 获取数据作为类型化切片（只读）
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.count)
        }
    }

    /// 随机访问（安全）
    pub fn get(&self, index: usize) -> Option<T> {
        if index < self.count {
            unsafe {
                let ptr = self.data.as_ptr().add(index * std::mem::size_of::<T>()) as *const T;
                Some(std::ptr::read(ptr))
            }
        } else {
            None
        }
    }

    /// 更新某一位置的数据
    pub fn set(&mut self, index: usize, value: T) -> bool {
        if index < self.count {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(index * std::mem::size_of::<T>()) as *mut T;
                std::ptr::write(ptr, value);
            }
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// 按需求扩容
    fn grow(&mut self, additional: usize) {
        let new_cap = (self.capacity + additional).max(16).next_power_of_two();
        self.capacity = new_cap;
        self.data.reserve(new_cap * std::mem::size_of::<T>() - self.data.len());
    }
}

impl<T: Copy> Default for Buffer<T> {
    fn default() -> Self {
        Self::new(BufferUsage::Vertex)
    }
}

/// 顶点缓冲区（类型别名）
pub type VertexBuffer = Buffer<f32>;

/// 索引缓冲区（类型别名）
pub type IndexBuffer = Buffer<u32>;

/// 索引缓冲区 16 位版本（适用于移动端 / 小网格）
pub type IndexBuffer16 = Buffer<u16>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_new_and_count() {
        let buf: Buffer<f32> = Buffer::new(BufferUsage::Vertex);
        assert_eq!(buf.count(), 0);
        assert!(buf.is_empty());
    }

    #[test]
    fn test_buffer_push() {
        let mut buf: Buffer<f32> = Buffer::with_capacity(BufferUsage::Vertex, 8);
        buf.push(1.0);
        buf.push(2.0);
        buf.push(3.0);
        assert_eq!(buf.count(), 3);
        assert_eq!(buf.get(0), Some(1.0));
        assert_eq!(buf.get(1), Some(2.0));
        assert_eq!(buf.get(2), Some(3.0));
        assert_eq!(buf.get(100), None);
    }

    #[test]
    fn test_buffer_extend_from_slice() {
        let mut buf: Buffer<f32> = Buffer::with_capacity(BufferUsage::Vertex, 4);
        buf.extend_from_slice(&[10.0, 20.0, 30.0, 40.0, 50.0]);
        assert_eq!(buf.count(), 5);
        assert_eq!(buf.get(4), Some(50.0));
    }

    #[test]
    fn test_buffer_set() {
        let mut buf: Buffer<u32> = Buffer::with_capacity(BufferUsage::Index, 4);
        buf.push(10);
        buf.push(20);
        buf.push(30);
        assert!(buf.set(1, 200));
        assert_eq!(buf.get(1), Some(200));
        assert!(!buf.set(100, 99));
    }

    #[test]
    fn test_buffer_clear() {
        let mut buf: Buffer<f32> = Buffer::new(BufferUsage::Vertex);
        buf.extend_from_slice(&[1.0, 2.0, 3.0]);
        assert!(!buf.is_empty());
        buf.clear();
        assert!(buf.is_empty());
        assert_eq!(buf.count(), 0);
    }

    #[test]
    fn test_buffer_dirty_flag() {
        let mut buf: Buffer<f32> = Buffer::new(BufferUsage::Vertex);
        buf.push(1.0);
        assert!(buf.is_dirty());
        buf.mark_uploaded();
        assert!(!buf.is_dirty());
        buf.push(2.0);
        assert!(buf.is_dirty());
    }

    #[test]
    fn test_buffer_usage_and_hint() {
        let mut buf: Buffer<u32> = Buffer::new(BufferUsage::Index);
        assert_eq!(buf.usage(), BufferUsage::Index);
        buf.set_memory_hint(BufferMemoryHint::Dynamic);
        assert_eq!(buf.memory_hint(), BufferMemoryHint::Dynamic);
    }

    #[test]
    fn test_buffer_element_size() {
        let buf: Buffer<f32> = Buffer::new(BufferUsage::Vertex);
        assert_eq!(buf.element_size(), 4);

        let buf2: Buffer<u16> = Buffer::new(BufferUsage::Index);
        assert_eq!(buf2.element_size(), 2);

        let buf3: Buffer<u32> = Buffer::new(BufferUsage::Index);
        assert_eq!(buf3.element_size(), 4);
    }

    #[test]
    fn test_buffer_byte_len() {
        let mut buf: Buffer<f32> = Buffer::new(BufferUsage::Vertex);
        buf.extend_from_slice(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(buf.byte_len(), 16);
    }

    #[test]
    fn test_buffer_as_bytes() {
        let mut buf: Buffer<f32> = Buffer::new(BufferUsage::Vertex);
        buf.push(1.0);
        let bytes = buf.as_bytes();
        assert_eq!(bytes.len(), 4);
    }

    #[test]
    fn test_buffer_reserve_grow() {
        let mut buf: Buffer<f32> = Buffer::with_capacity(BufferUsage::Vertex, 2);
        // 2 个元素 — 达到容量
        buf.push(1.0);
        buf.push(2.0);
        // 触发 grow
        buf.push(3.0);
        assert_eq!(buf.count(), 3);
        assert!(buf.capacity() > 2);
    }

    #[test]
    fn test_vertex_buffer_alias() {
        let vb: VertexBuffer = VertexBuffer::new(BufferUsage::Vertex);
        let _ = vb;
    }

    #[test]
    fn test_index_buffer_alias() {
        let mut ib: IndexBuffer = IndexBuffer::new(BufferUsage::Index);
        ib.push(0);
        ib.push(1);
        ib.push(2);
        assert_eq!(ib.count(), 3);
    }

    #[test]
    fn test_buffer_memory_hints() {
        let mut buf: Buffer<f32> = Buffer::new(BufferUsage::Vertex);
        buf.set_memory_hint(BufferMemoryHint::Static);
        assert_eq!(buf.memory_hint(), BufferMemoryHint::Static);
        buf.set_memory_hint(BufferMemoryHint::Stream);
        assert_eq!(buf.memory_hint(), BufferMemoryHint::Stream);
        buf.set_memory_hint(BufferMemoryHint::Dynamic);
        assert_eq!(buf.memory_hint(), BufferMemoryHint::Dynamic);
    }

    #[test]
    fn test_buffer_usage_enum_values() {
        assert_eq!(BufferUsage::Vertex as u32, 0);
    }

    #[test]
    fn test_buffer_large_push_and_get() {
        let mut buf: Buffer<f32> = Buffer::with_capacity(BufferUsage::Vertex, 4);
        for i in 0..1000 {
            buf.push(i as f32);
        }
        assert_eq!(buf.count(), 1000);
        assert_eq!(buf.get(999), Some(999.0));
    }

    #[test]
    fn test_default_buffer() {
        let buf: Buffer<f32> = Default::default();
        assert_eq!(buf.usage(), BufferUsage::Vertex);
        assert!(buf.is_empty());
    }
}
