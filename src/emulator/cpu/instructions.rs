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

macro_rules! LD_r_r {
    ( $( $name:ident,$recv:ident,$operand:ident ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
                let $operand: u8 = registers.$operand( Action::Read ).value();
                registers.$recv( Action::Write($operand as u16) );
                return 4;
            }
        )*
    }
}

macro_rules! LD_r_n {
    ( $( $name:ident,$recv:ident ),* ) => {
        $( 
            pub fn $name(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
                registers.$recv( Action::Write(operands[0] as u16) );
                return 8;
            }
        )*
    }
}

macro_rules! LD_r_dHL {
    ( $( $name:ident,$recv:ident ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {   
                let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();
                registers.$recv( Action::Write(dHL as u16) );
                return 8;
            }
        )*
    }
}

macro_rules! LD_dHL_r {
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
                let HL: u16 = registers.HL( Action::Read).value();
                let data: u8 = registers.$r( Action::Read ).value();
                mem.write_byte(HL, data);
                return 8;
            }
        )*
    }
}

macro_rules! LD_rr_nn {
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
                let value = Bus::to_short(operands);
                registers.$r( Action::Write(value) );
                return 12;
            }
        )*
    }
} 

macro_rules! PUSH_rr {
    ( $( $name:ident,$rr:ident ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
                let data: u16 = registers.$rr( Action::Read ).value();
                Instruction::push_to_stack(registers, mem, data);
                return 16;
            }
        )*
    }
} 
macro_rules! POP_rr {
    ( $( $name:ident,$rr:ident ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
                let popped = Instruction::pop_from_stack(registers, mem);
                registers.$rr( Action::Write(popped) );
                return 12;
            }
        )*
    }
} 

macro_rules! ADD_A_r {
    ( $( $name:ident,$r:ident,$carry:expr ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
                let reg: u8 = registers.$r( Action::Read ).value();
                Instruction::ADD_u8(registers, reg, $carry);
                return 4;
            }
        )*
    }
} 

macro_rules! SUB_A_r {
    ( $( $name:ident,$r:ident,$carry:expr ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
                let reg: u8 = registers.$r( Action::Read ).value();
                Instruction::SUB_u8(registers, reg, $carry);
                return 4;
            }
        )*
    }
} 

macro_rules! LOGIC_r {
    ( $( $name:ident, $op:ident ,$r:ident ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
                let reg: u8 = registers.$r( Action::Read ).value();
                Instruction::$op(registers, reg);
                return 4;
            }
        )*
    }
} 

macro_rules! LOGIC_n {
    ( $( $name:ident, $op:ident ),* ) => {
        $( 
            pub fn $name(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
                Instruction::$op(registers, operands[0]);
                return 8;
            }
        )*
    }
} 

macro_rules! INC_DEC {
    ( $( $name:ident, $op:ident ,$r:ident ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
                let mut reg: u8 = registers.$r( Action::Read ).value();
                reg = Instruction::$op(registers, reg);
                registers.$r( Action::Write(reg as u16) );
                return 4;
            }
        )*
    }
} 

macro_rules! INC_DEC_16 {
    ( $( $name:ident, $op:ident, $rr:ident ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 { 
                registers.$rr( Action::$op(1) );
                return 8;
            }
        )*
    }
} 
macro_rules! ADD_HL_rr {
    ( $( $name:ident,$rr:ident ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
                let reg: u16 = registers.$rr(Action::Read).value();
                let added = Instruction::ADD_u16(registers, reg);
                registers.HL(Action::Write(added));
                return 8;
            }
        )*
    }
} 

macro_rules! JR_cc_d {
    ( $( $name:ident, $not_name:ident, $flag:ident),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {
                if registers.test_flag($flag) {
                    Instruction::JR_n(_operands, registers, _mem);
                    return 12;
                }
                return 8;
            }
            pub fn $not_name(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8  {
                if !registers.test_flag($flag) {
                    Instruction::JR_n(_operands, registers, _mem);
                    return 12;
                }
                return 8;
            }
        )*
    }
} 

macro_rules! RET_cc {
    ( $( $name:ident, $not_name:ident, $flag:ident),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
                if registers.test_flag($flag) {
                    Instruction::RET(_operands, registers, mem);
                    return 20;
                }
                return 8;
            }
            pub fn $not_name(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
                if !registers.test_flag($flag) {
                    Instruction::RET(_operands, registers, mem);
                    return 20;
                }
                return 8;
            }
        )*
    }
} 
macro_rules! RST_n {
    ( $( $name:ident,$n:expr ),* ) => {
        $( 
            pub fn $name(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
                let PC: u16 = registers.PC( Action::Read ).value();
                Instruction::push_to_stack(registers, mem, PC);
                registers.PC( Action::Write($n) );  
                return 16;
            }
        )*
    }
} 

impl Instruction {

    LD_r_r!(
        LD_A_A, A, A,   LD_B_A, B, A,   LD_C_A, C, A,   LD_D_A, D, A,
        LD_A_B, A, B,   LD_B_B, B, B,   LD_C_B, C, B,   LD_D_B, D, B,
        LD_A_C, A, C,   LD_B_C, B, C,   LD_C_C, C, C,   LD_D_C, D, C,
        LD_A_D, A, D,   LD_B_D, B, D,   LD_C_D, C, D,   LD_D_D, D, D,
        LD_A_E, A, E,   LD_B_E, B, E,   LD_C_E, C, E,   LD_D_E, D, E,
        LD_A_H, A, H,   LD_B_H, B, H,   LD_C_H, C, H,   LD_D_H, D, H,
        LD_A_L, A, L,   LD_B_L, B, L,   LD_C_L, C, L,   LD_D_L, D, L,

        LD_E_A, E, A,   LD_H_A, H, A,   LD_L_A, L, A,
        LD_E_B, E, B,   LD_H_B, H, B,   LD_L_B, L, B,
        LD_E_C, E, C,   LD_H_C, H, C,   LD_L_C, L, C,
        LD_E_D, E, D,   LD_H_D, H, D,   LD_L_D, L, D,
        LD_E_E, E, E,   LD_H_E, H, E,   LD_L_E, L, E,
        LD_E_H, E, H,   LD_H_H, H, H,   LD_L_H, L, H,
        LD_E_L, E, L,   LD_H_L, H, L,   LD_L_L, L, L
    );

    LD_r_n!(
        LD_A_n, A,
        LD_B_n, B,
        LD_C_n, C,
        LD_D_n, D,
        LD_E_n, E,
        LD_H_n, H,
        LD_L_n, L
    );

    LD_r_dHL!(
        LD_A_dHL, A,
        LD_B_dHL, B,
        LD_C_dHL, C,
        LD_D_dHL, D,
        LD_E_dHL, E,
        LD_H_dHL, H,
        LD_L_dHL, L
    );

    LD_dHL_r!(
        LD_dHL_A, A,
        LD_dHL_B, B,
        LD_dHL_C, C,
        LD_dHL_D, D,
        LD_dHL_E, E,
        LD_dHL_H, H,
        LD_dHL_L, L
    );

    LD_rr_nn!(
        LD_BC_nn, BC,
        LD_DE_nn, DE,
        LD_HL_nn, HL,
        LD_SP_nn, SP
    );

    PUSH_rr!(
        PUSH_AF, AF,
        PUSH_BC, BC,
        PUSH_DE, DE,
        PUSH_HL, HL
    );
    
    POP_rr!(
        POP_BC, BC,
        POP_DE, DE,
        POP_HL, HL
    );

    pub fn POP_AF(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
    
        let mut popped = Instruction::pop_from_stack(registers, mem);
        popped = popped & 0xFFF0;
        registers.AF( Action::Write(popped) );
        return 12;
    }
    
    ADD_A_r!(
        ADD_A_A, A, false,  ADC_A_A, A, true,
        ADD_A_B, B, false,  ADC_A_B, B, true,
        ADD_A_C, C, false,  ADC_A_C, C, true,
        ADD_A_D, D, false,  ADC_A_D, D, true,
        ADD_A_E, E, false,  ADC_A_E, E, true,
        ADD_A_H, H, false,  ADC_A_H, H, true,
        ADD_A_L, L, false,  ADC_A_L, L, true
    );

    SUB_A_r!(
        SUB_A_A, A, false,  SBC_A_A, A, true,
        SUB_A_B, B, false,  SBC_A_B, B, true,
        SUB_A_C, C, false,  SBC_A_C, C, true,
        SUB_A_D, D, false,  SBC_A_D, D, true,
        SUB_A_E, E, false,  SBC_A_E, E, true,
        SUB_A_H, H, false,  SBC_A_H, H, true,
        SUB_A_L, L, false,  SBC_A_L, L, true
    );

    LOGIC_r!(
        AND_A, AND_u8, A,       OR_A, OR_u8, A,     XOR_A, XOR_u8, A,   CP_A, CP_u8, A,
        AND_B, AND_u8, B,       OR_B, OR_u8, B,     XOR_B, XOR_u8, B,   CP_B, CP_u8, B,
        AND_C, AND_u8, C,       OR_C, OR_u8, C,     XOR_C, XOR_u8, C,   CP_C, CP_u8, C,
        AND_D, AND_u8, D,       OR_D, OR_u8, D,     XOR_D, XOR_u8, D,   CP_D, CP_u8, D,
        AND_E, AND_u8, E,       OR_E, OR_u8, E,     XOR_E, XOR_u8, E,   CP_E, CP_u8, E,
        AND_H, AND_u8, H,       OR_H, OR_u8, H,     XOR_H, XOR_u8, H,   CP_H, CP_u8, H,
        AND_L, AND_u8, L,       OR_L, OR_u8, L,     XOR_L, XOR_u8, L,   CP_L, CP_u8, L
    );

    LOGIC_n!(
        AND_n, AND_u8, OR_n, OR_u8, XOR_n, XOR_u8, CP_n, CP_u8
    );

    INC_DEC!(
        INC_A, INC, A,  DEC_A, DEC, A,
        INC_B, INC, B,  DEC_B, DEC, B,
        INC_C, INC, C,  DEC_C, DEC, C,
        INC_D, INC, D,  DEC_D, DEC, D,
        INC_E, INC, E,  DEC_E, DEC, E,
        INC_H, INC, H,  DEC_H, DEC, H,
        INC_L, INC, L,  DEC_L, DEC, L
    );

    INC_DEC_16!(
        INC_BC, Increment, BC,  DEC_BC, Decrement, BC,
        INC_DE, Increment, DE,  DEC_DE, Decrement, DE,
        INC_HL, Increment, HL,  DEC_HL, Decrement, HL,
        INC_SP, Increment, SP,  DEC_SP, Decrement, SP 
    );

    ADD_HL_rr!( ADD_HL_BC, BC, ADD_HL_DE, DE, ADD_HL_HL, HL, ADD_HL_SP, SP ); 

    JR_cc_d!(
        JR_Z_n, JR_NZ_n, ZERO_FLAG, JR_C_n, JR_NC_n, CARRY_FLAG
    );

    RST_n!(
        RST_0, 0, RST_8, 8, RST_10, 0x10, RST_18, 0x18, RST_20, 0x20, RST_28, 0x28, RST_30, 0x30,  RST_38, 0x38
    );

    RET_cc!( RET_Z, RET_NZ, ZERO_FLAG, RET_C, RET_NC, CARRY_FLAG );

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

    //0x02
    pub fn LD_dBC_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8  {
        let BC = registers.BC(Action::Read).value();
        mem.write_byte(BC, registers.A(Action::Read).value() );
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

    //0x0A
    pub fn LD_A_dBC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let dBC: u8 = mem.read_byte( registers.BC(Action::Read).value() ).value();
        registers.A(Action::Write(dBC as u16));
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

    //0x12
    pub fn LD_dDE_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let DE: u16 = registers.DE( Action::Read ).value();
        mem.write_byte(DE, registers.A( Action::Read ).value());
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

    //0x1A
    pub fn LD_A_dDE(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let DE: u16 = registers.DE( Action::Read ).value();
        let read = mem.read_byte( DE );
        let value: u8 = read.value();
        registers.A(Action::Write(value as u16));
        return 8;
    }

    //0x1F
    pub fn RR_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8 {
        let mut A: u8 = registers.A( Action::Read ).value();
        A = Instruction::RR(registers, A, true, false);
        registers.A( Action::Write(A as u16) );
        return 4;
    }

    //0x22
    pub fn LDI_HL_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let hlad: u16 = registers.HL(Action::Read).value();
        registers.HL(Action::Increment(1));
        mem.write_byte(hlad, registers.A(Action::Read).value() );
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

    //0x2A 
    pub fn LDI_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let HL: u16 = registers.HL( Action::Read ).value();
        let rHL = mem.read_byte( HL );
        let dHL: u8 = rHL.value();
        registers.A( Action::Write(dHL as u16) );
        registers.HL( Action::Increment(1) );
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

    //0x32
    pub fn LDD_HL_A(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let hlad = registers.HL(Action::Read).value();
        registers.HL(Action::Decrement(1));
        mem.write_byte(hlad, registers.A(Action::Read).value() );
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

    //0x3A 
    pub fn LDD_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let dHL: u8 = mem.read_byte( registers.HL(Action::Read).value() ).value();
        registers.A( Action::Write(dHL as u16) );
        registers.HL( Action::Decrement(1) );
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

    //0x86
    pub fn ADD_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let dHL: u8 = mem.read_byte(registers.HL( Action::Read ).value()).value();
        Instruction::ADD_u8(registers, dHL, false);
        return 8;
    }

    //0x8E
    pub fn ADC_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let dHL: u8 = mem.read_byte(registers.HL( Action::Read ).value()).value();
        Instruction::ADD_u8(registers, dHL, true);
        return 8;
    }

    //0x96
    pub fn SUB_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let dHL: u8 = mem.read_byte(registers.HL( Action::Read ).value()).value();
        Instruction::SUB_u8(registers, dHL, false);
        return 8;
    }

    //0x9E
    pub fn SBC_A_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let dHL: u8 = mem.read_byte(registers.HL( Action::Read ).value()).value();
        Instruction::SUB_u8(registers, dHL, true);
        return 8;
    }

    //0xA6
    pub fn AND_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value()).value();
        Instruction::AND_u8(registers, dHL);
        return 8;
    }

    //0xAE
    pub fn XOR_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value()).value();
        Instruction::XOR_u8(registers, dHL);
        return 8;
    }

    //0xB6
    pub fn OR_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value()).value();
        Instruction::OR_u8(registers, dHL);
        return 8;
    }
    //0xBE
    pub fn CP_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        let dHL: u8 = mem.read_byte( registers.HL( Action::Read ).value() ).value();
        Instruction::CP_u8(registers, dHL);
        return 8;
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

    //0xC6
    pub fn ADD_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
        Instruction::ADD_u8(registers, operands[0], false);
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


    //0xD6
    pub fn SUB_A_n(operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{

        Instruction::SUB_u8(registers, operands[0], false);
        return 8;
    }

    //0xD9
    pub fn RETI(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {
        mem.enable_interrupts();
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

    //0xE0
    pub fn LDH_dn_A(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{

        let A: u8 = registers.A( Action::Read ).value();

        mem.write_byte(0xFF00 + operands[0] as u16, A);
        return 12;
    }


    //0xE2
    pub fn LDH_dC(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8{
        let C: u8 =  registers.C(Action::Read).value();
        
        
        mem.write_byte(0xFF00 + C as u16, registers.A(Action::Read).value() );
        return 8;
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

    //0xF0
    pub fn LDH_A_dn(operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) -> u8 {

        let result: u8 = mem.read_byte( 0xFF00 + operands[0] as u16).value();

        registers.A( Action::Write(result as u16) );
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
}