#![allow(non_snake_case)]

use crate::emulator::cpu::{*};
use crate::emulator::bus::{*};
use std::fmt;
use yansi::Paint;

const ZERO_FLAG: u8 = 0;
const NEGATIVE_FLAG: u8 = 1;
const HALFCARRY_FLAG: u8 = 2;
const CARRY_FLAG: u8 = 3;

#[derive(Clone, Copy)]
pub struct Instruction {
    pub disassembly: &'static str,
    pub function: fn([u8;2], &mut Registers, &mut Bus, Self), 
    pub args: u8,
    pub cycles: u16
}

const DEBUG_FLAG: bool = true;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.disassembly)
    }
}

impl Instruction {

    pub fn execute(self, params: [u8; 2], cpu: &mut Registers, mem: &mut Bus, inst: Instruction) -> u16 {
        let f = self.function;

        f(params, cpu, mem, inst);
    
        self.cycles
    }


    fn debug(x: &Instruction, operands: [u8; 2]) {
        if x.args != 0 {
            println!("{}\r\t\t\t{:#04x}", Paint::green(x), Bus::to_short(operands));
        } else {
            println!("{}\r\t\t\t-", Paint::green(x));
        }
    }

    //Generic functions appear first 

    //Stack management functions
    fn push_to_stack(registers: &mut Registers, mem: &mut Bus, short: u16){
        let sp: u16 = registers.SP(Action::Read).value();

        mem.write_byte(sp, short as u8);
        mem.write_byte(sp-1, (short >> 8) as u8); 

        registers.SP(Action::Write(sp-2));
    }

    fn pop_from_stack(registers: &mut Registers, mem: &mut Bus) -> u16 {
        let sp: u16 = registers.SP(Action::Read).value();
        registers.SP(Action::Write(sp+2));


        let b1: u8 = mem.read_byte(sp+2).value();
        let b2: u8 = mem.read_byte(sp+1).value();
        
        b1 as u16 | (b2 as u16) << 8 //>
    }

    //Generic increment function for 8 bit registers. 
    fn INC(registers: &mut Registers, mut val: u8) -> u8{
        //will lower nibble overflow?
        if (val & 0x0F) == 0x0F {
            registers.set_flag(HALFCARRY_FLAG);
        } else {
            registers.clear_flag(HALFCARRY_FLAG);
        }

        val = val.wrapping_add(1);

        if val == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.clear_flag(NEGATIVE_FLAG);

        return val;
    }

    //generic decrement function for 8 bit registers. 
    fn DEC(registers: &mut Registers, mut val: u8) -> u8{
        //will lower nibble overflow?
        if (val & 0x0F) == 0x0F {
            registers.set_flag(HALFCARRY_FLAG);
        } else {
            registers.clear_flag(HALFCARRY_FLAG);
        }

        val = val.wrapping_sub(1);

        if val == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.set_flag(NEGATIVE_FLAG);

        return val;
    }

    fn ADD_u8(registers: &mut Registers, X: u8, use_carry: bool) {

		let A: u8 = registers.A( Action::Read ).value();
        let C = (use_carry && registers.test_flag(CARRY_FLAG)) as u8;
        let result: u8 = A.wrapping_add(X).wrapping_add(C);

		let carry = (A as i16 + X as i16 + C as i16) > 0xFF;
		let half_carry = (A & 0xF) + (X & 0xF) + C > 0xF;
        
        if carry { 
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        if half_carry {
            registers.set_flag(HALFCARRY_FLAG);
        } else {
            registers.clear_flag(HALFCARRY_FLAG);
        }

        if result == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.clear_flag(NEGATIVE_FLAG);

		registers.A( Action::Write(result as u16)) ;
    }

    fn SUB_u8(registers: &mut Registers, X: u8, use_carry: bool) {

		let A: u8 = registers.A( Action::Read ).value();
		let C = (use_carry && registers.test_flag(CARRY_FLAG)) as u8;
		let result: u8 = A.wrapping_sub(X).wrapping_sub(C);
		let carry = (A as i16 - X as i16 - C as i16) < 0;
		let half_carry = (A & 0xF) as i16 - (X & 0xF) as i16 - (C as i16) < 0;
        
        if carry { 
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        if half_carry {
            registers.set_flag(HALFCARRY_FLAG);
        } else {
            registers.clear_flag(HALFCARRY_FLAG);
        }

        if result == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.set_flag(NEGATIVE_FLAG);

		registers.A( Action::Write(result as u16)) ;
    }

    fn AND_u8(registers: &mut Registers, X: u8) {
        let A: u8 = registers.A( Action::Read ).value();
        let result = A & X;

        registers.clear_flag(CARRY_FLAG);
        registers.set_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);

        if result == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.A( Action::Write(result as u16) );
    }

    fn OR_u8(registers: &mut Registers, X: u8) {
        let A: u8 = registers.A( Action::Read ).value();
        let result = A | X;

        registers.clear_flag(CARRY_FLAG);
        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);

        if result == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.A( Action::Write(result as u16) );
    }

    fn XOR_u8(registers: &mut Registers, X: u8) {
        let A: u8 = registers.A( Action::Read ).value();
        let result = A ^ X;

        registers.clear_flag(CARRY_FLAG);
        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);

        if result == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.A( Action::Write(result as u16) );
    }

    fn CP_u8(registers: &mut Registers, X: u8) {
		let A: u8 = registers.A( Action::Read ).value();
		Instruction::SUB_u8(registers, X, false);
        registers.A( Action::Write(A as u16) );
    }

    fn ADD_u16(registers: &mut Registers, X: u16, Y: u16) -> u16 {    
        let value: u16 = X+Y;

        let carry = (0x8000 & X) == 0x8000;
        let halfcarry = (0x0800 & X) == 0x0800;

        let new_carry = (0x8000 & value) == 0x8000;
        let new_halfcarry = (0x0800 & value) == 0x0800;

        if carry && !new_carry {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        if halfcarry && !new_halfcarry {
            registers.set_flag(HALFCARRY_FLAG);
        } else {
            registers.clear_flag(HALFCARRY_FLAG);
        }

        registers.clear_flag(ZERO_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);

        return value;
    }

    fn RR(registers: &mut Registers, mut value: u8) -> u8{
        let carry = registers.test_flag(CARRY_FLAG);

        if (value & 0x80) == 0x80 {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }
 
        value = value >> 1; //>
        value += (carry as u8) << 7; //>

        if value == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.clear_flag(NEGATIVE_FLAG);
        registers.clear_flag(HALFCARRY_FLAG);

        return value;
    }
    
    fn RRC(registers: &mut Registers, mut value: u8) -> u8{
        let carry = (value & 1) == 1;

        value = value >> 1; 

        if carry {
            registers.set_flag(CARRY_FLAG);
            value = value | 0x80;
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        if value == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.clear_flag(NEGATIVE_FLAG);
        registers.clear_flag(HALFCARRY_FLAG);

        return value;
    }

    fn RL(registers: &mut Registers, mut value: u8) -> u8{
        let carry = registers.test_flag(CARRY_FLAG);

        if (value & 0x80) == 0x80 {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        value = value << 1; //>
        value += carry as u8;

        if value == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.clear_flag(NEGATIVE_FLAG);
        registers.clear_flag(HALFCARRY_FLAG);

        return value;
    }

    fn RLC(registers: &mut Registers, mut value: u8) -> u8 {
        let carry = (value & 0x80) == 0x80;

        if carry {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        value = value << 1; //>
        value += carry as u8;

        if value == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.clear_flag(NEGATIVE_FLAG);
        registers.clear_flag(HALFCARRY_FLAG);

        return value;
    }



    //0x00
    pub fn NOP(operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, inst: Instruction ){
        if inst.disassembly != "NOP" {
            if inst.args != 0 {
                println!("{}\r\t\t\t{:#04x}", Paint::red(inst), Bus::to_short(operands));
            } else {
                println!("{}\r\t\t\t-", Paint::red(inst));
            }
        } else {
            println!("{}\r\t\t\t-", Paint::green(inst));
        }

        ;
    }

    //0x01
    pub fn LD_BC_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let ops = Bus::to_short(operands);
        registers.BC(Action::Write(ops));
        
        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}
    }

    //0x02
    pub fn LD_dBC_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction)  {
        let BC = registers.BC(Action::Read).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(BC, registers.A(Action::Read).value() );
    }

    //0x03
    pub fn INC_BC(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) { 
        registers.BC( Action::Increment(1) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

    }

    //0x04
    pub fn INC_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut val: u8 = registers.B(Action::Read).value();

        val = Instruction::INC(registers, val);

        registers.B(Action::Write(val as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

    }

    //0x05
    pub fn DEC_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut val: u8 = registers.B(Action::Read).value();

        val = Instruction::DEC(registers, val);

        registers.B(Action::Write(val as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

    }

    //0x06
    pub fn LD_B_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        registers.B( Action::Write(operands[0] as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

    }

    //0x07
    pub fn RLC_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let mut A: u8 = registers.A(Action::Read).value();

        A = Instruction::RLC(registers, A);

        registers.A(Action::Write(A as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

    }

    //0x08
    pub fn LD_dnn_SP(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        let pointer = Bus::to_short(operands);
        let SP: u16 = registers.SP( Action::Read ).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        mem.write_short(pointer, SP);
    }

    //0x09
    pub fn ADD_HL_BC(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let HL: u16 = registers.HL(Action::Read).value();
        let BC: u16 = registers.BC(Action::Read).value();

        let added = Instruction::ADD_u16(registers, HL, BC);

        registers.HL(Action::Write(added));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

    }

    //0x0A
    pub fn LD_A_dBC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let dBC: u8 = mem.read_byte( registers.BC(Action::Read).value() ).value();

        registers.A(Action::Write(dBC as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

    }

    //0x0B
    pub fn DEC_BC(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction)  {

        registers.BC(Action::Decrement(1));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

    }

    //0x0C
    pub fn INC_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut val: u8 = registers.C(Action::Read).value();

        val = Instruction::INC(registers, val);

        registers.C(Action::Write(val as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

    }

    //0x0D
    pub fn DEC_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut val: u8 = registers.C(Action::Read).value();

        val = Instruction::DEC(registers, val);

        registers.C(Action::Write(val as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x0E
    pub fn LD_C_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        registers.C(Action::Write(operands[0] as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0x0F
    pub fn RRC_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A = Instruction::RRC(registers, A);

        registers.A( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x10
    pub fn STOP(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x11
    pub fn LD_DE_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let value = Bus::to_short(operands);

        registers.DE( Action::Write(value) );

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0x12
    pub fn LD_dDE_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        let DE: u16 = registers.DE( Action::Read ).value();
        
        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};
        
        mem.write_byte(DE, registers.A( Action::Read ).value());
    }

    //0x13
    pub fn INC_DE(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        registers.DE(Action::Increment(1));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }
    
    //0x14
    pub fn INC_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let mut D: u8 = registers.D(Action::Read).value();
        
        D = Instruction::INC(registers, D);

        registers.D( Action::Write(D as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x15
    pub fn DEC_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut val: u8 = registers.D(Action::Read).value();

        val = Instruction::DEC(registers, val);

        registers.D(Action::Write(val as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }   

    //0x16
    pub fn LD_D_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        registers.D(Action::Write(operands[0] as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0x17
    pub fn RL_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut A: u8 = registers.A( Action::Read ).value();

        A = Instruction::RL(registers, A);

        registers.A( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x18
    pub fn JR_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction)  {
        let jump = operands[0] as i8;
        if jump >= 0 {
            registers.PC(Action::Increment(jump as u16));
        } else {
            registers.PC(Action::Decrement(jump.abs() as u16)); 
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

         
    }

    //0x19
    pub fn ADD_HL_DE(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let HL: u16 = registers.HL(Action::Read).value();
        let DE: u16 = registers.DE(Action::Read).value();

        let added = Instruction::ADD_u16(registers, HL, DE);

        registers.HL(Action::Write(added));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x1A
    pub fn LD_A_dDE(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let dDE: u8 = mem.read_byte( registers.DE(Action::Read).value() ).value();

        registers.A(Action::Write(dDE as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x1B
    pub fn DEC_DE(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        registers.DE(Action::Decrement(1));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x1C
    pub fn INC_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let mut E: u8 = registers.E(Action::Read).value();
        
        E = Instruction::INC(registers, E);

        registers.D( Action::Write(E as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x1D
    pub fn DEC_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut E: u8 = registers.E(Action::Read).value();

        E = Instruction::DEC(registers, E);

        registers.E(Action::Write(E as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    } 

    //0x1E
    pub fn LD_E_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        registers.E(Action::Write(operands[0] as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0x1F
    pub fn RR_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A = Instruction::RR(registers, A);

        registers.A( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x20
    pub fn JR_NZ_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction)  {
        if !registers.test_flag(ZERO_FLAG) {
            let jump = operands[0] as i8;
            if jump >= 0 {
                registers.PC(Action::Increment(jump as u16));
            } else {
                registers.PC(Action::Decrement(jump.abs() as u16)); 
            }
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0x21
    pub fn LD_HL_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let short = Bus::to_short(operands);
        registers.HL(Action::Write(short));

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0x22
    pub fn LDI_HL_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        let hlad = registers.HL(Action::Read).value();
        registers.HL(Action::Increment(1));
        
        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(hlad, registers.A(Action::Read).value() );


    }

    //0x23
    pub fn INC_HL(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        registers.HL(Action::Increment(1));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }
    
    //0x24
    pub fn INC_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let mut H: u8 = registers.H(Action::Read).value();
        
        H = Instruction::INC(registers, H);

        registers.H( Action::Write(H as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x25
    pub fn DEC_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut H: u8 = registers.H(Action::Read).value();

        H = Instruction::DEC(registers, H);

        registers.H(Action::Write(H as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    } 

    //0x26 
    pub fn LD_H_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        registers.H(Action::Write(operands[0] as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0x27
    pub fn DAA(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let mut A: u8 = registers.A( Action::Read ).value();

        let mut Adjust: u8 = if registers.test_flag(CARRY_FLAG) { 0x60 } else { 0x00 };

        if registers.test_flag(HALFCARRY_FLAG) { Adjust |= 0x06; };

        if !registers.test_flag(NEGATIVE_FLAG) {
            if A & 0x0F > 0x09 { Adjust |= 0x06 };
            if A > 0x99 { Adjust |= 0x60 };
            A += Adjust;
        } else {
            A -= Adjust;
        }

        registers.clear_flag(HALFCARRY_FLAG);
        
        if A == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        if Adjust >= 0x60 {
            registers.set_flag(CARRY_FLAG)
        } else {
            registers.clear_flag(CARRY_FLAG);
        }
        
        registers.A( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}
        
        
    }

    //0x28
    pub fn JR_Z_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction)  {
        if registers.test_flag(ZERO_FLAG){
            let jump = operands[0] as i8;
            if jump >= 0 {
                registers.PC(Action::Increment(jump as u16));
            } else {
                registers.PC(Action::Decrement(jump.abs() as u16)); 
            }
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0x29
    pub fn ADD_HL_HL(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let HL: u16 = registers.HL(Action::Read).value();

        let added = Instruction::ADD_u16(registers, HL, HL);

        registers.HL(Action::Write(added));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x2A 
    pub fn LDI_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let dHL: u8 = mem.read_byte( registers.HL(Action::Read).value() ).value();

        registers.A( Action::Write(dHL as u16) );

        registers.HL( Action::Increment(1) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x2B
    pub fn DEC_HL(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction)  {

        registers.HL(Action::Decrement(1));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x2C
    pub fn INC_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let mut L: u8 = registers.L(Action::Read).value();
        
        L = Instruction::INC(registers, L);

        registers.L( Action::Write(L as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x2D
    pub fn DEC_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut L: u8 = registers.L(Action::Read).value();

        L = Instruction::DEC(registers, L);

        registers.L(Action::Write(L as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    } 

    //0x2E
    pub fn LD_L_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        registers.L(Action::Write(operands[0] as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }
  
    //0x2F
    pub fn NOT_A(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let A: u8 = registers.A(Action::Read).value();

        registers.A( Action::Write(!A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0x30 
    pub fn JR_NC_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction)  {
        if !registers.test_flag(CARRY_FLAG) {
            let jump = operands[0] as i8;
            if jump >= 0 {
                registers.PC(Action::Increment(jump as u16));
            } else {
                registers.PC(Action::Decrement(jump.abs() as u16)); 
            }
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0x31
    pub fn LD_SP_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let short = Bus::to_short(operands);
        registers.SP(Action::Write(short));

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0x32
    pub fn LDD_HL_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        let hlad = registers.HL(Action::Read).value();
        registers.HL(Action::Decrement(1));
        
        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(hlad, registers.A(Action::Read).value() );


    }

    //0x33
    pub fn INC_SP(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        registers.SP(Action::Increment(1));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x34
    pub fn INC_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        let HL: u16 = registers.HL(Action::Read).value();
        
        let mut byte = mem.read_byte(HL).value();

        byte = Instruction::INC(registers, byte);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(HL, byte);
    }

    //0x35
    pub fn DEC_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        let HL: u16 = registers.HL(Action::Read).value();
        
        let mut byte = mem.read_byte(HL).value();

        byte = Instruction::DEC(registers, byte);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(HL, byte);
    }

    //0x36 
    pub fn LD_dHL_n(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        let HL: u16 = registers.HL( Action::Read).value();
        
        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        mem.write_byte(HL, operands[0]);
    }

    //0x37
    pub fn SCF(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        registers.set_flag(CARRY_FLAG);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x38
    pub fn JR_C_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction)  {
        if registers.test_flag(CARRY_FLAG) {
            let jump = operands[0] as i8;
            if jump >= 0 {
                registers.PC(Action::Increment(jump as u16));
            } else {
                registers.PC(Action::Decrement(jump.abs() as u16)); 
            }
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0x39
    pub fn ADD_HL_SP(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let HL: u16 = registers.HL(Action::Read).value();
        let SP: u16 = registers.SP(Action::Read).value();

        let added = Instruction::ADD_u16(registers, HL, SP);

        registers.HL(Action::Write(added));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x3A 
    pub fn LDD_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let dHL: u8 = mem.read_byte( registers.HL(Action::Read).value() ).value();

        registers.A( Action::Write(dHL as u16) );

        registers.HL( Action::Decrement(1) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x3B
    pub fn DEC_SP(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction)  {

        registers.SP(Action::Decrement(1));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x3C
    pub fn INC_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let mut A: u8 = registers.A(Action::Read).value();
        
        A = Instruction::INC(registers, A);

        registers.A( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x3D
    pub fn DEC_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut A: u8 = registers.A(Action::Read).value();

        A = Instruction::DEC(registers, A);

        registers.A(Action::Write(A as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    } 

    //0x3E
    pub fn LD_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        registers.A(Action::Write(operands[0] as u16));

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0x3F
    pub fn CCF(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        registers.clear_flag(CARRY_FLAG);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x40
    pub fn LD_B_B(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x41
    pub fn LD_B_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.B( Action::Write(C as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x42
    pub fn LD_B_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.B( Action::Write(D as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x43
    pub fn LD_B_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.B( Action::Write(E as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x44
    pub fn LD_B_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.B( Action::Write(H as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x45
    pub fn LD_B_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.B( Action::Write(L as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x46
    pub fn LD_B_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.B( Action::Write(dHL as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x47
    pub fn LD_B_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.B( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x48
    pub fn LD_C_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.C( Action::Write(B as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x49
    pub fn LD_C_C(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x4A
    pub fn LD_C_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.C( Action::Write(D as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x4B
    pub fn LD_C_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.C( Action::Write(E as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x4C
    pub fn LD_C_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.C( Action::Write(H as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x4D
    pub fn LD_C_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.C( Action::Write(L as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x4E
    pub fn LD_C_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.C( Action::Write(dHL as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x4F
    pub fn LD_C_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.C( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x50
    pub fn LD_D_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.D( Action::Write(B as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x51
    pub fn LD_D_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.D( Action::Write(C as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x52
    pub fn LD_D_D(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x53
    pub fn LD_D_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.D( Action::Write(E as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x54
    pub fn LD_D_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.D( Action::Write(H as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x55
    pub fn LD_D_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.D( Action::Write(L as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x56
    pub fn LD_D_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.D( Action::Write(dHL as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x57
    pub fn LD_D_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.D( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x58
    pub fn LD_E_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.E( Action::Write(B as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x59
    pub fn LD_E_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.E( Action::Write(C as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x5A
    pub fn LD_E_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.E( Action::Write(D as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x5B
    pub fn LD_E_E(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x5C
    pub fn LD_E_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.E( Action::Write(H as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x5D
    pub fn LD_E_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.E( Action::Write(L as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x5E
    pub fn LD_E_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.E( Action::Write(dHL as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x5F
    pub fn LD_E_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.E( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x60
    pub fn LD_H_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.H( Action::Write(B as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x61
    pub fn LD_H_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.H( Action::Write(C as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x62
    pub fn LD_H_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.H( Action::Write(D as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x63
    pub fn LD_H_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.H( Action::Write(E as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x64
    pub fn LD_H_H(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x65
    pub fn LD_H_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.H( Action::Write(L as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x66
    pub fn LD_H_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.H( Action::Write(dHL as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x67
    pub fn LD_H_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.H( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x68
    pub fn LD_L_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.L( Action::Write(B as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x69
    pub fn LD_L_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.L( Action::Write(C as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x6A
    pub fn LD_L_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.L( Action::Write(D as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x6B
    pub fn LD_L_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.L( Action::Write(E as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x6C
    pub fn LD_L_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.L( Action::Write(H as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x6D
    pub fn LD_L_L(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x6E
    pub fn LD_L_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.L( Action::Write(dHL as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x6F
    pub fn LD_L_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.L( Action::Write(A as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x70
    pub fn LD_dHL_B(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let B: u8 = registers.B( Action::Read ).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(HL, B);
    }

    //0x71
    pub fn LD_dHL_C(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let C: u8 = registers.C( Action::Read ).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(HL, C);
    }

    //0x72
    pub fn LD_dHL_D(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let D: u8 = registers.D( Action::Read ).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(HL, D);
    }

    //0x73
    pub fn LD_dHL_E(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let E: u8 = registers.E( Action::Read ).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(HL, E);
    }

    //0x74
    pub fn LD_dHL_H(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let H: u8 = registers.H( Action::Read ).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(HL, H);
    }

    //0x75
    pub fn LD_dHL_L(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let L: u8 = registers.L( Action::Read ).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(HL, L);
    }

    //0x76 
    pub fn HALT(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x77
    pub fn LD_dHL_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let A: u8 = registers.A( Action::Read ).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        mem.write_byte(HL, A);
    }

    //0x78
    pub fn LD_A_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.A( Action::Write(B as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x79
    pub fn LD_A_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.A( Action::Write(C as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x7A
    pub fn LD_A_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.A( Action::Write(D as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x7B
    pub fn LD_A_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.A( Action::Write(E as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x7C
    pub fn LD_A_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.A( Action::Write(H as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x7D
    pub fn LD_A_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.A( Action::Write(L as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x7E
    pub fn LD_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.A( Action::Write(dHL as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x7F
    pub fn LD_A_A(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x80
    pub fn ADD_A_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::ADD_u8(registers, B, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x81
    pub fn ADD_A_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::ADD_u8(registers, C, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x82
    pub fn ADD_A_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::ADD_u8(registers, D, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x83
    pub fn ADD_A_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::ADD_u8(registers, E, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x84
    pub fn ADD_A_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::ADD_u8(registers, H, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x85
    pub fn ADD_A_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::ADD_u8(registers, L, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x86
    pub fn ADD_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){

        let dHL: u8 = mem.read_byte(registers.L( Action::Read ).value()).value();

        Instruction::ADD_u8(registers, dHL, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x87
    pub fn ADD_A_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let A: u8 = registers.A( Action::Read ).value();

        Instruction::ADD_u8(registers, A, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x88
    pub fn ADC_A_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::ADD_u8(registers, B, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x89
    pub fn ADC_A_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::ADD_u8(registers, C, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x8A
    pub fn ADC_A_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::ADD_u8(registers, D, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x8B
    pub fn ADC_A_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::ADD_u8(registers, E, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x8C
    pub fn ADC_A_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::ADD_u8(registers, H, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x8D
    pub fn ADC_A_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::ADD_u8(registers, L, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x8E
    pub fn ADC_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){

        let dHL: u8 = mem.read_byte(registers.L( Action::Read ).value()).value();

        Instruction::ADD_u8(registers, dHL, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x8F
    pub fn ADC_A_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let A: u8 = registers.A( Action::Read ).value();

        Instruction::ADD_u8(registers, A, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x90
    pub fn SUB_A_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::SUB_u8(registers, B, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x91
    pub fn SUB_A_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::SUB_u8(registers, C, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x92
    pub fn SUB_A_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::SUB_u8(registers, D, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x93
    pub fn SUB_A_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::SUB_u8(registers, E, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x94
    pub fn SUB_A_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::SUB_u8(registers, H, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x95
    pub fn SUB_A_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::SUB_u8(registers, L, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x96
    pub fn SUB_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){

        let dHL: u8 = mem.read_byte(registers.L( Action::Read ).value()).value();

        Instruction::SUB_u8(registers, dHL, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x97
    pub fn SUB_A_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let A: u8 = registers.A( Action::Read ).value();

        Instruction::SUB_u8(registers, A, false);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x98
    pub fn SBC_A_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::SUB_u8(registers, B, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x99
    pub fn SBC_A_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::SUB_u8(registers, C, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x9A
    pub fn SBC_A_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::SUB_u8(registers, D, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x9B
    pub fn SBC_A_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::SUB_u8(registers, E, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x9C
    pub fn SBC_A_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::SUB_u8(registers, H, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x9D
    pub fn SBC_A_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::SUB_u8(registers, L, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x9E
    pub fn SBC_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){

        let dHL: u8 = mem.read_byte(registers.L( Action::Read ).value()).value();

        Instruction::SUB_u8(registers, dHL, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0x9F
    pub fn SBC_A_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let A: u8 = registers.A( Action::Read ).value();

        Instruction::SUB_u8(registers, A, true);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xA0
    pub fn AND_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::AND_u8(registers, B);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xA1
    pub fn AND_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::AND_u8(registers, C);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xA2
    pub fn AND_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::AND_u8(registers, D);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xA3
    pub fn AND_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::AND_u8(registers, E);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xA4
    pub fn AND_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::AND_u8(registers, H);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xA5
    pub fn AND_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::AND_u8(registers, L);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xA6
    pub fn AND_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){

        let dHL: u8 = mem.read_byte( registers.L( Action::Read ).value()).value();

        Instruction::AND_u8(registers, dHL);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xA7
    pub fn AND_A(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xA8
    pub fn XOR_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::XOR_u8(registers, B);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xA9
    pub fn XOR_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::XOR_u8(registers, C);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xAA
    pub fn XOR_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::XOR_u8(registers, D);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xAB
    pub fn XOR_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::XOR_u8(registers, E);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xAC
    pub fn XOR_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::XOR_u8(registers, H);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xAD
    pub fn XOR_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::XOR_u8(registers, L);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xAE
    pub fn XOR_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){

        let dHL: u8 = mem.read_byte( registers.L( Action::Read ).value()).value();

        Instruction::XOR_u8(registers, dHL);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xAF
    pub fn XOR_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        registers.A( Action::Write(0) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xB0
    pub fn OR_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::OR_u8(registers, B);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xB1
    pub fn OR_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::OR_u8(registers, C);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xB2
    pub fn OR_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::OR_u8(registers, D);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xB3
    pub fn OR_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::OR_u8(registers, E);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xB4
    pub fn OR_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::OR_u8(registers, H);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xB5
    pub fn OR_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::OR_u8(registers, L);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xB6
    pub fn OR_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){

        let dHL: u8 = mem.read_byte( registers.L( Action::Read ).value()).value();

        Instruction::OR_u8(registers, dHL);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xB7
    pub fn OR_A(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xB8
    pub fn CP_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::CP_u8(registers, B);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xB9
    pub fn CP_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::CP_u8(registers, C);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xBA
    pub fn CP_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::CP_u8(registers, D);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xBB
    pub fn CP_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::CP_u8(registers, E);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xBC
    pub fn CP_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::CP_u8(registers, H);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xBD
    pub fn CP_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::CP_u8(registers, L);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xBE
    pub fn CP_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let dHL: u8 = mem.read_byte( registers.L( Action::Read ).value()).value();

        Instruction::CP_u8(registers, dHL);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xBF
    pub fn CP_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let A: u8 = registers.A( Action::Read ).value();

        Instruction::CP_u8(registers, A);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xC0
    pub fn RET_NZ(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        if !registers.test_flag(ZERO_FLAG) {
            let pointer = Instruction::pop_from_stack(registers, mem);
            registers.PC( Action::Write(pointer) );
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xC1
    pub fn POP_BC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let popped = Instruction::pop_from_stack(registers, mem);
        registers.BC( Action::Write(popped) );


        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xC2
    pub fn JP_NZ_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        if !registers.test_flag(ZERO_FLAG) {
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xC3
    pub fn JP_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        let pointer = Bus::to_short(operands);
        registers.PC( Action::Write(pointer) );

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xC4
    pub fn CALL_NZ_nn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        if !registers.test_flag(ZERO_FLAG) {
            let PC: u16 = registers.PC( Action::Read ).value(); 
            Instruction::push_to_stack(registers, mem, PC);
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
        }


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xC5
    pub fn PUSH_BC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let BC: u16 = registers.BC( Action::Read ).value();
        Instruction::push_to_stack(registers, mem, BC);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xC6
    pub fn ADD_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        Instruction::ADD_u8(registers, operands[0], false);

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0xC7
    pub fn RST_0(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(0) );


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xC8
    pub fn RET_Z(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        if registers.test_flag(ZERO_FLAG) {
            let pointer = Instruction::pop_from_stack(registers, mem);
            registers.PC( Action::Write(pointer) );
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xC9
    pub fn RET(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        let pointer = Instruction::pop_from_stack(registers, mem);
        registers.PC( Action::Write(pointer) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xCA
    pub fn JP_Z_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        if registers.test_flag(ZERO_FLAG) { 
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xCB prefix, not a function

    //0xCC
    pub fn CALL_Z_nn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        if registers.test_flag(ZERO_FLAG) {
            let PC: u16 = registers.PC( Action::Read ).value(); 
            Instruction::push_to_stack(registers, mem, PC);
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
        }


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xCD
    pub fn CALL_nn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        let pointer = Bus::to_short(operands);
        registers.PC( Action::Write(pointer) );


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xCE
    pub fn ADC_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        Instruction::ADD_u8(registers, operands[0], true);

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0xCF
    pub fn RST_8(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(8) );


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xD0
    pub fn RET_NC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        if !registers.test_flag(CARRY_FLAG) {
            let pointer = Instruction::pop_from_stack(registers, mem);
            registers.PC( Action::Write(pointer) );
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xD1
    pub fn POP_DE(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let popped = Instruction::pop_from_stack(registers, mem);
        registers.DE( Action::Write(popped) );


        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xD2
    pub fn JP_NC_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        if !registers.test_flag(CARRY_FLAG) { 
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xD4
    pub fn CALL_NC_nn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        if !registers.test_flag(CARRY_FLAG) {
            let PC: u16 = registers.PC( Action::Read ).value(); 
            Instruction::push_to_stack(registers, mem, PC);
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
        }


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xD5
    pub fn PUSH_DE(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let DE: u16 = registers.DE( Action::Read ).value();
        Instruction::push_to_stack(registers, mem, DE);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xD6
    pub fn SUB_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        Instruction::SUB_u8(registers, operands[0], false);

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0xD7
    pub fn RST_10(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(10) );


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xD8
    pub fn RET_C(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        if registers.test_flag(CARRY_FLAG) {
            let pointer = Instruction::pop_from_stack(registers, mem);
            registers.PC( Action::Write(pointer) );
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xD9
    pub fn RETI(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let pointer = Instruction::pop_from_stack(registers, mem);
        registers.PC( Action::Write(pointer) );
        mem.enable_interrupts();

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xDA
    pub fn JP_C_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        if registers.test_flag(CARRY_FLAG) { 
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
        }

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xDC
    pub fn CALL_C_nn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        if registers.test_flag(CARRY_FLAG) {
            let PC: u16 = registers.PC( Action::Read ).value(); 
            Instruction::push_to_stack(registers, mem, PC);
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
        }


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }



    //0xDE
    pub fn SBC_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){

        Instruction::SUB_u8(registers, operands[0], true);

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0xDF
    pub fn RST_18(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(18) );


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xE0
    pub fn LDH_dn_A(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){

        let A: u8 = registers.A( Action::Read ).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};
        
        mem.write_byte(0xFF00 + operands[0] as u16, A);
    }

    //0xE1
    pub fn POP_HL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let popped = Instruction::pop_from_stack(registers, mem);
        registers.HL( Action::Write(popped) );


        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xE2
    pub fn LDH_dC(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        let C: u8 =  registers.C(Action::Read).value();
        
        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};
        
        mem.write_byte(0xFF00 + C as u16, registers.A(Action::Read).value() );
    }

    //0xE5
    pub fn PUSH_HL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let HL: u16 = registers.HL( Action::Read ).value();
        Instruction::push_to_stack(registers, mem, HL);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xE6
    pub fn AND_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){


        Instruction::AND_u8(registers, operands[0]);

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0xE7
    pub fn RST_20(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(20) );


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xE8
    pub fn ADD_SP_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        let SP: u16 = registers.SP(Action::Read).value();

        let jump = (operands[0] as i8) as i16;

        let result = ((SP as i16) + jump) as u16;

        if (result & 0xFF) < (SP & 0xFF) {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        if (result & 0xF) < (SP & 0xF) {
            registers.set_flag(HALFCARRY_FLAG);
        } else {
            registers.clear_flag(HALFCARRY_FLAG);
        }

        registers.clear_flag(ZERO_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);

        registers.SP( Action::Write(result) );

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0xE9
    pub fn JP_dHL(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let dHL: u8 = mem.read_byte(registers.L( Action::Read ).value()).value();

        registers.PC( Action::Write(dHL as u16) );


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xEA
    pub fn LD_dnn_A(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let pointer = Bus::to_short(operands);
        let A: u8 = registers.A( Action::Read ).value();

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        mem.write_byte(pointer, A);
    }

    //0xEE
    pub fn XOR_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        Instruction::XOR_u8(registers, operands[0]);

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xEF
    pub fn RST_28(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(28) );


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xF0
    pub fn LDH_A_dn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let result: u8 = mem.read_byte( 0xFF00 + operands[0] as u16).value();

        registers.A( Action::Write(result as u16) );
        
        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xF1
    pub fn POP_AF(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let popped = Instruction::pop_from_stack(registers, mem);
        registers.AF( Action::Write(popped) );


        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xF3
    pub fn DI(_operands: [u8; 2], _registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        mem.disable_interrupts();


        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xF5
    pub fn PUSH_AF(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        let AF: u16 = registers.AF( Action::Read ).value();
        Instruction::push_to_stack(registers, mem, AF);

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xF6
    pub fn OR_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {
        
        Instruction::OR_u8(registers, operands[0]);

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xF7
    pub fn RST_30(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(30) );


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xF8
    pub fn LDHL_SP_d(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction)  {
        let SP: u16 = registers.SP(Action::Read).value();

        let jump = (operands[0] as i8) as i16;

        let result = ((SP as i16) + jump) as u16;

        if (result & 0xFF) < (SP & 0xFF) {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        if (result & 0xF) < (SP & 0xF) {
            registers.set_flag(HALFCARRY_FLAG);
        } else {
            registers.clear_flag(HALFCARRY_FLAG);
        }

        registers.clear_flag(ZERO_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);

        registers.HL( Action::Write(result) );

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0xF9
    pub fn LD_SP_HL(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        
        let HL: u16 = registers.HL( Action::Read ).value();

        registers.SP( Action::Write(HL) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xFA
    pub fn LD_A_dnn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction){
        
        let nn: u16 = Bus::to_short(operands);

        let dnn: u8 = mem.read_byte(nn).value();

        registers.A( Action::Write(dnn as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)};

        
    }

    //0xFB
    pub fn EI(_operands: [u8; 2], _registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {
        
        mem.enable_interrupts();


        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)}

        
    }

    //0xFF
    pub fn CP_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction) {

        Instruction::CP_u8(registers, operands[0]);

        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //0xF7
    pub fn RST_38(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus, _inst: Instruction) {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(38) );


        if DEBUG_FLAG { Instruction::debug(&_inst, operands)}

        
    }

    //
    //  CB PREFIX INSTRUCTIONS
    //

    //0xCB 0x7C
    pub fn BIT_7H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        if registers.H(Action::TestBit(7)).value() {
            registers.clear_flag(ZERO_FLAG);
        } else {
            registers.set_flag(ZERO_FLAG);
        }
        registers.set_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);
 
        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

    //0xCB 0xCB
    pub fn RL_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus, _inst: Instruction){
        let mut C: u8 = registers.C( Action::Read ).value();

        C = Instruction::RL(registers, C);

        registers.C( Action::Write(C as u16) );

        if DEBUG_FLAG { Instruction::debug(&_inst, _operands)};

        
    }

}