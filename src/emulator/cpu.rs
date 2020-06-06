#![allow(non_snake_case)]

pub mod registers;
use registers::{*};

use super::bus::{*};
pub mod instructions;
pub use instructions::Instruction;
use super::bit_utils::{*};

mod decoder;
use decoder::{*};

#[derive(Default)]
pub struct CPU {
    pub registers: Registers,
}

pub const NOT_IMPLEMENTED: Instruction = Instruction {
    disassembly: "Not Implemented",
    function: Instruction::NOP,
    args: 0,
    cycles: 0
};

impl CPU {
    
    pub fn PC(&mut self) -> u16{
        self.registers.PC(Action::Read).value()
    }
    
    pub fn increment_PC(&mut self, increment: u8){
        self.registers.PC(Action::Increment(increment as u16));
    }

    pub fn decode(opcode: u8, subset: bool) -> Instruction{
        Decoder::decode(opcode, subset)
    }

    pub fn interrupts(&mut self, bus: &mut Bus){
        if bus.interrupts.master {
            
            let call_inst = CPU::decode(0xCD, false);
            let f = call_inst.function;

            let vec = bus.interrupts.get_vec();

            if vec == InterruptVector::None { return () }

            bus.interrupts.master = false;

            match vec {
                InterruptVector::VBlank => {
                    bus.interrupts.requests.reset_bit(0);
                    bus.interrupts.enable.set_bit(0);

                    let vector: [u8; 2] = [0x0040, 0x00]; 

                    f( vector , &mut self.registers, bus );
                },
                InterruptVector::LCDC => {
                    bus.interrupts.requests.reset_bit(1);
                    bus.interrupts.enable.set_bit(1);

                    let vector: [u8; 2] = [0x0048, 0x00]; 

                    f( vector , &mut self.registers, bus );
                },
                InterruptVector::Timer => {
                    bus.interrupts.requests.reset_bit(2);
                    bus.interrupts.enable.set_bit(2);

                    let vector: [u8; 2] = [0x0050, 0x00]; 

                    f( vector , &mut self.registers, bus);
                },
                InterruptVector::Serial => {
                    bus.interrupts.requests.reset_bit(3);
                    bus.interrupts.enable.set_bit(3);

                    let vector: [u8; 2] = [0x0058, 0x00]; 

                    f( vector , &mut self.registers, bus);
                },
                InterruptVector::Joypad => {
                    bus.interrupts.requests.reset_bit(4);
                    bus.interrupts.enable.set_bit(4);

                    let vector: [u8; 2] = [0x0060, 0x00]; 

                    f( vector , &mut self.registers, bus);
                },
                InterruptVector::None => {},
            }

        }
    }
}