#![allow(non_snake_case)]

use super::cpu::registers::Response;

//TODO, remove echo field, all logic will convert addresses in echo to wram
pub struct Memory {
    wram: [u8; 0x2000], //Internal RAM
    hram: [u8; 0x7F],   //High Ram (Stack)
}

enum Region {
    WRAM(usize),
    Echo(usize),
    HRAM(usize)
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            wram: [0; 0x2000], //Internal RAM
            hram: [0; 0x7F],   //High Ram (Stack)
        }
    }
}

impl Memory {
    pub fn write_byte(&mut self, addr: u16, byte: u8) -> Response {
        
        let into = Memory::translate(addr);

        match into {
            Region::WRAM(x) => {
                self.wram[x] = byte;
            },
            Region::Echo(x) => {
                self.wram[x] = byte;

            },
            Region::HRAM(x) => {
                self.hram[x] = byte;
            }
        }
        
        Response::None
    }
    pub fn read_byte(&self, addr: u16) -> Response {
        
        let from = Memory::translate(addr);

        match from {
            Region::WRAM(x) => {
                Response::Byte(self.wram[x])
            },
            Region::Echo(x) => {
                Response::Byte(self.wram[x])
            },
            Region::HRAM(x) => {
                Response::Byte(self.hram[x])
            }
        }
        
    }

    fn translate(addr: u16) -> Region {
        match addr {
            0xC000 ..= 0xDFFF => Region::WRAM(addr as usize - 0xC000 ),
            0xE000 ..= 0xFDFF => Region::Echo(addr as usize - 0xE000),
            0xFF80 ..= 0xFFFE => Region::HRAM(addr as usize - 0xFF80),
            _ => panic!("Error translating address in memory module")
        }
    }
}