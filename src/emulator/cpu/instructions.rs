#![allow(non_snake_case)]
#![macro_use]
use std::collections::VecDeque;
use super::Bus;
use super::registers::{Action, Registers, Value};
use super::generic::*;
use std::fmt;
use crate::emulator::bit_utils::BitUtils;

pub struct Instruction {
    pub disassembly: &'static str,
    pub operations: VecDeque<fn(&mut Self, &mut Registers, &mut Bus)>,
    buffer_u8: Vec<u8>,
    buffer_u16: u16,
    pub cycles: u8,
    flag: bool
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.disassembly)
    }
}

macro_rules! LD_r_r {
    ( $( $name:ident,$recv:ident,$operand:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus) {
                let $operand: u8 = registers.$operand( Action::Read ).value();
                registers.$recv( Action::Write($operand as u16) );
            }
        )*
    }
}

macro_rules! read_bus_with_rr {
    ( $( $name:ident,$rr:ident ),* ) => {
        $( 
            pub fn $name(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus) {
                let rr: u16 = registers.$rr( Action::Read ).value();
                let val: u8 = bus.read_byte(rr).value();
                inst.buffer_u8.push(val);
                inst.buffer_u16 = rr;
            }
        )*
    }
}

macro_rules! write_r_with_buffer {
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                let val = inst.buffer_u8.pop().unwrap();
                registers.$r( Action::Write(val as u16 ));
            }
        )*
    }
}

macro_rules! write_r_in_dHL {
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
                let val: u8 = registers.$r( Action::Read ).value();
                let hl: u16 = registers.HL( Action::Read ).value();
        
                bus.write_byte(hl, val);
            }
        )*
    }
}

macro_rules! write_r_in_dSP {
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
                let val: u8 = registers.$r( Action::Read ).value();
                let sp: u16 = registers.SP( Action::Read ).value();
        
                bus.write_byte(sp, val);
            }
        )*
    }
}

macro_rules! ADD_A_r {
    ( $( $name:ident,$r:ident,$carry:expr ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                let reg: u8 = registers.$r( Action::Read ).value();
                Instruction::ADD_u8(registers, reg, $carry);
            }
        )*
    }
} 
macro_rules! SUB_A_r {
    ( $( $name:ident,$r:ident,$carry:expr ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                let reg: u8 = registers.$r( Action::Read ).value();
                Instruction::SUB_u8(registers, reg, $carry);
            }
        )*
    }
}

macro_rules! LOGIC_r {
    ( $( $name:ident, $op:ident ,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                let reg: u8 = registers.$r( Action::Read ).value();
                Instruction::$op(registers, reg);
            }
        )*
    }
} 

macro_rules! LOGIC_with_buffer {
    ( $( $name:ident, $op:ident ),* ) => {
        $( 
            pub fn $name(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                let buf = inst.buffer_u8.pop().unwrap();
                Instruction::$op(registers, buf);
            }
        )*
    }
}

macro_rules! write_buffer_to_rr {
    ( $( $name:ident, $op:ident ),* ) => {
        $( 
            pub fn $name(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                registers.$op(Action::Write(inst.buffer_u16));
            }
        )*
    }
}

macro_rules! ADD_HL_rr {
    ( $( $name:ident, $rr:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                let val: u16 = registers.$rr(Action::Read).value();
                let added = Instruction::ADD_u16(registers, val);
                registers.HL(Action::Write(added));
            }
        )*
    }
}

macro_rules! INC_rr {
    ( $( $name:ident, $rr:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                registers.$rr(Action::Increment(1));
            }
        )*
    }
}

macro_rules! DEC_rr {
    ( $( $name:ident, $rr:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                registers.$rr(Action::Decrement(1));
            }
        )*
    }
}

macro_rules! INC_DEC {
    ( $( $name:ident, $op:ident ,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                let mut reg: u8 = registers.$r( Action::Read ).value();
                reg = Instruction::$op(registers, reg);
                registers.$r( Action::Write(reg as u16) );
            }
        )*
    }
} 

macro_rules! res_buffer {
    ( $( $name:ident, $bit:expr ),* ) => {
        $( 
            pub fn $name(inst: &mut Instruction, _registers: &mut Registers, _bus: &mut Bus){
                inst.buffer_u8[0].reset_bit($bit);
            }
        )*
    }
} 

macro_rules! set_buffer {
    ( $( $name:ident, $bit:expr ),* ) => {
        $( 
            pub fn $name(inst: &mut Instruction, _registers: &mut Registers, _bus: &mut Bus){
                inst.buffer_u8[0].set_bit($bit);
            }
        )*
    }
} 

macro_rules! bit_buffer {
    ( $( $name:ident, $bit:expr ),* ) => {
        $( 
            pub fn $name(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
                let val = inst.buffer_u8.pop().unwrap();
                Instruction::BIT(val, $bit, registers);
            }
        )*
    }
} 





impl Instruction {

    pub fn tick(&mut self, registers: &mut Registers, bus: &mut Bus){
        self.cycles -= 1;
        let func = self.operations.pop_front().unwrap();

        func(self, registers, bus);
    }

    pub fn holder() -> Self {
        Self{
            disassembly: "HOLDER",
            operations: vec![].into_iter().collect(),
            buffer_u16: 0,
            buffer_u8: vec![],
            cycles: 0,
            flag: false
        }
    }

    pub fn new(
        name: &'static str,
        ops: VecDeque<fn(&mut Self, &mut Registers, &mut Bus)>,
        cy: u8
    ) -> Self {
        return Instruction {
            disassembly: name,
            operations: ops,
            buffer_u16: 0,
            buffer_u8: vec![],
            cycles: cy,
            flag: false
        }
    }

    pub fn nop(_inst: &mut Instruction, _registers: &mut Registers, _bus: &mut Bus){}
    pub fn halt(_inst: &mut Instruction, _registers: &mut Registers, bus: &mut Bus){
        if !bus.interrupts.master{
            if (bus.interrupts.enable & bus.interrupts.requests & 0x1F) == 0 {
                bus.interrupts.halt_bug = false;
                bus.halt_cpu = true;
            } else {
                bus.halt_cpu = true;
                bus.interrupts.halt_bug = true;
            }
        } else {
            bus.halt_cpu = true;
            bus.interrupts.halt_bug = false;
        }
    }
    pub fn stop(_inst: &mut Instruction, _registers: &mut Registers, _bus: &mut Bus){}
    
    write_r_in_dHL!(
        write_B_in_dHL, B,
        write_C_in_dHL, C,
        write_D_in_dHL, D,
        write_E_in_dHL, E,
        write_H_in_dHL, H,
        write_L_in_dHL, L,
        write_A_in_dHL, A
    );

    write_r_in_dSP!(
        write_B_in_dSP, B,
        write_C_in_dSP, C,
        write_D_in_dSP, D,
        write_E_in_dSP, E,
        write_H_in_dSP, H,
        write_L_in_dSP, L,
        write_A_in_dSP, A
    );

    write_r_with_buffer!(
        write_B_with_buffer_u8, B,
        write_C_with_buffer_u8, C,
        write_D_with_buffer_u8, D,
        write_E_with_buffer_u8, E,
        write_H_with_buffer_u8, H,
        write_L_with_buffer_u8, L,
        write_A_with_buffer_u8, A
    );

    read_bus_with_rr!(
        read_bus_with_HL, HL,
        read_bus_with_BC, BC,
        read_bus_with_DE, DE,
        read_bus_with_SP, SP
    );

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

    ADD_A_r!(
        ADD_A_A, A, false,  ADC_A_A, A, true,
        ADD_A_B, B, false,  ADC_A_B, B, true,
        ADD_A_C, C, false,  ADC_A_C, C, true,
        ADD_A_D, D, false,  ADC_A_D, D, true,
        ADD_A_E, E, false,  ADC_A_E, E, true,
        ADD_A_H, H, false,  ADC_A_H, H, true,
        ADD_A_L, L, false,  ADC_A_L, L, true
    );

    pub fn add_with_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let value = inst.buffer_u8.pop().unwrap();
        Instruction::ADD_u8(registers, value, false)
    }

    pub fn adc_with_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let value = inst.buffer_u8.pop().unwrap();
        Instruction::ADD_u8(registers, value, true)
    }

    SUB_A_r!(
        SUB_A_A, A, false,  SBC_A_A, A, true,
        SUB_A_B, B, false,  SBC_A_B, B, true,
        SUB_A_C, C, false,  SBC_A_C, C, true,
        SUB_A_D, D, false,  SBC_A_D, D, true,
        SUB_A_E, E, false,  SBC_A_E, E, true,
        SUB_A_H, H, false,  SBC_A_H, H, true,
        SUB_A_L, L, false,  SBC_A_L, L, true
    );

    pub fn sub_with_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let value = inst.buffer_u8.pop().unwrap();
        Instruction::SUB_u8(registers, value, false)
    }

    pub fn sbc_with_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let value = inst.buffer_u8.pop().unwrap();
        Instruction::SUB_u8(registers, value, true)
    }

    LOGIC_r!(
        AND_A, AND_u8, A,       OR_A, OR_u8, A,     XOR_A, XOR_u8, A,   CP_A, CP_u8, A,
        AND_B, AND_u8, B,       OR_B, OR_u8, B,     XOR_B, XOR_u8, B,   CP_B, CP_u8, B,
        AND_C, AND_u8, C,       OR_C, OR_u8, C,     XOR_C, XOR_u8, C,   CP_C, CP_u8, C,
        AND_D, AND_u8, D,       OR_D, OR_u8, D,     XOR_D, XOR_u8, D,   CP_D, CP_u8, D,
        AND_E, AND_u8, E,       OR_E, OR_u8, E,     XOR_E, XOR_u8, E,   CP_E, CP_u8, E,
        AND_H, AND_u8, H,       OR_H, OR_u8, H,     XOR_H, XOR_u8, H,   CP_H, CP_u8, H,
        AND_L, AND_u8, L,       OR_L, OR_u8, L,     XOR_L, XOR_u8, L,   CP_L, CP_u8, L
    );

    LOGIC_with_buffer!(and_with_buffer, AND_u8, or_with_buffer, OR_u8, xor_with_buffer, XOR_u8, cp_with_buffer, CP_u8);

    pub fn load_immediate(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let pc: u16 = registers.PC(Action::Read ).value();
        inst.buffer_u8.push( bus.read_byte(pc).value() );
        registers.PC(Action::Write((pc+1) as u16));
    }

    pub fn load_short(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        Self::load_immediate(inst, registers, bus);
        let high = inst.buffer_u8.pop().unwrap();
        let low = inst.buffer_u8.pop().unwrap();

        inst.buffer_u16 = (high as u16) << 8 | low as u16;
    }

    pub fn write_sp_low(inst: &mut Instruction, registers: &mut Registers,  bus: &mut Bus){
        let sp: u16 = registers.SP(Action::Read ).value();
        bus.write_byte(inst.buffer_u16, sp as u8);
    }

    pub fn write_sp_high(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let sp: u16 = registers.SP(Action::Read ).value();
        bus.write_byte(inst.buffer_u16, (sp >> 8) as u8);
    }
    
    pub fn inc_buffer_u16(inst: &mut Instruction, _registers: &mut Registers, _bus: &mut Bus){
        inst.buffer_u16 = inst.buffer_u16.wrapping_add(1);
    }

    pub fn jp_nn(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        registers.PC(Action::Write(inst.buffer_u16));
    }


    pub fn jr_n(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let n = inst.buffer_u8.pop().unwrap() as i8;

        if n >= 0 {
            registers.PC(Action::Increment(n as u16));
        } else {
            registers.PC(Action::Decrement(n.abs() as u16)); 
        }
    }

    pub fn jr_if(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        if inst.flag {
            Self::jr_n(inst, registers, bus);
        }
    }

    pub fn compare_nz(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        if registers.test_flag(ZERO_FLAG) {
            inst.cycles = 0;
            inst.flag = false;
        } else {
            inst.flag = true;
        }
    }

    pub fn compare_z(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        if registers.test_flag(ZERO_FLAG) {
            inst.flag = true;
        } else {
            inst.cycles = 0;
            inst.flag = false;
        }
    }
    
    pub fn compare_nc(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        if registers.test_flag(CARRY_FLAG) {
            inst.cycles = 0;
            inst.flag = false;
        } else {
            inst.flag = true;
        }
    }
    
    pub fn compare_c(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        if registers.test_flag(CARRY_FLAG) {
            inst.flag = true;
        } else {
            inst.cycles = 0;
            inst.flag = false;
        }
    }
    
    write_buffer_to_rr!(
        write_buffer_to_BC, BC,
        write_buffer_to_DE, DE,
        write_buffer_to_HL, HL,
        write_buffer_to_SP, SP
    );

    ADD_HL_rr!(
        add_bc, BC,
        add_de, DE,
        add_hl, HL,
        add_sp, SP
    );

    pub fn ld_dBC_A(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let a: u8 = registers.A( Action::Read ).value();
        let bc: u16 = registers.BC( Action::Read ).value();

        bus.write_byte(bc, a);
    }

    pub fn ld_dDE_A(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let a: u8 = registers.A( Action::Read ).value();
        let de: u16 = registers.DE( Action::Read ).value();

        bus.write_byte(de, a);
    }
    
    INC_rr!(INC_BC, BC, INC_DE, DE, INC_HL, HL, INC_SP, SP);
    DEC_rr!(DEC_BC, BC, DEC_DE, DE, DEC_HL, HL, DEC_SP, SP);
    
    pub fn write_dHL_to_A(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let hl: u16 = registers.HL( Action::Read ).value();
        let val: u8 = bus.read_byte(hl).value();
        registers.A( Action::Write(val as u16 ));
    }

    INC_DEC!(
        INC_A, INC, A,  DEC_A, DEC, A,
        INC_B, INC, B,  DEC_B, DEC, B,
        INC_C, INC, C,  DEC_C, DEC, C,
        INC_D, INC, D,  DEC_D, DEC, D,
        INC_E, INC, E,  DEC_E, DEC, E,
        INC_H, INC, H,  DEC_H, DEC, H,
        INC_L, INC, L,  DEC_L, DEC, L
    );

    pub fn inc_buffer_u8(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let mut val = inst.buffer_u8.pop().unwrap();
        val = Instruction::INC(registers, val);
        bus.write_byte(inst.buffer_u16, val);
    }

    pub fn dec_buffer_u8(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let mut val = inst.buffer_u8.pop().unwrap();
        val = Instruction::DEC(registers, val);
        bus.write_byte(inst.buffer_u16, val);
    }

    pub fn write_buffer_to_dHL(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let val = inst.buffer_u8.pop().unwrap();
        let hl: u16 = registers.HL( Action::Read ).value();
        bus.write_byte(hl, val);
    }

    pub fn RLC_A(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
        let mut A: u8 = registers.A(Action::Read).value();
        A = Instruction::RL(registers, A, false, false);
        registers.A(Action::Write(A as u16));
    }

    pub fn RRC_A(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
        let mut A: u8 = registers.A( Action::Read ).value();
        A = Instruction::RR(registers, A, false, false);
        registers.A( Action::Write(A as u16) );  
    }

    pub fn RL_A(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
        let mut A: u8 = registers.A( Action::Read ).value();
        A = Instruction::RL(registers, A, true, false);
        registers.A( Action::Write(A as u16) );
    }

    pub fn RR_A(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
        let mut A: u8 = registers.A( Action::Read ).value();
        A = Instruction::RR(registers, A, true, false);
        registers.A( Action::Write(A as u16) );
    }

    pub fn DAA(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
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
    }

    pub fn NOT_A(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
        let A: u8 = registers.A(Action::Read).value();
        registers.A( Action::Write(!A as u16) );
        registers.set_flag(HALFCARRY_FLAG);
        registers.set_flag(NEGATIVE_FLAG);
    }

    pub fn SCF(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){  
        registers.set_flag(CARRY_FLAG);
        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);
    }

    pub fn CCF(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
        let bit = registers.test_flag(CARRY_FLAG) as u8;
        if (bit ^ 1) == 1 {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG)
        }
        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);
    }

    pub fn sum_ff00_to_C(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let c: u8 = registers.C( Action::Read ).value();
        inst.buffer_u16 = c as u16 + 0xFF00;
    }

    pub fn sum_ff00_to_b8(inst: &mut Instruction, _registers: &mut Registers, _bus: &mut Bus){
        let val: u8 = inst.buffer_u8.pop().unwrap();
        inst.buffer_u16 = val as u16 + 0xFF00;
    }
    
    pub fn read_b16_write_A(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let val: u8 = bus.read_byte(inst.buffer_u16).value();
        registers.A(Action::Write( val as u16) );
    }
    
    pub fn write_A_to_b16(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let val: u8 =  registers.A(Action::Read ).value();
        bus.write_byte(inst.buffer_u16, val);
    }
    
    pub fn write_F_in_dSP(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let val: u16 =  registers.AF(Action::Read ).value();
        let sp: u16 = registers.SP( Action::Read ).value();

        bus.write_byte(sp, val as u8);
    }

    pub fn write_P_in_dSP(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let val: u16 =  registers.PC(Action::Read ).value();
        let sp: u16 = registers.SP( Action::Read ).value();

        bus.write_byte(sp, (val >> 8) as u8);
    }

    pub fn write_PC_in_dSP(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let val: u16 =  registers.PC(Action::Read ).value();
        let sp: u16 = registers.SP( Action::Read ).value();

        bus.write_byte(sp, val as u8);
    }
    
    pub fn finish_ret(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let sp: u16 = registers.SP( Action::Read ).value();
        inst.buffer_u8.push( bus.read_byte(sp ).value() );
        registers.SP( Action::Increment(1) );
        
        let high = inst.buffer_u8.pop().unwrap();
        let low = inst.buffer_u8.pop().unwrap();
        inst.buffer_u16 = (high as u16) << 8 | low as u16;
        registers.PC( Action::Write( inst.buffer_u16 ));
    }

    pub fn finish_call(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        registers.SP( Action::Decrement(1) );
        let sp: u16 = registers.SP( Action::Read ).value();
        let pc: u16 = registers.PC( Action::Read ).value();

        bus.write_byte(sp, pc as u8);

        registers.PC( Action::Write( inst.buffer_u16 ) );
    }

    pub fn JP_HL(_inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let hl: u16 = registers.HL( Action::Read ).value();
        registers.PC( Action::Write( hl ) ); 
    }

    pub fn finish_pop_B(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        registers.SP( Action::Increment(1) );
        Self::read_bus_with_SP(inst, registers, bus);
        Self::write_B_with_buffer_u8(inst, registers, bus);
        registers.SP( Action::Increment(1) );
    }

    pub fn finish_pop_D(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        registers.SP( Action::Increment(1) );
        Self::read_bus_with_SP(inst, registers, bus);
        Self::write_D_with_buffer_u8(inst, registers, bus);
        registers.SP( Action::Increment(1) );
    }

    pub fn finish_pop_H(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        registers.SP( Action::Increment(1) );
        Self::read_bus_with_SP(inst, registers, bus);
        Self::write_H_with_buffer_u8(inst, registers, bus);
        registers.SP( Action::Increment(1) );
    }

    pub fn finish_pop_A(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        registers.SP( Action::Increment(1) );
        Self::read_bus_with_SP(inst, registers, bus);
        Self::write_A_with_buffer_u8(inst, registers, bus);
        registers.SP( Action::Increment(1) );
    }

    pub fn write_F_with_buffer_u8(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let val = inst.buffer_u8.pop().unwrap();
        registers.AF( Action::Write( (val & 0xF0) as u16 )  );
    }

    pub fn enable_interrupts(_inst: &mut Instruction, _registers: &mut Registers, bus: &mut Bus){
        bus.enable_interrupts();
    }

    pub fn ei(_inst: &mut Instruction, _registers: &mut Registers, bus: &mut Bus){
        bus.interrupts.master = true;
    }

    pub fn disable_interrupts(_inst: &mut Instruction, _registers: &mut Registers, bus: &mut Bus){
        bus.disable_interrupts();
    }

    pub fn ld_sp_hl(_inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let hl: u16 = registers.HL( Action::Read ).value();
        registers.SP(Action::Write(hl) );
    }

    pub fn rst_0(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let sp: u16 = registers.SP( Action::Read ).value();
        let pc: u16 = registers.PC( Action::Read ).value();

        bus.write_byte(sp, pc as u8);
        registers.PC( Action::Write(0) );
    }

    pub fn rst_8(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let sp: u16 = registers.SP( Action::Read ).value();
        let pc: u16 = registers.PC( Action::Read ).value();

        bus.write_byte(sp, pc as u8);
        registers.PC( Action::Write(8) );
    }

    pub fn rst_10(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let sp: u16 = registers.SP( Action::Read ).value();
        let pc: u16 = registers.PC( Action::Read ).value();

        bus.write_byte(sp, pc as u8);
        registers.PC( Action::Write(0x10) );
    }
    pub fn rst_18(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let sp: u16 = registers.SP( Action::Read ).value();
        let pc: u16 = registers.PC( Action::Read ).value();

        bus.write_byte(sp, pc as u8);
        registers.PC( Action::Write(0x18) );
    }
    pub fn rst_20(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let sp: u16 = registers.SP( Action::Read ).value();
        let pc: u16 = registers.PC( Action::Read ).value();

        bus.write_byte(sp, pc as u8);
        registers.PC( Action::Write(0x20) );
    }
    pub fn rst_28(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let sp: u16 = registers.SP( Action::Read ).value();
        let pc: u16 = registers.PC( Action::Read ).value();

        bus.write_byte(sp, pc as u8);
        registers.PC( Action::Write(0x28) );
    }
    pub fn rst_30(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let sp: u16 = registers.SP( Action::Read ).value();
        let pc: u16 = registers.PC( Action::Read ).value();

        bus.write_byte(sp, pc as u8);
        registers.PC( Action::Write(0x30) );
    }

    pub fn rst_38(_inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        let sp: u16 = registers.SP( Action::Read ).value();
        let pc: u16 = registers.PC( Action::Read ).value();

        bus.write_byte(sp, pc as u8);
        registers.PC( Action::Write(0x38) );
    }

    pub fn load_40(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        registers.PC(Action::Write(0x40));
        if bus.halt_cpu {
            inst.operations.push_back(Instruction::unhalt);
            inst.cycles += 1;
        }
    }

    pub fn load_48(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        registers.PC(Action::Write(0x48));
        if bus.halt_cpu {
            inst.operations.push_back(Instruction::unhalt);
            inst.cycles += 1;
        }
    }

    pub fn load_50(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        registers.PC(Action::Write(0x50));
        if bus.halt_cpu {
            inst.operations.push_back(Instruction::unhalt);
            inst.cycles += 1;
        }
    }

    pub fn load_58(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        registers.PC(Action::Write(0x58));
        if bus.halt_cpu {
            inst.operations.push_back(Instruction::unhalt);
            inst.cycles += 1;
        }
    }
    
    pub fn load_60(inst: &mut Instruction, registers: &mut Registers, bus: &mut Bus){
        registers.PC(Action::Write(0x60));
        if bus.halt_cpu {
            inst.operations.push_back(Instruction::unhalt);
            inst.cycles += 1;
        }
    }

    pub fn unhalt(_inst: &mut Instruction, _registers: &mut Registers, bus: &mut Bus){
        bus.halt_cpu = false;
    }




    pub fn add_sp_dd(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let SP: u16 = registers.SP(Action::Read).value();

        let jump = (inst.buffer_u8.pop().unwrap() as i8) as i16;

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
    }



    pub fn ldhl_sp_dd(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let SP: u16 = registers.SP(Action::Read).value();

        let jump = (inst.buffer_u8.pop().unwrap() as i8) as i16;

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
    }

    res_buffer!(
        res_0_buffer, 0,
        res_1_buffer, 1,
        res_2_buffer, 2,
        res_3_buffer, 3,
        res_4_buffer, 4,
        res_5_buffer, 5,
        res_6_buffer, 6,
        res_7_buffer, 7
    );

    set_buffer!(
        set_0_buffer, 0,
        set_1_buffer, 1,
        set_2_buffer, 2,
        set_3_buffer, 3,
        set_4_buffer, 4,
        set_5_buffer, 5,
        set_6_buffer, 6,
        set_7_buffer, 7
    );

    bit_buffer!(
        bit_0_buffer, 0,
        bit_1_buffer, 1,
        bit_2_buffer, 2,
        bit_3_buffer, 3,
        bit_4_buffer, 4,
        bit_5_buffer, 5,
        bit_6_buffer, 6,
        bit_7_buffer, 7
    );

    pub fn rlc_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let mut val = inst.buffer_u8.pop().unwrap();
        val = Instruction::RL(registers, val, false, true);
        inst.buffer_u8.push(val);
    }

    pub fn rl_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let mut val = inst.buffer_u8.pop().unwrap();
        val = Instruction::RL(registers, val, true, true);
        inst.buffer_u8.push(val);
    }

    pub fn rrc_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let mut val = inst.buffer_u8.pop().unwrap();
        val = Instruction::RR(registers, val, false, true);
        inst.buffer_u8.push(val);
    }

    pub fn rr_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let mut val = inst.buffer_u8.pop().unwrap();
        val = Instruction::RR(registers, val, true, true);
        inst.buffer_u8.push(val);
    }

    pub fn sla_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let mut val = inst.buffer_u8.pop().unwrap();
        val = Instruction::SL(val, registers);
        inst.buffer_u8.push(val);
    }

    pub fn sra_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let mut val = inst.buffer_u8.pop().unwrap();
        val = Instruction::SR(val, true,  registers);
        inst.buffer_u8.push(val);
    }

    pub fn srl_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let mut val = inst.buffer_u8.pop().unwrap();
        val = Instruction::SR(val, false,  registers);
        inst.buffer_u8.push(val);
    }

    pub fn swap_buffer(inst: &mut Instruction, registers: &mut Registers, _bus: &mut Bus){
        let mut val = inst.buffer_u8.pop().unwrap();
        val = Instruction::SWAP(val, registers);
        inst.buffer_u8.push(val);
    }

    pub fn write_b8_to_b16(inst: &mut Instruction, _registers: &mut Registers, bus: &mut Bus){
        bus.write_byte(inst.buffer_u16, inst.buffer_u8.pop().unwrap());
    }

}

#[macro_export]
macro_rules! atomic {
    ( $name:expr,$func:ident ) => {
        Instruction::new(
            $name,
            {
                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                o.push_back(Instruction::$func);
                o
            },
            1
        )
    }
}

#[macro_export]
macro_rules! subset_atomic {
    ( $name:expr,$func:ident ) => {
        Instruction::new(
            $name,
            {
                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                o.push_back(Instruction::nop);
                o.push_back(Instruction::$func);
                o
            },
            2
        )
    }
}
