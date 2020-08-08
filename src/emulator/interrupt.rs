#![allow(non_snake_case)]

use super::bit_utils::BitUtils;
use std::fmt::{Formatter, Result, Display};

#[derive(PartialEq)]
pub enum EI {
    Requested,
    Active,
    Disabled,
}

impl Default for EI {
    fn default() -> Self { EI::Disabled }
}


#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)] //Serial will be used when we implement it
pub enum Interrupt {
    VBlank,
    LCDC,
    Timer,
    Serial,
    Joypad,
    None
}

#[derive(PartialEq)]
pub enum InterruptVector {
    VBlank = 0x40,
    LCDC = 0x48,
    Timer = 0x50,
    Serial = 0x58,
    Joypad = 0x60,
    None
}

#[derive(Default)]
pub struct InterruptHandler  {
    pub master: bool,
    pub halt_bug: bool,
    pub ei_key: EI,
    pub enable: u8,
    pub requests: u8,
}

impl Display for InterruptHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        write!(f, "IME: {} | IE: {:#6x} | IF: {:#6x}", self.master, self.enable, self.requests)
    }
}

impl InterruptHandler {

    pub fn get_vec(&self) -> InterruptVector {

        for i in 0..5 {
            if (self.requests & 1 << i) == 1 << i { //>>
                match i {
                    0 => {
                        if self.enable.test_bit(0) {
                            return InterruptVector::VBlank;
                        }
                    },
                    1 =>{
                        if self.enable.test_bit(1) {
                            return InterruptVector::LCDC;
                        }
                    }
                    2 => {
                        if self.enable.test_bit(2) {
                            return InterruptVector::Timer
                        }
                    },
                    3 => {
                        if self.enable.test_bit(3) {
                            return InterruptVector::Serial
                        }
                    },
                    4 => {
                        if self.enable.test_bit(4) {
                            return InterruptVector::Joypad
                        }
                    },
                    _ => return InterruptVector::None
                }
            }
        }

        return InterruptVector::None;

    }

 
    pub fn request(&mut self, interrupt: Interrupt) {
        match interrupt {
            Interrupt::VBlank => {
                self.requests.set_bit(0)
            },
            Interrupt::LCDC => {
                self.requests.set_bit(1)
            },
            Interrupt::Timer => {
                self.requests.set_bit(2)
            },
            Interrupt::Serial => {
                self.requests.set_bit(3)
            },
            Interrupt::Joypad => {
                self.requests.set_bit(4)
            },
            _ => unreachable!("")
        }
    }
}
