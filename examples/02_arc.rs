use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct State {
    number: i32,
}

// Make this compile
fn main() {
    let five = State { number: 5 };

    for _ in 0..10 {
        thread::spawn(move || {
            println!("{:?}", five);
        });
    }
}