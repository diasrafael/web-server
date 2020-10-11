use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use rand::prelude::*;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
    
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {

        let id = random();
        let thread = thread::spawn(move || loop {
            println!("Worker {} waiting...", id);
            //let mutex_guard = receiver.lock().unwrap(); // would make server serial
            let message = receiver.lock().unwrap().recv().expect("Got an error! no more messages can ever be received on this channel");
            println!("Worker {} got a job; executing.", id);
            match message {
                Message::NewJob(job) => job(),
                Message::Terminate => break,
            }
        });
        Worker{id, thread: Some(thread)}
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

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
        self.sender.send(Message::NewJob(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        /*
        If we tried to send a message and join immediately 
        in the same loop, we couldn’t guarantee that the worker 
        in the current iteration would be the one to get 
        the message from the channel.
        */

        for worker in &mut self.workers {    
            println!("Shutting down worker {}", worker.id);

            if let Some(t) = worker.thread.take() {
                t.join().unwrap();
            }
        }
    }
}