#![allow(non_snake_case)]

use super::interrupt::{Interrupt};
use super::bit_utils::{*};

pub struct Joypad {
    JOYP: u8
}

impl Default for Joypad { fn default() -> Self { Joypad{ JOYP: 0x3F } } }

impl Joypad {
    pub fn up(&self)  -> Interrupt{     
        if !self.JOYP.test_bit(4) {
            Interrupt::Joypad
        } else {
            Interrupt::None
        }
    }
    pub fn left(&self)  -> Interrupt{
        if !self.JOYP.test_bit(4) {
            Interrupt::Joypad
        } else {
            Interrupt::None
        }
    }
    pub fn right(&self)  -> Interrupt{
        if !self.JOYP.test_bit(4) {
            Interrupt::Joypad
        } else {
            Interrupt::None
        }
    }
    pub fn down(&self)  -> Interrupt{
        if !self.JOYP.test_bit(4) {
            Interrupt::Joypad
        } else {
            Interrupt::None
        }
    }
    pub fn start(&self)  -> Interrupt{
        if !self.JOYP.test_bit(5) {
            Interrupt::Joypad
        } else {
            Interrupt::None
        }
    }
    pub fn select(&self)  -> Interrupt{
        if !self.JOYP.test_bit(5) {
            Interrupt::Joypad
        } else {
            Interrupt::None
        }
    }
    pub fn btn_a(&self)  -> Interrupt{
        if !self.JOYP.test_bit(5) {
            Interrupt::Joypad
        } else {
            Interrupt::None
        }
    }
    pub fn btn_b(&self)  -> Interrupt{
        if !self.JOYP.test_bit(5) {
            Interrupt::Joypad
        } else {
            Interrupt::None
        }
    }

    pub fn write(&mut self, byte: u8) {
        //clears bits 7,6,3,2,1 and 0 (only bits 4 and 5 are W)
        self.JOYP = byte & !0xCF;
    }
    pub fn read(&self) -> u8 {
        self.JOYP
    }
}