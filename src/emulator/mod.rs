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

use joypad::Joypad;
use std::thread;
use std::time::{Duration, Instant};


use cpu::{*};
use cpu::registers::{*};
use bus::{*};


use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

//max cycles after vblank, when this value is reached we draw the actual screen
const MAXCYCLES: u32 = 66576;

#[derive(Default)]
pub struct Gameboy {
    cpu: CPU,
    bus: Bus,
    screen: Vec<u32>
}

impl Gameboy {
    //main loop
    pub fn start(&mut self, debug: bool){

        let mut window = Gameboy::create_window();

        self.screen = vec![0;WIDTH*HEIGHT];

		let frame = Duration::new(0, 16600000); // 16.6 ms as nanoseconds
        
        
        while window.is_open() && !window.is_key_down(Key::Escape) {
            
            let mut cycles_now = 0;
            
            
            let start = Instant::now();
            while cycles_now < MAXCYCLES { 

                //execute the instruction pointed by PC
                let cycles = self.cpu_inst(debug);
                //update current cycles
                cycles_now += cycles as u32;
                
                let screen = &mut self.screen;
                //run the rest of the system
                self.bus.run_system(cycles, screen);
                
                Gameboy::get_input(&window, &mut self.bus.joypad, &mut self.bus.interrupts);

            };  
            
            let elapsed = start.elapsed();
            if elapsed < frame {
                thread::sleep(frame - elapsed);
            }
            
            
            // render next frame, this is VBLANK
            let up = window.update_with_buffer(&self.screen, WIDTH, HEIGHT);
            match up {
                Err(up) => println!("{}", up),
                _  => {},
            }
        }

        println!("{}\n{}", self.cpu.registers, self.bus.interrupts);
    }

    fn get_input(window: &Window, joypad: &mut Joypad, interrupts: &mut InterruptHandler) {
        if window.is_key_down(Key::Up)  {
            let int = joypad.up(true);

            if int == Interrupt::Joypad { 
                interrupts.request(Interrupt::Joypad);
            }

        } else {
            joypad.up(false);
        }
        if window.is_key_down(Key::Down)  {
            let int = joypad.down(true);

            if int == Interrupt::Joypad { 
                interrupts.request(Interrupt::Joypad);
            }
        } else {
            joypad.down(false);
        }

        if window.is_key_down(Key::Left)  {
            let int = joypad.left(true);

            if int == Interrupt::Joypad { 
                interrupts.request(Interrupt::Joypad);
            }
        } else {
            joypad.left(false);
        }

        if window.is_key_down(Key::Right)  {
            let int = joypad.right(true);

            if int == Interrupt::Joypad { 
                interrupts.request(Interrupt::Joypad);
            }
        } else {
            joypad.right(false);
        }

        if window.is_key_down(Key::F)  {
            let int = joypad.start(true);

            if int == Interrupt::Joypad { 
                interrupts.request(Interrupt::Joypad);
            }
        } else {
            joypad.start(false);
        }

        if window.is_key_down(Key::Z)  {
            let int = joypad.btn_a(true);

            if int == Interrupt::Joypad { 
                interrupts.request(Interrupt::Joypad);
            }
        } else {
            joypad.btn_a(false);
        }

        if window.is_key_down(Key::X)  {
            let int = joypad.btn_b(true);

            if int == Interrupt::Joypad { 
                interrupts.request(Interrupt::Joypad);
            }
        } else {
            joypad.btn_b(false);
        }

        if window.is_key_down(Key::G) {
            let select = joypad.select(true);
            if select == Interrupt::Joypad {
                interrupts.request(Interrupt::Joypad);
            }
        } else {
            joypad.select(false);
        }
    }

    fn create_window() -> Window {
        let win = Window::new(
            "Rusty GB",
            WIDTH,
            HEIGHT,
            WindowOptions {
                borderless: false,
                resize: false,
                scale: minifb::Scale::X4,
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
