#![allow(dead_code)]

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct EthernetHeader {
    dest_addr: [u8; 6],
    src_addr: [u8; 6],
    r#type: u16,
}

impl EthernetHeader {
    pub const IP: u16 = 0x0800;
    pub const VLAN: u16 = 0x8100;

    pub fn get_type(&self) -> u16 {
        u16::from_be(self.r#type)
    }
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<EthernetHeader>(), 14);
}
