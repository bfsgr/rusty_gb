#![allow(non_snake_case)]

use super::interrupt::{*};
use super::bit_utils::{*};
use super::cpu::registers::{Response};

const OAM_SEARCH: usize = 80;
const TRANSFER_CYCLES: usize = 252;
const HBLANK_CYCLES: usize = 456;
const FRAME_CYCLES: usize = 456 * 146;
const VBLANK_CYCLES: usize = FRAME_CYCLES + 456 * 10;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    Oam = 2,
    Transfer = 3
}

#[derive(Copy, Clone)]
struct Tile {
    dirty: bool,
    data: [u8; 16]
}

impl Default for Tile {
    fn default() -> Self { Tile { dirty: true, data: [0; 16]} }
}

#[derive(Copy, Clone, Debug)]
struct Sprite{
    dirty: bool,
    addr: u16,
    x: u8,
    y: u8,
    palette: bool,
    x_flip: bool,
    y_flip: bool,
    priority: bool
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite { dirty: true, addr: 0, x: 0, y: 0, palette: false, x_flip: false, y_flip: false, priority: false}
    }
}

pub struct GPU {
    pub mode: Mode,
    scanline_cycles: usize,
    frame_cycles: usize,

    sprites: [Sprite; 40],
    tile_cache: [Tile; 384],

    lock_vram: bool,
    lock_oam: bool,

    pub LCDC: u8,           //0xFF40     (R/W)
    pub STAT: u8,           //0xFF41     (R/W)
    pub scroll_y: u8,       //0xFF42     (R/W)
    pub scroll_x: u8,       //0xFF43     (R/W)
    pub lcd_y: u8,          //0xFF44     (R)
    pub lycompare: u8,      //0xFF45     (R/W)
    pub OAM_DMA: u8,        //0xFF46     (R/W)
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
            lock_vram: false,
            lock_oam: false,
            sprites: [Sprite::default(); 40],
            tile_cache: [Tile::default(); 384],
            LCDC: 0,           //0xFF40     (R/W)
            STAT: 0x82,           //0xFF41     (R/W)
            scroll_y: 0,       //0xFF42     (R/W)
            scroll_x: 0,       //0xFF43     (R/W)
            lcd_y: 0,          //0xFF44     (R)
            lycompare: 0,      //0xFF45     (R/W)
            window_y: 0,       //0xFF4A     (R/W)   
            window_x: 0,       //0xFF4B     (R/W)
            OAM_DMA: 0,        //0xFF46     (R/W)
            bg_palette: 0,     //0xFF47     (R/W)
            ob_palette0: 0,    //0xFF48     (R/W)
            ob_palette1: 0,    //0xFF49     (R/W)
            bgp_index: 0,      //0xFF68     (R/W) (GB Color only)
            bgp_data: 0,       //0xFF69     (R/W) (GB Color only)
            spt_index: 0,      //0xFF6A     (R/W) (GB Color only)   
            spt_data: 0,       //0xFF6B     (R/W) (GB Color only)
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            display: vec![0; 160*146]
        }
    }
}

enum Region {
    OAM(usize),
    VRAM(usize),
}

impl GPU {
    pub fn step(&mut self, cycles_made: u8, interrupt_handler: &mut InterruptHandler, screen: &mut Vec<u32>){
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
                    self.set_mode(Mode::VBlank);
                    interrupt_handler.request(Interrupt::VBlank);
                    *screen = self.display.clone();
                    //update interrupt flag
                    interrupt_status = self.STAT.test_bit(4);

                }
                //frame_cycles are bigger than the Vblank period, reset everything
                if self.frame_cycles > VBLANK_CYCLES {
                    self.frame_cycles = 0;
                    self.scanline_cycles = 0;
                    self.lcd_y = 0;
                    //compare LY to LYC
                    self.line_compare(interrupt_handler);
                    self.set_mode(Mode::Oam);
                }
            } else {
                //it's not vblank so test scanline cycles
                match self.scanline_cycles {
                    0 ..= OAM_SEARCH  => {
                        //OAM period
                        if cur_mode != Mode::Oam {
                            self.set_mode(Mode::Oam);
                            interrupt_status = self.STAT.test_bit(5);
                        }
                    },
                    OAM_SEARCH ..= TRANSFER_CYCLES => {
                        //Transfer period
                        if cur_mode != Mode::Transfer {
                            self.set_mode(Mode::Transfer);
                            self.draw();
                        }
                    },
                    TRANSFER_CYCLES ..= HBLANK_CYCLES => {
                        if cur_mode != Mode::HBlank {
                            self.set_mode(Mode::HBlank);
                            interrupt_status = self.STAT.test_bit(3);
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
            if self.STAT.test_bit(6) {
                interrupt.request(Interrupt::LCDC)
            }
        } else {
            self.STAT.reset_bit(2);
        }

    }

    pub fn enabled(&self) -> bool{
        self.LCDC.test_bit(7)
    }

    fn draw(&mut self){

        let mut priority = vec![false; 160];

        if self.LCDC.test_bit(0) {
            self.paint_background(&mut priority);
        }

        if self.LCDC.test_bit(5) && self.LCDC.test_bit(0) {
            //draw window
            self.paint_window(&mut priority)
        }
        if self.LCDC.test_bit(1) {
            //search the visible sprites
            let visible = self.search_oam();
            //draw sprite
            self.paint_sprites(visible, &mut priority);
        }
    }

    fn paint_background(&mut self, priority: &mut Vec<bool>){
        //draw bg

        //get palette
        let palette = self.bg_palette;

        //tile map address base
        let tile_map_addr = match self.LCDC.test_bit(3) {
            true => 0x9C00,
            false => 0x9800
        };
        
        //tile data address base
        let tile_data_addr = match self.LCDC.test_bit(4) {
            true => 0x8000,
            false => 0x9000
        };

        //Current line (Y axis)
        let LY = self.lcd_y;

        //Apply scroll effect, if any
        let Y = LY.wrapping_add(self.scroll_y);

        //Set buffer position (to display) as current line times 160 (screen width)
        let buffer: u32 = LY as u32 * 160;
        
        //Current row in a tile
        let row = Y / 8;

        //For each pixel in a line
        for i in 0..160 {
            //Apply scroll effect, if any
            let X = (i as u8).wrapping_add(self.scroll_x);

            //Current column in a tile
            let column = X / 8;


            // Upper 5 bits select the row, 5 first the column
            let index = ((row as u16) << 5) + column as u16; //> 

            let tile_index = tile_map_addr + index;

            let tile_pattern: u8 = self.vram[(tile_index - 0x8000) as usize];

            let raw_address = match self.LCDC.test_bit(4) {
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

            let id = (raw_address / 16) as usize;

            if self.tile_cache[id].dirty {
                self.update_tile(id, raw_address);
            }

            let cache = &self.tile_cache[ id ];

            let py = ((Y % 8) * 2) as u16;
            let px = X % 8;

            let mut t1 = cache.data[py as usize];    
            let mut t2 = cache.data[(py+1) as usize];

            t1 = GPU::reverse_order(t1);
            t2 = GPU::reverse_order(t2);
            
            let b1 = t1.test_bit(px);   
            let b0 = t2.test_bit(px);

            let pixel = (b1 as u8) << 1 | b0 as u8; //>
            

            let drawn = self.to_rgb(pixel, palette);

            priority[i] = pixel != 0;

            self.display[(buffer + i as u32) as usize] = drawn;
        }
    }

    
    fn paint_window(&mut self, priority: &mut Vec<bool>){

        let palette = self.bg_palette;

        let WY = self.window_y;
        let WX = self.window_x;

        //tile map address base
        let tile_map_addr = match self.LCDC.test_bit(6) {
            true => 0x9C00,
            false => 0x9800
        };
        
        //tile data address base
        let tile_data_addr = match self.LCDC.test_bit(4) {
            true => 0x8000,
            false => 0x9000
        };

        let LY = self.lcd_y;

        let buffer: u32 = LY as u32 * 160;

        let row = LY / 8;

        //window does not appear in this row
        if LY < WY || WY > 143 { return (); }
        if WX > 159 { return (); }

        for i in WX..160 {

            let column = i/8;

            // Upper 5 bits select the row, 5 first the column
            let index = ((row as u16) << 5) + column as u16; //> 

            let tile_index = tile_map_addr + index;

            let tile_number: u8 = self.vram[(tile_index - 0x8000) as usize];

            let raw_address = match self.LCDC.test_bit(4) {
                true => {
                    //8800-97FF (unsigned)
                    (tile_number as u16 * 16) + tile_data_addr - 0x8000
                },
                false => {
                    //8800-97FF (signed)
                    let adjusted = ((tile_number as i8) as i16) * 16;
                    let path = (tile_data_addr as i16) + adjusted;
                    (path as u16) - 0x8000
                },
            };

            let id = (raw_address / 16) as usize;

            if self.tile_cache[id].dirty {
                self.update_tile(id, raw_address);
            }

            let cache = self.tile_cache[ id ];

            let py = ((LY % 8) * 2) as u16;
            let px = i % 8;

            let mut t1 = cache.data[py as usize];    
            let mut t2 = cache.data[(py+1) as usize];

            t1 = GPU::reverse_order(t1);
            t2 = GPU::reverse_order(t2);
            
            let b1 = t1.test_bit(px);   
            let b0 = t2.test_bit(px);

            let pixel = (b1 as u8) << 1 | b0 as u8; //>
            

            let drawn = self.to_rgb(pixel, palette);

            priority[i as usize] = pixel != 0;

            self.display[(buffer + i as u32) as usize] = drawn;
        }
    }
    
    fn paint_sprites(&mut self, visible: Vec<Sprite>, priority: &mut Vec<bool>){

        let ly = self.lcd_y;
        let tall = self.LCDC.test_bit(2);

        for sprite in visible.iter().rev() {
            let sx = sprite.x;
            let sy = sprite.y;

            let y = ly.wrapping_sub(sy) % 8;

            let py = match sprite.y_flip {
                true =>  { ((y as i8 - 7) * -1) as u8 },
                false => y
            };

            let id = match tall {
                true => {
                    if ly.wrapping_sub(sy) < 8 {
                        if sprite.y_flip {
                            sprite.addr | 1
                        } else {
                            sprite.addr & 0xFE
                        }
                    } else {
                        if sprite.y_flip {
                            sprite.addr & 0xFE
                        } else {
                            sprite.addr | 1
                        }
                    }
                },
                false => sprite.addr
            };
            

            if self.tile_cache[id as usize].dirty {
                self.update_tile(id as usize, id*16);
            }

            let tile = self.tile_cache[id as usize];

            let palette = match sprite.palette {
                true => self.ob_palette1,
                false => self.ob_palette0,
            };

            for i in 0..8 {
                let actual_x = sx + i;

                if actual_x >= 160 { continue; }

                let px = match sprite.x_flip {
                    true => ((i as i8 - 7) * -1) as u8,
                    false => i
                };

                let mut t1 = tile.data[py as usize];    
                let mut t2 = tile.data[(py+1) as usize];
    
                t1 = GPU::reverse_order(t1);
                t2 = GPU::reverse_order(t2);
                
                let b1 = t1.test_bit(px);   
                let b0 = t2.test_bit(px);

                let pixel = (b1 as u8) << 1 | b0 as u8; //>

                if pixel == 0 { continue; }

                let drawn = self.to_rgb(pixel, palette);

                if sprite.priority && priority[actual_x as usize] { continue; }
    
                self.display[(ly as u16 * 160 + actual_x as u16) as usize] = drawn;
            }

        }
    }

    fn reverse_order(mut b: u8) -> u8{
        b = (b & 0xF0) >> 4 | (b & 0x0F) << 4; //>
        b = (b & 0xCC) >> 2 | (b & 0x33) << 2; //> 
        b = (b & 0xAA) >> 1 | (b & 0x55) << 1; //>
        b
    }

    fn to_rgb(&self, pixel: u8, palette: u8) -> u32{
        let colors = [
			0xE0F8D0, // 0 White
			0x346856, // 1 Light Gray
			0x88C070, // 2 Dark Gray
			0x081820, // 3 Black
        ];

		let shade = match pixel {
			0 =>  palette & 0b00000011,
			1 => (palette & 0b00001100) >> 2,
			2 => (palette & 0b00110000) >> 4,
			3 => (palette & 0b11000000) >> 6,
			_ => panic!("Invalid pixel number")
		};
		colors[shade as usize]
    }

    fn update_tile(&mut self, id: usize, raw_addr: u16) {
        let tile = &mut self.tile_cache[id];
        
        for i in (raw_addr..raw_addr+16).enumerate() {
            tile.data[i.0] = self.vram[i.1 as usize];
        }

        tile.dirty = false;
    }

    fn update_sprite(&mut self, mut index: usize){
        
        let current = &mut self.sprites[index];

        //Adjust index to oam address
        index *= 4;

        current.y = self.oam[index];
        current.x = self.oam[index+1];
        current.addr = self.oam[index+2] as u16;

        
        let flags = self.oam[index+3];
        
        current.priority = flags.test_bit(7);
        current.y_flip = flags.test_bit(6);
        current.x_flip = flags.test_bit(5);
        current.palette = flags.test_bit(4);

        current.dirty = false;
    }

    fn search_oam(&mut self) -> Vec<Sprite> { 
        let sprite_max: u8 = match self.LCDC.test_bit(2) {
            true => 15,
            false => 7
        };

        let mut visible_sprites: Vec<Sprite> = vec![];

        for i in 0..40 {    

            
            if self.sprites[i].dirty {
                self.update_sprite(i as usize);
            }
            
            let sprite = self.sprites[i];

            if sprite.x > 8 && self.lcd_y >= sprite.y && self.lcd_y <= sprite.y + sprite_max && sprite.x < 160 && visible_sprites.len() < 11 {
                
                visible_sprites.push(self.sprites[i]);
            }

        }
        visible_sprites
    }

    pub fn set_mode(&mut self, mode: Mode) {
        let save = self.STAT;
        match mode {
            Mode::HBlank => {
                self.STAT = save & 0xFC;
                self.lock_vram = false;
                self.lock_oam = false;
            },
            Mode::VBlank => {
                self.STAT = save & 0xFC;
                self.STAT.set_bit(0);
                self.lock_vram = false;
                self.lock_oam = false;
            },
            Mode::Oam => {
                self.STAT = save & 0xFC;
                self.STAT.set_bit(1);
                self.lock_oam = true;
                self.lock_vram = false;
            },
            Mode::Transfer => {
                self.STAT.set_bit(0);
                self.STAT.set_bit(1);
                self.lock_vram = true;
                self.lock_oam = true;
            }
        }
        self.mode = mode;
    }

    pub fn write_lcdc(&mut self, byte: u8) {
        if !byte.test_bit(7) && self.enabled() {
            if self.mode != Mode::VBlank {
                panic!("Turned LCD off outside of Vblank")
            }
            self.lcd_y = 0;
            self.STAT = 0x80;
            self.mode = Mode::HBlank;
            self.scanline_cycles = 0;
            self.frame_cycles = 0;
        }
        if byte.test_bit(7) && !self.enabled() {
            if self.lcd_y == self.lycompare { 
                self.STAT.set_bit(2);
            } else {
                self.STAT.reset_bit(2)
            }
        }
        self.LCDC = byte
    }

    pub fn write_stat(&mut self, byte: u8) {
        //only keep bytes 3-6
        let data = (byte & 0xF8) | 0x80;
        self.STAT = data;
    }

    pub fn read_stat(&mut self) -> u8 {
        self.STAT | 0x80
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) -> Response {

        let into = GPU::translate(addr);

        match into {
            Region::VRAM(x) => {
                if !self.lock_vram {
                    self.vram[x] = byte;

                    if x < 0x1800 {
                        self.tile_cache[ (x/16) as usize].dirty = true;
                    }
                }
            }
            Region::OAM(x) => {
                if !self.lock_oam {
                    self.sprites[x/4].dirty = true;
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