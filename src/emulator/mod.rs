#![allow(dead_code)]

mod cpu;
mod gpu;
mod memory;
mod io_constants;
mod cartrigbe;
mod interrupt;
mod bit_utils;
mod bus;

use cpu::{*};
use cpu::registers::{*};
use bus::{*};


use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

//max cycles after vblank, when this value is reached we draw the actual screen
const MAXCYCLES: u32 = 65664;


#[derive(Default)]
pub struct Gameboy {
    cpu: CPU,
    bus: Bus,
    //cartrigbe
    //sound
    //timers
    //joypad
    //interrupt
    //dma
}

impl Gameboy {
    //main loop
    pub fn start(&mut self){

        let mut window = Gameboy::create_window();

        while window.is_open() && !window.is_key_down(Key::Escape) {

            let mut cycles_now = 0;

            while cycles_now < MAXCYCLES { 

                //execute the instruction pointed by PC
                let cycles = self.cpu_inst();
                //update current cycles
                cycles_now += cycles as u32;

                // self.gpu.step(state.0, &mut self.interrupt, &self.memory);

                //update pc to execute interrupt vector if any
                // self.interrupt.execute(&mut self.cpu);
                //sync harware registers to memory
                // self.sync_to_mem();
            };



            // render next frame, this is VBLANK
            let up = window.update_with_buffer(&self.bus.gpu.display, WIDTH, HEIGHT);
            match up {
                Err(up) => println!("{}", up),
                _  => {},
            }
        }
    }

    fn create_window() -> Window {
        let mut win = Window::new(
            "Rusty GB",
            WIDTH,
            HEIGHT,
            WindowOptions {
                borderless: false,
                resize: false,
                scale: minifb::Scale::X2,
                scale_mode: minifb::ScaleMode::Center,
                title: true,
                topmost: false
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        win.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        return win;
    }

    //get an opcode byte and convert it into an Instruction object
    fn decode(&mut self, mut opcode: u8, pc: u16) -> Instruction {
        //if instruction is 0xCB, get next byte and decode it through subset instructions array
        if opcode != 0xCB {
            CPU::decode(opcode, false)
        } else {
            opcode = self.bus.read_byte(pc+1).value();
            self.cpu.increment_PC(1);
            CPU::decode(opcode, true)
        }
    }
    //execute instruction pointed by PC, increment it as needed, return number of cycles it took and if an IO write was made
    fn cpu_inst(&mut self) -> u16 {
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

        print!("{:#04x}: ", opcode);

        instruction.execute(operands, &mut self.cpu.registers, &mut self.bus, instruction)
    }



    pub fn insert(&mut self, file_name: String){
        self.bus.insert_cartrigbe(file_name);
    }
}
