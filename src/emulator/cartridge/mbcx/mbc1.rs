use super::super::mbcx::{*};
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Default)]
pub struct MBC1 {
    title: String,
    battery: bool,
    mode: Mode,
    ram_on: bool,
    rom_bank: u8,
    ram_bank: u8,
    sram: Vec<u8>,
    data: Vec<u8>,
}

impl MBC for MBC1 {
    fn write(&mut self, addr: u16, mut byte: u8) {
        match addr {
            0 ..= 0x1FFF => {
                if (byte & 0xA0) == 0xA0 {
                    self.ram_on = true;
                } else {
                    self.ram_on = false;
                }
            },
            0x2000 ..= 0x3FFF => {
                //only use 5 bits of byte
                byte = byte & 0x1F;
                //clears all bits of rom_bank except bits 5 and 6
                self.rom_bank = self.rom_bank & 0xE0;

                self.rom_bank |= byte;

                self.rom_bank = fix_rom_bank(self.rom_bank);
            },
            0x4000 ..= 0x5FFF => {
                if self.mode == Mode::RAM {
                    self.ram_bank = byte;
                } else {
                    self.rom_bank = self.rom_bank & !0xE0;
                    byte = (byte & 0x3) << 5; //>
                    self.rom_bank |= byte;
                    self.rom_bank = fix_rom_bank(self.rom_bank);
                }
            },            
            0x6000 ..= 0x7FFF => {
                if byte == 0 {
                    self.mode = Mode::ROM
                } else {
                    self.mode = Mode::RAM
                }
            },            
            //SRAM
            0xA000 ..= 0xBFFF => {
                if !self.ram_on { return (); }

                let offset;
                if self.mode == Mode::RAM {
                    offset = (0x2000 * self.ram_bank as usize) + (addr - 0xA000) as usize;
                } else {
                    offset = (addr - 0xA000) as usize;
                }

                self.sram[offset] = byte;
            },
            _ => {}
        }
    }
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0 ..= 0x3FFF => {
                return self.data[addr as usize]
            },

            0x4000 ..= 0x7FFF => {
                
                let adjusted;
                if self.mode == Mode::ROM {
                    adjusted = ((addr - 0x4000) as usize) + (0x4000 * self.rom_bank as usize)
                } else {
                    adjusted = 0x4000 * self.rom_bank as usize;
                }
                return self.data[adjusted as usize];
            }

            0xA000 ..= 0xBFFF => {
                if !self.ram_on {
                    return 0xFF;
                }
                let offset;

                if self.mode == Mode::RAM {
                    offset = (0x2000 * self.ram_bank as usize) + (addr - 0xA000) as usize;
                } else {
                    offset = (addr - 0xA000) as usize;
                }

                self.sram[offset]
            }

            _ => { panic!("Wrong cartrigbe address") }
        }   

    }
    fn load(&mut self, title: String, ctype: u8, rom_size: u8, ram_size: u8, data: Vec<u8> ) {

        match ctype {
            1 => {
                let size = rom_to_size(rom_size);
                if ram_size != 0 { panic!("RAM size and cartrigbe type doesn't match") }
                if size != data.len() { panic!("Data size and doesn't match header info") }
            },
            2 => {
                let size = rom_to_size(rom_size);
                let rsize = ram_to_size(ram_size);
                if size != data.len() { panic!("Data size and doesn't match header info") }
                self.sram = vec![0; rsize];
            },
            3 => {
                let size = rom_to_size(rom_size);
                let rsize = ram_to_size(ram_size);
                if size != data.len() { panic!("Data size and doesn't match header info") }
                self.try_load(rsize);
                self.battery = true;
            },
            _ => { panic!("Wrong type for MBC1: {:x}", ctype); }
        };
        
        self.rom_bank = 1;
        self.title = title;
        self.data = data;
    }
}

impl MBC1 {
    fn try_load(&mut self, ram_size: usize) {
        let mut name = self.title.clone();
        name.push_str(".sav");
        let file = File::open(name);

        match file {
            Ok(mut file) => {
                let _ = file.read_to_end(&mut self.sram);
                if ram_size != self.sram.len() { panic!("Save has wrong size") }
            },
            Err(_) => {
                self.sram = vec![0; ram_size];
            }
        }
    }
}

impl Drop for MBC1 {
    fn drop(&mut self) {
        if self.battery {
            let mut name = self.title.clone();
            name.push_str(".sav");
            let mut file = File::create(name).unwrap();
            file.write_all(&self.sram).unwrap();
        }
    }
}