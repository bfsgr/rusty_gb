#![allow(non_snake_case)]
use super::Instruction;
use crate::emulator::cpu::registers::*;
use crate::emulator::Bus;
// use crate::emulator::bit_utils::BitUtils;

macro_rules! RL_r{
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus) {
                let mut val: u8 = registers.$r( Action::Read ).value();

                val = Instruction::RL(registers, val, true, true);
        
                registers.$r( Action::Write(val as u16) );
            }
        )*
    }
}

macro_rules! RLC_r{
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus) {
                let mut val: u8 = registers.$r( Action::Read ).value();

                val = Instruction::RL(registers, val, false, true);
        
                registers.$r( Action::Write(val as u16) );
            }
        )*
    }
}

macro_rules! RRC_r{
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
                let mut val: u8 = registers.$r( Action::Read ).value();
        
                val = Instruction::RR(registers, val, false, true);
        
                registers.$r( Action::Write(val as u16) );
        
            }
        )*
    }
}

macro_rules! RR_r{
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
                let mut val: u8 = registers.$r( Action::Read ).value();
        
                val = Instruction::RR(registers, val, true, true);
        
                registers.$r( Action::Write(val as u16) );
        
            }
        )*
    }
}

macro_rules! SLA_r{
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
                let mut val: u8 = registers.$r( Action::Read ).value();
        
                val = Instruction::SL(val, registers);
        
                registers.$r( Action::Write(val as u16) );
        
            }
        )*
    }
}

macro_rules! SRA_r{
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
                let mut val: u8 = registers.$r( Action::Read ).value();
        
                val = Instruction::SR(val, true, registers);
        
                registers.$r( Action::Write(val as u16) );
        
            }
        )*
    }
}

macro_rules! SRL_r{
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
                let mut val: u8 = registers.$r( Action::Read ).value();
        
                val = Instruction::SR(val, false, registers);
        
                registers.$r( Action::Write(val as u16) );
        
            }
        )*
    }
}

macro_rules! SWAP_r{
    ( $( $name:ident,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus){
                let mut val: u8 = registers.$r( Action::Read ).value();
                
                val = Instruction::SWAP(val, registers);
                
                registers.$r( Action::Write(val as u16) );
        
            }
        )*
    }
}

macro_rules! bit_nr {
    ( $( $name:ident,$num:expr,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus) {
                let val: u8 = registers.$r( Action::Read ).value();
                Instruction::BIT(val, $num, registers);
            }
        )*
    }
}

impl Instruction {
    RRC_r!( RRC_B, B, RRC_C, C, RRC_D, D, RRC_E, E, RRC_H, H, RRC_L, L, RRC_A_CB, A);
    RR_r!( RR_B, B, RR_C, C, RR_D, D, RR_E, E, RR_H, H, RR_L, L, RR_A_CB, A);

    RLC_r!( RLC_B, B, RLC_C, C, RLC_D, D, RLC_E, E, RLC_H, H, RLC_L, L, RLC_A_CB, A);
    RL_r!( RL_B, B, RL_C, C, RL_D, D, RL_E, E, RL_H, H, RL_L, L, RL_A_CB, A);

    SLA_r!( SLA_B, B, SLA_C, C, SLA_D, D, SLA_E, E, SLA_H, H, SLA_L, L, SLA_A, A );
    SRA_r!( SRA_B, B, SRA_C, C, SRA_D, D, SRA_E, E, SRA_H, H, SRA_L, L, SRA_A, A );
    SRL_r!( SRL_B, B, SRL_C, C, SRL_D, D, SRL_E, E, SRL_H, H, SRL_L, L, SRL_A, A );

    SWAP_r!( SWAP_B, B, SWAP_C, C, SWAP_D, D, SWAP_E, E, SWAP_H, H, SWAP_L, L, SWAP_A, A);

    bit_nr!(
        BIT_0B, 0, B,  BIT_0C, 0, C, BIT_0D, 0, D,  BIT_0E, 0, E, BIT_0H, 0, H,  BIT_0L, 0, L, BIT_0A, 0, A,
        BIT_1B, 1, B,  BIT_1C, 1, C, BIT_1D, 1, D,  BIT_1E, 1, E, BIT_1H, 1, H,  BIT_1L, 1, L, BIT_1A, 1, A,
        BIT_2B, 2, B,  BIT_2C, 2, C, BIT_2D, 2, D,  BIT_2E, 2, E, BIT_2H, 2, H,  BIT_2L, 2, L, BIT_2A, 2, A,
        BIT_3B, 3, B,  BIT_3C, 3, C, BIT_3D, 3, D,  BIT_3E, 3, E, BIT_3H, 3, H,  BIT_3L, 3, L, BIT_3A, 3, A,
        BIT_4B, 4, B,  BIT_4C, 4, C, BIT_4D, 4, D,  BIT_4E, 4, E, BIT_4H, 4, H,  BIT_4L, 4, L, BIT_4A, 4, A,
        BIT_5B, 5, B,  BIT_5C, 5, C, BIT_5D, 5, D,  BIT_5E, 5, E, BIT_5H, 5, H,  BIT_5L, 5, L, BIT_5A, 5, A,
        BIT_6B, 6, B,  BIT_6C, 6, C, BIT_6D, 6, D,  BIT_6E, 6, E, BIT_6H, 6, H,  BIT_6L, 6, L, BIT_6A, 6, A,
        BIT_7B, 7, B,  BIT_7C, 7, C, BIT_7D, 7, D,  BIT_7E, 7, E, BIT_7H, 7, H,  BIT_7L, 7, L, BIT_7A, 7, A
    );
}