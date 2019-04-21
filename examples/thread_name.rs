use std::thread;

fn main() {
    let handle1 = thread::spawn(move || {
        loop {
            thread::sleep_ms(100);
        }
    });

    let thread_builder = thread::Builder::new()
        .name("Named Child".into());
    let handle2 = thread_builder.spawn(move || {
        loop {
            thread::sleep_ms(100);
        }
    }).unwrap();

    handle1.join().unwrap();
    handle2.join().unwrap();
}