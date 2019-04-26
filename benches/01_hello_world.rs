#[macro_use]
extern crate criterion;

use criterion::Criterion;

pub fn hello_world() {
    let t = std::thread::spawn(move || {
        println!("Hello World");    
    });
    t.join();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hello_world", |b| b.iter(hello_world));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
