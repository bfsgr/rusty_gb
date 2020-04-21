#![allow(warnings)]

mod cpu;
mod memory;
mod cartrigbe;
use cpu::CPU;
use cartrigbe::Cartrigbe;

struct GameBoy {
    cpu: CPU,
    cartrigbe: Cartrigbe
}

impl Default for GameBoy {
    fn default() -> Self {
        GameBoy {
            cpu: CPU::default(),
            cartrigbe: Cartrigbe::default()
        }
    }
}

impl GameBoy {
    pub fn start(&mut self){
        self.cpu.mmu.push_range( &(self.cartrigbe.banks[0].info).to_vec() , 256, 0x4000);
        for i in 0..0x6050 { self.cpu.run() };
    }
}


fn main() {
    let mut test = GameBoy::default();
    test.cartrigbe.insert("Tetris2.gb".to_string());
    // println!("{}", test.cartrigbe.info);
    test.start();

    // test.cpu.mmu.dump();
    // println!("{}", test.cpu.registers);
}

