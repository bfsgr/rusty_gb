pub mod registers;
use registers::*;
mod instructions;
pub mod decoder;
mod generic;
mod subops;

use super::Bus;
use decoder::Decoder;
use instructions::Instruction;
pub struct CPU{
    registers: Registers,
    instruction: Instruction
}

impl Default for CPU {
    fn default() -> Self {
        Self{
            registers: Registers::default(),
            instruction: Instruction::holder()
        }
    }
}

impl CPU {
    pub fn tick(&mut self, bus: &mut Bus){
        //instruction still running
        if self.instruction.cycles > 0 {
            self.instruction.tick(&mut self.registers, bus);
        } else {
            //fetch pc, decode, append and tick instruction
            let data = self.fetch(bus);
            self.instruction = Decoder::decode(data.0, data.1).unwrap();
            self.instruction.tick(&mut self.registers, bus);
            println!("{}", self.instruction)
        }
    }

    fn fetch(&mut self, bus: &mut Bus) -> (u8, bool) {
        let pc: u16 = self.registers.PC( Action::Read ).value();
        print!("{:#10x}: ", pc);

        self.registers.PC( Action::Increment(1) );

        let mut opcode: u8 = bus.read_byte(pc).value();

        if opcode != 0xCB {
            return (opcode, false);
        } else {
            opcode = bus.read_byte(pc+1).value();
            self.registers.PC( Action::Increment(1) );
            return (opcode, true);
        }
    }
}
