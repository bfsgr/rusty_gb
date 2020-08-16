#![allow(non_snake_case)]
use super::Instruction;
use crate::emulator::cpu::registers::*;
use crate::emulator::Bus;
use crate::emulator::bit_utils::BitUtils;

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

macro_rules! res_nr {
    ( $( $name:ident,$num:expr,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus) {
                let mut val: u8 = registers.$r( Action::Read ).value();

                val.reset_bit($num);
        
                registers.$r( Action::Write( val as u16 ) );
            }
        )*
    }
}

macro_rules! set_nr {
    ( $( $name:ident,$num:expr,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus) {
                let mut val: u8 = registers.$r( Action::Read ).value();

                val.set_bit($num);
        
                registers.$r( Action::Write( val as u16 ) );
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

    res_nr!(
        RES_0B, 0, B,  RES_0C, 0, C, RES_0D, 0, D,  RES_0E, 0, E, RES_0H, 0, H,  RES_0L, 0, L, RES_0A, 0, A,
        RES_1B, 1, B,  RES_1C, 1, C, RES_1D, 1, D,  RES_1E, 1, E, RES_1H, 1, H,  RES_1L, 1, L, RES_1A, 1, A,
        RES_2B, 2, B,  RES_2C, 2, C, RES_2D, 2, D,  RES_2E, 2, E, RES_2H, 2, H,  RES_2L, 2, L, RES_2A, 2, A,
        RES_3B, 3, B,  RES_3C, 3, C, RES_3D, 3, D,  RES_3E, 3, E, RES_3H, 3, H,  RES_3L, 3, L, RES_3A, 3, A,
        RES_4B, 4, B,  RES_4C, 4, C, RES_4D, 4, D,  RES_4E, 4, E, RES_4H, 4, H,  RES_4L, 4, L, RES_4A, 4, A,
        RES_5B, 5, B,  RES_5C, 5, C, RES_5D, 5, D,  RES_5E, 5, E, RES_5H, 5, H,  RES_5L, 5, L, RES_5A, 5, A,
        RES_6B, 6, B,  RES_6C, 6, C, RES_6D, 6, D,  RES_6E, 6, E, RES_6H, 6, H,  RES_6L, 6, L, RES_6A, 6, A,
        RES_7B, 7, B,  RES_7C, 7, C, RES_7D, 7, D,  RES_7E, 7, E, RES_7H, 7, H,  RES_7L, 7, L, RES_7A, 7, A
    );

    set_nr!(
        SET_0B, 0, B,  SET_0C, 0, C, SET_0D, 0, D,  SET_0E, 0, E, SET_0H, 0, H,  SET_0L, 0, L, SET_0A, 0, A,
        SET_1B, 1, B,  SET_1C, 1, C, SET_1D, 1, D,  SET_1E, 1, E, SET_1H, 1, H,  SET_1L, 1, L, SET_1A, 1, A,
        SET_2B, 2, B,  SET_2C, 2, C, SET_2D, 2, D,  SET_2E, 2, E, SET_2H, 2, H,  SET_2L, 2, L, SET_2A, 2, A,
        SET_3B, 3, B,  SET_3C, 3, C, SET_3D, 3, D,  SET_3E, 3, E, SET_3H, 3, H,  SET_3L, 3, L, SET_3A, 3, A,
        SET_4B, 4, B,  SET_4C, 4, C, SET_4D, 4, D,  SET_4E, 4, E, SET_4H, 4, H,  SET_4L, 4, L, SET_4A, 4, A,
        SET_5B, 5, B,  SET_5C, 5, C, SET_5D, 5, D,  SET_5E, 5, E, SET_5H, 5, H,  SET_5L, 5, L, SET_5A, 5, A,
        SET_6B, 6, B,  SET_6C, 6, C, SET_6D, 6, D,  SET_6E, 6, E, SET_6H, 6, H,  SET_6L, 6, L, SET_6A, 6, A,
        SET_7B, 7, B,  SET_7C, 7, C, SET_7D, 7, D,  SET_7E, 7, E, SET_7H, 7, H,  SET_7L, 7, L, SET_7A, 7, A
    );


}