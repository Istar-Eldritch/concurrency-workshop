use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct State {
    number: i32,
    
}

// Make this compile
fn main() {
    let five = Arc::new(State { number: 5 });

    for _ in 0..10 {
        let five = Arc::clone(&five);
        thread::spawn(move || {
            println!("{:?}", five);
        });
    }
}