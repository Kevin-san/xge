use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use futures_lite::future::Boxed;
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct ThreadPool {
    sender: Sender<Boxed<'static, dyn FnOnce() -> () + Send>>,
    workers: Vec<thread::JoinHandle<()>>,
    shutdown_signal: Arc<std::sync::atomic::AtomicBool>,
}

pub struct JoinHandle<T> {
    inner: Option<thread::JoinHandle<T>>,
}

impl<T> JoinHandle<T> {
    pub fn join(self) -> thread::Result<T> {
        self.inner.unwrap().join()
    }
}

impl ThreadPool {
    pub fn new(num_threads: Option<usize>) -> Result<Self, anyhow::Error> {
        let num_threads = num_threads.unwrap_or_else(|| {
            std::cmp::max(1, num_cpus::get() - 1)
        });

        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let shutdown_signal = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let mut workers = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            let receiver = receiver.clone();
            let shutdown_signal = shutdown_signal.clone();
            
            let worker = thread::spawn(move || {
                while !shutdown_signal.load(std::sync::atomic::Ordering::SeqCst) {
                    if let Ok(task) = receiver.lock().recv_timeout(std::time::Duration::from_millis(100)) {
                        task();
                    }
                }
            });
            
            workers.push(worker);
        }

        Ok(Self {
            sender,
            workers,
            shutdown_signal,
        })
    }

    pub fn spawn<F, R>(&self, f: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (result_sender, result_receiver) = channel();
        
        let task = Box::pin(move || {
            let result = f();
            let _ = result_sender.send(result);
        });
        
        self.sender.send(task).unwrap();
        
        let handle = thread::spawn(move || {
            result_receiver.recv().unwrap()
        });
        
        JoinHandle { inner: Some(handle) }
    }

    pub fn try_spawn<F, R>(&self, f: F) -> Result<JoinHandle<R>, anyhow::Error>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (result_sender, result_receiver) = channel();
        
        let task = Box::pin(move || {
            let result = f();
            let _ = result_sender.send(result);
        });
        
        self.sender.try_send(task)?;
        
        let handle = thread::spawn(move || {
            result_receiver.recv().unwrap()
        });
        
        Ok(JoinHandle { inner: Some(handle) })
    }

    pub fn block_on<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.spawn(f).join().unwrap()
    }

    pub fn shutdown(&mut self) {
        self.shutdown_signal.store(true, std::sync::atomic::Ordering::SeqCst);
        
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }

    pub fn active_count(&self) -> usize {
        self.workers.len()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn tp_new() {
        let tp = ThreadPool::new(Some(2)).unwrap();
        assert_eq!(tp.active_count(), 2);
    }

    #[test]
    fn tp_spawn() {
        let tp = ThreadPool::new(Some(2)).unwrap();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        let handle = tp.spawn(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
        
        handle.join().unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn tp_spawn_with_result() {
        let tp = ThreadPool::new(Some(2)).unwrap();
        let handle = tp.spawn(|| 42);
        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn tp_block_on() {
        let tp = ThreadPool::new(Some(2)).unwrap();
        let result = tp.block_on(|| 100);
        assert_eq!(result, 100);
    }

    #[test]
    fn tp_shutdown() {
        let mut tp = ThreadPool::new(Some(2)).unwrap();
        assert_eq!(tp.active_count(), 2);
        tp.shutdown();
        assert_eq!(tp.active_count(), 0);
    }

    #[test]
    fn tp_multiple_tasks() {
        let tp = ThreadPool::new(Some(4)).unwrap();
        let counter = Arc::new(AtomicUsize::new(0));
        
        let mut handles = Vec::new();
        for _ in 0..10 {
            let counter_clone = counter.clone();
            handles.push(tp.spawn(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }
}
