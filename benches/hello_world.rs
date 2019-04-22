#[macro_use]
extern crate criterion;
#[macro_use]
extern crate log;
extern crate env_logger;

use criterion::Criterion;

pub fn hello_world() {
    info!("Hello World");
}

fn criterion_benchmark(c: &mut Criterion) {
    env_logger::init();
    c.bench_function("hello_world", |b| b.iter(hello_world));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
