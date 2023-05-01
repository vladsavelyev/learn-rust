use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert_ne!(size, 0, "Pool size cannot be zero");

        let (sender, receiver) = mpsc::channel();

        // There can be only on receiver (mpsc = multiple producer single consumer),
        // thus compiler won't let us copy receivers in a loop. But we actually aren't
        // going to use more than one receiver at a time! That's why we are going to
        // wrap the references to our receiver in a mutex, so only one worker at a time
        // can access the receiver; additionally, we will wrap them in an Arc
        // (atomic reference counter), so the receiver would stay alive as long as our
        // Arc object and all references wrapped in Arc::clone are alive. Atomic means
        // that we can pass it around to other threads.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        println!("Queueing into the thread tool");
        println!("Number of workers: {}", self.workers.len());
        if let Some(sender) = self.sender.as_ref() {
            sender.send(Box::new(f)).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for w in &mut self.workers {
            println!("Shutting down worker {}", w.id);
            if let Some(t) = w.thread.take() {
                t.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {id} got a job, executing");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected, shutting down");
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
