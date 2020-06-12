#![allow(dead_code)]

mod cpu;
mod gpu;
mod memory;
mod io_constants;
mod cartrigbe;
mod interrupt;
mod bit_utils;
mod bus;
mod timer;

const DEBUG_FLAG: bool = false;

use cpu::{*};
use cpu::registers::{*};
use bus::{*};


use minifb::{Key, Window, WindowOptions, KeyRepeat};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

//max cycles after vblank, when this value is reached we draw the actual screen
const MAXCYCLES: u32 = 65664;

#[derive(Default)]
pub struct Gameboy {
    cpu: CPU,
    bus: Bus,
}

impl Gameboy {
    //main loop
    pub fn start(&mut self){

        let mut debug = false;

        let mut window = Gameboy::create_window();

        while window.is_open() && !window.is_key_down(Key::Escape) {

            let mut cycles_now = 0;

            while cycles_now < MAXCYCLES { 

                if window.is_key_pressed(Key::D, KeyRepeat::Yes) {
                    if debug { debug = false } else { debug = true };
                }

                //execute the instruction pointed by PC
                let cycles = self.cpu_inst(debug);
                //update current cycles
                cycles_now += cycles as u32;

                //run the rest of the system
                self.bus.run_system(cycles);

            };

            // render next frame, this is VBLANK
            let up = window.update_with_buffer(&self.bus.gpu.display, WIDTH, HEIGHT);
            match up {
                Err(up) => println!("{}", up),
                _  => {},
            }
        }

        println!("{}\n{}", self.cpu.registers, self.bus.interrupts);
    }

    fn interrupt_running(&self) -> bool {
        self.bus.interrupts.enable & 0x00FF != 0
    }

    fn create_window() -> Window {
        let win = Window::new(
            "Rusty GB",
            WIDTH,
            HEIGHT,
            WindowOptions {
                borderless: false,
                resize: false,
                scale: minifb::Scale::X2,
                scale_mode: minifb::ScaleMode::AspectRatioStretch,
                title: true,
                topmost: false
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // win.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        return win;
    }

    //get an opcode byte and convert it into an Instruction object
    fn decode(&mut self, mut opcode: u8, pc: u16) -> Instruction {
        let instruction;
        //if instruction is 0xCB, get next byte and decode it through subset instructions array
        if opcode != 0xCB {
            instruction = CPU::decode(opcode, false);
        } else {
            opcode = self.bus.read_byte(pc+1).value();
            self.cpu.increment_PC(1);
            instruction = CPU::decode(opcode, true);
        }

        let not = cpu::NOT_IMPLEMENTED;

        if instruction == not {
            println!("0xCB {:#04x} not implemented", opcode)
        }

        return instruction;
    }



    //execute instruction pointed by PC, increment it as needed, return number of cycles it took and if an IO write was made
    fn cpu_inst(&mut self, debug_flag: bool) -> u16 {
        self.cpu.interrupts(&mut self.bus);
        
        if !self.bus.halt_cpu {
            let pc = self.cpu.PC();
            let opcode = self.bus.read_byte(pc).value();
    
            let instruction = self.decode(opcode, pc);
    
    
    
            let mut operands = [0;2];
    
            match instruction.args {
                0 => {
                    self.cpu.increment_PC(1);
                },
                1 => {
                    operands[0] = self.bus.read_byte(pc+1).value();
                    self.cpu.increment_PC(2);
                },
                2 => {
                    operands[0] = self.bus.read_byte(pc+1).value();
                    operands[1] = self.bus.read_byte(pc+2).value();
                    self.cpu.increment_PC(3)
                }
                _ => {
                    panic!("Instruction has wrong number of args \"{}\"", instruction);
                },
            }
    
            if debug_flag {
                let oprnds = Bus::to_short(operands);
                println!("{:#04x}: {}\r\t\t\t{:#10x}", opcode, instruction.disassembly, oprnds);
            }

            if pc == 0xC34E {
                print!("");
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
