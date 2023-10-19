#![allow(dead_code)]

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct IpHeader {
    ihl_version: u8,
    tos: u8,
    total_length: u16,
    id: u16,
    flags_fo: u16,
    ttl: u8,
    pub protocol: u8,
    checksum: u16,
    src_addr: u32,
    dest_addr: u32,
}

impl IpHeader {
    pub const UDP: u8 = 17;
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<IpHeader>(), 20);
}
