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
mod joypad;

const DEBUG_FLAG: bool = false;

use cpu::{*};
use cpu::registers::{*};
use bus::{*};


use minifb::{Key, Window, WindowOptions, KeyRepeat};

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
    pub fn start(&mut self){

        let mut debug = false;

        let mut window = Gameboy::create_window();

        self.screen = vec![0;WIDTH*HEIGHT];

        while window.is_open() && !window.is_key_down(Key::Escape) {

            let mut cycles_now = 0;

            while cycles_now < MAXCYCLES { 

                if window.is_key_pressed(Key::D, KeyRepeat::Yes) {
                    if debug { debug = false } else { debug = true };
                }

                if window.is_key_pressed(Key::Up, KeyRepeat::No) {
                    let jp_int = self.bus.joypad.up();
                    if jp_int == Interrupt::Joypad { 
                        self.bus.interrupts.request(jp_int);
                    }
                }
                if window.is_key_pressed(Key::Left, KeyRepeat::No) {
                    let jp_int = self.bus.joypad.left();
                    if jp_int == Interrupt::Joypad { 
                        self.bus.interrupts.request(jp_int);
                    }
                }
                if window.is_key_pressed(Key::Right, KeyRepeat::No) {
                    let jp_int = self.bus.joypad.right();
                    if jp_int == Interrupt::Joypad { 
                        self.bus.interrupts.request(jp_int);
                    }
                }
                if window.is_key_pressed(Key::Down, KeyRepeat::No) {
                    let jp_int = self.bus.joypad.down();
                    if jp_int == Interrupt::Joypad { 
                        self.bus.interrupts.request(jp_int);
                    }
                }
                if window.is_key_pressed(Key::Z, KeyRepeat::No) {
                    let jp_int = self.bus.joypad.btn_a();
                    if jp_int == Interrupt::Joypad { 
                        self.bus.interrupts.request(jp_int);
                    }
                }
                if window.is_key_pressed(Key::X, KeyRepeat::No) {
                    let jp_int = self.bus.joypad.btn_b();
                    if jp_int == Interrupt::Joypad { 
                        self.bus.interrupts.request(jp_int);
                    }
                }
                if window.is_key_pressed(Key::D, KeyRepeat::No) {
                    let jp_int = self.bus.joypad.start();
                    if jp_int == Interrupt::Joypad { 
                        self.bus.interrupts.request(jp_int);
                    }
                }
                if window.is_key_pressed(Key::F, KeyRepeat::No) {
                    let jp_int = self.bus.joypad.start();
                    if jp_int == Interrupt::Joypad { 
                        self.bus.interrupts.request(jp_int);
                    }
                }

                //execute the instruction pointed by PC
                let cycles = self.cpu_inst(debug);
                //update current cycles
                cycles_now += cycles as u32;

                let screen = &mut self.screen;
                //run the rest of the system
                self.bus.run_system(cycles, screen);

            };

            // render next frame, this is VBLANK
            let up = window.update_with_buffer(&self.screen, WIDTH, HEIGHT);
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
        self.cpu.interrupts(&mut self.bus);

        // if int_cylces != 0 { return int_cylces; }
        
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
               
            if pc == 0x0F15 || pc == 0x344 {
                print!("");
            }
            
            let ret = CPU::decode(0xC9, false);
            
            if ret == instruction {
                print!("")
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
