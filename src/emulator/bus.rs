use super::io_constants::{*};

use super::gpu::{*};
use super::memory::{*};
use super::timer::{*};
use super::cartrigbe::{*};
use super::joypad::{*};
use super::cpu::registers::Response;
use super::cpu::registers::Value;
pub use super::interrupt::{*};

#[derive(Default)]
pub struct Bus {
    memory: Memory,
    pub gpu: GPU,
    cartrigbe: Cartrigbe,
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

            Module::GPU => { self.gpu.write_byte(addr, byte); },

            Module::Memory => { self.memory.write_byte(addr, byte); },

            Module::Interrupt => { self.interrupts.enable = byte; },

            Module::IO => {
                match addr {
                    LCDC => self.gpu.write_lcdc(byte),
                    STAT => self.gpu.write_stat(byte),
                    SCY => self.gpu.scroll_y = byte,
                    SCX => self.gpu.scroll_x = byte,
                    LY => self.gpu.lcd_y = byte,
                    LYC => self.gpu.lycompare = byte,
                    OAM_DMA => {
                        self.gpu.OAM_DMA = byte;
                        self.perform_dma();
                    }
                    BGP => self.gpu.bg_palette = byte,
                    OBP0 => self.gpu.ob_palette0 = byte,
                    OBP1 => self.gpu.ob_palette1 = byte,
                    WY => self.gpu.window_y = byte,
                    WX => self.gpu.window_x = byte,
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
            Module::GPU => self.gpu.read_byte(addr),
            Module::Memory => self.memory.read_byte(addr),
            Module::Interrupt => Response::Byte(self.interrupts.enable),
            Module::IO => {
                match addr {
                    LCDC => { Response::Byte( self.gpu.LCDC ) },
                    STAT => { Response::Byte( self.gpu.read_stat() ) },
                    SCY => { Response::Byte( self.gpu.scroll_y ) },
                    SCX => { Response::Byte( self.gpu.scroll_x ) },
                    LY => { Response::Byte( self.gpu.lcd_y ) },
                    LYC => { Response::Byte( self.gpu.lycompare ) },
                    OAM_DMA => { Response::Byte( self.gpu.OAM_DMA ) },
                    BGP => { Response::Byte( self.gpu.bg_palette ) },
                    OBP0 => { Response::Byte( self.gpu.ob_palette0 ) },
                    OBP1 => { Response::Byte( self.gpu.ob_palette1 ) },
                    WY => { Response::Byte( self.gpu.window_y ) },
                    WX => { Response::Byte( self.gpu.window_x ) },
                    JOYP => { Response::Byte( self.joypad.read() ) }
                    
                    IF => { Response::Byte( self.interrupts.requests | 0xE0 ) },
                    _ => { Response::Byte(0xFF) }
                }
            },
            Module::Unusable => { panic!("Unusable was read") },
            Module::Timer => { self.timer.read_byte(addr) },
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


    fn classify(address: u16) -> Module{
        match address {
            0      ..= 0x7FFF => Module::Cartrigbe,    
            0x8000 ..= 0x9FFF => Module::GPU,    
            0xA000 ..= 0xBFFF => Module::Cartrigbe,   
            0xC000 ..= 0xFDFF => Module::Memory,    
            0xFE00 ..= 0xFE9F => Module::GPU,     
            0xFEA0 ..= 0xFEFF => Module::Memory,
            TMA | TIMA | DIV | TAC => Module::Timer,
            0xFF00 ..= 0xFF7F => Module::IO, 
            0xFF80 ..= 0xFFFE => Module::Memory,   
            0xFFFF            => Module::Interrupt,
        }
    }

    pub fn insert_cartrigbe(&mut self, file_name: String) {
        self.cartrigbe.insert(file_name);
    }

    pub fn to_short(bytes: [u8; 2]) -> u16 {
        bytes[0] as u16 | (bytes[1] as u16) << 8 //>
    }

    pub fn enable_interrupts(&mut self){
        self.interrupts.ei_key = EI::Requested;
    }
    
    pub fn disable_interrupts(&mut self){
        self.interrupts.master = false;
    }

    pub fn run_system(&mut self, cycles: u8, screen: &mut Vec<u32>) {
        self.gpu.step(cycles, &mut self.interrupts, screen);
        self.timer.step(cycles, &mut self.interrupts);
    }
    //maybe not an optimal solution, performs the dma all at once. The rom will wait 160 cycles either way
    fn perform_dma(&mut self) {
        //Max transfer start is 0xF100
        if self.gpu.OAM_DMA <= 0xF1 {

            let start = (self.gpu.OAM_DMA as u16) << 8; //>
            let end = (self.gpu.OAM_DMA as u16) << 8 | 0x9F; //>

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
