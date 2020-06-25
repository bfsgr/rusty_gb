use super::super::mbcx::{*};

#[derive(Default)]
pub struct MBC1 {
    title: String,
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
                    self.ram_bank = byte & 3; 
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
            2 => {},
            3 => {},
            _ => { panic!("Wrong type for MBC1: {:x}", ctype); }
        };
        
        self.rom_bank = 1;
        self.title = title;
        self.data = data;
    }
}