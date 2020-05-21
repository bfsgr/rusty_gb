#![allow(non_snake_case)]

mod map;
use map::{*};

pub mod io;

pub struct Memory {
    pub map: Map,
    boot_rom_enable: bool
}

impl Default for Memory {
    fn default() -> Memory {
        Memory { map: Map::default(), boot_rom_enable: true }
    }
}

impl Memory {
    fn classify(address: u16) -> Region{
        match address {
            0..=0x7FFF => Region::Cartrigbe,    //32kb
            0x8000..=0x9FFF => Region::VRAM,    //8kb
            0xA000..=0xBFFF => Region::SRAM,    //8kb
            0xC000..=0xDFFF  => Region::WRAM,   //8kb
            0xE000..=0xFDFF => Region::Echo,    //8kb
            0xFE00..=0xFE9F => Region::OAM,     
            0xFEA0..=0xFEFF => Region::Unsable,
            0xFF00..=0xFF4B => Region::IO,
            0xFF4C..=0xFF7F => Region::GbcIO, 
            0xFF80..=0xFFFE => Region::Stack,   //127 bytes
            0xFFFF => Region::Interrupt         //1 byte
        }
    }
    fn simplify(address: u16, from: Region) -> u16{
        match from {
            Region::Cartrigbe => address,
            Region::VRAM => (address - 0x8000),
            Region::SRAM => (address - 0xA000),
            Region::WRAM => (address - 0xC000),
            Region::Echo => (address - 0xE000),
            Region::OAM => (address - 0xFE00),
            Region::Unsable => (address - 0xFEA0), //there's no reason to use this conversion
            Region::IO =>  (address - 0xFF00),
            Region::GbcIO => (address - 0xFF4C),
            Region::Stack => (address - 0xFF80),
            Region::Interrupt => 0xFFFF,
        }
    }

    fn convert(address: u16) -> u16{
        let portion = Memory::classify(address);
        Memory::simplify(address, portion)
    }


    pub fn read_byte(&self, address: u16) -> Option<u8>{
        match Memory::classify(address) {
            Region::Cartrigbe => self.map.read(Region::Cartrigbe, Memory::convert(address)),
            Region::VRAM => self.map.read(Region::VRAM, Memory::convert(address)),
            Region::SRAM => self.map.read(Region::SRAM, Memory::convert(address)),
            Region::WRAM => self.map.read(Region::WRAM, Memory::convert(address)),
            Region::Echo => self.map.read(Region::Echo, Memory::convert(address)),
            Region::OAM =>  self.map.read(Region::OAM, Memory::convert(address)),
            Region::IO =>   self.map.read(Region::IO, Memory::convert(address)),
            // Region::GbcIO => {},
            Region::Stack => self.map.read(Region::Stack, Memory::convert(address)),
            Region::Interrupt => self.map.read(Region::Interrupt, 0),
            _ => None,
        }
    }
    pub fn write_byte(&mut self, address: u16, byte: u8) -> bool{
        match Memory::classify(address) {
            Region::Cartrigbe => {
                self.map.write(Region::Cartrigbe, Memory::convert(address), byte);
                return false;
            },
            Region::VRAM => {
                self.map.write(Region::VRAM, Memory::convert(address), byte);
                return false;
            },
            Region::SRAM => {
                self.map.write(Region::SRAM, Memory::convert(address), byte);
                return false;
            },
            Region::WRAM => {
                self.map.write(Region::WRAM, Memory::convert(address), byte);
                return false;
            },
            Region::Echo => {
                self.map.write(Region::Echo, Memory::convert(address), byte);
                return false;
            },
            Region::OAM => {
                self.map.write(Region::OAM, Memory::convert(address), byte);
                return false;
            },
            Region::Unsable => { false },
            Region::IO =>  {
                self.map.write(Region::IO, Memory::convert(address), byte);
                return true;
            },
            Region::GbcIO => { false },
            Region::Stack => {
                self.map.write(Region::Stack, Memory::convert(address), byte);
                return false;
            },
            Region::Interrupt => {
                self.map.write(Region::Interrupt, 0, byte);
                return true;
            }
        }
    }

    pub fn write_short(&mut self, address: u16, short: u16) -> bool{

        //TODO, add validation regarding limits of memory sections 

        let op1 = self.write_byte(address, short as u8);
        let op2 = self.write_byte(address + 1, (short >> 8) as u8 );

        return op1 || op2
    }


    pub fn push_rom(&mut self, data: [u8; 0x4000], first_2k: bool){
        if first_2k {
            if self.boot_rom_enable {
                for i in 256..0x4000 {
                    self.map.cartrigbe[i] = data[i];
                }
            } else {
                for i in 0..0x4000 {
                    self.map.cartrigbe[i] = data[i];
                }
            }
        } else {
            for i in 0x4000..0x8000 {
                self.map.cartrigbe[i] = data[i];
            }
        }
    }

    pub fn to_short(bytes: [u8; 2]) -> u16 {
        bytes[0] as u16 | (bytes[1] as u16) << 8 //>
    }

    pub fn get_io(&self) -> [u8; 0x4C] {
        self.map.io
    }

    pub fn enable_interrupts(&mut self){
        self.map.interrupt_switch = true;
    }

    pub fn disable_interrupts(&mut self){
        self.map.interrupt_switch = false;
    }

    pub fn interrupt_state(&self) -> bool{
        self.map.interrupt_switch
    }
}