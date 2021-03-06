#![allow(non_snake_case)]

use super::io_constants::{*};
use super::cpu::registers::Response;
use super::bit_utils::BitUtils;
use super::interrupt::{*};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Frequency {
    Mode0 = 4096,
    Mode1 = 262144,
    Mode2 = 65536,
    Mode3 = 16384,
}

impl Default for Frequency {
    fn default() -> Self { Frequency::Mode0 }
}

pub struct Timer {
    TIMA: u8,
    TMA: u8,
    TAC: u8,
    SYS_CLK: u16,
    timer_count: i32,
    frequency: Frequency
}

impl Default for Timer {
    fn default() -> Self {
        Timer{
            TIMA: 0,
            TMA: 0,
            TAC: 0,
            SYS_CLK: 0,
            timer_count: 4194304 / 16384,
            frequency: Frequency::Mode0,
        } 
    }
}

impl Timer {
    pub fn write_byte(&mut self, addr: u16, byte: u8) -> Response {
        match addr {
            DIV => self.SYS_CLK = 0,
            TIMA => self.TIMA = byte,
            TMA => self.TMA = byte,
            TAC => self.TAC = byte,
            _ => {}
        }

        Response::None
    }

    pub fn read_byte(&self, addr: u16) -> Response {
        match addr {
            DIV => Response::Byte( (self.SYS_CLK >> 8) as u8 ),
            TIMA => Response::Byte( self.TIMA ),
            TMA => Response::Byte( self.TMA ),
            TAC => Response::Byte( self.TAC ),
            _ => unreachable!()
        }
    }

    pub fn step(&mut self, cycles: u8, interrupts: &mut InterruptHandler) {
        self.update_div(cycles);
        
        
        if self.enabled() {

            let freq = self.get_freq();

            if freq != self.frequency {
                self.set_frequency(freq);
            }
            self.timer_count -= cycles as i32;

            if self.timer_count <= 0{

                self.set_frequency(freq);

                if self.TIMA == 255 {
                    self.TIMA = self.TMA;
                    interrupts.request(Interrupt::Timer);
                } else {
                    self.TIMA += 1;
                }
            }
        }

    }

    fn enabled(&self) -> bool {
        self.TAC.test_bit(2)
    }

    fn update_div(&mut self, cycles: u8) {
        self.SYS_CLK = self.SYS_CLK.wrapping_add(cycles as u16);
    }

    fn set_frequency(&mut self, frequency: Frequency) {
        self.frequency = frequency;
        self.timer_count = 4194304 / (frequency as i32);
    }

    fn get_freq(&self) -> Frequency {
        match self.TAC & 3 {
            0 => Frequency::Mode0,
            1 => Frequency::Mode1,
            2 => Frequency::Mode2,
            3 => Frequency::Mode3,
            _ => unreachable!()
        }
    }
}