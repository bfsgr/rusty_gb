use super::super::mbcx::{*};

#[derive(Default)]
pub struct MBC0 {
    data: Vec<u8>
}

impl MBC for MBC0 {
    fn write(&mut self, _addr: u16, _byte: u8) {}
    fn read(&self, addr: u16) -> u8 {
        if addr > 0x7FFF { return 0xFF; }
        self.data[addr as usize]
    }
    fn load(&mut self, _: String, _: u8, _: u8, _: u8, data: Vec<u8> ) {
        self.data = data;
    }
}