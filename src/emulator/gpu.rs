#![allow(non_snake_case)]

use super::interrupt::{*};
use super::bit_utils::{*};
use super::cpu::registers::{Response};

const OAM_SEARCH: usize = 79;
const TRANSFER_CYCLES: usize = OAM_SEARCH + 172;
const HBLANK_CYCLES: usize = 456;
const FRAME_CYCLES: usize = HBLANK_CYCLES * 146;
const VBLANK_CYCLES: usize = FRAME_CYCLES + 4560;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    Oam = 2,
    Transfer = 3
}

// #[derive(Default, Copy, Clone)]
// struct Sprite{
//     id: u8,
//     x: u8,
//     y: u8,
//     palette: bool,
//     x_flip: bool,
//     y_flip: bool,
//     priority: bool
// }

//should be easier to just use a Vec, but I avoid using heap structures
// #[derive(Default)]
// struct SpriteList{
//     sprites: [u8; 10],
//     size: i8
// }

// impl SpriteList {
//     fn empty(&self) -> bool {
//         self.size == -1
//     }

//     fn full(&self) -> bool {
//         self.size >= 10
//     }

//     fn clear(&mut self) {
//         self.size = -1;
//     }
    
//     fn push(&mut self, x: u8) {
//         if !self.full() {
//             self.sprites[self.size as usize] = x;
//             self.size += 1;
//         }
//     }
// }

pub struct GPU {
    mode: Mode,
    scanline_cycles: usize,
    frame_cycles: usize,
    // sprites: Vec<Sprite>,
    // visible_sprites: SpriteList,

    lock_vram: bool,
    lock_oam: bool,

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
    vram: [u8;0x2000],
    oam: [u8; 0xA0],
    pub display: Vec<u32>
}

impl Default for GPU {
    fn default() -> GPU{
        GPU {
            mode: Mode::Oam,
            scanline_cycles: 0,
            frame_cycles: 0,
            // sprites: vec![Sprite::default(); 40],
            // visible_sprites: SpriteList::default(),
            lock_vram: false,
            lock_oam: false,
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
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            display: vec![0; 160*144]
        }
    }
}

enum Region {
    OAM(usize),
    VRAM(usize),
}

impl GPU {
    pub fn step(&mut self, cycles_made: u16, interrupt_handler: &mut InterruptHandler){
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
                    self.lock_vram = false;
                    self.lock_oam = false;
                    //update interrupt flag
                    interrupt_status = true;
                }
                //frame_cycles are bigger than the Vblank period, reset everything
                if self.frame_cycles > VBLANK_CYCLES {
                    self.frame_cycles = 0;
                    self.scanline_cycles = 0;
                    self.lcd_y = 0;
                    self.lock_vram = false;
                    self.lock_oam = false;
                    //compare LY to LYC
                    self.line_compare(interrupt_handler);
                    //reset mode to OAM
                    self.mode = Mode::Oam;
                }
            } else {
                //it's not vblank so test scanline cycles
                match self.scanline_cycles {
                    0 ..= OAM_SEARCH  => {
                        //OAM period
                        if cur_mode != Mode::Oam {
                            self.mode = Mode::Oam;
                            self.STAT.set_bit(5);
                            self.STAT.set_bit(1);
                            self.STAT.reset_bit(0);
                            interrupt_status = true;

                            self.lock_oam = true;
                            self.lock_vram = false;

                            // self.search_oam();
                        }
                    },
                    OAM_SEARCH ..= TRANSFER_CYCLES => {
                        //Transfer period
                        if cur_mode != Mode::Transfer {
                            self.mode = Mode::Transfer;
                            self.STAT.set_bit(0);
                            self.STAT.set_bit(1);

                            self.lock_vram = true;
                            self.lock_oam = true;

                            self.transfer();
                        }
                    },
                    TRANSFER_CYCLES ..= HBLANK_CYCLES => {
                        if cur_mode != Mode::HBlank {
                            self.mode = Mode::HBlank;
                            self.STAT.set_bit(3);
                            self.STAT.reset_bit(1);
                            self.STAT.reset_bit(0);

                            self.lock_vram = false;
                            self.lock_oam = false;

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

    fn transfer(&mut self){
        if self.LCDC.test_bit(0) {
            //draw bg
            let palette = self.bg_palette;

            let tile_map_addr = match self.LCDC.test_bit(3) {
                true => 0x9C00,
                false => 0x9800
            };

            let tile_data_addr = match self.LCDC.test_bit(4) {
                true => 0x9000,
                false => 0x8000
            };

            let dY = self.lcd_y;

            let Y = dY.wrapping_add(self.scroll_y);

            let buffer: u32 = dY as u32 * 160;
            //title based addresses
            let row = Y / 8;

            for i in 0..160 {
                let X = (i as u8).wrapping_add(self.scroll_x);

                let column = X / 8;

                let index = row as u16 * 32 + column as u16;

                let tile_index = tile_map_addr + index;

                let tile_pattern: u8 = self.vram[(tile_index - 0x8000) as usize];

                let vram_address = match self.LCDC.test_bit(4) {
                    true => {
                        //8800-97FF (unsigned)
                        (tile_pattern as u16 * 16) + tile_data_addr - 0x8000
                    },
                    false => {
                        //8800-97FF (signed)
                        let adjusted = ((tile_pattern as i8) as i16) * 16;
                        let path = (tile_data_addr as i16) + adjusted;
                        (path as u16) - 0x8000
                    },
                };
                
                let py = ((Y % 8) * 2) as u16;


                let t1 = self.vram[ (vram_address + py as u16) as usize ];
                let t2 = self.vram[ (vram_address + py + 1 as u16) as usize ];


                let px = i % 8;

                // let line = (t1 as u16) << 8 | t2 as u16; //>

                let b1 = t1.test_bit(px);
                let b0 = t2.test_bit(px);

                let pixel = (b1 as u8) << 1 | b0 as u8; //>

                let drawn = self.to_rgb(pixel, palette);

                self.display[(buffer + i as u32) as usize] = drawn;
            }
        }

        if self.LCDC.test_bit(5) {
            //draw window
        }
        if self.LCDC.test_bit(1) {
            //draw sprite
        }
    }

    fn to_rgb(&self, pixel: u8, palette: u8) -> u32{
        let colors = [
			0xEEEEEE, // 0 White
			0x999999, // 1 Light Gray
			0x666666, // 2 Dark Gray
			0x222222, // 3 Black
        ];

		let shade = match pixel {
			0 =>  palette & 0b00000011,
			1 => (palette & 0b00001100) >> 2,
			2 => (palette & 0b00110000) >> 4,
			3 => (palette & 0b11000000) >> 6,
			_ => panic!("Invalid Palette Shade!")
		};
		colors[shade as usize]
    }

    // fn search_oam(&mut self){ 
    //     let sprite_max: u8 = match self.LCDC.test_bit(2) {
    //         true => 15,
    //         false => 7
    //     };

    //     self.visible_sprites.clear();

    //     for sprite in self.sprites.iter() {
    //         if sprite.x != 0 && self.lcd_y + sprite_max <= sprite.y && sprite.x != 160 && !self.visible_sprites.full() {
    //             self.visible_sprites.push(sprite.id);
    //         }

    //     }
    // }

    pub fn write_byte(&mut self, addr: u16, byte: u8) -> Response {

        let into = GPU::translate(addr);

        match into {
            Region::VRAM(x) => {
                if !self.lock_vram {
                    self.vram[x] = byte;
                }
            }
            Region::OAM(x) => {
                if !self.lock_oam {
                    self.oam[x] = byte
                }
            }
        }

        Response::None
    }

    pub fn read_byte(&self, addr: u16) -> Response {
        let from = GPU::translate(addr);

        match from {
            Region::VRAM(x) => {
                if !self.lock_vram {
                    return Response::Byte( self.vram[x] as u8 );
                } else {
                    return Response::Byte( 0xFF );
                }
            }
            Region::OAM(x) => {
                if !self.lock_oam {
                    return Response::Byte( self.oam[x] as u8 );
                } else {
                    return Response::Byte( 0xFF );
                }
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
}