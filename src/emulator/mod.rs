#![allow(dead_code)]

mod cpu;
mod gpu;
mod memory;
mod cartrigbe;
mod interrupt;
mod bit_utils;
mod bus;

use cpu::{*};
use gpu::{*};
use memory::{*};
use memory::io::{*};
use cartrigbe::{*};
use interrupt::{*};


use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

//max cycles after vblank, when this value is reached we draw the actual screen
const MAXCYCLES: u32 = 65664;


#[derive(Default)]
pub struct Gameboy {
    cpu: CPU,
    memory: Memory,
    cartrigbe: Cartrigbe,
    gpu: GPU,
    interrupt: InterruptHandler,
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
                let state = self.cpu_inst();
                //update current cycles
                cycles_now += state.0 as u32;
                //if IO write was made, sync it
                if state.1 { self.sync(); }


                self.gpu.step(state.0, &mut self.interrupt, &self.memory);

                //update pc to execute interrupt vector if any
                self.interrupt.execute(&mut self.cpu);
                //sync harware registers to memory
                self.sync_to_mem();
            };

            // render next frame, this is VBLANK
            let up = window.update_with_buffer(&self.gpu.display, WIDTH, HEIGHT);
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

    fn sync(&mut self){
        let buff = self.memory.get_io();

        self.gpu.LCDC = buff[LCDC as usize];
        self.gpu.STAT = buff[STAT as usize];
        self.gpu.scroll_y = buff[SCY as usize];
        self.gpu.scroll_x = buff[SCX as usize];
        self.gpu.lcd_y = buff[LY as usize];
        self.gpu.lycompare = buff[LYC as usize];
        self.gpu.window_y = buff[WY as usize];
        self.gpu.window_x = buff[WX as usize];
        self.gpu.bg_palette = buff[BGP as usize];
        self.gpu.ob_palette0 = buff[OBP0 as usize];
        self.gpu.ob_palette1 = buff[OBP1 as usize];

        self.interrupt.enable = self.memory.read_byte( 0xFFFF ).unwrap();
        self.interrupt.requests = buff[IF as usize];
        self.interrupt.master = self.memory.interrupt_state();
    }

    fn sync_to_mem(&mut self){
        self.memory.write_byte( 0xFF00 + LCDC,  self.gpu.LCDC);
        self.memory.write_byte( 0xFF00 + STAT, self.gpu.STAT);
        self.memory.write_byte( 0xFF00 + SCY, self.gpu.scroll_y);
        self.memory.write_byte( 0xFF00 + SCX, self.gpu.scroll_x);
        self.memory.write_byte( 0xFF00 + LY, self.gpu.lcd_y);
        self.memory.write_byte( 0xFF00 + LYC, self.gpu.lycompare);
        self.memory.write_byte( 0xFF00 + WY, self.gpu.window_y);
        self.memory.write_byte( 0xFF00 + WX, self.gpu.window_x);
        self.memory.write_byte( 0xFF00 + BGP, self.gpu.bg_palette);
        self.memory.write_byte( 0xFF00 + OBP0, self.gpu.ob_palette0);
        self.memory.write_byte( 0xFF00 + OBP1, self.gpu.ob_palette1);
    }

    //get an opcode byte and convert it into an Instruction object
    fn decode(&mut self, mut opcode: u8, pc: u16) -> Instruction {
        //if instruction is 0xCB, get next byte and decode it through subset instructions array
        if opcode != 0xCB {
            CPU::decode(opcode, false)
        } else {
            opcode = self.memory.read_byte(pc+1).unwrap();
            self.cpu.increment_PC(1);
            CPU::decode(opcode, true)
        }
    }
    //execute instruction pointed by PC, increment it as needed, return number of cycles it took and if an IO write was made
    fn cpu_inst(&mut self) -> (u16, bool) {
        let pc = self.cpu.PC();
        let opcode = self.memory.read_byte(pc).unwrap();
        let instruction = self.decode(opcode, pc);


        let mut operands = [0;2];

        match instruction.args {
            0 => {
                self.cpu.increment_PC(1);
            },
            1 => {
                operands[0] = self.memory.read_byte(pc+1).unwrap();
                self.cpu.increment_PC(2);
            },
            2 => {
                operands[0] = self.memory.read_byte(pc+1).unwrap();
                operands[1] = self.memory.read_byte(pc+2).unwrap();
                self.cpu.increment_PC(3)
            }
            _ => {
                panic!("Instruction has wrong number of args \"{}\"", instruction);
            },
        }

        print!("{:#04x}: ", opcode);

        instruction.execute(operands, &mut self.cpu.registers, &mut self.memory, instruction)
    }

    fn load_rom(&mut self, bank_number: u8){
        let bank = &self.cartrigbe.banks[bank_number as usize].info;
        self.memory.push_rom(*bank, true);
    }

    pub fn insert(&mut self, file_name: String){
        self.cartrigbe.insert(file_name);
        self.load_rom(0);
    }
}
