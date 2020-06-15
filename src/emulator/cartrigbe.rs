mod header;
mod bank;
use bank::BankController;
use header::Header;
use std::fs::File;
use std::io::Read;
use super::cpu::registers::Response;

#[derive(Default)]
pub struct Cartrigbe {
    header: Header,
    controller: BankController,
}

impl Cartrigbe {
    pub fn insert(&mut self, fname: String) {
        let file = File::open(fname);

        let mut data: Vec<u8> = vec![];

        match file {
            Ok(file) => {
                for byte in file.bytes() {
                    data.push(byte.unwrap());
                }
            }
            Err(file) => {
                println!("{}. Exiting", file);
                std::process::exit(1);
            }
        }

        self.header.parse(&data);
        self.controller.save_to_memory(data, self.header.ram_size != 0);
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.controller.write(addr, byte);
    }

    pub fn read_byte(&self, addr: u16) -> Response {
        Response::Byte( self.controller.read(addr) )
    }

    pub fn bios_control(&mut self, byte: u8) {
        self.controller.bios_control = byte;
    }
}