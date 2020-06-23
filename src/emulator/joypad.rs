#![allow(non_snake_case)]

use super::interrupt::{Interrupt};
use super::bit_utils::{*};

pub struct Joypad {
    JOYP: u8
}

impl Default for Joypad { fn default() -> Self { Joypad{ JOYP: 0xFF } } }

impl Joypad {
    pub fn up(&mut self, pressed: bool)  -> Interrupt{     
        if !self.JOYP.test_bit(4) {
            match pressed {
                true => {
                    self.JOYP.reset_bit(2);
                    Interrupt::Joypad
                },
                false => {
                    self.JOYP.set_bit(2);
                    Interrupt::None
                }
            }
        } else {
            Interrupt::None
        }
    }
    pub fn left(&mut self, pressed: bool)  -> Interrupt{
        if !self.JOYP.test_bit(4) {
            match pressed {
                true => {
                    self.JOYP.reset_bit(1);
                    Interrupt::Joypad
                },
                false => {
                    self.JOYP.set_bit(1);
                    Interrupt::None
                }
            }
        } else {
            Interrupt::None
        }
    }
    pub fn right(&mut self, pressed: bool)  -> Interrupt{
        if !self.JOYP.test_bit(4) {
            match pressed {
                true => {
                    self.JOYP.reset_bit(0);
                    Interrupt::Joypad
                },
                false => {
                    self.JOYP.set_bit(0);
                    Interrupt::None
                }
            }
        } else {
            Interrupt::None
        }
    }
    pub fn down(&mut self, pressed: bool)  -> Interrupt{
        if !self.JOYP.test_bit(4) {
            match pressed {
                true => {
                    self.JOYP.reset_bit(3);
                    Interrupt::Joypad
                },
                false => {
                    self.JOYP.set_bit(3);
                    Interrupt::None
                }
            }
        } else {
            Interrupt::None
        }
    }
    pub fn start(&mut self, pressed: bool)  -> Interrupt{
        if !self.JOYP.test_bit(5) {
            match pressed {
                true => {
                    self.JOYP.reset_bit(3);
                    Interrupt::Joypad
                },
                false => {
                    self.JOYP.set_bit(3);
                    Interrupt::None
                }
            }
        } else {
            Interrupt::None
        }
    }
    pub fn select(&mut self, pressed: bool)  -> Interrupt{
        if !self.JOYP.test_bit(5) {
            match pressed {
                true => {
                    self.JOYP.reset_bit(2);
                    Interrupt::Joypad
                },
                false => {
                    self.JOYP.set_bit(2);
                    Interrupt::None
                }
            }
        } else {
            Interrupt::None
        }
    }
    pub fn btn_a(&mut self, pressed: bool)  -> Interrupt{
        if !self.JOYP.test_bit(5) {
            match pressed {
                true => {
                    self.JOYP.reset_bit(0);
                    Interrupt::Joypad
                },
                false => {
                    self.JOYP.set_bit(0);
                    Interrupt::None
                }
            }
        } else {
            Interrupt::None
        }
    }
    pub fn btn_b(&mut self, pressed: bool)  -> Interrupt{
        if !self.JOYP.test_bit(5) {
            match pressed {
                true => {
                    self.JOYP.reset_bit(1);
                    Interrupt::Joypad
                },
                false => {
                    self.JOYP.set_bit(1);
                    Interrupt::None
                }
            }
        } else {
            Interrupt::None
        }
    }

    pub fn write(&mut self, byte: u8) {
        //clears bits 7,6,3,2,1 and 0 (only bits 4 and 5 are W)
        let data = byte & 0x30;
        self.JOYP = self.JOYP & !0x30;
        self.JOYP |= data;
    }
    pub fn read(&self) -> u8 {
        self.JOYP
    }
}