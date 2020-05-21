#![allow(non_snake_case)]

use super::interrupt::{*};
use super::memory::{*};
use super::bit_utils::{*};

const OAM_CYCLES: usize = 79;
const TRANSFER_CYCLES: usize = OAM_CYCLES + 172;
const HBLANK_CYCLES: usize = 456;
const FRAME_CYCLES: usize = HBLANK_CYCLES * 144;
const VBLANK_CYCLES: usize = FRAME_CYCLES + 4560;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    Oam = 2,
    Transfer = 3
}

pub struct GPU {
    pub mode: Mode,
    pub scanline_cycles: usize,
    pub frame_cycles: usize,

    pub LCDC: u8,           //0xFF40     (R/W)
    pub STAT: u8,           //0xFF41     (R/W)
    pub scroll_y: u8,       //0xFF42     (R/W)
    pub scroll_x: u8,       //0xFF43     (R/W)
    pub lcd_y: u8,          //0xFF44     (R)
    pub lycompare: u8,      //0xFF45     (R/W)
    pub OAM_DMA: u8,        //0xFF46     (?)
    pub window_y: u8,       //0xFF4A     (R/W)   
    pub window_x: u8,       //0xFF4B     (R/W)
    pub bg_palette: u8,     //0xFF47     (R/W)
    pub ob_palette0: u8,    //0xFF48     (R/W)
    pub ob_palette1: u8,    //0xFF49     (R/W)
    pub bgp_index: u8,      //0xFF68     (R/W) (GB Color only)
    pub bgp_data: u8,       //0xFF69     (R/W) (GB Color only)
    pub spt_index: u8,      //0xFF6A     (R/W) (GB Color only)   
    pub spt_data: u8,       //0xFF6B     (R/W) (GB Color only)

    pub display: Vec<u32>
}

impl Default for GPU {
    fn default() -> GPU{
        GPU {
            mode: Mode::Oam,
            scanline_cycles: 0,
            frame_cycles: 0,

            LCDC: 0,           //0xFF40     (R/W)
            STAT: 0,           //0xFF41     (R/W)
            scroll_y: 0,       //0xFF42     (R/W)
            scroll_x: 0,       //0xFF43     (R/W)
            lcd_y: 0,          //0xFF44     (R)
            lycompare: 0,      //0xFF45     (R/W)
            window_y: 0,       //0xFF4A     (R/W)   
            window_x: 0,       //0xFF4B     (R/W)
            OAM_DMA: 0,
            bg_palette: 0,     //0xFF47     (R/W)
            ob_palette0: 0,    //0xFF48     (R/W)
            ob_palette1: 0,    //0xFF49     (R/W)
            bgp_index: 0,      //0xFF68     (R/W) (GB Color only)
            bgp_data: 0,       //0xFF69     (R/W) (GB Color only)
            spt_index: 0,      //0xFF6A     (R/W) (GB Color only)   
            spt_data: 0,       //0xFF6B     (R/W) (GB Color only)
            display: vec![0; 160*144]
        }
    }
}

impl GPU {
    pub fn step(&mut self, cycles_made: u16, interrupt_handler: &mut InterruptHandler, memory: &Memory){
        //check if display is enabled
        if self.enabled() {
            //save the current mode
            let cur_mode = self.mode;

            //sync the internal cycles
            self.scanline_cycles += cycles_made as usize;
            self.frame_cycles += cycles_made as usize;
            
            //flag for interrupt request
            let mut interrupt_status = false;
            
            //if frame_cycles is bigger than 65664 it's VBLANK period
            if self.frame_cycles > FRAME_CYCLES {
                //cur_mode is not equal to VBlank so change it
                if cur_mode != Mode::VBlank {
                    //set STAT bits
                    self.STAT.set_bit(4);
                    self.STAT.set_bit(0);
                    self.STAT.reset_bit(1);
                    //update interrupt flag
                    interrupt_status = true;
                }
                //frame_cycles are bigger than the Vblank period, reset everything
                if self.frame_cycles > VBLANK_CYCLES {
                    self.frame_cycles = 0;
                    self.scanline_cycles = 0;
                    self.lcd_y = 0;
                    //compare LY to LYC
                    self.line_compare(interrupt_handler);
                    //reset mode to OAM
                    self.mode = Mode::Oam;
                }
            } else {
                //it's not vblank so test scanline cycles
                match self.scanline_cycles {
                    0 ..= OAM_CYCLES  => {
                        //OAM period
                        if cur_mode != Mode::Oam {
                            self.mode = Mode::Oam;
                            self.STAT.set_bit(5);
                            self.STAT.set_bit(1);
                            self.STAT.reset_bit(0);
                            interrupt_status = true;
                        }
                    },
                    OAM_CYCLES ..= TRANSFER_CYCLES => {
                        //Transfer period
                        if cur_mode != Mode::Transfer {
                            self.mode = Mode::Transfer;
                            self.STAT.set_bit(0);
                            self.STAT.set_bit(1);

                            self.transfer(memory);
                        }
                    },
                    TRANSFER_CYCLES ..= HBLANK_CYCLES => {
                        if cur_mode != Mode::HBlank {
                            self.mode = Mode::HBlank;
                            self.STAT.set_bit(3);
                            self.STAT.reset_bit(1);
                            self.STAT.reset_bit(0);
                            interrupt_status = true;
                        }
                    },

                    _ => {}
                }
            }

            if interrupt_status {
                interrupt_handler.request(Interrupt::LCDC);
            }

            if self.scanline_cycles > HBLANK_CYCLES {
                self.lcd_y += 1;
                self.scanline_cycles = 0;
                self.line_compare(interrupt_handler);
            }

        }
    }

    fn line_compare(&mut self, interrupt: &mut InterruptHandler){
        if self.lycompare == self.lcd_y {
            self.STAT.set_bit(2);
            interrupt.request(Interrupt::LCDC)
        } else {
            self.STAT.reset_bit(2);
        }

    }

    pub fn enabled(&self) -> bool{
        self.LCDC.test_bit(7)
    }

    fn transfer(&mut self, memory: &Memory){
        if self.LCDC.test_bit(0) {
            //draw bg
        }
        if self.LCDC.test_bit(5) {
            //draw window
        }
        if self.LCDC.test_bit(1) {
            //draw sprite
        }
    }

}