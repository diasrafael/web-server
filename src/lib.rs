use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use rand::prelude::*;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
    
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {

        let id = random();
        let thread = thread::spawn(move || loop {
            println!("Worker {} waiting...", id);
            //let mutex_guard = receiver.lock().unwrap(); // would make server serial
            let job = receiver.lock().unwrap().recv().expect("Got an error! no more messages can ever be received on this channel");
            println!("Worker {} got a job; executing.", id);
            job();
        });
        Worker{id, thread}
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new TheadPool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }
        ThreadPool{workers, sender}
    }

    pub fn execute<F>(&mut self, f: F)
    where F: FnOnce() + Send + 'static
    {
        println!("Thread pool sending job to workers...");
        self.sender.send(Box::new(f)).unwrap();
    }
}