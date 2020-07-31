#![allow(dead_code)]

mod cpu;
mod gpu;
mod memory;
mod io_constants;
mod cartridge;
mod interrupt;
mod bit_utils;
mod bus;
mod timer;
mod joypad;
use cpu::{*};
use cpu::registers::{*};
use bus::{*};

//max cycles after vblank, when this value is reached we draw the actual screen

#[derive(Default)]
pub struct Gameboy {
    cpu: CPU,
    bus: Bus,
    pub screen: Vec<u32>
}

macro_rules! jp_input {
    ( $( $key:ident ),* ) => {
        $(
            pub fn $key(&mut self, pressed: bool) {
                if pressed  {
                     let int = self.bus.joypad.$key(true);
                    if int == Interrupt::Joypad { 
                        self.bus.interrupts.request(Interrupt::Joypad);
                    }
                } else {
                    self.bus.joypad.$key(false);
                }
            }
        )*
    }
}

impl Gameboy {
    //main loop
    pub fn run(&mut self, debug: bool) -> u8{
        //execute the instruction pointed by PC
        let cycles = self.cpu_inst(debug);
        
        let screen = &mut self.screen;
        //run the rest of the system
        self.bus.run_system(cycles, screen);

        return cycles;
    }

    jp_input!(up, down, left, right, btn_a, btn_b, start, select);

    //get an opcode byte and convert it into an Instruction object
    fn decode(&mut self, mut opcode: u8, pc: u16) -> Instruction {
        let instruction;
        //if instruction is 0xCB, get next byte and decode it through subset instructions array
        if opcode != 0xCB {
            instruction = CPU::decode(opcode, false);
        } else {
            
            if !self.bus.interrupts.halt_bug {
                opcode = self.bus.read_byte(pc+1).value();
                instruction = CPU::decode(opcode, true);
                self.cpu.increment_PC(1);
            } else {
                instruction = CPU::decode(0xCB, true);
                self.cpu.increment_PC(1);
            }
        }

        return instruction;
    }

    //execute instruction pointed by PC, increment it as needed, return number of cycles it took and if an IO write was made
    fn cpu_inst(&mut self, debug_flag: bool) -> u8 {
        let int_cycles = self.cpu.interrupts(&mut self.bus);

        if int_cycles != 0 { return int_cycles; }
        
        if !self.bus.halt_cpu {
            let pc = self.cpu.PC();
            let opcode = self.bus.read_byte(pc).value();
    
            let instruction = self.decode(opcode, pc);
    
            let mut operands = [0;2];
    
            match instruction.args {
                0 => {
                    if !self.bus.interrupts.halt_bug {
                        self.cpu.increment_PC(1);
                    } else {
                        self.bus.interrupts.halt_bug = false;
                    }
                },
                1 => {
                    
                    if !self.bus.interrupts.halt_bug {
                        self.cpu.increment_PC(2);
                        operands[0] = self.bus.read_byte(pc+1).value();
                    } else {
                        operands[0] = self.bus.read_byte(pc).value();
                        self.cpu.increment_PC(1);
                        self.bus.interrupts.halt_bug = false;
                    }
                },
                2 => {
                    if !self.bus.interrupts.halt_bug {
                        self.cpu.increment_PC(3);
                        operands[0] = self.bus.read_byte(pc+1).value();
                        operands[1] = self.bus.read_byte(pc+2).value();
                    } else {
                        self.cpu.increment_PC(2);
                        operands[0] = self.bus.read_byte(pc).value();
                        operands[1] = self.bus.read_byte(pc+1).value();
                        self.bus.interrupts.halt_bug = false;
                    }
                }
                _ => {
                    panic!("Instruction has wrong number of args \"{}\"", instruction);
                },
            }
    
            if debug_flag && pc > 256 {
                let oprnds = Bus::to_short(operands);
                println!("{:#10x}: {}\r\t\t\t{:#10x}", pc, instruction.disassembly, oprnds);
            }   


            
            let cycles = instruction.execute(operands, &mut self.cpu.registers, &mut self.bus);


            return cycles;


        } else {
            return 4;
        }
    }
    
    pub fn insert(&mut self, file_name: String){
        self.bus.insert_cartrigbe(file_name);
    }
}
