
use std::thread;
use std::sync::mpsc::Sender;

pub type Job =  Box<FnOnce() + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>, // Keep the list of handlers.
    tx: Sender<Job> // Send jobs to the threads
}

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        // Create a new ThreadPool
        panic!("Not implemented");
    }

    pub fn execute<F>(&self, function: F)
        where
            F: FnOnce() + Send + 'static 
    {
        let job = Box::new(function);
        // TODO: send the job to be executed to one of the threads
    }
}