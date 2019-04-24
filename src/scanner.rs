use std::net::Ipv4Addr;
use std::thread;

use crate::utils::{to_i32, to_ipv4};

pub fn scan_network(from: Ipv4Addr, to: Ipv4Addr) {
    let from = to_i32(from);
    let to = to_i32(to);

    for ip in from..=to {
        scan_machine(to_ipv4(ip));
    }
}


pub fn scan_machine(addr: Ipv4Addr) {
    if is_online(addr) {
        for port in 0..=65535 {
            scan_port(addr, port);
        }
    }
}

fn is_online(_addr: Ipv4Addr) ->  bool {
    thread::sleep(std::time::Duration::from_millis(10));
    false
}

fn scan_port(_addr: Ipv4Addr, _port: u32) {
    thread::sleep(std::time::Duration::from_millis(10));
}