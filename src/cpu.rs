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
        let save_pc = self.registers.PC; //get Program Counter Pointer
        let mut PC = self.mmu.read_byte(save_pc); //dereference PC pointer
        
        //check if instruction is one or two-bytes 
        if PC != 0xCB { 
            //get the number of immediate bytes needed to run the instruction
            let operands = self.operands_needed(PC);
            //switch over operands
            match operands {
                //case 0 bytes needed, increment PC by 1 and run instruction
                0 => { 
                    self.registers.PC += 1;
                    self.execute(PC, [0;2], false);

                },
                //case 1 byte needed, read it, increment PC by 2 and run 
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

    //fn operands_needed_bit(&self, opcode: u8) -> u8{
    //    self.decoder.bit_inst[opcode as usize].osize
    //}

    fn execute(&mut self, opcode: u8, operands: [u8; 2], two_bit: bool){
        if !two_bit {
            let function = self.decoder.inst[opcode as usize].inst;
            let inst = self.decoder.inst[opcode as usize].clone(); //clone instruction info since we can't borrow here then again after
            CPU::debug(&inst, opcode, operands);
            function(inst, opcode, operands,  self);
        } else {
            let function = self.decoder.bit_inst[opcode as usize].inst;
            let inst = self.decoder.bit_inst[opcode as usize].clone();
            CPU::debug(&inst, opcode, operands);
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

    //Stack management functions
    fn push_to_stack(&mut self, short: u16){
        self.registers.SP = self.registers.SP - 2;
        self.mmu.write_short(self.registers.SP, short);
    }

    fn pop_from_stack(&mut self) -> u16 {
        let capture = self.mmu.read_short(self.registers.SP);
        self.registers.SP = self.registers.SP + 2;
        capture
    }

    //generic increment function
    fn inc(mut val: u8, regs: &mut Registers) -> u8{
        //will lower nibble overflow?
        if (val & 0x0F) == 0x0F {
            regs.set_flag(HALFCARRY_FLAG);
        } else {
            regs.clear_flag(HALFCARRY_FLAG);
        }

        val += 1;

        if val == 0 {
            regs.set_flag(ZERO_FLAG);
        } else {
            regs.clear_flag(ZERO_FLAG);
        }

        regs.clear_flag(NEGATIVE_FLAG);

        return val;
    }

    //generic decrement function
    fn dec(mut val: u8, regs: &mut Registers) -> u8{
        //will lower nibble overflow?
        if (val & 0x0F) == 0x0F {
            regs.set_flag(HALFCARRY_FLAG);
        } else {
            regs.clear_flag(HALFCARRY_FLAG);
        }

        val -= 1;

        if val == 0 {
            regs.set_flag(ZERO_FLAG);
        } else {
            regs.clear_flag(ZERO_FLAG);
        }

        regs.set_flag(NEGATIVE_FLAG);

        return val;
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
    
    //0x31
    fn ld_sp_nn(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let short = MMU::to_short_lsb(operand);
        state.registers.SP = short;
    }

    //0xAF
    fn xor_a(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let a = state.registers.AF.T;

        state.registers.AF.T = a ^ a; 
    }

    //0x21
    fn ld_hl_nn(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let short = MMU::to_short_lsb(operand);
        state.registers.HL.w16b(short);
    }

    //0x32
    fn ldd_HL_A(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let hlad = state.registers.HL.r16b();
        state.mmu.write_byte(hlad, state.registers.AF.T);
        state.registers.HL.decrement();
    }

    //0xCB 0x7C
    fn bit_7h(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        if state.registers.HL.T.test(7) {
            state.registers.set_flag(ZERO_FLAG);
        }
        state.registers.set_flag(HALFCARRY_FLAG);
        state.registers.clear_flag(NEGATIVE_FLAG);
    }

    //0x20
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

    //0x0E
    fn ld_C_n(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        state.registers.BC.L = operand[0];
    }

    //0x3E
    fn ld_A_n(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        state.registers.AF.T = operand[0];
    }

    //0xE2
    fn ldh_dC(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let pointer: u16 = 0xFF00 + state.registers.BC.L as u16;
        state.mmu.write_byte(pointer, state.registers.AF.T);
    }

    //0x0C
    fn inc_C(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        state.registers.BC.L = CPU::inc(state.registers.BC.L, &mut state.registers)

    }

    //0x05
    fn dec_B(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        state.registers.BC.T = CPU::dec(state.registers.BC.T, &mut state.registers)

    }

    //0x77
    fn ld_dHL_A(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let pointer = state.registers.HL.r16b();
        state.mmu.write_byte(pointer, state.registers.AF.T);

    }

    //0xE0
    fn ldh_dn_A(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let pointer = 0xFF00 + operand[0] as u16;
        state.mmu.write_byte(pointer, state.registers.AF.T);
    }

    //0x11
    fn ld_DE_nn(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let short = MMU::to_short_lsb(operand);
        state.registers.DE.w16b(short);
    }

    //0x1A
    fn ld_A_dDE(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let info_pointed_by_DE = state.mmu.read_byte(state.registers.DE.r16b());
        
        state.registers.AF.T = info_pointed_by_DE;
    }

    //0xCD
    fn call_nn(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        state.push_to_stack(state.registers.PC); //save PC onto stack
        state.registers.PC = MMU::to_short_lsb(operand);
    } 

    //0x4F
    fn ld_C_A(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        state.registers.AF.T = state.registers.BC.L;
    } 

    //0x06
    fn ld_B_n(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        state.registers.BC.T = operand[0];
    } 

    //0xC5
    fn push_BC(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        state.push_to_stack(state.registers.BC.r16b());

    } 

    //0xC1
    fn pop_BC(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let stack = state.pop_from_stack();
        state.registers.BC.w16b(stack);

    } 

    //0xCB11
    fn rlc(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let carry = state.registers.is_set_flag(CARRY_FLAG) as u8;

        if (state.registers.BC.L & 0x80) == 0x80 {
            state.registers.set_flag(CARRY_FLAG);
        } else {
            state.registers.clear_flag(CARRY_FLAG);
        }

        state.registers.BC.L = state.registers.BC.L << 1; //>
        state.registers.BC.L += carry;

        if state.registers.BC.L == 0 {
            state.registers.set_flag(ZERO_FLAG);
        } else {
            state.registers.clear_flag(ZERO_FLAG);
        }

        state.registers.clear_flag(NEGATIVE_FLAG);
        state.registers.clear_flag(HALFCARRY_FLAG);
    }   

    //0x17
    fn rla(op: Instruction, opcode: u8, operand: [u8; 2], state: &mut CPU){
        let carry = state.registers.is_set_flag(CARRY_FLAG) as u8;

        if (state.registers.AF.T & 0x80) == 0x80 {
            state.registers.set_flag(CARRY_FLAG);
        } else {
            state.registers.clear_flag(CARRY_FLAG);
        }

        state.registers.AF.T = state.registers.AF.T << 1; //>
        state.registers.AF.T += carry;

        if state.registers.AF.T == 0 {
            state.registers.set_flag(ZERO_FLAG);
        } else {
            state.registers.clear_flag(ZERO_FLAG);
        }

        state.registers.clear_flag(NEGATIVE_FLAG);
        state.registers.clear_flag(HALFCARRY_FLAG);
    }   



}

