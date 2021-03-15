use super::io_constants::{*};

use super::ppu::{*};
use super::memory::{*};
use super::timer::{*};
use super::cartridge::{*};
use super::joypad::{*};
use super::cpu::registers::Response;
use super::cpu::registers::Value;
pub use super::interrupt::{*};

#[derive(Default)]
pub struct Bus {
    memory: Memory,
    pub ppu: PPU,
    cartrigbe: Cartridge,
    pub interrupts: InterruptHandler,
    timer: Timer,
    pub joypad: Joypad,
    pub halt_cpu: bool
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
    Timer,
}

impl Bus {
    pub fn write_byte(&mut self, addr: u16, byte: u8) -> Response {

        let into = Bus::classify(addr);

        match into {
            Module::Cartrigbe => { self.cartrigbe.write_byte(addr, byte); },

            Module::GPU => { self.ppu.write_byte(addr, byte); },

            Module::Memory => { self.memory.write_byte(addr, byte); },

            Module::Interrupt => { self.interrupts.enable = byte; },

            Module::IO => {
                match addr {
                    LCDC => self.ppu.write_lcdc(byte),
                    STAT => self.ppu.write_stat(byte),
                    OAM_DMA => {
                        self.ppu.write_register(addr, byte);
                        self.perform_dma();
                    },
                    SCY ..= WX  => self.ppu.write_register(addr, byte),
                    BROM => self.cartrigbe.bios_control(byte),
                    JOYP => self.joypad.write(byte),
                    
                    IF => self.interrupts.requests = byte | 0xE0,
                    _ => {}
                }
            },
            Module::Unusable => { panic!("Unusable was written") },
            Module::Timer => { self.timer.write_byte(addr, byte); }
        }

        Response::None
    }
    pub fn read_byte(&mut self, addr: u16) -> Response {

        let from = Bus::classify(addr);

        match from {
            Module::Cartrigbe => self.cartrigbe.read_byte(addr),
            Module::GPU => self.ppu.read_byte(addr),
            Module::Memory => self.memory.read_byte(addr),
            Module::Interrupt => Response::Byte(self.interrupts.enable),
            Module::IO => {
                match addr {
                    LCDC => { Response::Byte( self.ppu.lcdc ) },
                    STAT => { Response::Byte( self.ppu.read_stat() ) },
                    SCY ..= WX => { Response::Byte( self.ppu.read_register(addr) ) },
                    JOYP => { Response::Byte( self.joypad.read() ) }
                    
                    IF => { Response::Byte( self.interrupts.requests | 0xE0 ) },
                    _ => { Response::Byte(0xFF) }
                }
            },
            Module::Unusable => { panic!("Unusable was read") },
            Module::Timer => { self.timer.read_byte(addr) },
        }


    }

    fn classify(address: u16) -> Module{
        match address {
            0      ..= 0x7FFF => Module::Cartrigbe,    
            0x8000 ..= 0x9FFF => Module::GPU,    
            0xA000 ..= 0xBFFF => Module::Cartrigbe,   
            0xC000 ..= 0xFDFF => Module::Memory,    
            0xFE00 ..= 0xFE9F => Module::GPU,     
            0xFEA0 ..= 0xFEFF => Module::Unusable,
            TMA | TIMA | DIV | TAC => Module::Timer,
            0xFF00 ..= 0xFF7F => Module::IO, 
            0xFF80 ..= 0xFFFE => Module::Memory,   
            0xFFFF            => Module::Interrupt,
        }
    }

    pub fn insert_cartrigbe(&mut self, file_name: String) {
        self.cartrigbe.insert(file_name);
    }

    pub fn enable_interrupts(&mut self){
        self.interrupts.ei_key = EI::Requested;
    }
    
    pub fn disable_interrupts(&mut self){
        self.interrupts.master = false;
    }

    pub fn run(&mut self, _screen: &mut Vec<u32>) {
        
        //update the screen 4 times
        for _ in 0..4 {
            // self.gpu.step(&mut self.interrupts, screen);
        }
        
        self.timer.step(4, &mut self.interrupts);
    }
    //maybe not an optimal solution, performs the dma all at once. The rom will wait 160 cycles either way
    fn perform_dma(&mut self) {
        //Max transfer start is 0xF100
        if self.ppu.read_register(OAM_DMA) <= 0xF1 {

            let start = (self.ppu.read_register(OAM_DMA) as u16) << 8; //>
            let end = (self.ppu.read_register(OAM_DMA) as u16) << 8 | 0x9F; //>

            let mut oam_start = 0xFE00;

            //will run 160 times
            for addr in start ..= end {
                let byte: u8 = self.read_byte(addr).value();

                self.write_byte(oam_start, byte);
                oam_start += 1;
            }


        } else {
            panic!("Wrong OAM_DMA address")
        }     
    }
}
