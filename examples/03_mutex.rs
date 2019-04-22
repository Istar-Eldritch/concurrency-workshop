use std::thread;

// We want to increase the counter up to 10
// This program prints 0. Why?
// Do the required modifications to make it work as intented.
fn main() {
    let mut counter = 0;
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            counter += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter);
}