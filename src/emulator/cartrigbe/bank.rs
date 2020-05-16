use std::fmt;
use std::fmt::Display;

pub struct Bank {
    pub id: u32,
    pub is_ram: bool,
    pub info: [u8; 0x4000]
}

impl Display for Bank{
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        write!(f, "Id: {}\nRAM: {}\nFirst 10B: {:x?}", self.id, self.is_ram, &self.info[0..10])
    }
}