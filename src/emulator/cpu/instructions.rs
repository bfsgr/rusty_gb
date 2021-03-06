#![allow(non_snake_case)]

use crate::emulator::cpu::{*};
use crate::emulator::bus::{Bus};
use std::fmt;
use std::mem::transmute;

pub const ZERO_FLAG: u8 = 0;
pub const NEGATIVE_FLAG: u8 = 1;
pub const HALFCARRY_FLAG: u8 = 2;
pub const CARRY_FLAG: u8 = 3;

#[derive(Clone, Copy)]
pub struct Instruction {
    pub disassembly: &'static str,
    pub function: fn([u8;2], &mut Registers, &mut Bus) -> u8, 
    pub args: u8,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.disassembly)
    }
}


//Two instruction are equal if they point to the same function
impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        //using transmute is unsafe
        unsafe {
            //transmute fn item to fn pointer, which implements PartialEq
            let a: fn([u8;2], *mut Registers, *mut Bus) = transmute(self.function as fn([u8;2], &mut Registers, &mut Bus) -> u8);
            let b: fn([u8;2], *mut Registers, *mut Bus) = transmute(other.function as fn([u8;2], &mut Registers, &mut Bus) -> u8);
            
            //compare and return
            return a == b;
        }
    }
}

impl Instruction {

    pub fn execute(self, params: [u8; 2], cpu: &mut Registers, mem: &mut Bus) -> u8 {
        let f = self.function;
        
        f(params, cpu, mem)
    }

    pub fn PANIC(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus) -> u8{
        panic!("REMOVED OPCODE WAS CALLED");
     }

    //0x00
    pub fn NOP(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus) -> u8{
        return 4;
    }

    //0x01
    pub fn LD_BC_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let ops = Bus::to_short(operands);
        registers.BC(Action::Write(ops));
        return 12;
    }

    //0x02
    pub fn LD_dBC_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8  {
        let BC = registers.BC(Action::Read).value();

        mem.write_byte(BC, registers.A(Action::Read).value() );
        return 8;
    }

    //0x03
    pub fn INC_BC(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 { 
        registers.BC( Action::Increment(1) );

        return 8;
    }

    //0x04
    pub fn INC_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let mut val: u8 = registers.B(Action::Read).value();

        val = Instruction::INC(registers, val);

        registers.B(Action::Write(val as u16));

        return 4;
    }

    //0x05
    pub fn DEC_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let mut val: u8 = registers.B(Action::Read).value();

        val = Instruction::DEC(registers, val);

        registers.B(Action::Write(val as u16));

        return 4;
    }

    //0x06
    pub fn LD_B_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        registers.B( Action::Write(operands[0] as u16) );
        
        return 8;
    }

    //0x07 
    pub fn RLC_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let mut A: u8 = registers.A(Action::Read).value();

        A = Instruction::RL(registers, A, false, false);

        registers.A(Action::Write(A as u16));

        return 4;
    }

    //0x08
    pub fn LD_dnn_SP(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let pointer = Bus::to_short(operands);
        let SP: u16 = registers.SP( Action::Read ).value();

        mem.write_short(pointer, SP);

        return 20;
    }

    //0x09
    pub fn ADD_HL_BC(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let BC: u16 = registers.BC(Action::Read).value();

        let added = Instruction::ADD_u16(registers, BC);

        registers.HL(Action::Write(added));

        return 8;
    }

    //0x0A
    pub fn LD_A_dBC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let dBC: u8 = mem.read_byte( registers.BC(Action::Read).value() ).value();

        registers.A(Action::Write(dBC as u16));

        return 8;
    }

    //0x0B
    pub fn DEC_BC(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {

        registers.BC(Action::Decrement(1));

        return 8;
    }

    //0x0C
    pub fn INC_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let mut val: u8 = registers.C(Action::Read).value();

        val = Instruction::INC(registers, val);

        registers.C(Action::Write(val as u16));

        return 4;
    }

    //0x0D
    pub fn DEC_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let mut val: u8 = registers.C(Action::Read).value();

        val = Instruction::DEC(registers, val);

        registers.C(Action::Write(val as u16));

        return 4;
    }

    //0x0E
    pub fn LD_C_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        registers.C(Action::Write(operands[0] as u16));
        return 8;
    }

    //0x0F
    pub fn RRC_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let mut A: u8 = registers.A( Action::Read ).value();

        A = Instruction::RR(registers, A, false, false);

        registers.A( Action::Write(A as u16) );  
        
        return 4;
    }

    //0x10
    pub fn STOP(_operands: [u8; 2], _registers: &mut Registers, mem: &mut Bus) -> u8 {
        mem.halt_cpu = true;
        mem.interrupts.halt_bug = false;
        return 4;
    }

    //0x11
    pub fn LD_DE_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let value = Bus::to_short(operands);

        registers.DE( Action::Write(value) );
        return 12;
    }

    //0x12
    pub fn LD_dDE_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let DE: u16 = registers.DE( Action::Read ).value();

        
        mem.write_byte(DE, registers.A( Action::Read ).value());

        return 8;
    }

    //0x13
    pub fn INC_DE(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        registers.DE(Action::Increment(1));

        return 8;
    }
    
    //0x14
    pub fn INC_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let mut D: u8 = registers.D(Action::Read).value();
        
        D = Instruction::INC(registers, D);

        registers.D( Action::Write(D as u16) );

        return 4;
    }

    //0x15
    pub fn DEC_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let mut val: u8 = registers.D(Action::Read).value();

        val = Instruction::DEC(registers, val);

        registers.D(Action::Write(val as u16));

        return 4;
    }   

    //0x16
    pub fn LD_D_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        registers.D(Action::Write(operands[0] as u16));

        return 8;
    }

    //0x17
    pub fn RL_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let mut A: u8 = registers.A( Action::Read ).value();

        A = Instruction::RL(registers, A, true, false);

        registers.A( Action::Write(A as u16) );

        return 4;
    }

    //0x18
    pub fn JR_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {
        let jump = operands[0] as i8;
        if jump >= 0 {
            registers.PC(Action::Increment(jump as u16));
        } else {
            registers.PC(Action::Decrement(jump.abs() as u16)); 
        }

        return 12;
    }

    //0x19
    pub fn ADD_HL_DE(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let DE: u16 = registers.DE(Action::Read).value();

        let added = Instruction::ADD_u16(registers, DE);

        registers.HL(Action::Write(added));
        
        return 8;
    }

    //0x1A
    pub fn LD_A_dDE(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let DE: u16 = registers.DE( Action::Read ).value();

        let read = mem.read_byte( DE );
        
        let value: u8 = read.value();

        registers.A(Action::Write(value as u16));

        return 8;
    }

    //0x1B
    pub fn DEC_DE(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        registers.DE(Action::Decrement(1));

        return 8;
    }

    //0x1C
    pub fn INC_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let mut E: u8 = registers.E(Action::Read).value();
        
        E = Instruction::INC(registers, E);

        registers.E( Action::Write(E as u16) );

        return 4;
    }

    //0x1D
    pub fn DEC_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let mut E: u8 = registers.E(Action::Read).value();

        E = Instruction::DEC(registers, E);

        registers.E(Action::Write(E as u16));

        return 4;
    } 

    //0x1E
    pub fn LD_E_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        registers.E(Action::Write(operands[0] as u16));
        
        return 8;
    }

    //0x1F
    pub fn RR_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let mut A: u8 = registers.A( Action::Read ).value();

        A = Instruction::RR(registers, A, true, false);

        registers.A( Action::Write(A as u16) );

        return 4;
    }

    //0x20
    pub fn JR_NZ_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {
        if !registers.test_flag(ZERO_FLAG) {
            let jump = operands[0] as i8;
            if jump >= 0 {
                registers.PC(Action::Increment(jump as u16));
            } else {
                registers.PC(Action::Decrement(jump.abs() as u16)); 
            }
            return 12;
        }
        return 8;
    }

    //0x21
    pub fn LD_HL_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let short = Bus::to_short(operands);
        registers.HL(Action::Write(short));
        
        return 12;
    }

    //0x22
    pub fn LDI_HL_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let hlad: u16 = registers.HL(Action::Read).value();
        registers.HL(Action::Increment(1));
        
        

        mem.write_byte(hlad, registers.A(Action::Read).value() );

        return 8;
    }

    //0x23
    pub fn INC_HL(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        registers.HL(Action::Increment(1));

        

        return 8;
    }
    
    //0x24
    pub fn INC_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let mut H: u8 = registers.H(Action::Read).value();
        
        H = Instruction::INC(registers, H);

        registers.H( Action::Write(H as u16) );

        

        return 4;
    }

    //0x25
    pub fn DEC_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let mut H: u8 = registers.H(Action::Read).value();

        H = Instruction::DEC(registers, H);

        registers.H(Action::Write(H as u16));

        

        return 4;
    } 

    //0x26 
    pub fn LD_H_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        registers.H(Action::Write(operands[0] as u16));

        return 8;
    }

    //0x27
    pub fn DAA(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let mut A: u8 = registers.A( Action::Read ).value();

        let mut adjust: u8 = if registers.test_flag(CARRY_FLAG) { 0x60 } else { 0x00 };

        if registers.test_flag(HALFCARRY_FLAG) { adjust |= 0x06; };

        if !registers.test_flag(NEGATIVE_FLAG) {
            if (A & 0x0F) > 0x09 { adjust |= 0x06 };
            if A > 0x99 { adjust |= 0x60 };
            A = A.wrapping_add(adjust);
        } else {
            A = A.wrapping_sub(adjust);
        }

        registers.clear_flag(HALFCARRY_FLAG);
        
        if A == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        if adjust >= 0x60 {
            registers.set_flag(CARRY_FLAG)
        } else {
            registers.clear_flag(CARRY_FLAG);
        }
        
        registers.A( Action::Write(A as u16) );

        return 4;
    }

    //0x28
    pub fn JR_Z_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {
        if registers.test_flag(ZERO_FLAG){
            let jump = operands[0] as i8;
            if jump >= 0 {
                registers.PC(Action::Increment(jump as u16));
            } else {
                registers.PC(Action::Decrement(jump.abs() as u16)); 
            }
            return 12;
        }
        return 8;
    }

    //0x29
    pub fn ADD_HL_HL(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let HL: u16 = registers.HL(Action::Read).value();

        let added = Instruction::ADD_u16(registers, HL);

        registers.HL(Action::Write(added));

        

        return 8;
    }

    //0x2A 
    pub fn LDI_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let HL: u16 = registers.HL( Action::Read ).value();

        let rHL = mem.read_byte( HL );

        let dHL: u8 = rHL.value();

        registers.A( Action::Write(dHL as u16) );

        registers.HL( Action::Increment(1) );

        

        return 8;
    }

    //0x2B
    pub fn DEC_HL(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {

        registers.HL(Action::Decrement(1));

        

        return 8;
    }

    //0x2C
    pub fn INC_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let mut L: u8 = registers.L(Action::Read).value();
        
        L = Instruction::INC(registers, L);

        registers.L( Action::Write(L as u16) );

        

        return 4;
    }

    //0x2D
    pub fn DEC_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let mut L: u8 = registers.L(Action::Read).value();

        L = Instruction::DEC(registers, L);

        registers.L(Action::Write(L as u16));

        

        return 4;
    } 

    //0x2E
    pub fn LD_L_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        registers.L(Action::Write(operands[0] as u16));
        return 8;
    }
  
    //0x2F
    pub fn NOT_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let A: u8 = registers.A(Action::Read).value();

        registers.A( Action::Write(!A as u16) );
        
        registers.set_flag(HALFCARRY_FLAG);
        registers.set_flag(NEGATIVE_FLAG);
        return 4;
    }

    //0x30 
    pub fn JR_NC_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {
        if !registers.test_flag(CARRY_FLAG) {
            let jump = operands[0] as i8;
            if jump >= 0 {
                registers.PC(Action::Increment(jump as u16));
            } else {
                registers.PC(Action::Decrement(jump.abs() as u16)); 
            }
            return 12;
        }
        return 8;
    }

    //0x31
    pub fn LD_SP_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let short = Bus::to_short(operands);
        registers.SP(Action::Write(short));
        return 12;
    }

    //0x32
    pub fn LDD_HL_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let hlad = registers.HL(Action::Read).value();
        registers.HL(Action::Decrement(1));
        
        

        mem.write_byte(hlad, registers.A(Action::Read).value() );

        return 8;
    }

    //0x33
    pub fn INC_SP(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        registers.SP(Action::Increment(1));

        

        return 8;
    }

    //0x34
    pub fn INC_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let HL: u16 = registers.HL(Action::Read).value();
        
        let mut byte = mem.read_byte(HL).value();

        byte = Instruction::INC(registers, byte);

        

        mem.write_byte(HL, byte);

        return 12;
    }

    //0x35
    pub fn DEC_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let HL: u16 = registers.HL(Action::Read).value();
        
        let mut byte = mem.read_byte(HL).value();

        byte = Instruction::DEC(registers, byte);

        

        mem.write_byte(HL, byte);
        return 12;
    }

    //0x36 
    pub fn LD_dHL_n(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let HL: u16 = registers.HL( Action::Read).value();
        

        mem.write_byte(HL, operands[0]);

        return 12;
    }

    //0x37
    pub fn SCF(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{  
        registers.set_flag(CARRY_FLAG);
        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);
        return 4;
    }

    //0x38
    pub fn JR_C_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {
        if registers.test_flag(CARRY_FLAG) {
            let jump = operands[0] as i8;
            if jump >= 0 {
                registers.PC(Action::Increment(jump as u16));
            } else {
                registers.PC(Action::Decrement(jump.abs() as u16)); 
            }
            return 12;
        }
        return 8;
    }

    //0x39
    pub fn ADD_HL_SP(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let SP: u16 = registers.SP(Action::Read).value();

        let added = Instruction::ADD_u16(registers, SP);

        registers.HL(Action::Write(added));

        

        return 8;
    }

    //0x3A 
    pub fn LDD_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let dHL: u8 = mem.read_byte( registers.HL(Action::Read).value() ).value();

        registers.A( Action::Write(dHL as u16) );

        registers.HL( Action::Decrement(1) );
        return 8;
    }

    //0x3B
    pub fn DEC_SP(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {

        registers.SP(Action::Decrement(1));

        

        return 8;
    }

    //0x3C
    pub fn INC_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let mut A: u8 = registers.A(Action::Read).value();
        
        A = Instruction::INC(registers, A);

        registers.A( Action::Write(A as u16) );

        

        return 4;
    }

    //0x3D
    pub fn DEC_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let mut A: u8 = registers.A(Action::Read).value();

        A = Instruction::DEC(registers, A);

        registers.A(Action::Write(A as u16));

        

        return 4;
    } 

    //0x3E
    pub fn LD_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        registers.A(Action::Write(operands[0] as u16));
        return 8;
    }

    //0x3F
    pub fn CCF(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let bit = registers.test_flag(CARRY_FLAG) as u8;

        if (bit ^ 1) == 1 {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG)
        }

        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);
        return 4;
    }

    //0x40
    pub fn LD_B_B(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus) -> u8{
        return 4;
    }

    //0x41
    pub fn LD_B_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.B( Action::Write(C as u16) );

        

        return 4;
    }

    //0x42
    pub fn LD_B_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.B( Action::Write(D as u16) );

        

        return 4;
    }

    //0x43
    pub fn LD_B_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.B( Action::Write(E as u16) );

        

        return 4;
    }

    //0x44
    pub fn LD_B_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.B( Action::Write(H as u16) );

        

        return 4;
    }

    //0x45
    pub fn LD_B_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.B( Action::Write(L as u16) );

        

        return 4;
    }

    //0x46
    pub fn LD_B_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.B( Action::Write(dHL as u16) );

        

        return 8;
    }

    //0x47
    pub fn LD_B_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.B( Action::Write(A as u16) );

        

        return 4;
    }

    //0x48
    pub fn LD_C_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.C( Action::Write(B as u16) );

        

        return 4;
    }

    //0x49
    pub fn LD_C_C(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus) -> u8{
        return 4;
    }

    //0x4A
    pub fn LD_C_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.C( Action::Write(D as u16) );

        return 4;
    }

    //0x4B
    pub fn LD_C_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.C( Action::Write(E as u16) );

        return 4;
    }

    //0x4C
    pub fn LD_C_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.C( Action::Write(H as u16) );

        return 4;
    }

    //0x4D
    pub fn LD_C_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.C( Action::Write(L as u16) );

        

        return 4;
    }

    //0x4E
    pub fn LD_C_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.C( Action::Write(dHL as u16) );

        

        return 8;
    }

    //0x4F
    pub fn LD_C_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.C( Action::Write(A as u16) );

        

        return 4;
    }

    //0x50
    pub fn LD_D_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.D( Action::Write(B as u16) );

        

        return 4;
    }

    //0x51
    pub fn LD_D_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.D( Action::Write(C as u16) );

        

        return 4;
    }

    //0x52
    pub fn LD_D_D(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        

        return 4;
    }

    //0x53
    pub fn LD_D_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.D( Action::Write(E as u16) );

        

        return 4;
    }

    //0x54
    pub fn LD_D_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.D( Action::Write(H as u16) );

        return 4;
    }

    //0x55
    pub fn LD_D_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.D( Action::Write(L as u16) );

        

        return 4;
    }

    //0x56
    pub fn LD_D_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.D( Action::Write(dHL as u16) );

        

        return 8;
    }

    //0x57
    pub fn LD_D_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.D( Action::Write(A as u16) );

        

        return 4;
    }

    //0x58
    pub fn LD_E_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.E( Action::Write(B as u16) );

        

        return 4;
    }

    //0x59
    pub fn LD_E_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.E( Action::Write(C as u16) );

        

        return 4;
    }

    //0x5A
    pub fn LD_E_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.E( Action::Write(D as u16) );

        

        return 4;
    }

    //0x5B
    pub fn LD_E_E(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus) -> u8{

        

        return 4;
    }

    //0x5C
    pub fn LD_E_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.E( Action::Write(H as u16) );

        

        return 4;
    }

    //0x5D
    pub fn LD_E_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.E( Action::Write(L as u16) );

        

        return 4;
    }

    //0x5E
    pub fn LD_E_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.E( Action::Write(dHL as u16) );

        

        return 8;
    }

    //0x5F
    pub fn LD_E_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.E( Action::Write(A as u16) );

        

        return 4;
    }

    //0x60
    pub fn LD_H_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.H( Action::Write(B as u16) );

        

        return 4;
    }

    //0x61
    pub fn LD_H_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.H( Action::Write(C as u16) );

        

        return 4;
    }

    //0x62
    pub fn LD_H_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.H( Action::Write(D as u16) );

        

        return 4;
    }

    //0x63
    pub fn LD_H_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.H( Action::Write(E as u16) );

        

        return 4;
    }

    //0x64
    pub fn LD_H_H(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        

        return 4;
    }

    //0x65
    pub fn LD_H_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.H( Action::Write(L as u16) );

        

        return 4;
    }

    //0x66
    pub fn LD_H_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.H( Action::Write(dHL as u16) );

        

        return 8;
    }

    //0x67
    pub fn LD_H_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.H( Action::Write(A as u16) );

        

        return 4;
    }

    //0x68
    pub fn LD_L_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.L( Action::Write(B as u16) );

        

        return 4;
    }

    //0x69
    pub fn LD_L_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.L( Action::Write(C as u16) );

        

        return 4;
    }

    //0x6A
    pub fn LD_L_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.L( Action::Write(D as u16) );

        

        return 4;
    }

    //0x6B
    pub fn LD_L_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.L( Action::Write(E as u16) );

        

        return 4;
    }

    //0x6C
    pub fn LD_L_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.L( Action::Write(H as u16) );

        

        return 4;
    }

    //0x6D
    pub fn LD_L_L(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus) -> u8{

        
        return 4;
        
    }

    //0x6E
    pub fn LD_L_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.L( Action::Write(dHL as u16) );

        
        return 8;
        
    }

    //0x6F
    pub fn LD_L_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let A: u8 = registers.A( Action::Read ).value();

        registers.L( Action::Write(A as u16) );

        

        return 4;
    }

    //0x70
    pub fn LD_dHL_B(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let B: u8 = registers.B( Action::Read ).value();

        

        mem.write_byte(HL, B);
        return 8;
    }

    //0x71
    pub fn LD_dHL_C(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let C: u8 = registers.C( Action::Read ).value();

        

        mem.write_byte(HL, C);
        return 8;
    }

    //0x72
    pub fn LD_dHL_D(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let D: u8 = registers.D( Action::Read ).value();

        

        mem.write_byte(HL, D);
        return 8;
    }

    //0x73
    pub fn LD_dHL_E(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let E: u8 = registers.E( Action::Read ).value();

        

        mem.write_byte(HL, E);
        return 8;
    }

    //0x74
    pub fn LD_dHL_H(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let H: u8 = registers.H( Action::Read ).value();

        

        mem.write_byte(HL, H);
        return 8;
    }

    //0x75
    pub fn LD_dHL_L(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let L: u8 = registers.L( Action::Read ).value();

        

        mem.write_byte(HL, L);
        return 8;
    }

    //0x76 
    pub fn HALT(_operands: [u8; 2], _registers: &mut Registers, mem: &mut Bus) -> u8{

        if !mem.interrupts.master{

            if (mem.interrupts.enable & mem.interrupts.requests & 0x1F) == 0 {

                mem.interrupts.halt_bug = false;
                mem.halt_cpu = true;
                
            } else {
                
                mem.halt_cpu = true;
                mem.interrupts.halt_bug = true;

            }

        } else {
            mem.halt_cpu = true;
            mem.interrupts.halt_bug = false;
        }
        return 4;
    }

    //0x77
    pub fn LD_dHL_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let HL: u16 = registers.HL( Action::Read ).value();
        let A: u8 = registers.A( Action::Read ).value();

        

        mem.write_byte(HL, A);
        return 8;
    }

    //0x78
    pub fn LD_A_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let B: u8 = registers.B( Action::Read ).value();

        registers.A( Action::Write(B as u16) );

        

        return 4;
    }

    //0x79
    pub fn LD_A_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let C: u8 = registers.C( Action::Read ).value();

        registers.A( Action::Write(C as u16) );

        

        return 4;
    }

    //0x7A
    pub fn LD_A_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let D: u8 = registers.D( Action::Read ).value();

        registers.A( Action::Write(D as u16) );

        

        return 4;
    }

    //0x7B
    pub fn LD_A_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let E: u8 = registers.E( Action::Read ).value();

        registers.A( Action::Write(E as u16) );

        

        return 4;
    }

    //0x7C
    pub fn LD_A_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let H: u8 = registers.H( Action::Read ).value();

        registers.A( Action::Write(H as u16) );

        

        return 4;
    }

    //0x7D
    pub fn LD_A_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        
        let L: u8 = registers.L( Action::Read ).value();

        registers.A( Action::Write(L as u16) );

        

        return 4;
    }

    //0x7E
    pub fn LD_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        registers.A( Action::Write(dHL as u16) );

        

        return 8;
    }

    //0x7F
    pub fn LD_A_A(_operands: [u8; 2], _registers: &mut Registers, _mem: &mut Bus) -> u8{

        
        return 4;
        
    }

    //0x80
    pub fn ADD_A_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::ADD_u8(registers, B, false);

        

        return 4;
    }

    //0x81
    pub fn ADD_A_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::ADD_u8(registers, C, false);

        

        return 4;
    }

    //0x82
    pub fn ADD_A_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::ADD_u8(registers, D, false);

        

        return 4;
    }

    //0x83
    pub fn ADD_A_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::ADD_u8(registers, E, false);

        

        return 4;
    }

    //0x84
    pub fn ADD_A_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::ADD_u8(registers, H, false);

        

        return 4;
    }

    //0x85
    pub fn ADD_A_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::ADD_u8(registers, L, false);

        

        return 4;
    }

    //0x86
    pub fn ADD_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{

        let dHL: u8 = mem.read_byte(registers.HL( Action::Read ).value()).value();

        Instruction::ADD_u8(registers, dHL, false);

        

        return 8;
    }

    //0x87
    pub fn ADD_A_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let A: u8 = registers.A( Action::Read ).value();

        Instruction::ADD_u8(registers, A, false);

        

        return 4;
    }

    //0x88
    pub fn ADC_A_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::ADD_u8(registers, B, true);

        

        return 4;
    }

    //0x89
    pub fn ADC_A_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::ADD_u8(registers, C, true);

        

        return 4;
    }

    //0x8A
    pub fn ADC_A_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::ADD_u8(registers, D, true);

        

        return 4;
    }

    //0x8B
    pub fn ADC_A_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::ADD_u8(registers, E, true);

        

        return 4;
    }

    //0x8C
    pub fn ADC_A_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::ADD_u8(registers, H, true);

        

        return 4;
    }

    //0x8D
    pub fn ADC_A_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::ADD_u8(registers, L, true);

        

        return 4;
    }

    //0x8E
    pub fn ADC_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{

        let dHL: u8 = mem.read_byte(registers.HL( Action::Read ).value()).value();

        Instruction::ADD_u8(registers, dHL, true);

        

        return 8;
    }

    //0x8F
    pub fn ADC_A_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let A: u8 = registers.A( Action::Read ).value();

        Instruction::ADD_u8(registers, A, true);

        

        return 4;
    }

    //0x90
    pub fn SUB_A_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::SUB_u8(registers, B, false);

        

        return 4;
    }

    //0x91
    pub fn SUB_A_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::SUB_u8(registers, C, false);

        

        return 4;
    }

    //0x92
    pub fn SUB_A_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::SUB_u8(registers, D, false);

        

        return 4;
    }

    //0x93
    pub fn SUB_A_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::SUB_u8(registers, E, false);

        

        return 4;
    }

    //0x94
    pub fn SUB_A_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::SUB_u8(registers, H, false);

        

        return 4;
    }

    //0x95
    pub fn SUB_A_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::SUB_u8(registers, L, false);

        

        return 4;
    }

    //0x96
    pub fn SUB_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{

        let dHL: u8 = mem.read_byte(registers.HL( Action::Read ).value()).value();

        Instruction::SUB_u8(registers, dHL, false);

        

        return 8;
    }

    //0x97
    pub fn SUB_A_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let A: u8 = registers.A( Action::Read ).value();

        Instruction::SUB_u8(registers, A, false);

        

        return 4;
    }

    //0x98
    pub fn SBC_A_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::SUB_u8(registers, B, true);

        

        return 4;
    }

    //0x99
    pub fn SBC_A_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::SUB_u8(registers, C, true);

        

        return 4;
    }

    //0x9A
    pub fn SBC_A_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::SUB_u8(registers, D, true);

        

        return 4;
    }

    //0x9B
    pub fn SBC_A_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::SUB_u8(registers, E, true);

        

        return 4;
    }

    //0x9C
    pub fn SBC_A_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::SUB_u8(registers, H, true);

        

        return 4;
    }

    //0x9D
    pub fn SBC_A_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::SUB_u8(registers, L, true);

        

        return 4;
    }

    //0x9E
    pub fn SBC_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{

        let dHL: u8 = mem.read_byte(registers.HL( Action::Read ).value()).value();

        Instruction::SUB_u8(registers, dHL, true);

        

        return 8;
    }

    //0x9F
    pub fn SBC_A_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let A: u8 = registers.A( Action::Read ).value();

        Instruction::SUB_u8(registers, A, true);

        

        return 4;
    }

    //0xA0
    pub fn AND_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::AND_u8(registers, B);

        

        return 4;
    }

    //0xA1
    pub fn AND_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::AND_u8(registers, C);

        

        return 4;
    }

    //0xA2
    pub fn AND_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::AND_u8(registers, D);

        

        return 4;
    }

    //0xA3
    pub fn AND_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::AND_u8(registers, E);

        

        return 4;
    }

    //0xA4
    pub fn AND_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::AND_u8(registers, H);

        

        return 4;
    }

    //0xA5
    pub fn AND_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::AND_u8(registers, L);

        

        return 4;
    }

    //0xA6
    pub fn AND_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{

        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value()).value();

        Instruction::AND_u8(registers, dHL);

        

        return 8;
    }

    //0xA7
    pub fn AND_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let A: u8 = registers.A( Action::Read ).value();

        Instruction::AND_u8(registers, A);
        return 4;
    }

    //0xA8
    pub fn XOR_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::XOR_u8(registers, B);

        

        return 4;
    }

    //0xA9
    pub fn XOR_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::XOR_u8(registers, C);

        

        return 4;
    }

    //0xAA
    pub fn XOR_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::XOR_u8(registers, D);

        

        return 4;
    }

    //0xAB
    pub fn XOR_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::XOR_u8(registers, E);

        

        return 4;
    }

    //0xAC
    pub fn XOR_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::XOR_u8(registers, H);

        

        return 4;
    }

    //0xAD
    pub fn XOR_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::XOR_u8(registers, L);

        

        return 4;
    }

    //0xAE
    pub fn XOR_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{

        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value()).value();

        Instruction::XOR_u8(registers, dHL);

        

        return 8;
    }

    //0xAF
    pub fn XOR_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let A: u8 = registers.A( Action::Read ).value();

        Instruction::XOR_u8(registers, A);

        return 4;
    }

    //0xB0
    pub fn OR_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::OR_u8(registers, B);

        

        return 4;
    }

    //0xB1
    pub fn OR_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::OR_u8(registers, C);

        

        return 4;
    }

    //0xB2
    pub fn OR_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::OR_u8(registers, D);

        

        return 4;
    }

    //0xB3
    pub fn OR_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::OR_u8(registers, E);

        

        return 4;
    }

    //0xB4
    pub fn OR_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::OR_u8(registers, H);

        

        return 4;
    }

    //0xB5
    pub fn OR_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::OR_u8(registers, L);

        

        return 4;
    }

    //0xB6
    pub fn OR_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{

        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value()).value();

        Instruction::OR_u8(registers, dHL);

        

        return 8;
    }

    //0xB7
    pub fn OR_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let A: u8 = registers.A( Action::Read ).value();

        Instruction::OR_u8(registers, A);
        return 4;
    }

    //0xB8
    pub fn CP_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let B: u8 = registers.B( Action::Read ).value();

        Instruction::CP_u8(registers, B);

        

        return 4;
    }

    //0xB9
    pub fn CP_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let C: u8 = registers.C( Action::Read ).value();

        Instruction::CP_u8(registers, C);

        

        return 4;
    }

    //0xBA
    pub fn CP_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let D: u8 = registers.D( Action::Read ).value();

        Instruction::CP_u8(registers, D);
        return 4;
    }

    //0xBB
    pub fn CP_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let E: u8 = registers.E( Action::Read ).value();

        Instruction::CP_u8(registers, E);
        return 4;
    }

    //0xBC
    pub fn CP_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let H: u8 = registers.H( Action::Read ).value();

        Instruction::CP_u8(registers, H);
        return 4;
    }

    //0xBD
    pub fn CP_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let L: u8 = registers.L( Action::Read ).value();

        Instruction::CP_u8(registers, L);
        return 4;
    }

    //0xBE
    pub fn CP_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {


        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();

        Instruction::CP_u8(registers, dHL);
        return 8;
    }

    //0xBF
    pub fn CP_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let A: u8 = registers.A( Action::Read ).value();

        Instruction::CP_u8(registers, A);
        return 4;
    }

    //0xC0
    pub fn RET_NZ(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        if !registers.test_flag(ZERO_FLAG) {
            let pointer = Instruction::pop_from_stack(registers, mem);
            registers.PC( Action::Write(pointer) );
            return 20;
        }

        

        return 8;
    }

    //0xC1
    pub fn POP_BC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let popped = Instruction::pop_from_stack(registers, mem);
        registers.BC( Action::Write(popped) );


        

        return 12;
    }

    //0xC2
    pub fn JP_NZ_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        if !registers.test_flag(ZERO_FLAG) {
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
            
            return 16;
        }

        return 12;
    }

    //0xC3
    pub fn JP_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let pointer = Bus::to_short(operands);
        registers.PC( Action::Write(pointer) );

        

        return 16;
    }

    //0xC4
    pub fn CALL_NZ_nn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let pointer = Bus::to_short(operands);
        if !registers.test_flag(ZERO_FLAG) {
            let PC: u16 = registers.PC( Action::Read ).value(); 
            Instruction::push_to_stack(registers, mem, PC);
            registers.PC( Action::Write(pointer) );
            return 24;

        }

        

        return 12;
    }

    //0xC5
    pub fn PUSH_BC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let BC: u16 = registers.BC( Action::Read ).value();
        Instruction::push_to_stack(registers, mem, BC);

        

        return 16;
    }

    //0xC6
    pub fn ADD_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        Instruction::ADD_u8(registers, operands[0], false);

        return 8;
    }

    //0xC7
    pub fn RST_0(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(0x0) );
        return 16;
    }

    //0xC8
    pub fn RET_Z(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        if registers.test_flag(ZERO_FLAG) {
            let pointer = Instruction::pop_from_stack(registers, mem);
            registers.PC( Action::Write(pointer) );
            return 20;
        }

        

        return 8;
    }

    //0xC9
    pub fn RET(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let pointer = Instruction::pop_from_stack(registers, mem);
        registers.PC( Action::Write(pointer) );

        

        return 16;
    }

    //0xCA
    pub fn JP_Z_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        if registers.test_flag(ZERO_FLAG) { 
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
            
            return 16;
        }

        

        return 12;
    }

    //0xCB prefix, not a function

    //0xCC
    pub fn CALL_Z_nn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        if registers.test_flag(ZERO_FLAG) {
            let PC: u16 = registers.PC( Action::Read ).value(); 
            Instruction::push_to_stack(registers, mem, PC);
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
            return 24;
        }


        

        return 12;
    }

    //0xCD
    pub fn CALL_nn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        let pointer = Bus::to_short(operands);
        registers.PC( Action::Write(pointer) );
        return 24;
    }

    //0xCE
    pub fn ADC_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        Instruction::ADD_u8(registers, operands[0], true);
        return 8;
    }

    //0xCF
    pub fn RST_8(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(0x08) );


        

        return 16;
    }

    //0xD0
    pub fn RET_NC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        if !registers.test_flag(CARRY_FLAG) {
            let pointer = Instruction::pop_from_stack(registers, mem);
            registers.PC( Action::Write(pointer) );
            return 20;
        }

        

        return 8;
    }

    //0xD1
    pub fn POP_DE(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let popped = Instruction::pop_from_stack(registers, mem);
        registers.DE( Action::Write(popped) );


        

        return 12;
    }

    //0xD2
    pub fn JP_NC_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        if !registers.test_flag(CARRY_FLAG) { 
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
            return 16;
        }

        

        return 12;
    }

    //0xD4
    pub fn CALL_NC_nn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        if !registers.test_flag(CARRY_FLAG) {
            let PC: u16 = registers.PC( Action::Read ).value(); 
            Instruction::push_to_stack(registers, mem, PC);
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
            return 24;
        }


        

        return 12;
    }

    //0xD5
    pub fn PUSH_DE(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let DE: u16 = registers.DE( Action::Read ).value();
        Instruction::push_to_stack(registers, mem, DE);

        

        return 16;
    }

    //0xD6
    pub fn SUB_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        Instruction::SUB_u8(registers, operands[0], false);
        return 8;
    }

    //0xD7
    pub fn RST_10(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(0x10) );


        

        return 16;
    }

    //0xD8
    pub fn RET_C(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        if registers.test_flag(CARRY_FLAG) {
            let pointer = Instruction::pop_from_stack(registers, mem);
            registers.PC( Action::Write(pointer) );
            return 20;
        }

        

        return 8;
    }

    //0xD9
    pub fn RETI(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        mem.interrupts.master = true;
        let pointer = Instruction::pop_from_stack(registers, mem);
        registers.PC( Action::Write(pointer) ); 
        return 16;
    }

    //0xDA
    pub fn JP_C_nn(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        if registers.test_flag(CARRY_FLAG) { 
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
            return 16;
        }

        

        return 12;
    }

    //0xDC
    pub fn CALL_C_nn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        if registers.test_flag(CARRY_FLAG) {
            let PC: u16 = registers.PC( Action::Read ).value(); 
            Instruction::push_to_stack(registers, mem, PC);
            let pointer = Bus::to_short(operands);
            registers.PC( Action::Write(pointer) );
            return 24;
        }


        

        return 12;
    }



    //0xDE
    pub fn SBC_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        Instruction::SUB_u8(registers, operands[0], true);
        return 8;
    }

    //0xDF
    pub fn RST_18(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(0x18) );


        

        return 16;
    }

    //0xE0
    pub fn LDH_dn_A(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{

        let A: u8 = registers.A( Action::Read ).value();

        mem.write_byte(0xFF00 + operands[0] as u16, A);
        return 12;
    }

    //0xE1
    pub fn POP_HL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let popped = Instruction::pop_from_stack(registers, mem);
        registers.HL( Action::Write(popped) );


        

        return 12;
    }

    //0xE2
    pub fn LDH_dC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let C: u8 =  registers.C(Action::Read).value();
        
        
        mem.write_byte(0xFF00 + C as u16, registers.A(Action::Read).value() );
        return 8;
    }

    //0xE5
    pub fn PUSH_HL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let HL: u16 = registers.HL( Action::Read ).value();
        Instruction::push_to_stack(registers, mem, HL);

        

        return 16;
    }

    //0xE6
    pub fn AND_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{


        Instruction::AND_u8(registers, operands[0]);
        return 8;
    }

    //0xE7
    pub fn RST_20(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(0x20) );


        

        return 16;
    }

    //0xE8
    pub fn ADD_SP_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let SP: u16 = registers.SP(Action::Read).value();

        let jump = (operands[0] as i8) as i16;

        let result = ((SP as i16).wrapping_add(jump)) as u16;

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
        
        return 16;
    }

    //0xE9
    pub fn JP_dHL(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        let HL: u16 = registers.HL( Action::Read ).value();

        registers.PC( Action::Write(HL) );
        return 4;
    }

    //0xEA
    pub fn LD_dnn_A(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let pointer = Bus::to_short(operands);
        let A: u8 = registers.A( Action::Read ).value();

        

        mem.write_byte(pointer, A);
        return 16;
    }

    //0xEE
    pub fn XOR_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {

        Instruction::XOR_u8(registers, operands[0]);
        return 8;
    }

    //0xEF
    pub fn RST_28(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(0x28) );


        

        return 16;
    }

    //0xF0
    pub fn LDH_A_dn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let result: u8 = mem.read_byte( 0xFF00 + operands[0] as u16).value();

        registers.A( Action::Write(result as u16) );
        return 12;
    }

    //0xF1
    pub fn POP_AF(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let mut popped = Instruction::pop_from_stack(registers, mem);
        popped = popped & 0xFFF0;
        registers.AF( Action::Write(popped) );
        return 12;
    }


    //0xF2
    pub fn LDH_A_dC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let C: u8 = registers.C( Action::Read ).value();

        let result: u8 = mem.read_byte( 0xFF00 + C as u16).value();

        registers.A( Action::Write(result as u16) );
        return 8;
    }

    //0xF3
    pub fn DI(_operands: [u8; 2], _registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        mem.disable_interrupts();
    
        return 4;
    }

    //0xF5
    pub fn PUSH_AF(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        
        let AF: u16 = registers.AF( Action::Read ).value();
        Instruction::push_to_stack(registers, mem, AF);

        

        return 16;
    }

    //0xF6
    pub fn OR_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {  
        Instruction::OR_u8(registers, operands[0]);
        return 8;
    }

    //0xF7
    pub fn RST_30(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let PC: u16 = registers.PC( Action::Read ).value(); 
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(0x30) );
        return 16;
    }

    //0xF8
    pub fn LDHL_SP_d(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {
        let SP: u16 = registers.SP(Action::Read).value();

        let jump = (operands[0] as i8) as i16;

        let result = ((SP as i16).wrapping_add(jump)) as u16;

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
        
        return 12;
    }

    //0xF9
    pub fn LD_SP_HL(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        let HL: u16 = registers.HL( Action::Read ).value();
        registers.SP( Action::Write(HL) );
        return 8;
    }

    //0xFA
    pub fn LD_A_dnn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let nn: u16 = Bus::to_short(operands);
        let dnn: u8 = mem.read_byte(nn).value();
        registers.A( Action::Write(dnn as u16) );
        return 16;
    }

    //0xFB
    pub fn EI(_operands: [u8; 2], _registers: &mut Registers, mem: &mut Bus) -> u8 { 
        mem.enable_interrupts();
        return 4;
    }

    //0xFE
    pub fn CP_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        Instruction::CP_u8(registers, operands[0]);
        return 8;
    }

    //0xFF
    pub fn RST_38(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let PC: u16 = registers.PC( Action::Read ).value();
        Instruction::push_to_stack(registers, mem, PC);
        registers.PC( Action::Write(0x38) );  
        return 16;
    }


}