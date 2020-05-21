pub struct Map {
    pub cartrigbe: [u8; 0x8000],
    pub vram: [u8; 0x2000],
    sram: [u8; 0x2000],
    wram: [u8; 0x2000],
    echo: [u8; 0x1E00],
    oam: [u8; 0xA0],
    pub io: [u8; 0x4C],
    hram: [u8; 0x7F], //stack
    interrupt_enable: u8, //this is 0xFFFF (R/W)
    pub interrupt_switch: bool //this disable/enable interrupts
}

pub enum Region {
    Cartrigbe,  //ROM bank #0 + swichable bank
    VRAM,       //Vidio RAM
    SRAM,       //Switchable RAM
    WRAM,       //Internal RAM
    Echo,       //Echo of internal RAM 
    OAM,        //Object Attribute Memory
    Unsable,    //Unsable portions
    IO,         //Regions that map I/O registers
    Stack,      //High RAM intented to be used as stack portion
    Interrupt,  //Interrupt Enable Register (0xFFFF)
    GbcIO,     //Gameboy Color I/O registers reserved 

}

impl Map {
    pub fn write(&mut self, location: Region, address: u16, byte: u8){
        match location {
            Region::Cartrigbe => {
                self.cartrigbe[address as usize] = byte;
            },
            Region::VRAM => {
                self.vram[address as usize] = byte;
            },
            Region::SRAM => {
                self.sram[address as usize] = byte;
            },
            Region::WRAM => {
                self.wram[address as usize] = byte;
            },
            Region::Echo => {
                self.echo[address as usize] = byte;
                self.wram[address as usize] = byte;
            },
            Region::OAM => {
                self.oam[address as usize] = byte;
            },
            Region::IO => {
                self.io[address as usize] = byte;
            }
            // Region::GbcIO => {},
            Region::Stack => {
                self.hram[address as usize] = byte;
            },
            Region::Interrupt => {
                self.interrupt_enable = byte;
            }
            _ => {}
        }
    }

    pub fn read(&self, location: Region, address: u16) -> Option<u8>{
        match location {
            Region::Cartrigbe => {
                Some(self.cartrigbe[address as usize])
            },
            Region::VRAM => {
                Some(self.vram[address as usize])
            },
            Region::SRAM => {
                Some(self.sram[address as usize])
            },
            Region::WRAM => {
                Some(self.wram[address as usize])
            },
            Region::Echo => {
                Some(self.echo[address as usize])
            },
            Region::OAM => {
                Some(self.oam[address as usize])
            },
            Region::IO => {
                Some(self.io[address as usize])
            }
            // Region::GbcIO => {},
            Region::Stack => {
                Some(self.hram[address as usize])
            },
            Region::Interrupt => {
                Some(self.interrupt_enable)
            }
            _ => { None }
        }
    }

}


impl Default for Map {
    fn default() -> Map {
        let mut ret = Map {
            cartrigbe: [0; 0x8000],
            vram: [0; 0x2000],
            sram: [0; 0x2000],
            wram: [0; 0x2000],
            echo: [0; 0x1E00],
            oam: [0; 0xA0],
            io: [0; 0x4C],
            hram: [0; 0x7F],
            interrupt_enable: 0,
            interrupt_switch: false
        };
        
        for i in 0..256 {
            ret.cartrigbe[i] = BOOT_SEQUENCE[i];
        }

        ret
    }
}


const BOOT_SEQUENCE: [u8; 256]= [0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 
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
0xB9, 0xA5, 0x42, 0x3C, 0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x00, 0x00, 0x23, 
0x7D, 0xFE, 0x34, 0x20, 0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x00, 0x00, 
0x3E, 0x01, 0xE0, 0x50];
