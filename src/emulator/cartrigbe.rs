mod header;
mod bank;

use bank::Bank;
use header::Header;
use std::fs::File;
use std::io::Read;

pub struct Cartrigbe {
    pub info: Header,
    pub banks: Vec<Bank>,
    pub total_banks: usize,
    pub register: u8,
    pub has_ram: bool,
    pub ram_is_on: bool,
    content: Vec<u8>,
    content_size: usize
}

impl Default for Cartrigbe {
    fn default() -> Self {
        Cartrigbe {
            info: Header::default(),
            banks: vec![],
            total_banks: 0,
            register: 0,
            has_ram: false,
            ram_is_on: false,
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

    }
}