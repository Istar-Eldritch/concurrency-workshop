use std::net::Ipv4Addr;

extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::icmp::{Icmp};
use pnet::packet::icmp::echo_request::{EchoRequestPacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

use std::env;

// Invoke as echo <interface name>
fn main() {
    let interface_name = env::args().nth(1).unwrap();
    dbg!(interface_name.clone());
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();
    dbg!(interface.clone());

    // Create a new channel, dealing with layer 2 packets
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    // 6 bytes DA + 6 bytes SA + 2 bytes Type + 20 bytes IP header + 4 bytes ICMP header + 4 bytes identifier+sequence number + 32 bytes actual data = 74
    let buffer = [0x0, 74];
    let packet = EchoRequestPacket::new(&buffer).unwrap();

    tx.send_to(packet.packet(), None);

    loop {
        match rx.next() {
            Ok(packet) => {
                // Capture here ICMP Reply packages
                dbg!(packet);
            },
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

fn to_ipv4(n: u32) -> Ipv4Addr {
    let bytes = n.to_be_bytes();
    Ipv4Addr::from(bytes)
}

fn to_i32(addr: Ipv4Addr) -> u32 {
    let oct = addr.octets();
    ((oct[0] as u32) << 24) | ((oct[1] as u32) << 16) | ((oct[2] as u32) << 8) | oct[3] as u32
}