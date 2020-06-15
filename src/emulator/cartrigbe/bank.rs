
#[derive(PartialEq)]
enum Mode {
    ROM,
    RAM
}
impl Default for Mode { fn default() -> Self { Mode::ROM } }

#[derive(Default)]
pub struct BankController {
    mode: Mode,
    ram_enable: bool,
    rom_bank: u8,
    ram_bank: u8,
    pub bios_control: u8,
    sram: Vec<u8>,
    data: Vec<u8>,
    data_size: usize,
}
//this is MCB1 only
impl BankController {
    pub fn save_to_memory(&mut self, data: Vec<u8>, has_ram: bool) {
        if has_ram { self.sram = vec![0;0x8000]; }
        self.rom_bank = 1;
        self.data = data;
        self.data_size = self.data.len();
    }

    pub fn write(&mut self, addr: u16, mut byte: u8) {
        match addr {
            0 ..= 0x1FFF => {
                if (byte & 0xA0) == 0xA0 {
                    self.ram_enable = true;
                } else {
                    self.ram_enable = false;
                }
            },
            0x2000 ..= 0x3FFF => {
                //only use 5 bits of byte
                byte = byte & 0x1F;
                //clears all bits of rom_bank except bits 5 and 6
                self.rom_bank = self.rom_bank & 0xE0;

                self.rom_bank |= byte;

                self.fix_rom_bank();
            },
            0x4000 ..= 0x5FFF => {
                if self.mode == Mode::RAM {
                    self.ram_bank = byte & 3; 
                } else {
                    self.rom_bank = self.rom_bank & !0xE0;
                    byte = (byte & 0x3) << 5; //>
                    self.rom_bank |= byte;
                    self.fix_rom_bank();
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
                if !self.ram_enable { return (); }

                let offset = (0x2000 * self.ram_bank as usize) + (addr - 0xA000) as usize;

                self.sram[offset] = byte;
            },

            _ => {panic!("Write to illegal cartrigbe position")}
        }
    }

    pub fn read(&self, addr: u16) -> u8 {

        match addr {
            0 ..= 0x3FFF => {
                if self.bios_control == 0 && addr < 256 {
                    return BIOS[addr as usize];
                } else {
                    return self.data[addr as usize]
                }
            },

            0x4000 ..= 0x7FFF => {
                let adjusted = ((addr - 0x4000) as usize) + (0x4000 * self.rom_bank as usize);
                return self.data[adjusted as usize];
            }

            0xA000 ..= 0xBFFF => {
                if !self.ram_enable {
                    return 0xFF;
                }

                let offset = (0x2000 * self.ram_bank as usize) + (addr - 0xA000) as usize;

                return self.sram[offset];
            }

            _ => { panic!("Wrong cartrigbe address") }
        }   



    }

    fn fix_rom_bank(&mut self) {
        // let number = self.rom_bank & 0x1F;

        match self.rom_bank {
            0 | 0x20 | 0x40 | 0x60 => self.rom_bank += 1,
            _ => {},
        }
    }
}

const BIOS: [u8; 256]= [0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 
0xFB,0x21, 0x26, 0xFF, 0x0E, 0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77,
0x77, 0x3E, 0xFC, 0xE0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD,
0x96, 0x00, 0x13, 0x7B, 0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22,
0x23, 0x05, 0x20, 0xF9, 0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 
0x08, 0x32, 0x0D, 0x20, 0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E,
0x91, 0xE0, 0x40, 0x04, 0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20,
0xF7, 0x1D, 0x20, 0xF2, 0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1,
0xFE, 0x64, 0x20, 0x06, 0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 
0x20, 0xD2, 0x05, 0x20, 0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17,
0xC1, 0xCB, 0x11, 0x17, 0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 
0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 
0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 
0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 
0xB9, 0xA5, 0x42, 0x3C, 0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xfe, 0x23, 
0x7D, 0xFE, 0x34, 0x20, 0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xfe, 
0x3E, 0x01, 0xE0, 0x50];