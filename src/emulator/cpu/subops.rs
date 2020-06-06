#![allow(non_snake_case)]

use crate::emulator::cpu::{*};
use crate::emulator::bus::{Bus};
use crate::emulator::cpu::{Instruction};

impl Instruction {
    
    //0xCB 0x00
    pub fn RLC_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut B: u8 = registers.B( Action::Read ).value();

        B = Instruction::RL(registers, B, false, true);

        registers.B( Action::Write(B as u16) );
    }

    //0xCB 0x01
    pub fn RLC_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut C: u8 = registers.C( Action::Read ).value();

        C = Instruction::RL(registers, C, false, true);

        registers.C( Action::Write(C as u16) );
    }

    //0xCB 0x02
    pub fn RLC_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut D: u8 = registers.D( Action::Read ).value();

        D = Instruction::RL(registers, D, false, true);

        registers.D( Action::Write(D as u16) );
    }

    //0xCB 0x03
    pub fn RLC_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut E: u8 = registers.E( Action::Read ).value();

        E = Instruction::RL(registers, E, false, true);

        registers.E( Action::Write(E as u16) );
    }

    //0xCB 0x04
    pub fn RLC_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut H: u8 = registers.H( Action::Read ).value();

        H = Instruction::RL(registers, H, false, true);

        registers.H( Action::Write(H as u16) );
    }

    //0xCB 0x05
    pub fn RLC_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut L: u8 = registers.L( Action::Read ).value();

        L = Instruction::RL(registers, L, false, true);

        registers.L( Action::Write(L as u16) );
    }

    //0xCB 0x06
    pub fn RLC_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let mut val = mem.read_byte(dHL).value();

        val = Instruction::RL(registers, val, false, true);

        mem.write_byte(dHL, val);
    }

    //0xCB 0x07
    pub fn RLC_A_CB(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut A: u8 = registers.A( Action::Read ).value();

        A = Instruction::RL(registers, A, false, true);

        registers.A( Action::Write(A as u16) );
    }

    //0xCB 0x08
    pub fn RRC_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.B( Action::Read ).value();

        val = Instruction::RR(registers, val, false, true);

        registers.B( Action::Write(val as u16) );
    }

    //0xCB 0x09
    pub fn RRC_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.C( Action::Read ).value();

        val = Instruction::RR(registers, val, false, true);

        registers.C( Action::Write(val as u16) );
    }

    //0xCB 0x0A
    pub fn RRC_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.D( Action::Read ).value();

        val = Instruction::RR(registers, val, false, true);

        registers.D( Action::Write(val as u16) );
    }

    //0xCB 0x0B
    pub fn RRC_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.E( Action::Read ).value();

        val = Instruction::RR(registers, val, false, true);

        registers.E( Action::Write(val as u16) );
    }
    //0xCB 0x0C
    pub fn RRC_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.H( Action::Read ).value();

        val = Instruction::RR(registers, val, false, true);

        registers.H( Action::Write(val as u16) );
    }
    //0xCB 0x0D
    pub fn RRC_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.L( Action::Read ).value();

        val = Instruction::RR(registers, val, false, true);

        registers.L( Action::Write(val as u16) );
    }

    //0xCB 0x0E
    pub fn RRC_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let mut val = mem.read_byte(dHL).value();

        val = Instruction::RR(registers, val, false, true);

        mem.write_byte(dHL, val);
    }

    //0xCB 0x0F
    pub fn RRC_A_CB(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A = Instruction::RR(registers, A, false, true);

        registers.A( Action::Write(A as u16) );  
    }

    //0xCB 0x10
    pub fn RL_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.B( Action::Read ).value();

        val = Instruction::RL(registers, val, true, true);

        registers.B( Action::Write(val as u16) );
    }

    //0xCB 0x11
    pub fn RL_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut C: u8 = registers.C( Action::Read ).value();

        C = Instruction::RL(registers, C, true, true);

        registers.C( Action::Write(C as u16) );
    }

    //0xCB 0x12
    pub fn RL_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.D( Action::Read ).value();

        val = Instruction::RL(registers, val, true, true);

        registers.D( Action::Write(val as u16) );
    }

    //0xCB 0x13
    pub fn RL_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.E( Action::Read ).value();

        val = Instruction::RL(registers, val, true, true);

        registers.E( Action::Write(val as u16) );
    }

    //0xCB 0x14
    pub fn RL_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.H( Action::Read ).value();

        val = Instruction::RL(registers, val, true, true);

        registers.H( Action::Write(val as u16) );
    }

    //0xCB 0x15
    pub fn RL_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.L( Action::Read ).value();

        val = Instruction::RL(registers, val, true, true);

        registers.L( Action::Write(val as u16) );
    }

    //0xCB 0x16
    pub fn RL_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let mut val = mem.read_byte(dHL).value();

        val = Instruction::RL(registers, val, true, true);

        mem.write_byte(dHL, val);
    }

    //0xCB 0x17
    pub fn RL_A_CB(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.A( Action::Read ).value();

        val = Instruction::RL(registers, val, true, true);

        registers.A( Action::Write(val as u16) );
    }

    //0xCB 0x18
    pub fn RR_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.B( Action::Read ).value();

        val = Instruction::RR(registers, val, true, true);

        registers.B( Action::Write(val as u16) );
    }

    //0xCB 0x19
    pub fn RR_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.C( Action::Read ).value();

        val = Instruction::RR(registers, val, true, true);

        registers.C( Action::Write(val as u16) );
    }

    //0xCB 0x1A
    pub fn RR_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.D( Action::Read ).value();

        val = Instruction::RR(registers, val, true, true);

        registers.D( Action::Write(val as u16) );
    }

    //0xCB 0x1B
    pub fn RR_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.E( Action::Read ).value();

        val = Instruction::RR(registers, val, true, true);

        registers.E( Action::Write(val as u16) );
    }

    //0xCB 0x1C
    pub fn RR_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.H( Action::Read ).value();

        val = Instruction::RR(registers, val, true, true);

        registers.H( Action::Write(val as u16) );
    }

    //0xCB 0x1D
    pub fn RR_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.L( Action::Read ).value();

        val = Instruction::RR(registers, val, true, true);

        registers.L( Action::Write(val as u16) );
    }

    //0xCB 0x1E
    pub fn RR_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let mut val = mem.read_byte(dHL).value();

        val = Instruction::RR(registers, val, true, true);

        mem.write_byte(dHL, val);
    }

    //0xCB 0x1F
    pub fn RR_A_CB(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A = Instruction::RR(registers, A, true, true);

        registers.A( Action::Write(A as u16) );
    }

    //0xCB 0x21
    pub fn SLA_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.C( Action::Read ).value();


        val = Instruction::SL(val, true, registers);

        registers.C( Action::Write(val as u16) );
    }





    //0xCB 0x87
    pub fn RES_0_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(0);

        registers.A( Action::Write( A as u16 ) );
    }
    //0xCB 0x8F
    pub fn RES_1_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(1);

        registers.A( Action::Write( A as u16 ) );
    }
    //0xCB 0x97
    pub fn RES_2_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(2);

        registers.A( Action::Write( A as u16 ) );
    }
    //0xCB 0x9F
    pub fn RES_3_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(3);

        registers.A( Action::Write( A as u16 ) );
    }
    //0xCB 0xA7
    pub fn RES_4_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(4);

        registers.A( Action::Write( A as u16 ) );
    }
    //0xCB 0xAF
    pub fn RES_5_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(5);

        registers.A( Action::Write( A as u16 ) );
    }
    //0xCB 0xB7
    pub fn RES_6_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(6);

        registers.A( Action::Write( A as u16 ) );
    }
    //0xCB 0xBF
    pub fn RES_7_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(7);

        registers.A( Action::Write( A as u16 ) );
    }

    //0xCB 0x37
    pub fn SWAP_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.A( Action::Read ).value();
        
        val = Instruction::swap(val, registers);
        
        registers.A( Action::Write(val as u16) );
    }

    //0xCB 0x38
    pub fn SRL_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.B( Action::Read ).value();
        
        val = Instruction::SR(val, false, registers);
        
        registers.B( Action::Write(val as u16) );
    }

    //0xCB 0x3F
    pub fn SRL_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.A( Action::Read ).value();
        
        val = Instruction::SR(val, false, registers);
        
        registers.A( Action::Write(val as u16) );
    }

    //0xCB 0x40
    pub fn BIT_0B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::test_bit(val, 0, registers);
    }

    //0xCB 0x41
    pub fn BIT_0C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::test_bit(val, 0, registers);
    }

    //0xCB 0x42
    pub fn BIT_0D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::test_bit(val, 0, registers);
    }

    //0xCB 0x43
    pub fn BIT_0E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::test_bit(val, 0, registers);
    }

    //0xCB 0x44
    pub fn BIT_0H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::test_bit(val, 0, registers);
    }

    //0xCB 0x45
    pub fn BIT_0L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::test_bit(val, 0, registers);
    }

    //0xCB 0x46
    pub fn BIT_0dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::test_bit(val, 0, registers);
    }

    //0xCB 0x47
    pub fn BIT_0A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::test_bit(val, 0, registers);
    }

    //0xCB 0x48
    pub fn BIT_1B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::test_bit(val, 1, registers);
    }

    //0xCB 0x49
    pub fn BIT_1C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::test_bit(val, 1, registers);
    }

    //0xCB 0x4A
    pub fn BIT_1D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::test_bit(val, 1, registers);
    }

    //0xCB 0x4B
    pub fn BIT_1E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::test_bit(val, 1, registers);
    }

    //0xCB 0x4C
    pub fn BIT_1H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::test_bit(val, 1, registers);
    }

    //0xCB 0x4D
    pub fn BIT_1L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::test_bit(val, 1, registers);
    }

    //0xCB 0x4E
    pub fn BIT_1dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::test_bit(val, 1, registers);
    }

    //0xCB 0x4F
    pub fn BIT_1A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::test_bit(val, 1, registers);
    }

    //0xCB 0x50
    pub fn BIT_2B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::test_bit(val, 2, registers);
    }

    //0xCB 0x51
    pub fn BIT_2C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::test_bit(val, 2, registers);
    }

    //0xCB 0x52
    pub fn BIT_2D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::test_bit(val, 2, registers);
    }

    //0xCB 0x53
    pub fn BIT_2E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::test_bit(val, 2, registers);
    }

    //0xCB 0x54
    pub fn BIT_2H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::test_bit(val, 2, registers);
    }

    //0xCB 0x55
    pub fn BIT_2L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::test_bit(val, 2, registers);
    }

    //0xCB 0x56
    pub fn BIT_2dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::test_bit(val, 2, registers);
    }

    //0xCB 0x57
    pub fn BIT_2A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::test_bit(val, 2, registers);
    }

    //0xCB 0x58
    pub fn BIT_3B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::test_bit(val, 3, registers);
    }

    //0xCB 0x59
    pub fn BIT_3C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::test_bit(val, 3, registers);
    }

    //0xCB 0x5A
    pub fn BIT_3D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::test_bit(val, 3, registers);
    }

    //0xCB 0x5B
    pub fn BIT_3E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::test_bit(val, 3, registers);
    }

    //0xCB 0x5C
    pub fn BIT_3H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::test_bit(val, 3, registers);
    }

    //0xCB 0x5D
    pub fn BIT_3L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::test_bit(val, 3, registers);
    }

    //0xCB 0x5E
    pub fn BIT_3dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::test_bit(val, 3, registers);
    }

    //0xCB 0x5F
    pub fn BIT_3A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::test_bit(val, 3, registers);
    }

    //0xCB 0x60
    pub fn BIT_4B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::test_bit(val, 4, registers);
    }

    //0xCB 0x61
    pub fn BIT_4C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::test_bit(val, 4, registers);
    }

    //0xCB 0x62
    pub fn BIT_4D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::test_bit(val, 4, registers);
    }

    //0xCB 0x63
    pub fn BIT_4E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::test_bit(val, 4, registers);
    }

    //0xCB 0x64
    pub fn BIT_4H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::test_bit(val, 4, registers);
    }

    //0xCB 0x65
    pub fn BIT_4L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::test_bit(val, 4, registers);
    }

    //0xCB 0x66
    pub fn BIT_4dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::test_bit(val, 4, registers);
    }

    //0xCB 0x67
    pub fn BIT_4A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::test_bit(val, 4, registers);
    }

    //0xCB 0x68
    pub fn BIT_5B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::test_bit(val, 5, registers);
    }

    //0xCB 0x69
    pub fn BIT_5C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::test_bit(val, 5, registers);
    }

    //0xCB 0x6A
    pub fn BIT_5D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::test_bit(val, 5, registers);
    }

    //0xCB 0x6B
    pub fn BIT_5E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::test_bit(val, 5, registers);
    }

    //0xCB 0x6C
    pub fn BIT_5H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::test_bit(val, 5, registers);
    }

    //0xCB 0x6D
    pub fn BIT_5L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::test_bit(val, 5, registers);
    
    }

    //0xCB 0x6E
    pub fn BIT_5dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::test_bit(val, 5, registers);
    }

    //0xCB 0x6F
    pub fn BIT_5A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::test_bit(val, 5, registers);
    
    }

    //0xCB 0x70
    pub fn BIT_6B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::test_bit(val, 6, registers);
    }

    //0xCB 0x71
    pub fn BIT_6C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::test_bit(val, 6, registers);
    }

    //0xCB 0x72
    pub fn BIT_6D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::test_bit(val, 6, registers);
    }

    //0xCB 0x73
    pub fn BIT_6E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::test_bit(val, 6, registers);
    }

    //0xCB 0x74
    pub fn BIT_6H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::test_bit(val, 6, registers);
    }

    //0xCB 0x75
    pub fn BIT_6L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::test_bit(val, 6, registers);
    }

    //0xCB 0x76
    pub fn BIT_6dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::test_bit(val, 6, registers);
    }

    //0xCB 0x77
    pub fn BIT_6A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::test_bit(val, 6, registers);
    }

    //0xCB 0x78
    pub fn BIT_7B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::test_bit(val, 7, registers);
    }

    //0xCB 0x79
    pub fn BIT_7C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::test_bit(val, 7, registers);
    }

    //0xCB 0x7A
    pub fn BIT_7D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::test_bit(val, 7, registers);
    }

    //0xCB 0x7B
    pub fn BIT_7E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::test_bit(val, 7, registers);
    }

    //0xCB 0x7C
    pub fn BIT_7H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::test_bit(val, 7, registers);
    }

    //0xCB 0x7D
    pub fn BIT_7L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::test_bit(val, 7, registers);
    }

    //0xCB 0x7E
    pub fn BIT_7dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::test_bit(val, 7, registers);
    }

    //0xCB 0x7F
    pub fn BIT_7A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::test_bit(val, 7, registers);
    }

    //0xCB 0xC7
    pub fn SET_0A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.A( Action::Read ).value();
        
        val.set_bit(0);

        registers.A( Action::Write(val as u16) );
    }

    //0xCB 0xF2
    pub fn SET_6D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.D( Action::Read ).value();
        
        val.set_bit(6);

        registers.D( Action::Write(val as u16) );
    }

}