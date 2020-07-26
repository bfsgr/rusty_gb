pub mod mbc0;
pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
use super::header::Header;

pub trait MBC {
    fn write(&mut self, addr: u16, byte: u8);
    fn read(&self, addr: u16) -> u8;
    fn load(&mut self, data: Vec<u8>, header: Header );
}
#[derive(PartialEq)]
pub enum Mode {
    ROM,
    RAM
}

impl Default for Mode{
    fn default() -> Self { Mode::ROM }
}

pub fn rom_to_size(code: u8) -> usize {
    match code {
        0 => 0x8000,
        1 => 0x10000,
        2 => 0x20000,
        3 => 0x40000,
        4 => 0x80000,
        5 => 0x100000,
        6 => 0x200000,
        7 => 0x400000,
        8 => 0x800000,
        0x52 => 0x120000,
        0x53 => 0x140000,
        0x54 => 0x180000,

        _ => { unreachable!("Invalid ROM size code: {:x}", code) }
    }
}   

pub fn ram_to_size(code: u8) -> usize {
    match code {
        0 => 0,
        1 => 0x800,
        2 => 0x2000,
        3 => 0x8000,
        4 => 0x20000,
        5 => 0x10000,
        
        _ => { unreachable!("Invalid ROM size code: {:x}", code) }
    }
}   

pub fn fix_rom_bank(rom_bank: u8) -> u8 {
    match rom_bank {
        0 | 0x20 | 0x40 | 0x60 => rom_bank + 1,
        _ => rom_bank,
    }
}