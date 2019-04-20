#![feature(test)]

extern crate test;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::thread;

fn hello_world() {
    info!("Hello World");
}

fn hello_world_threaded() {
    let t = thread::spawn(|| {
        info!("Hello World Threaded!");
    });

    t.join().unwrap();
}

fn main() {
    env_logger::init();
    hello_world();
    hello_world_threaded();
}

#[cfg(test)]
mod test_hello {
    use test::Bencher;
    use super::{hello_world, hello_world_threaded};

    #[bench]
    fn bench_hello_world(b: &mut Bencher) {
        b.iter(|| hello_world());
    }

    #[bench]
    fn bench_hello_world_threaded(b: &mut Bencher) {
        env_logger::init();
        b.iter(|| hello_world_threaded());
    }
}
