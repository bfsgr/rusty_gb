#![allow(non_snake_case)]

use super::cpu::CPU;
use super::bit_utils::BitUtils;

pub enum Interrupt {
    VBlank,
    LCDC,
    Timer,
    Serial,
    Joypad,
}

enum InterruptVector {
    VBlank = 0x40,
    LCDC = 0x48,
    Timer = 0x50,
    Serial = 0x58,
    Joypad = 0x60,
}

#[derive(Default)]
pub struct InterruptHandler  {
    pub master: bool,               //reduntant from memory field interrupt switch
    pub enable: u8,             //reduntant from memory 0xFFFF
    pub requests: u8,           //reduntant from memory field 0xFF0F
    pub saved_state: [u16; 2]
}

impl InterruptHandler {
    pub fn execute(&mut self, state: &mut CPU) {

        if self.master {
            for i in 0..5 {
                if (self.requests & i) == 1 << i { //>
                    match i {
                        0 => self.VBlank(state),
                        1 => self.LCDC(state),
                        2 => self.Timer(state),
                        3 => self.Serial(state),
                        4 => self.Joypad(state),
                        _ => panic!("Interrupt processing error"),
                    }
                }
            }
        }
    }

    fn VBlank(&mut self, state: &mut CPU){
        self.master = false;

        self.saved_state[0] = state.PC();
        self.saved_state[1] = state.SP();

        state.set_PC(InterruptVector::VBlank as u16);
    }
    fn LCDC(&mut self, state: &mut CPU){
        self.master = false;

        self.saved_state[0] = state.PC();
        self.saved_state[1] = state.SP();

        state.set_PC(InterruptVector::LCDC as u16);
    }
    fn Timer(&mut self, state: &mut CPU){
        self.master = false;

        self.saved_state[0] = state.PC();
        self.saved_state[1] = state.SP();

        state.set_PC(InterruptVector::Timer as u16);
    }
    fn Serial(&mut self, state: &mut CPU){
        self.master = false;

        self.saved_state[0] = state.PC();
        self.saved_state[1] = state.SP();

        state.set_PC(InterruptVector::Serial as u16);
    }
    fn Joypad(&mut self, state: &mut CPU){
        self.master = false;

        self.saved_state[0] = state.PC();
        self.saved_state[1] = state.SP();

        state.set_PC(InterruptVector::Joypad as u16);
    }

    pub fn request(&mut self, interrupt: Interrupt) {
        if self.master {
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
            }
        }
    }
}
