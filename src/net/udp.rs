#![allow(dead_code)]

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl UdpHeader {}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<UdpHeader>(), 8);
}
