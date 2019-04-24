use std::net::Ipv4Addr;

pub fn to_ipv4(n: u32) -> Ipv4Addr {
    let bytes = n.to_be_bytes();
    Ipv4Addr::from(bytes)
}

pub fn to_i32(addr: Ipv4Addr) -> u32 {
    let oct = addr.octets();
    (u32::from(oct[0]) << 24) | (u32::from(oct[1]) << 16) | (u32::from(oct[2]) << 8) | u32::from(oct[3])
}