mod bus;
mod cpu;
mod interrupt;
mod joypad;
mod cartridge;
mod io_constants;
mod bit_utils;
mod memory;
mod gpu;
mod timer;
use interrupt::Interrupt;
use bus::Bus;
use cpu::CPU;
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
    pub fn tick(&mut self){
        self.cpu.run(&mut self.bus);
        self.bus.run(&mut self.screen)
    }

    pub fn insert(&mut self, file_name: String){
        self.bus.insert_cartrigbe(file_name);
    }

    jp_input!(up, down, left, right, btn_a, btn_b, start, select);
}