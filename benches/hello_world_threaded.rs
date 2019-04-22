#[macro_use]
extern crate criterion;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::thread;
use criterion::Criterion;

pub fn hello_world_threaded() {
    let t = thread::spawn(|| {
        info!("Hello World Threaded!");
    });

    t.join().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    env_logger::init();
    c.bench_function("hello_world_threaded", |b| b.iter(hello_world_threaded));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
