use super::super::mbcx::{*};
use super::super::header::Header;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Default)]
pub struct MBC2 {
    header: Header,
    ram_on: bool,
    rom_bank: u8,
    sram: Vec<u8>,
    data: Vec<u8>,
}

impl MBC for MBC2 {
    fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            0 ..= 0x1FFF => {
                //check if addr accepts ram enable/disable
                if (addr & 0x100) == 0 {
                    self.ram_on = (byte & 0x0A) == 0x0A;
                }
            },
            0x2000 ..= 0x3FFF => {
                if (addr & 0x100) == 0x100 {
                    self.rom_bank = byte & 0x0F;
                }
            },
            0xA000 ..= 0xA1FF => {
                self.sram[ (addr - 0xA000) as usize ] = byte & 0x0F;
            },
            _ => { panic!("Wrong cartridge address")}
        }
    }
    fn read(&self, addr: u16) -> u8 {
        
        match addr {
            0 ..= 0x3FFF => self.data[addr as usize],

            0x4000 ..= 0x7FFF => {
                let adjusted = ((addr - 0x4000) as usize) + (0x4000 * self.rom_bank as usize);
                return self.data[adjusted];
            },

            0xA000 ..= 0xA1FF => self.sram[ (addr - 0xA000) as usize] & 0x0F,

            _ => {panic!("Wrong cartridge address")}
        }
    }
    fn load(&mut self, data: Vec<u8>, header: Header ) {
        self.header = header;

        match self.header.cartridge_type {
            5 => {
                let size = rom_to_size(self.header.rom_size);
                if self.header.ram_size != 0 { panic!("ram_size != 0 for MBC2") }
                if size != data.len() { panic!("Data size doesn't match header") }
                self.sram = vec![0; 512];
            },
            6 => {
                let size = rom_to_size(self.header.rom_size);
                if self.header.ram_size != 0 { panic!("ram_size != 0 for MBC2") }
                if size != data.len() { panic!("Data size doesn't match header") }
                self.try_load();
            },
            _ => panic!("Wrong type for MBC2: {:x}", self.header.cartridge_type )
        }

        self.data = data;
    }
}

impl MBC2 {
    fn try_load(&mut self) {
        let rsize = 512;

        let mut name = self.header.title.clone();
        name.push_str(".sav");
        let file = File::open(name);

        match file {
            Ok(mut file) => {
                match file.read_to_end(&mut self.sram) {
                    Ok(file_size) => {
                        if file_size != rsize {
                            println!("WARNING: Loading SRAM data failed -> .sav size mismatch");
                            self.sram = vec![0; rsize];
                        } else {
                            println!("SRAM data loaded")
                        }
                    },
                    Err(er) => {
                        println!("WARNING: Loading SRAM data failed -> {}", er);
                        self.sram = vec![0; rsize];
                    }
                }
            },
            Err(_) => {
                self.sram = vec![0; rsize];
            }
        }
    }
}

impl Drop for MBC2 {
    fn drop(&mut self) {
        if self.header.has_battery() {
            let mut name = self.header.title.clone();
            name.push_str(".sav");
            let fp = File::create(name);
    
            match fp {
                Ok(mut file) => {
                    match file.write_all(&self.sram) {
                        Ok(_) => println!("SRAM data saved"),
                        Err(er) => println!("WARNING: Saving SRAM data failed -> {}", er)
                    } 
                },
                Err(er) => println!("WARNING: Saving SRAM data failed -> {}", er)
            }
        }
    }
}