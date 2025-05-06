use std::error::Error;
use std::thread::{self};
use std::sync::{mpsc, Arc, Mutex};

/// Simple thread-pool realization
///
/// Consist of workers vec that represents threads
/// and sender that transfer closures
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

/// This struct made for optimising thread-pool management
/// Workers represent threads in thread-pool
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciver) = mpsc::channel();
        let reciver = Arc::new(Mutex::new(reciver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciver)));
        }

        ThreadPool{ workers, sender: Some(sender) }
    }

    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, &'static str> {
        if size < 1 {
            return Err("Size must be greater than 0");
        }

        let (sender, reciver) = mpsc::channel();
        let reciver = Arc::new(Mutex::new(reciver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciver)));
        }

        Ok(ThreadPool { workers: workers, sender: Some(sender) })
    }

    /// Send a job to the thread pool
    /// 
    /// 
    /// # Panics
    /// 
    /// The `execute` function will panic if the thread pool has been shut down.
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers.drain(..) {
            println!("Shutting down {} thread", worker.id);

            worker.thread.join().unwrap();
        }
    }
}


impl Worker {
    /// This function creates single thread-pool's thread
    /// and goes on infinity loop of closure awaiting
    fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = reciver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker{ id, thread }
    }

    /// This function creates single thread-pool's thread
    /// and goes on infinity loop of closure awaiting
    pub fn build(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, Box<dyn Error>> {
        let thread = thread::spawn(move || loop {
            let lock = reciver.lock();
            match lock {
                Ok(mutex) => {
                    let message = mutex.recv();
                    match message {
                        Ok(job) => {
                            println!("Worker {id} got a job; executing.");
                            job();
                        },
                        Err(e) => {eprint!("Worker {id} disconnected; shutting down.");panic!("{e}")}
                    }
                }
                Err(e) => { eprintln!("Error in locking closure {:#?} at {} thread", e, id); panic!("{e}")}
            }
        });

        Ok(Worker{ id, thread })

    }
}