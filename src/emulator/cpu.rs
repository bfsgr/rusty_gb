#![allow(non_snake_case)]

mod registers;
use registers::{*};

pub mod instructions;
pub use instructions::Instruction;

mod decoder;
use decoder::{*};

#[derive(Default)]
pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    
    pub fn PC(&mut self) -> u16{
        self.registers.PC(Action::Read).value()
    }

    pub fn set_PC(&mut self, short: u16) {
        self.registers.PC( Action::Write(short) );
    }
   
    pub fn set_SP(&mut self, short: u16) {
        self.registers.SP( Action::Write(short) );
    }
    
    pub fn SP(&mut self) -> u16{
        self.registers.SP(Action::Read).value()
    }
    pub fn increment_PC(&mut self, increment: u8){
        self.registers.PC(Action::Increment(increment as u16));
    }

    pub fn decode(opcode: u8, subset: bool) -> Instruction{
        Decoder::decode(opcode, subset)
    }
}