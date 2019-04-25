#[macro_use]
extern crate criterion;
extern crate concurrency_workshop;

use criterion::Criterion;
use std::net::Ipv4Addr;

use concurrency_workshop::scanner::{scan_network, scan_machine};

fn criterion_benchmark_scan_network(c: &mut Criterion) {
    c.bench_function("scan_network", |b| b.iter(|| scan_network(Ipv4Addr::new(192, 168, 1, 1), Ipv4Addr::new(192, 168, 1, 2), 1, 20)));
}

fn criterion_benchmark_scan_machine(c: &mut Criterion) {
    c.bench_function("scan_machine", |b| b.iter(|| scan_machine(Ipv4Addr::new(192, 168, 1, 1), 1, 20)));
}

criterion_group!(benches, criterion_benchmark_scan_machine, criterion_benchmark_scan_network);
criterion_main!(benches);
