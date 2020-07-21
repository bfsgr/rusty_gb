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
    fn load(&mut self, data: Vec<u8>, _: Header ) {
        if data.len() != 0x8000 { panic!("Wrong data size for MBC0") }
        self.data = data;
    }
}