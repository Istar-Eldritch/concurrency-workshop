use std::thread;

// We want to increase the counter up to 10
// There is a bug in this program, the counter remains 0 always. Why?
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