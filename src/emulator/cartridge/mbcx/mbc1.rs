use super::super::mbcx::{*};
use super::super::header::Header;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Default)]
pub struct MBC1 {
    header: Header,
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
                if (byte & 0x0A) == 0x0A {
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
                //only enable banking ram if ram size is 32kb or higher
                if self.mode == Mode::RAM && self.header.ram_size > 2 {
                    self.ram_bank = byte;
                } else {
                    //only enable rom banking if rom size is 1mb or higher
                    if self.header.rom_size > 4 {
                        self.rom_bank = self.rom_bank & !0xE0;
                        byte = (byte & 0x3) << 5; //>
                        self.rom_bank |= byte;
                        self.rom_bank = fix_rom_bank(self.rom_bank);
                    }
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
                    adjusted = (addr - 0x4000) as usize
                }
                return self.data[adjusted];
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

            _ => { panic!("Wrong cartridge address") }
        }   

    }
    fn load(&mut self, data: Vec<u8>, header: Header ) {
        self.header = header; //move header to controller

        match self.header.cartridge_type {
            1 => {
                let size = rom_to_size(self.header.rom_size);
                if self.header.ram_size != 0 { panic!("Cartridge type = 1 but ram_size != 0") }
                if size != data.len() { panic!("Data size doesn't match header") }
            },
            2 => {
                let size = rom_to_size(self.header.rom_size);
                let rsize = ram_to_size(self.header.ram_size);
                if size != data.len() { panic!("Data size doesn't match header") }
                self.sram = vec![0; rsize];
            },
            3 => {
                let size = rom_to_size(self.header.rom_size);
                if size != data.len() { panic!("Data size and doesn't match header info") }
                self.try_load();
            },
            _ => panic!("Wrong type for MBC1: {:x}", self.header.cartridge_type )
        }

        self.rom_bank = 1;
        self.data = data; //move data to controller
    }
}

impl MBC1 {
    fn try_load(&mut self) {
        let rsize = ram_to_size(self.header.ram_size);

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

impl Drop for MBC1 {
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