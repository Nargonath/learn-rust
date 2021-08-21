use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for count in 0..size {
            workers.push(Worker::new(count, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    // NOTE: This implementation didn't work because the for loop doesn't drop temporary value
    // in its right hand component until the end of its associated block unlike the let operator.
    // This means the MutexGuard, returned by .lock(), responsible for holding the lock is kept alive until the
    // job callback is done running which prevents the others worker to pick up a job in the
    // meantime.
    //
    // fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    //     let thread = thread::spawn(move || {
    //         for job in (*receiver.lock().unwrap()).iter() {
    //             println!("Worker {} got a job; executing.", id);
    //             job();
    //         }
    //     });

    //     Worker { id, thread }
    // }
    //
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}
