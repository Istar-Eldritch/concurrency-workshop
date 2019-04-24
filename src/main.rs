
extern crate concurrency_workshop;

use concurrency_workshop::scanner::scan_network;

use std::net::Ipv4Addr;

fn main() {
    scan_network(Ipv4Addr::new(192, 168, 1, 1), Ipv4Addr::new(192, 168, 1, 255));
}