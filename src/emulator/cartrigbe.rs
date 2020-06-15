mod header;
mod bank;

use bank::Bank;
use header::Header;
use std::fs::File;
use std::io::Read;
use super::cpu::registers::{Response};

#[derive(PartialEq)]
enum Mode {
    RAM,
    ROM
}

pub struct Cartrigbe {
    info: Header,
    banks: Vec<Bank>,
    total_banks: usize,
    current_bank: usize,
    mode: Mode,
    pub bios_on: u8,
    ram_bank: u8,
    has_ram: bool,
    ram_enable: bool,
    content: Vec<u8>,
    content_size: usize
}

impl Default for Cartrigbe {
    fn default() -> Self {
        Cartrigbe {
            info: Header::default(),
            banks: vec![],
            total_banks: 0,
            current_bank: 1,
            mode: Mode::ROM,
            bios_on: 0,
            ram_bank: 0,
            has_ram: false,
            ram_enable: false,
            content: vec![],
            content_size: 0
        }
    }
}

impl Cartrigbe {
    pub fn insert(&mut self, fname: String){

        let file = File::open(fname);

        match file {
            Ok(file) => {
                for byte in file.bytes() {
                    self.content.push(byte.unwrap());
                }
                self.content_size = self.content.len();
                self.process();
            }
            Err(file) => {
                println!("{}. Exiting", file);
                std::process::exit(1);
            }
        }
    }

    fn get_total_banks(&mut self){
        self.total_banks = self.info.convert_to_rom_banks();
    }

    fn assign_banks(&mut self){
        for i in 0..self.total_banks {
            let mut bank_array: [u8; 0x4000] = [0; 0x4000];
            bank_array.copy_from_slice(&self.content[i*0x4000..0x4000*(i+1)]);
            self.banks.push(
                Bank{
                    id: i as u32,
                    is_ram: false,
                    info: bank_array
                }
            )
        }
    }

    fn process(&mut self){
        //write header

        for i in 0x0134..=0x0143{
            self.info.title.push(self.content[i] as char)
        }

        for i in 0x013F..=0x0142 {
            self.info.manufacturer.push(self.content[i] as char); 
        }

        for i in 0x0100..=0x0103 {
            self.info.entry[(i - 0x0100) as usize] = self.content[i]; 
        }

        self.info.cgb_flag = self.content[0x0143];
        self.info.sgb_flag = self.content[0x0146];
        self.info.ctype = self.content[0x0147];
        self.info.rom_size = self.content[0x0148];
        self.info.ram_size = self.content[0x0149];
        self.info.japan = self.content[0x014A];
        self.info.license = self.content[0x0144] as u16 | (self.content[0x0145] as u16) << 8; //>// change to read short function
        self.info.old_license = self.content[0x014B];
        self.info.version = self.content[0x014C];
        self.info.hchecksum = self.content[0x014D];
        self.info.gchecksum = self.content[0x014E] as u16 | (self.content[0x014F] as u16) << 8; //>// change to read short function

        if self.info.ram_size != 0 {
            self.has_ram = true;
        }

        self.get_total_banks();
        self.assign_banks();
        
        if self.info.ctype > 2 { panic!("ONLY MBC1 and default ROM implemented")}

        println!("{}", self.info);
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        match addr {
            0 ..= 0x1FFF => {
                if (byte & 0x0A) == 0x0A {
                    self.ram_enable = true;
                } else {
                    self.ram_enable = false;
                }
            },

            0x2000 ..= 0x3FFF => {
                let select = byte & 0x1F;
                self.current_bank = self.current_bank & 0x60;

                match select {
                    0 | 0x20 | 0x40 | 0x60 => self.current_bank |= (select+1) as usize,

                    _ => self.current_bank |= select as usize
                }
            },
            0x4000 ..= 0x5FFF => {
                if self.mode == Mode::RAM {
                    self.ram_bank = byte;
                } else {
                    self.current_bank = self.current_bank & !(0x60);
                    self.current_bank = self.current_bank | (byte as usize) << 5; //>
                }
            },
            0x6000 ..= 0x7FFF => {
                if byte == 1 {
                    self.mode = Mode::RAM;
                } else {
                    self.mode = Mode::ROM;
                }
            },
            0xA000 ..= 0xBFFF => {
                
                
            },
            _ => panic!("wrong addr to cartrigbe {}", addr)
        }

        if self.current_bank > self.total_banks {
            panic!("Tried to access unreachable bank");
        }
    }

    pub fn read_byte(&self, addr: u16) -> Response {
        match addr {
            0 ..= 0x3FFF => {
                if self.bios_on == 0 && addr < 256 {
                    return Response::Byte( BIOS[addr as usize] );
                } else {
                    return Response::Byte( self.banks[0].info[addr as usize] ); 
                }
            },
            0x4000 ..= 0x7FFF => {
                return Response::Byte( self.banks[ self.current_bank as usize ].info[ (addr - 0x4000) as usize ] );
            },
            0xA000 ..= 0xBFFF => { 
                if self.has_ram {

                    match self.mode {
                        Mode::RAM => {
                            // Responde::Byte( self. )
                        },
                        Mode::ROM => {

                        },
                    }
                    
                }    
                Response::Byte(0xFF) 
            
            },
            _ => panic!("Wrong address {:#10x}", addr)
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