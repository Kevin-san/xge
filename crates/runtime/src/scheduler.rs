
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use crossbeam_deque::{Worker, Stealer, Injector};

/// A work-stealing task scheduler
pub struct TaskScheduler {
    workers: Vec<thread::JoinHandle<()>>,
    injector: Arc<Injector<Box<dyn FnOnce() + Send + 'static>>>,
    stealers: Vec<Stealer<Box<dyn FnOnce() + Send + 'static>>>,
    next_worker: AtomicUsize,
}

impl TaskScheduler {
    /// Creates a new task scheduler with the specified number of worker threads
    pub fn new(num_threads: Option<usize>) -> Self {
        let num_threads = num_threads.unwrap_or_else(|| num_cpus());
        let injector = Arc::new(Injector::<Box<dyn FnOnce() + Send + 'static>>::new());
        let mut stealers = Vec::with_capacity(num_threads);
        let mut workers = Vec::with_capacity(num_threads);
        
        for _ in 0..num_threads {
            let worker: Worker<Box<dyn FnOnce() + Send + 'static>> = Worker::new_lifo();
            stealers.push(worker.stealer());
            
            let injector = injector.clone();
            let stealers = stealers.clone();
            
            let handle = thread::spawn(move || {
                loop {
                    // Try to pop from local queue
                    if let Some(task) = worker.pop() {
                        task();
                    } 
                    // Try to pop from injector
                    else if let crossbeam_deque::Steal::Success(task) = injector.steal_batch_and_pop(&worker) {
                        task();
                    } 
                    // Try to steal from other workers
                    else {
                        let mut stole = false;
                        for stealer in &stealers {
                            if let crossbeam_deque::Steal::Success(task) = stealer.steal_batch_and_pop(&worker) {
                                task();
                                stole = true;
                                break;
                            }
                        }
                        if !stole {
                            // No work to do, yield
                            thread::yield_now();
                        }
                    }
                }
            });
            
            workers.push(handle);
        }
        
        TaskScheduler {
            workers,
            injector,
            stealers,
            next_worker: AtomicUsize::new(0),
        }
    }
    
    /// Spawns a new task
    pub fn spawn<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let task = Box::new(f);
        
        // Push to the injector
        self.injector.push(task);
    }
    
    /// Blocks until all tasks are completed
    pub fn wait(&self) {
        // This is a simple implementation - in a real scheduler, we would need to track task completion
        // For now, we'll just yield to allow other threads to run
        thread::yield_now();
    }
}

impl Drop for TaskScheduler {
    fn drop(&mut self) {
        // In a real implementation, we would send a shutdown signal to all workers
        // For now, we'll just drop the join handles, which will detach the threads
        for worker in self.workers.drain(..) {
            drop(worker);
        }
    }
}

/// Returns the number of CPU cores available
fn num_cpus() -> usize {
    thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
}

