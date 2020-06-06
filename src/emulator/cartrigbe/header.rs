use std::fmt;
use std::fmt::Display;


pub struct Header {
    pub title: String,
    pub manufacturer: String,
    pub cgb_flag: u8,
    pub sgb_flag: u8,
    pub ctype: u8,
    pub license: u16,
    pub rom_size: u8,
    pub ram_size: u8,
    pub japan: u8,
    pub old_license: u8,
    pub version: u8,
    pub hchecksum: u8,
    pub gchecksum: u16,
    pub entry: [u8; 4]
}

impl Default for Header {
    fn default() -> Self {
        Header {
            title: "".to_string(),
            manufacturer: "".to_string(),
            cgb_flag: 0,
            sgb_flag: 0,
            ctype: 0,
            license: 0,
            rom_size: 0,
            ram_size: 0,
            japan: 0,
            old_license: 0,
            version: 0,
            hchecksum: 0,
            gchecksum: 0,
            entry: [0; 4]
        }
    }
}

impl Header {
    pub fn convert_to_rom_banks(&self) -> usize {
        match self.rom_size {
            0x0 => 2,
            0x1 => 4,
            0x2 => 8,
            0x3 => 16,
            0x4 => 32,
            0x5 => 64,
            0x6 => 128,
            0x7 => 256,
            0x8 => 512,
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => 0
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Title: {}\nManufacturer: {}\nCGB flag: {:#04x}\nSGB flag: {:#04x}\nJP flag: {:#04x}\nLicense: {:#04x}\nOld License: {:#04x}\nCartrigbe Type: {:#04x}\nROM size: {:#04x}\nRAM size: {:#04x}\nVersion: {:#04x}\nHeader checksum: {:#04x}\nGlobal checksum: {:#04x}\nRaw entry point: {:#04x?}", self.title, self.manufacturer, self.cgb_flag, self.sgb_flag, self.japan, self.license, self.old_license, self.ctype, self.rom_size, self.ram_size, self.version, self.hchecksum, self.gchecksum, self.entry )
    }
}