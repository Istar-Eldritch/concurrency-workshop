use std::thread;

fn main() {
    let t = thread::spawn(|| {
        println!("Hello, world!");
    });

    t.join().unwrap();
}