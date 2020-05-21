use super::io_constants::{*};

use super::gpu::{*};
use super::memory::{*};
use super::cartrigbe::{*};
use super::cpu::registers::Response;
use super::cpu::registers::Value;

#[derive(Default)]
pub struct Bus {
    memory: Memory,
    pub gpu: GPU,
    cartrigbe: Cartrigbe,
    //everything with memory mapped I/O registers goes in here
}

#[derive(PartialEq)]
pub enum Module {
    Cartrigbe,  
    GPU,       
    Memory,      
    IO,        
    Interrupt,
    Unusable,  
}

impl Bus {
    pub fn write_byte(&mut self, addr: u16, byte: u8) -> Response {

        let into = Bus::classify(addr);

        match into {
            Module::Cartrigbe => {},
            Module::GPU => {},
            Module::Memory => {
                self.memory.write_byte(addr, byte);
            },
            Module::Interrupt => {},
            Module::IO => {},
            Module::Unusable => {},
        }

        Response::None
    }
    pub fn read_byte(&mut self, addr: u16) -> Response {

        let from = Bus::classify(addr);

        match from {
            Module::Cartrigbe => self.cartrigbe.read_byte(addr),
            Module::GPU => self.gpu.read_byte(addr),
            Module::Memory => self.memory.read_byte(addr),
            Module::Interrupt => { Response::None },
            Module::IO => {
                match addr {
                    LCDC => { Response::Byte( self.gpu.LCDC ) }
                    STAT => { Response::Byte( self.gpu.STAT ) }
                    _ => { Response::Byte(0) }
                }
            },
            Module::Unusable => { Response::None },
        }


    }

    pub fn write_short(&mut self, addr: u16, short: u16) {
        let b1 = Bus::classify(addr);
        let b2 = Bus::classify(addr+1);

        if b1 == b2 {

            self.write_byte(addr, short as u8);
            self.write_byte(addr + 1, (short >> 8) as u8 );

        } else {
            panic!("Tried to write short along different modules")
        }
    }

    pub fn read_short(&mut self, addr: u16) -> Response{
        let b1 = Bus::classify(addr);
        let b2 = Bus::classify(addr+1);

        if b1 == b2 {

            let lsb: u8 = self.read_byte(addr).value();
            let msb: u8 = self.read_byte(addr+1).value();

            Response::Short( lsb as u16 | (msb as u16) << 8  ) //>

        } else {
            panic!("Tried to read short along different modules")
        }
    }

    fn classify(address: u16) -> Module{
        match address {
            0..=0x7FFF => Module::Cartrigbe,    //32kb
            0x8000..=0x9FFF => Module::GPU,    //8kb
            0xA000..=0xBFFF => Module::Cartrigbe,    //8kb
            0xC000..=0xFDFF => Module::Memory,    //8kb
            0xFE00..=0xFE9F => Module::GPU,     
            0xFEA0..=0xFEFF => Module::Unusable,
            0xFF00..=0xFF7F => Module::IO, 
            0xFF80..=0xFFFE => Module::Memory,   //127 bytes
            0xFFFF => Module::Interrupt         //1 byte
        }
    }

    pub fn insert_cartrigbe(&mut self, file_name: String) {
        self.cartrigbe.insert(file_name);
    }

    pub fn to_short(bytes: [u8; 2]) -> u16 {
        bytes[0] as u16 | (bytes[1] as u16) << 8 //>
    }

    pub fn enable_interrupts(&mut self){
        
    }
    
    pub fn disable_interrupts(&mut self){

    }
}
