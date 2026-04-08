use std::sync::{Mutex as StdMutex, RwLock as StdRwLock, Arc as StdArc};

/// A mutual exclusion primitive
#[derive(Debug)]
pub struct Mutex<T: Send + Sync> {
    inner: StdMutex<T>,
}

impl<T: Send + Sync> Mutex<T> {
    /// Creates a new mutex wrapping the given value
    pub fn new(value: T) -> Self {
        Mutex {
            inner: StdMutex::new(value),
        }
    }

    /// Locks the mutex and returns a guard
    pub fn lock(&self) -> Result<MutexGuard<'_, T>, std::sync::PoisonError<std::sync::MutexGuard<'_, T>>> {
        self.inner.lock().map(|guard| MutexGuard { guard })
    }
}

/// A guard that provides mutable access to the mutex's data
#[derive(Debug)]
pub struct MutexGuard<'a, T: Send + Sync> {
    guard: std::sync::MutexGuard<'a, T>,
}

impl<'a, T: Send + Sync> std::ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a, T: Send + Sync> std::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

/// A reader-writer lock
#[derive(Debug)]
pub struct RwLock<T: Send + Sync> {
    inner: StdRwLock<T>,
}

impl<T: Send + Sync> RwLock<T> {
    /// Creates a new RwLock wrapping the given value
    pub fn new(value: T) -> Self {
        RwLock {
            inner: StdRwLock::new(value),
        }
    }

    /// Acquires a read lock
    pub fn read(&self) -> Result<RwLockReadGuard<'_, T>, std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>> {
        self.inner.read().map(|guard| RwLockReadGuard { guard })
    }

    /// Acquires a write lock
    pub fn write(&self) -> Result<RwLockWriteGuard<'_, T>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>> {
        self.inner.write().map(|guard| RwLockWriteGuard { guard })
    }
}

/// A guard that provides shared access to the RwLock's data
#[derive(Debug)]
pub struct RwLockReadGuard<'a, T: Send + Sync> {
    guard: std::sync::RwLockReadGuard<'a, T>,
}

impl<'a, T: Send + Sync> std::ops::Deref for RwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

/// A guard that provides mutable access to the RwLock's data
#[derive(Debug)]
pub struct RwLockWriteGuard<'a, T: Send + Sync> {
    guard: std::sync::RwLockWriteGuard<'a, T>,
}

impl<'a, T: Send + Sync> std::ops::Deref for RwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a, T: Send + Sync> std::ops::DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

/// An atomically reference counted pointer
#[derive(Debug, Clone)]
pub struct Arc<T: Send + Sync> {
    inner: StdArc<T>,
}

impl<T: Send + Sync> Arc<T> {
    /// Creates a new Arc wrapping the given value
    pub fn new(value: T) -> Self {
        Arc {
            inner: StdArc::new(value),
        }
    }

    /// Gets a reference to the inner value
    pub fn get(&self) -> &T {
        &*self.inner
    }
}

/// Atomic boolean
pub use std::sync::atomic::AtomicBool;

/// Atomic 32-bit signed integer
pub use std::sync::atomic::AtomicI32;

/// Atomic 32-bit unsigned integer
pub use std::sync::atomic::AtomicU32;

/// Atomic 64-bit signed integer
pub use std::sync::atomic::AtomicI64;

/// Atomic 64-bit unsigned integer
pub use std::sync::atomic::AtomicU64;

/// Atomic pointer-sized signed integer
pub use std::sync::atomic::AtomicIsize;

/// Atomic pointer-sized unsigned integer
pub use std::sync::atomic::AtomicUsize;

/// Memory ordering modes
pub use std::sync::atomic::Ordering;
