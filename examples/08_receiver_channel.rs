use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() {

    let mut threads = vec![];

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    for tn in 1..=2 {
        let rx = rx.clone();
        let t = thread::spawn(move || {
            loop {
                let _ = rx.lock().unwrap().recv().unwrap();
                println!("Received on thread {}", tn);
            }            
        });

        threads.push(t);
    }

    for i in 1..=4 {
        tx.send(format!("test {}", i)).unwrap();
    }

    for handler in threads {
        handler.join().unwrap();
    }
}