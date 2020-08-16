pub mod registers;
use registers::*;
mod instructions;
pub mod decoder;
mod generic;
mod subops;

use super::bit_utils::BitUtils;
use super::interrupt::*;
use super::Bus;
use decoder::Decoder;
use instructions::Instruction;

use std::collections::VecDeque;
pub struct CPU{
    registers: Registers,
    instruction: Instruction
}

impl Default for CPU {
    fn default() -> Self {
        Self{
            registers: Registers::default(),
            instruction: Instruction::holder()
        }
    }
}

impl CPU {
    pub fn tick(&mut self, bus: &mut Bus){
        //instruction still running

        self.check_interrupt(bus);

        if !bus.halt_cpu {
            if self.instruction.cycles > 0 {
                self.instruction.tick(&mut self.registers, bus);
            } else {
                //fetch pc, decode, append and tick instruction
                let data = self.fetch(bus);
                self.instruction = Decoder::decode(data.0, data.1).unwrap();
        
                // println!("{}", self.instruction);
    
                self.instruction.tick(&mut self.registers, bus);
            }
        }
    }

    fn fetch(&mut self, bus: &mut Bus) -> (u8, bool) {
        let pc: u16 = self.registers.PC( Action::Read ).value();
        // print!("{:#10x}: ", pc);

        self.registers.PC( Action::Increment(1) );

        let mut opcode: u8 = bus.read_byte(pc).value();

        if opcode != 0xCB {
            return (opcode, false);
        } else {
            opcode = bus.read_byte(pc+1).value();
            self.registers.PC( Action::Increment(1) );
            return (opcode, true);
        }
    }

    fn check_interrupt(&mut self, bus: &mut Bus){
        if bus.interrupts.ei_key == EI::Requested {
            bus.interrupts.ei_key = EI::Active;
        }

        if bus.interrupts.ei_key == EI::Active {
            bus.interrupts.master = true;
            bus.interrupts.ei_key = EI::Disabled;
        }

        if bus.interrupts.master || bus.halt_cpu {
            

            let vec = bus.interrupts.get_vec();

            if vec == InterruptVector::None { return (); }

            if bus.halt_cpu && !bus.interrupts.master {
                bus.halt_cpu = false;
                return ();
            };

            bus.interrupts.master = false;

            match vec {
                InterruptVector::VBlank => {
                    bus.interrupts.requests.reset_bit(0);
                    self.instruction = Instruction::new(
                        "CALL 0x40",
                        {
                            let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                            o.push_back(Instruction::DEC_SP);
                            o.push_back(Instruction::write_P_in_dSP);
                            o.push_back(Instruction::DEC_SP);
                            o.push_back(Instruction::write_PC_in_dSP);
                            o.push_back(Instruction::load_40);
                            o
                        },
                        5
                    )
                },
                InterruptVector::LCDC => {
                    self.instruction = Instruction::new(
                        "CALL 0x48",
                        {
                            let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                            o.push_back(Instruction::DEC_SP);
                            o.push_back(Instruction::write_P_in_dSP);
                            o.push_back(Instruction::DEC_SP);
                            o.push_back(Instruction::write_PC_in_dSP);
                            o.push_back(Instruction::load_48);
                            o
                        },
                        5
                    )
                },
                InterruptVector::Timer => {
                    bus.interrupts.requests.reset_bit(2);
                    self.instruction = Instruction::new(
                        "CALL 0x50",
                        {
                            let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                            o.push_back(Instruction::DEC_SP);
                            o.push_back(Instruction::write_P_in_dSP);
                            o.push_back(Instruction::DEC_SP);
                            o.push_back(Instruction::write_PC_in_dSP);
                            o.push_back(Instruction::load_50);
                            o
                        },
                        5
                    )
                },
                InterruptVector::Serial => {
                    bus.interrupts.requests.reset_bit(3);
                    self.instruction = Instruction::new(
                        "CALL 0x58",
                        {
                            let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                            o.push_back(Instruction::DEC_SP);
                            o.push_back(Instruction::write_P_in_dSP);
                            o.push_back(Instruction::DEC_SP);
                            o.push_back(Instruction::write_PC_in_dSP);
                            o.push_back(Instruction::load_58);
                            o
                        },
                        5
                    )
                },
                InterruptVector::Joypad => {
                    bus.interrupts.requests.reset_bit(4);
                    self.instruction = Instruction::new(
                        "CALL 0x60",
                        {
                            let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                            o.push_back(Instruction::DEC_SP);
                            o.push_back(Instruction::write_P_in_dSP);
                            o.push_back(Instruction::DEC_SP);
                            o.push_back(Instruction::write_PC_in_dSP);
                            o.push_back(Instruction::load_60);
                            o
                        },
                        5
                    )
                },
                _ => unreachable!("This should never happen"),
            }
        }
    }
}
