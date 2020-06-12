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

    //0xCB 0x20
    pub fn SLA_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.B( Action::Read ).value();


        val = Instruction::SL(val, registers);

        registers.B( Action::Write(val as u16) );
    }

    //0xCB 0x21
    pub fn SLA_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.C( Action::Read ).value();


        val = Instruction::SL(val, registers);

        registers.C( Action::Write(val as u16) );
    }

    //0xCB 0x22
    pub fn SLA_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.D( Action::Read ).value();


        val = Instruction::SL(val, registers);

        registers.D( Action::Write(val as u16) );
    }

    //0xCB 0x23
    pub fn SLA_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.E( Action::Read ).value();


        val = Instruction::SL(val, registers);

        registers.E( Action::Write(val as u16) );
    }

    //0xCB 0x24
    pub fn SLA_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.H( Action::Read ).value();


        val = Instruction::SL(val, registers);

        registers.H( Action::Write(val as u16) );
    }

    //0xCB 0x25
    pub fn SLA_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.L( Action::Read ).value();


        val = Instruction::SL(val, registers);

        registers.L( Action::Write(val as u16) );
    }

    //0xCB 0x26
    pub fn SLA_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();

        val = Instruction::SL(val, registers);

        mem.write_byte(HL, val);
    }

    //0xCB 0x27
    pub fn SLA_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.A( Action::Read ).value();


        val = Instruction::SL(val, registers);

        registers.A( Action::Write(val as u16) );
    }

    //0xCB 0x28
    pub fn SRA_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.B( Action::Read ).value();


        val = Instruction::SR(val, true, registers);

        registers.B( Action::Write(val as u16) );
    }

    //0xCB 0x29
    pub fn SRA_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.C( Action::Read ).value();


        val = Instruction::SR(val, true, registers);

        registers.C( Action::Write(val as u16) );
    }

    //0xCB 0x2A
    pub fn SRA_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.D( Action::Read ).value();


        val = Instruction::SR(val, true, registers);

        registers.D( Action::Write(val as u16) );
    }

    //0xCB 0x2B
    pub fn SRA_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.E( Action::Read ).value();


        val = Instruction::SR(val, true, registers);

        registers.E( Action::Write(val as u16) );
    }

    //0xCB 0x2C
    pub fn SRA_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.H( Action::Read ).value();


        val = Instruction::SR(val, true, registers);

        registers.H( Action::Write(val as u16) );
    }

    //0xCB 0x2D
    pub fn SRA_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.L( Action::Read ).value();


        val = Instruction::SR(val, true, registers);

        registers.L( Action::Write(val as u16) );
    }

    //0xCB 0x2E
    pub fn SRA_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();

        val = Instruction::SR(val, true, registers);

        mem.write_byte(HL, val);
    }

    //0xCB 0x2F
    pub fn SRA_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.A( Action::Read ).value();


        val = Instruction::SR(val, true, registers);

        registers.A( Action::Write(val as u16) );
    }

    //0xCB 0x30
    pub fn SWAP_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.B( Action::Read ).value();
        
        val = Instruction::SWAP(val, registers);
        
        registers.B( Action::Write(val as u16) );
    }

    //0xCB 0x31
    pub fn SWAP_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.C( Action::Read ).value();
        
        val = Instruction::SWAP(val, registers);
        
        registers.C( Action::Write(val as u16) );
    }

    //0xCB 0x32
    pub fn SWAP_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.D( Action::Read ).value();
        
        val = Instruction::SWAP(val, registers);
        
        registers.D( Action::Write(val as u16) );
    }

    //0xCB 0x33
    pub fn SWAP_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.E( Action::Read ).value();
        
        val = Instruction::SWAP(val, registers);
        
        registers.E( Action::Write(val as u16) );
    }

    //0xCB 0x34
    pub fn SWAP_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.H( Action::Read ).value();
        
        val = Instruction::SWAP(val, registers);
        
        registers.H( Action::Write(val as u16) );
    }

    //0xCB 0x35
    pub fn SWAP_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.L( Action::Read ).value();
        
        val = Instruction::SWAP(val, registers);
        
        registers.L( Action::Write(val as u16) );
    }

    //0xCB 0x36
    pub fn SWAP_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();
        
        val = Instruction::SWAP(val, registers);
        
        mem.write_byte(HL, val);
    }
    
    //0xCB 0x37
    pub fn SWAP_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.A( Action::Read ).value();
        
        val = Instruction::SWAP(val, registers);
        
        registers.A( Action::Write(val as u16) );
    }

    //0xCB 0x38
    pub fn SRL_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.B( Action::Read ).value();
        
        val = Instruction::SR(val, false, registers);
        
        registers.B( Action::Write(val as u16) );
    }

    //0xCB 0x39
    pub fn SRL_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.C( Action::Read ).value();
        
        val = Instruction::SR(val, false, registers);
        
        registers.C( Action::Write(val as u16) );
    }

    //0xCB 0x3A
    pub fn SRL_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.D( Action::Read ).value();
        
        val = Instruction::SR(val, false, registers);
        
        registers.D( Action::Write(val as u16) );
    }

    //0xCB 0x3B
    pub fn SRL_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.E( Action::Read ).value();
        
        val = Instruction::SR(val, false, registers);
        
        registers.E( Action::Write(val as u16) );
    }

    //0xCB 0x3C
    pub fn SRL_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.H( Action::Read ).value();
        
        val = Instruction::SR(val, false, registers);
        
        registers.H( Action::Write(val as u16) );
    }

    //0xCB 0x3D
    pub fn SRL_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let mut val: u8 = registers.L( Action::Read ).value();
        
        val = Instruction::SR(val, false, registers);
        
        registers.L( Action::Write(val as u16) );
    }

    //0xCB 0x3E
    pub fn SRL_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();
        
        val = Instruction::SR(val, false, registers);
        
        mem.write_byte(HL, val);
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
        Instruction::BIT(val, 0, registers);
    }

    //0xCB 0x41
    pub fn BIT_0C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::BIT(val, 0, registers);
    }

    //0xCB 0x42
    pub fn BIT_0D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::BIT(val, 0, registers);
    }

    //0xCB 0x43
    pub fn BIT_0E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::BIT(val, 0, registers);
    }

    //0xCB 0x44
    pub fn BIT_0H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::BIT(val, 0, registers);
    }

    //0xCB 0x45
    pub fn BIT_0L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::BIT(val, 0, registers);
    }

    //0xCB 0x46
    pub fn BIT_0dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::BIT(val, 0, registers);
    }

    //0xCB 0x47
    pub fn BIT_0A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::BIT(val, 0, registers);
    }

    //0xCB 0x48
    pub fn BIT_1B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::BIT(val, 1, registers);
    }

    //0xCB 0x49
    pub fn BIT_1C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::BIT(val, 1, registers);
    }

    //0xCB 0x4A
    pub fn BIT_1D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::BIT(val, 1, registers);
    }

    //0xCB 0x4B
    pub fn BIT_1E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::BIT(val, 1, registers);
    }

    //0xCB 0x4C
    pub fn BIT_1H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::BIT(val, 1, registers);
    }

    //0xCB 0x4D
    pub fn BIT_1L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::BIT(val, 1, registers);
    }

    //0xCB 0x4E
    pub fn BIT_1dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::BIT(val, 1, registers);
    }

    //0xCB 0x4F
    pub fn BIT_1A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::BIT(val, 1, registers);
    }

    //0xCB 0x50
    pub fn BIT_2B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::BIT(val, 2, registers);
    }

    //0xCB 0x51
    pub fn BIT_2C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::BIT(val, 2, registers);
    }

    //0xCB 0x52
    pub fn BIT_2D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::BIT(val, 2, registers);
    }

    //0xCB 0x53
    pub fn BIT_2E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::BIT(val, 2, registers);
    }

    //0xCB 0x54
    pub fn BIT_2H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::BIT(val, 2, registers);
    }

    //0xCB 0x55
    pub fn BIT_2L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::BIT(val, 2, registers);
    }

    //0xCB 0x56
    pub fn BIT_2dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::BIT(val, 2, registers);
    }

    //0xCB 0x57
    pub fn BIT_2A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::BIT(val, 2, registers);
    }

    //0xCB 0x58
    pub fn BIT_3B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::BIT(val, 3, registers);
    }

    //0xCB 0x59
    pub fn BIT_3C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::BIT(val, 3, registers);
    }

    //0xCB 0x5A
    pub fn BIT_3D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::BIT(val, 3, registers);
    }

    //0xCB 0x5B
    pub fn BIT_3E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::BIT(val, 3, registers);
    }

    //0xCB 0x5C
    pub fn BIT_3H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::BIT(val, 3, registers);
    }

    //0xCB 0x5D
    pub fn BIT_3L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::BIT(val, 3, registers);
    }

    //0xCB 0x5E
    pub fn BIT_3dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::BIT(val, 3, registers);
    }

    //0xCB 0x5F
    pub fn BIT_3A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::BIT(val, 3, registers);
    }

    //0xCB 0x60
    pub fn BIT_4B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::BIT(val, 4, registers);
    }

    //0xCB 0x61
    pub fn BIT_4C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::BIT(val, 4, registers);
    }

    //0xCB 0x62
    pub fn BIT_4D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::BIT(val, 4, registers);
    }

    //0xCB 0x63
    pub fn BIT_4E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::BIT(val, 4, registers);
    }

    //0xCB 0x64
    pub fn BIT_4H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::BIT(val, 4, registers);
    }

    //0xCB 0x65
    pub fn BIT_4L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::BIT(val, 4, registers);
    }

    //0xCB 0x66
    pub fn BIT_4dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::BIT(val, 4, registers);
    }

    //0xCB 0x67
    pub fn BIT_4A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::BIT(val, 4, registers);
    }

    //0xCB 0x68
    pub fn BIT_5B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::BIT(val, 5, registers);
    }

    //0xCB 0x69
    pub fn BIT_5C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::BIT(val, 5, registers);
    }

    //0xCB 0x6A
    pub fn BIT_5D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::BIT(val, 5, registers);
    }

    //0xCB 0x6B
    pub fn BIT_5E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::BIT(val, 5, registers);
    }

    //0xCB 0x6C
    pub fn BIT_5H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::BIT(val, 5, registers);
    }

    //0xCB 0x6D
    pub fn BIT_5L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::BIT(val, 5, registers);
    
    }

    //0xCB 0x6E
    pub fn BIT_5dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::BIT(val, 5, registers);
    }

    //0xCB 0x6F
    pub fn BIT_5A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::BIT(val, 5, registers);
    
    }

    //0xCB 0x70
    pub fn BIT_6B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::BIT(val, 6, registers);
    }

    //0xCB 0x71
    pub fn BIT_6C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::BIT(val, 6, registers);
    }

    //0xCB 0x72
    pub fn BIT_6D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::BIT(val, 6, registers);
    }

    //0xCB 0x73
    pub fn BIT_6E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::BIT(val, 6, registers);
    }

    //0xCB 0x74
    pub fn BIT_6H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::BIT(val, 6, registers);
    }

    //0xCB 0x75
    pub fn BIT_6L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::BIT(val, 6, registers);
    }

    //0xCB 0x76
    pub fn BIT_6dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::BIT(val, 6, registers);
    }

    //0xCB 0x77
    pub fn BIT_6A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::BIT(val, 6, registers);
    }

    //0xCB 0x78
    pub fn BIT_7B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.B( Action::Read ).value();
        Instruction::BIT(val, 7, registers);
    }

    //0xCB 0x79
    pub fn BIT_7C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.C( Action::Read ).value();
        Instruction::BIT(val, 7, registers);
    }

    //0xCB 0x7A
    pub fn BIT_7D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.D( Action::Read ).value();
        Instruction::BIT(val, 7, registers);
    }

    //0xCB 0x7B
    pub fn BIT_7E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.E( Action::Read ).value();
        Instruction::BIT(val, 7, registers);
    }

    //0xCB 0x7C
    pub fn BIT_7H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.H( Action::Read ).value();
        Instruction::BIT(val, 7, registers);
    }

    //0xCB 0x7D
    pub fn BIT_7L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.L( Action::Read ).value();
        Instruction::BIT(val, 7, registers);
    }

    //0xCB 0x7E
    pub fn BIT_7dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){
        let dHL: u16 = registers.HL( Action::Read ).value();

        let val = mem.read_byte(dHL).value();

        Instruction::BIT(val, 7, registers);
    }

    //0xCB 0x7F
    pub fn BIT_7A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){
        let val: u8 = registers.A( Action::Read ).value();
        Instruction::BIT(val, 7, registers);
    }

    //0xCB 0x80
    pub fn RES_0_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.B( Action::Read ).value();

        val.reset_bit(0);

        registers.A( Action::Write( val as u16 ) );
    }

    //0xCB 0x81
    pub fn RES_0_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.C( Action::Read ).value();

        val.reset_bit(0);

        registers.C( Action::Write( val as u16 ) );
    }

    //0xCB 0x82
    pub fn RES_0_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.D( Action::Read ).value();

        val.reset_bit(0);

        registers.D( Action::Write( val as u16 ) );
    }

    //0xCB 0x83
    pub fn RES_0_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.E( Action::Read ).value();

        val.reset_bit(0);

        registers.E( Action::Write( val as u16 ) );
    }

    //0xCB 0x84
    pub fn RES_0_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.H( Action::Read ).value();

        val.reset_bit(0);

        registers.H( Action::Write( val as u16 ) );
    }

    //0xCB 0x85
    pub fn RES_0_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.L( Action::Read ).value();

        val.reset_bit(0);

        registers.L( Action::Write( val as u16 ) );
    }

    //0xCB 0x86
    pub fn RES_0_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) {

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();

        val.reset_bit(0);

        mem.write_byte(HL, val);
    }

    //0xCB 0x87
    pub fn RES_0_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(0);

        registers.A( Action::Write( A as u16 ) );
    }

    //0xCB 0x88
    pub fn RES_1_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.B( Action::Read ).value();

        val.reset_bit(1);

        registers.B( Action::Write( val as u16 ) );
    }

    //0xCB 0x89
    pub fn RES_1_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.C( Action::Read ).value();

        val.reset_bit(1);

        registers.C( Action::Write( val as u16 ) );
    }

    //0xCB 0x8A
    pub fn RES_1_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.D( Action::Read ).value();

        val.reset_bit(1);

        registers.D( Action::Write( val as u16 ) );
    }

    //0xCB 0x8B
    pub fn RES_1_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.E( Action::Read ).value();

        val.reset_bit(1);

        registers.E( Action::Write( val as u16 ) );
    }

    //0xCB 0x8C
    pub fn RES_1_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.H( Action::Read ).value();

        val.reset_bit(1);

        registers.H( Action::Write( val as u16 ) );
    }

    //0xCB 0x8D
    pub fn RES_1_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.L( Action::Read ).value();

        val.reset_bit(1);

        registers.L( Action::Write( val as u16 ) );
    }

    //0xCB 0x8E
    pub fn RES_1_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) {

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();

        val.reset_bit(1);

        mem.write_byte(HL, val);
    }

    //0xCB 0x8F
    pub fn RES_1_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(1);

        registers.A( Action::Write( A as u16 ) );
    }

    //0xCB 0x90
    pub fn RES_2_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.B( Action::Read ).value();

        val.reset_bit(2);

        registers.B( Action::Write( val as u16 ) );
    }

    //0xCB 0x91
    pub fn RES_2_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.C( Action::Read ).value();

        val.reset_bit(2);

        registers.C( Action::Write( val as u16 ) );
    }

    //0xCB 0x92
    pub fn RES_2_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.D( Action::Read ).value();

        val.reset_bit(2);

        registers.D( Action::Write( val as u16 ) );
    }

    //0xCB 0x93
    pub fn RES_2_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.E( Action::Read ).value();

        val.reset_bit(2);

        registers.E( Action::Write( val as u16 ) );
    }

    //0xCB 0x94
    pub fn RES_2_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.H( Action::Read ).value();

        val.reset_bit(2);

        registers.H( Action::Write( val as u16 ) );
    }

    //0xCB 0x95
    pub fn RES_2_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.L( Action::Read ).value();

        val.reset_bit(2);

        registers.L( Action::Write( val as u16 ) );
    }

    //0xCB 0x96
    pub fn RES_2_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) {

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();

        val.reset_bit(2);

        mem.write_byte(HL, val);
    }

    //0xCB 0x97
    pub fn RES_2_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(2);

        registers.A( Action::Write( A as u16 ) );
    }

    //0xCB 0x98
    pub fn RES_3_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.B( Action::Read ).value();

        val.reset_bit(3);

        registers.B( Action::Write( val as u16 ) );
    }

    //0xCB 0x99
    pub fn RES_3_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.C( Action::Read ).value();

        val.reset_bit(3);

        registers.C( Action::Write( val as u16 ) );
    }

    //0xCB 0x9A
    pub fn RES_3_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.D( Action::Read ).value();

        val.reset_bit(3);

        registers.D( Action::Write( val as u16 ) );
    }

    //0xCB 0x9B
    pub fn RES_3_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.E( Action::Read ).value();

        val.reset_bit(3);

        registers.E( Action::Write( val as u16 ) );
    }

    //0xCB 0x9C
    pub fn RES_3_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.H( Action::Read ).value();

        val.reset_bit(3);

        registers.H( Action::Write( val as u16 ) );
    }

    //0xCB 0x9D
    pub fn RES_3_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.L( Action::Read ).value();

        val.reset_bit(3);

        registers.L( Action::Write( val as u16 ) );
    }

    //0xCB 0x9E
    pub fn RES_3_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) {

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();

        val.reset_bit(3);

        mem.write_byte(HL, val);
    }

    //0xCB 0x9F
    pub fn RES_3_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(3);

        registers.A( Action::Write( A as u16 ) );
    }

    //0xCB 0xA0
    pub fn RES_4_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.B( Action::Read ).value();

        val.reset_bit(4);

        registers.B( Action::Write( val as u16 ) );
    }

    //0xCB 0xA1
    pub fn RES_4_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.C( Action::Read ).value();

        val.reset_bit(4);

        registers.C( Action::Write( val as u16 ) );
    }

    //0xCB 0xA2
    pub fn RES_4_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.D( Action::Read ).value();

        val.reset_bit(4);

        registers.D( Action::Write( val as u16 ) );
    }

    //0xCB 0xA3
    pub fn RES_4_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.E( Action::Read ).value();

        val.reset_bit(4);

        registers.E( Action::Write( val as u16 ) );
    }

    //0xCB 0xA4
    pub fn RES_4_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.H( Action::Read ).value();

        val.reset_bit(4);

        registers.H( Action::Write( val as u16 ) );
    }

    //0xCB 0xA5
    pub fn RES_4_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.L( Action::Read ).value();

        val.reset_bit(4);

        registers.L( Action::Write( val as u16 ) );
    }
    
    //0xCB 0xA6
    pub fn RES_4_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) {

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();

        val.reset_bit(4);

        mem.write_byte(HL, val);
    } 

    //0xCB 0xA7
    pub fn RES_4_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(4);

        registers.A( Action::Write( A as u16 ) );
    }

    //0xCB 0xA8
    pub fn RES_5_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.B( Action::Read ).value();

        val.reset_bit(5);

        registers.B( Action::Write( val as u16 ) );
    }

    //0xCB 0xA9
    pub fn RES_5_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.C( Action::Read ).value();

        val.reset_bit(5);

        registers.C( Action::Write( val as u16 ) );
    }

    //0xCB 0xAA
    pub fn RES_5_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.D( Action::Read ).value();

        val.reset_bit(5);

        registers.D( Action::Write( val as u16 ) );
    }

    //0xCB 0xAB
    pub fn RES_5_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.E( Action::Read ).value();

        val.reset_bit(5);

        registers.E( Action::Write( val as u16 ) );
    }

    //0xCB 0xAC
    pub fn RES_5_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.H( Action::Read ).value();

        val.reset_bit(5);

        registers.H( Action::Write( val as u16 ) );
    }

    //0xCB 0xAD
    pub fn RES_5_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.L( Action::Read ).value();

        val.reset_bit(5);

        registers.L( Action::Write( val as u16 ) );
    }

    //0xCB 0xAE
    pub fn RES_5_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) {

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();

        val.reset_bit(5);

        mem.write_byte(HL, val);
    } 

    //0xCB 0xAF
    pub fn RES_5_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(5);

        registers.A( Action::Write( A as u16 ) );
    }

    //0xCB 0xB0
    pub fn RES_6_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.B( Action::Read ).value();

        val.reset_bit(6);

        registers.B( Action::Write( val as u16 ) );
    }

    //0xCB 0xB1
    pub fn RES_6_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.C( Action::Read ).value();

        val.reset_bit(6);

        registers.C( Action::Write( val as u16 ) );
    }

    //0xCB 0xB2
    pub fn RES_6_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.D( Action::Read ).value();

        val.reset_bit(6);

        registers.D( Action::Write( val as u16 ) );
    }

    //0xCB 0xB3
    pub fn RES_6_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.E( Action::Read ).value();

        val.reset_bit(6);

        registers.E( Action::Write( val as u16 ) );
    }

    //0xCB 0xB4
    pub fn RES_6_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.H( Action::Read ).value();

        val.reset_bit(6);

        registers.H( Action::Write( val as u16 ) );
    }

    //0xCB 0xB5
    pub fn RES_6_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.L( Action::Read ).value();

        val.reset_bit(6);

        registers.L( Action::Write( val as u16 ) );
    }

    //0xCB 0xB6
    pub fn RES_6_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) {

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();

        val.reset_bit(6);

        mem.write_byte(HL, val);
    } 

    //0xCB 0xB7
    pub fn RES_6_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(6);

        registers.A( Action::Write( A as u16 ) );
    }

    //0xCB 0xB8
    pub fn RES_7_B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.B( Action::Read ).value();

        val.reset_bit(7);

        registers.B( Action::Write( val as u16 ) );
    }

    //0xCB 0xB9
    pub fn RES_7_C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.C( Action::Read ).value();

        val.reset_bit(7);

        registers.C( Action::Write( val as u16 ) );
    }

    //0xCB 0xBA
    pub fn RES_7_D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.D( Action::Read ).value();

        val.reset_bit(7);

        registers.D( Action::Write( val as u16 ) );
    }

    //0xCB 0xBB
    pub fn RES_7_E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.E( Action::Read ).value();

        val.reset_bit(7);

        registers.E( Action::Write( val as u16 ) );
    }

    //0xCB 0xBC
    pub fn RES_7_H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.H( Action::Read ).value();

        val.reset_bit(7);

        registers.H( Action::Write( val as u16 ) );
    }

    //0xCB 0xBD
    pub fn RES_7_L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut val: u8 = registers.L( Action::Read ).value();

        val.reset_bit(7);

        registers.L( Action::Write( val as u16 ) );
    }

    //0xCB 0xBE
    pub fn RES_7_dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus) {

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();

        val.reset_bit(7);

        mem.write_byte(HL, val);
    } 

    //0xCB 0xBF
    pub fn RES_7_A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) {
        let mut A: u8 = registers.A( Action::Read ).value();

        A.reset_bit(7);

        registers.A( Action::Write( A as u16 ) );
    }
    
    //0xCB 0xC0
    pub fn SET_0B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.B( Action::Read ).value();
        
        val.set_bit(0);

        registers.B( Action::Write(val as u16) );
    }
    
    //0xCB 0xC1
    pub fn SET_0C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.C( Action::Read ).value();
        
        val.set_bit(0);

        registers.C( Action::Write(val as u16) );
    }
    
    //0xCB 0xC2
    pub fn SET_0D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.D( Action::Read ).value();
        
        val.set_bit(0);

        registers.D( Action::Write(val as u16) );
    }

    //0xCB 0xC3
    pub fn SET_0E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.E( Action::Read ).value();
        
        val.set_bit(0);

        registers.E( Action::Write(val as u16) );
    }

    //0xCB 0xC4
    pub fn SET_0H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.H( Action::Read ).value();
        
        val.set_bit(0);

        registers.H( Action::Write(val as u16) );
    }

    //0xCB 0xC5
    pub fn SET_0L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.L( Action::Read ).value();
        
        val.set_bit(0);

        registers.L( Action::Write(val as u16) );
    }

    //0xCB 0xC6
    pub fn SET_0dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();
        
        val.set_bit(0);

        mem.write_byte(HL, val);
    }

    //0xCB 0xC7
    pub fn SET_0A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.A( Action::Read ).value();
        
        val.set_bit(0);

        registers.A( Action::Write(val as u16) );
    }

    //0xCB 0xC8
    pub fn SET_1B(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.B( Action::Read ).value();
        
        val.set_bit(1);

        registers.B( Action::Write(val as u16) );
    }

    //0xCB 0xC9
    pub fn SET_1C(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.C( Action::Read ).value();
        
        val.set_bit(1);

        registers.C( Action::Write(val as u16) );
    }

    //0xCB 0xCA
    pub fn SET_1D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.D( Action::Read ).value();
        
        val.set_bit(1);

        registers.D( Action::Write(val as u16) );
    }

    //0xCB 0xCB
    pub fn SET_1E(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.E( Action::Read ).value();
        
        val.set_bit(1);

        registers.E( Action::Write(val as u16) );
    }

    //0xCB 0xCC
    pub fn SET_1H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.H( Action::Read ).value();
        
        val.set_bit(1);

        registers.H( Action::Write(val as u16) );
    }

    //0xCB 0xCD
    pub fn SET_1L(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.L( Action::Read ).value();
        
        val.set_bit(1);

        registers.L( Action::Write(val as u16) );
    }

    //0xCB 0xCE
    pub fn SET_1dHL(_operands: [u8; 2], registers: &mut Registers, mem: &mut Bus){

        let HL: u16 = registers.HL( Action::Read ).value();

        let mut val: u8 = mem.read_byte(HL).value();
        
        val.set_bit(1);

        mem.write_byte(HL, val);
    }

    //0xCB 0xCF
    pub fn SET_1A(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.A( Action::Read ).value();
        
        val.set_bit(1);

        registers.A( Action::Write(val as u16) );
    }


    //0xCB 0xF2
    pub fn SET_6D(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus){

        let mut val: u8 = registers.D( Action::Read ).value();
        
        val.set_bit(6);

        registers.D( Action::Write(val as u16) );
    }

}