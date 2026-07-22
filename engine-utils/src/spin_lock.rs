use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        SpinLockGuard { lock: self }
    }

    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        if !self.locked.swap(true, Ordering::Acquire) {
            Some(SpinLockGuard { lock: self })
        } else {
            None
        }
    }
}

impl<T: Default> Default for SpinLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::thread;

    #[test]
    fn test_lock_unlock() {
        let lock = SpinLock::new(0);
        {
            let mut guard = lock.lock();
            *guard = 42;
        }
        let guard = lock.lock();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_try_lock() {
        let lock = SpinLock::new(0);
        let guard1 = lock.try_lock();
        assert!(guard1.is_some());

        let guard2 = lock.try_lock();
        assert!(guard2.is_none());
    }

    #[test]
    fn test_concurrent_access() {
        let lock = Arc::new(SpinLock::new(AtomicUsize::new(0)));
        let mut handles = Vec::new();

        for _ in 0..10 {
            let lock_clone = lock.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    let mut guard = lock_clone.lock();
                    guard.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let guard = lock.lock();
        assert_eq!(guard.load(std::sync::atomic::Ordering::Relaxed), 10000);
    }

    #[test]
    fn test_default() {
        let lock: SpinLock<i32> = SpinLock::default();
        let mut guard = lock.lock();
        assert_eq!(*guard, 0);
        *guard = 5;
    }
}