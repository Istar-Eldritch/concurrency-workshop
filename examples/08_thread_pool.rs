
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;

pub type Job =  Box<FnOnce() + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>, // Keep the list of handlers.
    tx: Sender<Job> // Send jobs to the threads
}

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        // Create a new ThreadPool
        let (tx, rx) = channel();
        let mut threads = vec![];
        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..size {
            let rx = rx.clone();
            let t = thread::spawn(move || {
                loop {
                    let job = rx.lock().unwrap().recv();
                    job();
                }
            });
            threads.push(t);
        }

        ThreadPool { threads, tx}
    }

    pub fn execute<F>(&self, function: F)
        where
            F: FnOnce() + Send + 'static 
    {
        let job = Box::new(function);
        self.tx.send(job);

        // TODO: send the job to be executed to one of the threads
    }
}


let pool = ThreadPool::new(4);

pool.execute(|| {

});
