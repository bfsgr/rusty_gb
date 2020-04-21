pub mod registers;
use registers::Registers;
use registers::Operations;
use registers::BitTest;

mod decoder;
use decoder::Decoder;
use decoder::Instruction;

use super::memory;
use memory::mmu::MMU;

use yansi::Paint;

const ZERO_FLAG: u8 = 1;
const NEGATIVE_FLAG: u8 = 2;
const HALFCARRY_FLAG: u8 = 3;
const CARRY_FLAG: u8 = 4;


pub struct CPU {
    pub registers: Registers,
    pub decoder: Decoder,
    pub mmu: MMU
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            registers: Registers::default(),
            decoder: Decoder::default(),
            mmu: MMU::default()
        }
    }
}

impl CPU {
    //start decoding instructions at 0x0000
    pub fn run(&mut self){
        let save_pc = self.registers.PC;
        let mut PC = self.mmu.read_byte(save_pc);
        
        if PC != 0xCB {
            let operands = self.operands_needed(PC);
            match operands {
                0 => { 
                    self.registers.PC += 1;
                    self.execute(PC, [0;2], false);

                },

                1 => {
                    let operand = self.mmu.read_byte(self.registers.PC+1);
                    self.registers.PC += 2;
                    self.execute(PC, [
                        operand,
                        0
                    ], false);
                }

                2 => {
                    let operand1 = self.mmu.read_byte(self.registers.PC+1);
                    let operand2 = self.mmu.read_byte(self.registers.PC+2);
                    self.registers.PC += 3;
                    self.execute(PC, [
                            operand2, operand1 //16bit operands is LSB->MSB
                        ], false);
 
                }
                _ => { 
                    //two-byte instruction
                }
            }
        } else {
            PC = self.mmu.read_byte(save_pc+1);
            self.registers.PC += 2;
            self.execute(PC, [0;2], true);
        }
    }

    fn operands_needed(&self, opcode: u8) -> u8{
        self.decoder.inst[opcode as usize].osize
    }

    fn operands_needed_bit(&self, opcode: u8) -> u8{
        self.decoder.bit_inst[opcode as usize].osize
    }

    fn execute(&mut self, opcode: u8, operands: [u8; 2], two_bit: bool){
        if !two_bit {
            let function = self.decoder.inst[opcode as usize].inst;
            let inst = self.decoder.inst[opcode as usize].clone(); //clone instruction info since we can't borrow here then again after
            // CPU::debug(&inst, opcode, operands);
            function(inst, opcode, operands,  self);
        } else {
            let function = self.decoder.bit_inst[opcode as usize].inst;
            let inst = self.decoder.bit_inst[opcode as usize].clone();
            // CPU::debug(&inst, opcode, operands);
            function(inst, opcode, operands, self);
        }
    }

    fn debug(operation: &Instruction, opcode: u8, operands: [u8;2]){
        if operation.osize == 0 {
            println!("{:#04x}: {}\r\t\t\t{}\t\t{}", opcode, Paint::green(operation.disassembly), Paint::cyan("-"), Paint::magenta("TRYING"))
        } else if operation.osize == 1 { 
            println!("{:#04x}: {}\r\t\t\t{:#04x?}\t\t{}",opcode, Paint::green(operation.disassembly), Paint::cyan(operands[0]), Paint::magenta("TRYING"));
        } else {
            let operand = MMU::to_short_lsb(operands);
            println!("{:#04x}: {}\r\t\t\t{:#04x?}\t\t{}", opcode, Paint::green(operation.disassembly), Paint::cyan(operand), Paint::magenta("TRYING"));
        }   
    }

    fn nop(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        if op.osize == 0 {
            println!("{:#04x}: {}\r\t\t\t{}\t\t{}", opcode, Paint::green(op.disassembly), Paint::cyan("-"), Paint::red("MISSING"))
        } else if op.osize == 1 { 
            println!("{:#04x}: {}\r\t\t\t{:#04x?}\t\t{}",opcode, Paint::green(op.disassembly), Paint::cyan(operand[0]), Paint::red("MISSING"));
        } else {
            let operand = MMU::to_short_lsb(operand);
            println!("{:#04x}: {}\r\t\t\t{:#04x?}\t\t{}", opcode, Paint::green(op.disassembly), Paint::cyan(operand), Paint::red("MISSING")) ;
        }
    }

    fn ld_sp_nn(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let short = MMU::to_short_lsb(operand);
        state.registers.SP = short;
    }

    fn xor_a(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let a = state.registers.AF.T;

        state.registers.AF.T = a ^ a; 
    }

    fn ld_hl_nn(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let short = MMU::to_short_lsb(operand);
        state.registers.HL.w16b(short);
    }

    fn ldd_HL_A(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let hlad = state.registers.HL.r16b();
        state.mmu.write_byte(hlad, state.registers.AF.T);
        state.registers.HL.decrement();
    }

    fn bit_7h(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        if state.registers.HL.T.test(7) {
            state.registers.set_flag(ZERO_FLAG);
        }
        state.registers.set_flag(HALFCARRY_FLAG);
        state.registers.clear_flag(NEGATIVE_FLAG);
    }

    fn jp_nz_n(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        if state.registers.AF.L.test(7) {
            let jump = operand[0] as i8;
            if jump >= 0 {
                state.registers.PC += jump as u16;
            } else {
                state.registers.PC -= jump.abs() as u16; 
            }
        }
    }

    fn ld_C_n(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        
    }

}

