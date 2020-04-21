// READ AND WRITE OPERATONS OVER RAM

pub mod mmu;

use mmu::MMU;
use super::cpu::registers::Registers;


impl MMU {
    //Stack management functions
    pub fn write_short_to_stack(&mut self, reg: &mut Registers, short: u16){
        reg.SP = reg.SP - 2;
        self.write_short(reg.SP, short);
    }

    pub fn read_short_from_stack(&self, reg: &mut Registers) -> u16 {
        let capture = self.read_short(reg.SP);
        reg.SP = reg.SP - 2;
        capture
    }
    pub fn push_range(&mut self, data: &Vec<u8>, start: usize, lenght: usize ){
        for i in start..lenght {
            self.memory[i] = data[i];
        }
    }

}


