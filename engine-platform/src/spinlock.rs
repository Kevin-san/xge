//! 轻量自旋锁 — 用于高频短持有场景

use core::sync::atomic::{AtomicBool, Ordering};
use core::ops::{Deref, DerefMut};

/// 轻量自旋锁
pub struct SpinLock<T> {
    lock: AtomicBool,
    data: core::cell::UnsafeCell<T>,
}

// SAFETY: SpinLock 通过原子操作保证互斥访问
unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: core::cell::UnsafeCell::new(data),
        }
    }

    /// 获取锁（自旋等待）
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self.lock.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
        SpinLockGuard { lock: self }
    }

    /// 尝试获取锁
    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        if self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            Some(SpinLockGuard { lock: self })
        } else {
            None
        }
    }
}

/// 自旋锁守卫
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: Guard holds the lock, so exclusive access is guaranteed
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: Guard holds the lock with mutable access
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.lock.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic_lock() {
        let lock = SpinLock::new(42);
        let guard = lock.lock();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_mutate() {
        let lock = SpinLock::new(0);
        {
            let mut guard = lock.lock();
            *guard = 100;
        }
        assert_eq!(*lock.lock(), 100);
    }

    #[test]
    fn test_try_lock() {
        let lock = SpinLock::new(1);
        let guard1 = lock.try_lock();
        assert!(guard1.is_some());
        let guard2 = lock.try_lock();
        assert!(guard2.is_none());
    }

    #[test]
    fn test_concurrent_access() {
        let lock = Arc::new(SpinLock::new(0usize));
        let mut handles = vec![];

        for _ in 0..4 {
            let lock_clone = lock.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    let mut guard = lock_clone.lock();
                    *guard += 1;
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*lock.lock(), 4000);
    }
}
