use std::collections::vec_deque::*;
use interface::*;
use super::interface;

pub struct PixelFIFO {
    pub wx: u8,
    pub wy: u8,
    pub sx: u8,
    pub sy: u8,
    fifo: VecDeque<u32>
}

impl Default for PixelFIFO {
    fn default() -> Self {
        Self {
            wy: 0,
            wx: 0,
            sx: 0,
            sy: 0,
            fifo: VecDeque::with_capacity(16)
        }
    }
}

impl PixelFIFO {
    pub fn step(&mut self, screen: &mut Vec<u32>, position: usize ) {
        if self.fifo.len() > 8 {
            screen[position] = self.fifo.pop_front().unwrap();
        }
    }
    
    pub fn try_push_pixels(&mut self, pixels: &mut VecDeque<u32>) -> bool {
        if self.fifo.len() > 8 {
            return false;
        }
        //push 8 pixels into the FIFO
        self.fifo.append(pixels);
        return true;    
    }
}

