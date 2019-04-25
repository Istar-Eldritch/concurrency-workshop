use std::net::Ipv4Addr;
use std::thread;
use rand;
use std::io::Write;

use crate::utils::{to_i32, to_ipv4,};

pub fn scan_network(from_ip: Ipv4Addr, to_ip: Ipv4Addr, from_port: u32, to_port: u32) {
    let from = to_i32(from_ip);
    let to = to_i32(to_ip);

    for ip in from..=to {
        scan_machine(to_ipv4(ip), from_port, to_port);
    }
}

pub fn scan_machine(addr: Ipv4Addr, from: u32, to: u32) {
    if is_online(addr) {
        let mut open_ports = vec![];
        for port in from..=to {
            if is_port_open(addr, port) {
                open_ports.push(port);
            }
        }
        persist_machine_ports(addr, open_ports);
        print!("-");
    } else {
        print!(".")
    }
    std::io::stdout().flush().unwrap();
}


// --- This are fake functions. You do not need to worry about them

fn is_online(_addr: Ipv4Addr) ->  bool {
    thread::sleep(std::time::Duration::from_millis(1));
    let f: f64 = rand::random();
    (f * 100_f64) > 95_f64 // Only 5% return true
}

fn is_port_open(_addr: Ipv4Addr, _port: u32) -> bool {
    thread::sleep(std::time::Duration::from_millis(1));
    let f: f64 = rand::random();
    (f * 100_f64) > 95_f64 // Only 5% return true
}

fn persist_machine_ports(_addr: Ipv4Addr, _open_ports: Vec<u32>) {
    thread::sleep(std::time::Duration::from_millis(1));
}