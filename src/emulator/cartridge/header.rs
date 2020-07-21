#![allow(non_snake_case)]

#[derive(Default)]
pub struct Header {
    pub title: String,
    pub cartridge_type: u8,
    GCB_flag: u8,
    SGB_flag: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    header_checksum: u8,
    global_checksum: u16
}

impl Header {
    pub fn parse(data: &Vec<u8>) -> Self {

        let mut header = Header::default();

        for i in 0x0134..=0x0143{
            let c = data[i];
            match c {
                0 => {},
                _ => header.title.push(data[i] as char)
            }
        }

        header.GCB_flag = data[0x0143];
        header.SGB_flag = data[0x0146];
        header.cartridge_type = data[0x0147];
        header.rom_size = data[0x0148];
        header.ram_size = data[0x0149];
        header.header_checksum = data[0x014D];
        header.global_checksum = data[0x014E] as u16 | (data[0x014F] as u16) << 8; //>

        Self::validate(&header, &data);

        return header;
    }

    fn validate(header: &Header, data: &Vec<u8>) {
        if (header.GCB_flag & 0xC0 ) == 0xC0 { panic!("Game is GameBoy Color only") }

        let mut x: u8 = 0;

        for i in 0x0134 ..= 0x014C {
            x = x.wrapping_sub(data[i]).wrapping_sub(1);
        }

        if x != header.header_checksum { panic!("Header checksum doesn't match") }


        //at this point we have a valid cartrigbe
    }

    pub fn has_battery(&self) -> bool {
        match self.cartridge_type {
            3 => true,
            _ => false
        }
    }
}