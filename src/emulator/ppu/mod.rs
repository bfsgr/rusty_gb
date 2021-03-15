#![allow(warnings)]

mod interface;
mod pixelFIFO;
mod fetcher;

use std::char::DecodeUtf16;

use fetcher::Fetcher;
use pixelFIFO::PixelFIFO;
use super::interrupt::*;
use super::bit_utils::*;
use super::cpu::registers::Response;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    Oam = 2,
    Transfer = 3
}

pub struct PPU {
    //internal mode switcher
    pub mode: Mode,

    fetcher: Fetcher,
    queue: PixelFIFO,
    //registers
    pub lcdc: u8,           //0xFF40     (R/W)
    pub stat: u8,           //0xFF41     (R/W)
    pub lycompare: u8,      //0xFF45     (R/W)
}

impl Default for PPU {
    fn default() -> PPU{
        PPU {
            mode: Mode::HBlank,
            fetcher: Fetcher::default(),
            queue: PixelFIFO::default(),
            lcdc: 0,           //0xFF40     (R/W)
            stat: 0x80,        //0xFF41     (R/W)
            lycompare: 0,      //0xFF45     (R/W)
            
        }
    }
}

enum Region {
    OAM(usize),
    VRAM(usize),
}

impl PPU {
    pub fn step(&mut self, interrupt_handler: &mut InterruptHandler, screen: &mut Vec<u32>){

        
    }
}