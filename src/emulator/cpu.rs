#![allow(non_snake_case)]

pub mod registers;
use registers::{*};

use super::bus::{*};
pub mod instructions;
pub mod subops;
pub mod generic_ops;
pub use instructions::Instruction;
use super::bit_utils::{*};

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
    
    pub fn increment_PC(&mut self, increment: u8){
        self.registers.PC(Action::Increment(increment as u16));
    }

    pub fn decode(opcode: u8, subset: bool) -> Instruction{
        Decoder::decode(opcode, subset)
    }

    pub fn interrupts(&mut self, bus: &mut Bus) -> u8{

        if bus.interrupts.ei_key == EI::Requested {
            bus.interrupts.ei_key = EI::Active;
        }

        if bus.interrupts.ei_key == EI::Active {
            bus.interrupts.master = true;
            bus.interrupts.ei_key = EI::Disabled;
        }

        //if halt was called with interrupts disabled we will wait an interrupt anyway
        if bus.interrupts.master || bus.halt_cpu {
            
            let call_inst = CPU::decode(0xCD, false);
            let f = call_inst.function;

            let vec = bus.interrupts.get_vec();

            if vec == InterruptVector::None { return 0 }

            let mut cycles = 20;

            //HALT was called and caused PC bug
            if bus.interrupts.halt_bug {
                bus.halt_cpu = false;
                return 0;
            };

            //if HALT was called with IME off then we wait until an interrupt is called, but we don't execute it we just unhalt the cpu (and didn't bug)
            if bus.halt_cpu && !bus.interrupts.master {
                bus.halt_cpu = false;
                return 0;
            };

            //HALT was called with IME on
            if bus.halt_cpu { 
                bus.halt_cpu = false;
                cycles = 24;
            };

            bus.interrupts.master = false;

            match vec {
                InterruptVector::VBlank => {
                    bus.interrupts.requests.reset_bit(0);

                    let vector: [u8; 2] = [0x0040, 0x00]; 

                    f( vector , &mut self.registers, bus );
                    
                },
                InterruptVector::LCDC => {
                    bus.interrupts.requests.reset_bit(1);

                    let vector: [u8; 2] = [0x0048, 0x00]; 

                    f( vector , &mut self.registers, bus );
                },
                InterruptVector::Timer => {
                    bus.interrupts.requests.reset_bit(2);

                    let vector: [u8; 2] = [0x0050, 0x00]; 

                    f( vector , &mut self.registers, bus);
                },
                InterruptVector::Serial => {
                    bus.interrupts.requests.reset_bit(3);

                    let vector: [u8; 2] = [0x0058, 0x00]; 

                    f( vector , &mut self.registers, bus);
                },
                InterruptVector::Joypad => {
                    bus.interrupts.requests.reset_bit(4);

                    let vector: [u8; 2] = [0x0060, 0x00]; 

                    f( vector , &mut self.registers, bus);
                },
                _ => unreachable!("This should never happen"),
            }
            return cycles;
        }

        return 0;
    }
}