#[macro_use]
extern crate criterion;
extern crate concurrency_workshop;

use criterion::Criterion;
use std::net::Ipv4Addr;

use concurrency_workshop::scanner::scan_network;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("scanner", |b| b.iter(|| scan_network(Ipv4Addr::new(192, 168, 1, 1), Ipv4Addr::new(192, 168, 1, 2))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
