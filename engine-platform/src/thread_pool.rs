use futures_lite::future;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type TaskQueue = Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    task_queue: TaskQueue,
}

struct Worker {
    #[allow(dead_code)]
    thread: Option<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new() -> Self {
        Self::new_with_threads(num_cpus())
    }

    pub fn new_with_threads(num_threads: usize) -> Self {
        let task_queue = Arc::new(Mutex::new(VecDeque::new()));

        let mut workers = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            let queue = task_queue.clone();
            let worker = Worker::new(queue);
            workers.push(worker);
        }

        Self {
            workers,
            task_queue,
        }
    }

    pub fn spawn<F: FnOnce() + Send + 'static>(&self, f: F) {
        let mut queue = self.task_queue.lock().unwrap();
        queue.push_back(Box::new(f));
    }

    pub fn spawn_future<F>(&self, future: F)
    where
        F: futures_lite::Future<Output = ()> + Send + 'static,
    {
        self.spawn(move || {
            future::block_on(future);
        });
    }

    pub fn try_spawn<F: FnOnce() + Send + 'static>(&self, f: F) -> bool {
        let mut queue = self.task_queue.lock().unwrap();
        if queue.len() < self.workers.len() * 10 {
            queue.push_back(Box::new(f));
            true
        } else {
            false
        }
    }

    pub fn block_on<F>(&self, f: F) -> F::Output
    where
        F: futures_lite::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        future::block_on(f)
    }

    pub fn shutdown(&self) {
        for _ in &self.workers {
            self.spawn(|| {});
        }
    }

    pub fn active_count(&self) -> usize {
        self.workers.len()
    }

    pub fn num_threads(&self) -> usize {
        self.workers.len()
    }

    pub fn pending_tasks(&self) -> usize {
        self.task_queue.lock().unwrap().len()
    }
}

impl Worker {
    fn new(task_queue: TaskQueue) -> Self {
        let thread = thread::spawn(move || loop {
            let task = {
                let mut queue = task_queue.lock().unwrap();
                queue.pop_front()
            };

            if let Some(task) = task {
                task();
            } else {
                thread::park_timeout(std::time::Duration::from_millis(1));
            }
        });

        Self {
            thread: Some(thread),
        }
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        Self::new()
    }
}

fn num_cpus() -> usize {
    thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_spawn() {
        let pool = ThreadPool::new_with_threads(2);
        let counter = Arc::new(AtomicUsize::new(0));

        for _i in 0..10 {
            let c = counter.clone();
            pool.spawn(move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_spawn_future() {
        let pool = ThreadPool::new_with_threads(2);
        let counter = Arc::new(AtomicUsize::new(0));

        for _i in 0..10 {
            let c = counter.clone();
            pool.spawn_future(async move {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_num_threads() {
        let pool = ThreadPool::new_with_threads(4);
        assert_eq!(pool.num_threads(), 4);
    }

    #[test]
    fn test_active_count() {
        let pool = ThreadPool::new_with_threads(4);
        assert_eq!(pool.active_count(), 4);
    }

    #[test]
    fn test_block_on() {
        let pool = ThreadPool::new();
        let result = pool.block_on(async { 42 });
        assert_eq!(result, 42);
    }
}