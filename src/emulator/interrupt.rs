#![allow(non_snake_case)]

use super::cpu::CPU;

pub enum Interrupt {
    VBlank,
    LCDC,
    Timer,
    Serial,
    Joypad,
}

enum InterruptBit {
    VBlank =    0b00000001,
    LCDC =      0b00000010,
    Timer =     0b00000100,
    Serial =    0b00001000,
    Joypad =    0b00010000
}

pub enum InterruptVector {
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

        state.set_PC(40);
    }
    fn LCDC(&mut self, state: &mut CPU){
        self.master = false;

        self.saved_state[0] = state.PC();
        self.saved_state[1] = state.SP();

        state.set_PC(48);
    }
    fn Timer(&mut self, state: &mut CPU){
        self.master = false;

        self.saved_state[0] = state.PC();
        self.saved_state[1] = state.SP();

        state.set_PC(50);
    }
    fn Serial(&mut self, state: &mut CPU){
        self.master = false;

        self.saved_state[0] = state.PC();
        self.saved_state[1] = state.SP();

        state.set_PC(58);
    }
    fn Joypad(&mut self, state: &mut CPU){
        self.master = false;

        self.saved_state[0] = state.PC();
        self.saved_state[1] = state.SP();

        state.set_PC(60);
    }

    pub fn request(&mut self, interrupt: Interrupt) {
        if self.master {
            match interrupt {
                Interrupt::VBlank => {
                    self.requests = self.requests | InterruptBit::VBlank as u8
                },
                Interrupt::LCDC => {
                    self.requests = self.requests | InterruptBit::LCDC as u8
                },
                Interrupt::Timer => {
                    self.requests = self.requests | InterruptBit::Timer as u8
                },
                Interrupt::Serial => {
                    self.requests = self.requests | InterruptBit::Serial as u8
                },
                Interrupt::Joypad => {
                    self.requests = self.requests | InterruptBit::Joypad as u8
                },
            }
        }
    }
}
