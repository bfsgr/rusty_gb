#![allow(non_snake_case)]

use crate::emulator::cpu::registers::*;
use super::instructions::*;
use crate::emulator::bit_utils::BitUtils;

pub const ZERO_FLAG: u8 = 0;
pub const NEGATIVE_FLAG: u8 = 1;
pub const HALFCARRY_FLAG: u8 = 2;
pub const CARRY_FLAG: u8 = 3;

impl Instruction {
//Generic functions

    //Generic increment function for 8 bit registers. 
    pub fn INC(registers: &mut Registers, mut val: u8) -> u8{
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
    pub fn DEC(registers: &mut Registers, mut val: u8) -> u8{
        //will lower nibble overflow?
        if (val & 0x0F) == 0x00 {
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

    pub fn ADD_u8(registers: &mut Registers, X: u8, use_carry: bool) {

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

    pub fn SUB_u8(registers: &mut Registers, X: u8, use_carry: bool) {

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

    pub fn AND_u8(registers: &mut Registers, X: u8) {
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

    pub fn OR_u8(registers: &mut Registers, X: u8) {
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

    pub fn XOR_u8(registers: &mut Registers, X: u8) {
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

    pub fn CP_u8(registers: &mut Registers, X: u8) {
		let A: u8 = registers.A( Action::Read ).value();
		Instruction::SUB_u8(registers, X, false);
        registers.A( Action::Write(A as u16) );
    }

    pub fn ADD_u16(registers: &mut Registers, X: u16) -> u16 {
        
        let HL: u16 = registers.HL( Action::Read ).value();
        
        let value: u16 = HL.wrapping_add(X);

        let hc = (((HL & 0xFFF) + (X & 0xFFF)) & 0x1000) != 0;

        if hc {
            registers.set_flag(HALFCARRY_FLAG);
        } else {
            registers.clear_flag(HALFCARRY_FLAG);
        }

        if HL > 0xFFFF - X {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        registers.clear_flag(NEGATIVE_FLAG);

        return value;
    }

    pub fn RR(registers: &mut Registers, mut value: u8, carry: bool, zflag_on: bool) -> u8{

        let bit = value.test_bit(0);

        value = match carry {
            true => (value >> 1) | ((registers.test_flag(CARRY_FLAG) as u8) << 7), //>
            false => value.rotate_right(1)
        };

        if bit {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);

        if value == 0 && zflag_on {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        return value;
    }

    pub fn RL(registers: &mut Registers, mut value: u8, carry: bool, zflag_on: bool) -> u8{

        let bit = value.test_bit(7);

        value = match carry {
            true => (value << 1) | registers.test_flag(CARRY_FLAG) as u8, //>
            false => value.rotate_left(1)
        };

        if bit {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);

        if value == 0 && zflag_on {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        return value;
    }

    pub fn SR(byte: u8, preserve: bool, registers: &mut Registers) -> u8 {
		let shifted = match preserve {
			true => (byte >> 1) | (byte & 0x80),
			false => byte >> 1,
        };
        
        if byte.test_bit(0) {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        if shifted == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);
        
        shifted
	}

    pub fn SL(byte: u8, registers: &mut Registers) -> u8 {
		let shifted = byte << 1; //>

        
        if byte.test_bit(7) {
            registers.set_flag(CARRY_FLAG);
        } else {
            registers.clear_flag(CARRY_FLAG);
        }

        if shifted == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);
        
        shifted
	}

    pub fn BIT(value: u8, bit: u8, registers: &mut Registers) {
        if value.test_bit(bit) {
            registers.clear_flag(ZERO_FLAG);
        } else {
            registers.set_flag(ZERO_FLAG);
        }

        registers.set_flag(HALFCARRY_FLAG);
        registers.clear_flag(NEGATIVE_FLAG);
    }

    pub fn SWAP(value: u8, registers: &mut Registers) -> u8 {

        if value == 0 {
            registers.set_flag(ZERO_FLAG);
        } else {
            registers.clear_flag(ZERO_FLAG);
        }

        let upper = value & 0xF0;
        let lower = value & 0x0F;

        registers.clear_flag(NEGATIVE_FLAG);
        registers.clear_flag(HALFCARRY_FLAG);
        registers.clear_flag(CARRY_FLAG);

        lower << 4 | upper >> 4
    }
}