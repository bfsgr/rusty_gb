use fetcher::{DATA_ADDR_BASE, DATA_ADDR_UPPER, MAP_ADDR_BASE, MAP_ADDR_UPPER};

use super::*;
use crate::emulator::io_constants::*;

impl PPU {
    pub fn write_lcdc(&mut self, byte: u8) {
        if !byte.test_bit(7) && self.enabled() {
            if self.mode != Mode::VBlank {
                // panic!("Turned LCD off outside of Vblank")
                println!("WARN: Turned LCD off outside of vblank")
            }
            self.stat = 0x80;
            self.mode = Mode::HBlank;
        }
        if byte.test_bit(7) && !self.enabled() {
            if *self.fetcher.line() == self.lycompare { 
                self.stat.set_bit(2);
            } else {
                self.stat.reset_bit(2)
            }
            self.set_mode(Mode::Oam);
        }

        if byte.test_bit(3) {
            self.fetcher.tile_map_addr = MAP_ADDR_UPPER;
        } else {
            self.fetcher.tile_map_addr = MAP_ADDR_BASE;
        }
        
        if byte.test_bit(4) {
            self.fetcher.tile_data_addr = DATA_ADDR_BASE;
        } else {
            self.fetcher.tile_data_addr = DATA_ADDR_UPPER;
        }

        self.lcdc = byte
    }

    pub fn enabled(&self) -> bool{
        self.lcdc.test_bit(7)
    }

    pub fn write_stat(&mut self, byte: u8) {
        //only keep bits 3-6
        let data = (byte & 0xF8) | 0x80;
        self.stat = data;
    }

    pub fn read_stat(&mut self) -> u8 {
        self.stat | 0x80
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) -> Response {
        let into = PPU::translate(addr);

        match into {
            Region::VRAM(x) => {
                self.fetcher.vram[x] = byte;
            }
            Region::OAM(x) => {
                self.fetcher.oam[x] = byte
            }
        }

        Response::None
    }

    pub fn read_byte(&self, addr: u16) -> Response {
        let from = PPU::translate(addr);

        match from {
            Region::VRAM(x) => {
                return Response::Byte( self.fetcher.vram[x] as u8 );
            }
            Region::OAM(x) => {
                return Response::Byte( self.fetcher.oam[x] as u8 );
            }
        }
    }

    fn translate(addr: u16) -> Region {
        match addr {
            0x8000 ..= 0x9FFF => Region::VRAM( addr as usize - 0x8000 ),
            0xFE00 ..= 0xFE9F => Region::OAM( addr as usize - 0xFE00 ),
            _ => panic!("Error translating address in GPU module")
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        let save = self.stat;
        match mode {
            Mode::HBlank => {
                self.stat = save & 0xFC;
            },
            Mode::VBlank => {
                self.stat = save & 0xFC;
                self.stat.set_bit(0);
            },
            Mode::Oam => {
                self.stat = save & 0xFC;
                self.stat.set_bit(1);
            },
            Mode::Transfer => {
                self.stat.set_bit(0);
                self.stat.set_bit(1);
            }
        }
        self.mode = mode;
    }

    pub fn read_register(&self, addr: u16) -> u8{
        match addr {
            
            SCX => { self.queue.sx },
            SCY => { self.queue.sy },
            LY => { *self.fetcher.line() },
            LYC => { self.lycompare },
            OAM_DMA => { self.fetcher.dma },
            BGP => { self.fetcher.bgp },
            OBP0 => { self.fetcher.obp0 },
            OBP1 => { self.fetcher.obp1 },
            WX => { self.queue.wx },
            WY => { self.queue.wy },

            
            _ => { unreachable!("No register in addr {}", addr) }
        }
    }

    pub fn write_register(&mut self, addr: u16, data: u8){
        match addr {
            
            SCX => { self.queue.sx = data },
            SCY => { self.queue.sy = data },
            LYC => { self.lycompare = data },
            OAM_DMA => { self.fetcher.dma = data },
            BGP => { self.fetcher.bgp = data },
            OBP0 => { self.fetcher.obp0 = data },
            OBP1 => { self.fetcher.obp1 = data },
            WX => { self.queue.wx = data },
            WY => { self.queue.wy = data },

            
            _ => { unreachable!("No register in addr {}", addr) }
        }
    }
}