#![allow(non_snake_case)]

#[derive(Default)]
pub struct Header {
    title: String,
    manufacturer_code: String,
    cartrigbe_type: u8,
    license_code: u16,
    GCB_flag: u8,
    SGB_flag: u8,
    rom_size: u8,
    pub ram_size: u8,
    jp_flag: u8,
    old_license: u8,
    version: u8,
    header_checksum: u8,
    global_checksum: u16
}

impl Header {
    pub fn parse(&mut self, data: &Vec<u8>) -> (String, u8, u8, u8) {

        for i in 0x0134..=0x0143{
            let c = data[i];
            match c {
                0 => {},
                _ => self.title.push(data[i] as char)
            }
        }

        for i in 0x013F..=0x0142 {
            self.manufacturer_code.push(data[i] as char); 
        }

        self.GCB_flag = data[0x0143];
        self.SGB_flag = data[0x0146];
        self.cartrigbe_type = data[0x0147];
        self.rom_size = data[0x0148];
        self.ram_size = data[0x0149];
        self.jp_flag = data[0x014A];
        self.license_code = data[0x0144] as u16 | (data[0x0145] as u16) << 8; //>
        self.old_license = data[0x014B];
        self.version = data[0x014C];
        self.header_checksum = data[0x014D];
        self.global_checksum = data[0x014E] as u16 | (data[0x014F] as u16) << 8; //>

        self.validate(data);

        (self.title.clone(), self.cartrigbe_type, self.rom_size, self.ram_size)
    }

    fn validate(&self, data: &Vec<u8>) {
        if (self.GCB_flag & 0xC0 ) == 0xC0 { panic!("Game is GameBoy Color only") }

        let mut x: u8 = 0;

        for i in 0x0134 ..= 0x014C {
            x = x.wrapping_sub(data[i]).wrapping_sub(1);
        }

        if x != self.header_checksum { panic!("Header checksum doesn't match") }


        //at this point we have a valid cartrigbe
    }
}