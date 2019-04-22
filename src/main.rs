
extern crate pnet;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::net::{IpAddr, Ipv4Addr};

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::icmp::{Icmp, IcmpPacket, IcmpTypes};
use pnet::packet::icmp::echo_request::{MutableEchoRequestPacket};
use pnet::packet::ipv4::{MutableIpv4Packet};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket, EtherTypes};

use std::env;

// Invoke as echo <interface name>
fn main() {
    // dbg!(datalink::interfaces());
    let interface_name = env::args().nth(1).unwrap();
    dbg!(interface_name.clone());
    let interface_names_match =
        |iface: &NetworkInterface| iface.name.find(&interface_name).is_some();

    // Find the network interface with the provided name
    info!("Searching for \"{}\" interface", interface_name);
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();
    // dbg!(interface.clone());
    info!("Found matching interface");

    is_online(interface, Ipv4Addr::new(127, 0, 0, 1));
}

fn is_online(interface: NetworkInterface, addr: Ipv4Addr) ->  bool {
    let source_ip = interface
        .ips
        .iter()
        .find(|ip| ip.is_ipv4())
        .map(|ip| match ip.ip() {
            IpAddr::V4(ip) => ip,
            _ => unreachable!(),
        })
    .unwrap();

    let mut ipv4_buffer = [0u8; 42];
    let mut ipv4_packet = MutableIpv4Packet::new(&mut ipv4_buffer).unwrap();

    ipv4_packet.set_destination(addr);
    ipv4_packet.set_source(source_ip);
    
    // Create a new channel, dealing with layer 2 packets
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    let mut buffer = [0; 8];
    let mut packet = MutableEchoRequestPacket::new(&mut buffer).unwrap();
    packet.set_icmp_type(IcmpTypes::EchoRequest);

    ipv4_packet.set_payload(packet.packet());

    tx.send_to(ipv4_packet.packet(), None);

    loop {
        match rx.next() {
            Ok(packet) => {
                // Capture here ICMP Reply packages
                dbg!(packet);
                return false;
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