use std::thread;

// Make this compile!

#[derive(Debug)]
struct Test {
    n: u32,
}

fn print(obj: &Test) {
    let t = thread::spawn(move || {
        dbg!(obj);
    });

    t.join().unwrap();
}

fn main() {
    let obj = Test { n : 3};
    print(&obj);
    dbg!(obj);
}