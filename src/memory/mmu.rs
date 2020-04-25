pub struct MMU {
    // memory: [u8; 0xFFFF+1],
    
    cartrigbe: [u8; 0x8000],
    vram: [u8; 0x2000],
    sram: [u8; 0x2000],
    wram: [u8; 0x2000],
    echo: [u8; 0x1E00],
    oam: [u8; 0xA0],
    io: [u8; 0x4C],
    hram: [u8; 0x7F], //stack
    interrupt: u8

}

impl Default for MMU {
    fn default() -> MMU {
        let mut mem = MMU { 
            cartrigbe: [0; 0x8000],
            vram: [0; 0x2000],
            sram: [0; 0x2000],
            wram: [0; 0x2000],
            echo: [0; 0x1E00],
            oam: [0; 0xA0],
            io: [0; 0x4C],
            hram: [0; 0x7F],
            interrupt: 0
        };

        for i in 0..256 {
            mem.cartrigbe[i] = BOOT_SEQUENCE[i];
        }

        return mem;
        // MMU { memory: [0; 0xFFFF] }
    }
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


impl MMU {
    fn classify(address: u16) -> Region{
        match address {
            0..=0x7FFF => Region::Cartrigbe,    //32kb
            0x8000..=0x9FFF => Region::VRAM,    //8kb
            0xA000..=0xBFFF => Region::SRAM,    //8kb
            0xC000..=0xDFFF  => Region::WRAM,   //8kb
            0xE000..=0xFDFF => Region::Echo,    //8kb
            0xFE00..=0xFE9F => Region::OAM,     
            0xFEA0..=0xFEFF => Region::Unsable,
            0xFF00..=0xFF4B => Region::IO,
            0xFF4C..=0xFF7F => Region::GbcIO, 
            0xFF80..=0xFFFE => Region::Stack,   //127 bytes
            0xFFFF => Region::Interrupt         //1 byte
        }
    }
    fn simple_addr(address: u16, from: Region) -> usize{
        match from {
            Region::Cartrigbe => address as usize,
            Region::VRAM => (address - 0x8000) as usize,
            Region::SRAM => (address - 0xA000) as usize,
            Region::WRAM => (address - 0xC000) as usize,
            Region::Echo => (address - 0xE000) as usize,
            Region::OAM => (address - 0xFE00)  as usize,
            Region::Unsable => (address - 0xFEA0)  as usize, //there's no reason to use this conversion
            Region::IO =>  (address - 0xFF00)  as usize,
            Region::GbcIO => (address - 0xFF4C)  as usize,
            Region::Stack => (address - 0xFF80)  as usize,
            Region::Interrupt => 0xFFFF  as usize,
        }
    }

    pub fn convert(address: u16) -> usize{
        let portion = MMU::classify(address);
        MMU::simple_addr(address, portion)
    }

    pub fn write_byte(&mut self, address: u16, byte: u8 ){
        match MMU::classify(address) {
            Region::Cartrigbe => {
                self.cartrigbe[address as usize] = byte;
            },
            Region::VRAM => {
                self.vram[MMU::convert(address)] = byte;
            },
            Region::SRAM => {
                self.sram[MMU::convert(address)] = byte;
            },
            Region::WRAM => {
                self.wram[MMU::convert(address)] = byte;
            },
            Region::Echo => {
                self.echo[MMU::convert(address)] = byte;
            },
            Region::OAM => {
                self.oam[MMU::convert(address)] = byte;
            },
            Region::Unsable => {},
            Region::IO =>  {
                self.io[MMU::convert(address)] = byte;
            },
            Region::GbcIO => {},
            Region::Stack => {
                self.hram[MMU::convert(address)] = byte;
            },
            Region::Interrupt => {0xFFFF;}
        }
    }

    fn validade_short(address: u16) -> bool{
        match MMU::classify(address) {
            Region::Cartrigbe => address < 0x7FFF,
            Region::VRAM => address < 0x9FFF,
            Region::SRAM => address < 0xBFFF,
            Region::WRAM => address < 0xDFFF,
            Region::Echo => address < 0xFDFF,
            Region::OAM => address < 0xFE9F,
            Region::Unsable => false,
            Region::IO => address < 0xFF4B,
            Region::GbcIO => false,
            Region::Stack => address < 0xFFFE,
            Region::Interrupt => false
        }

    }

    pub fn write_short(&mut self, address: u16, short: u16){
        if MMU::validade_short(address){
            self.write_byte(address, short as u8);
            self.write_byte(address+1, (short >> 8) as u8);
        } else {
            println!("ILLEGAL ATTEMPT TO WRITE SHORT\n");
            std::process::exit(1);
        }

    }
    pub fn read_byte(&self, address: u16) -> u8 {
        match MMU::classify(address) {
            Region::Cartrigbe => self.cartrigbe[address as usize],
            Region::VRAM => self.vram[MMU::convert(address)],
            Region::SRAM => self.sram[MMU::convert(address)],
            Region::WRAM => self.wram[MMU::convert(address)],
            Region::Echo => self.echo[MMU::convert(address)],
            Region::OAM => self.oam[MMU::convert(address)],
            Region::Unsable => {return 0xFF},
            Region::IO => self.io[MMU::convert(address)],
            Region::GbcIO => {return 0xFF},
            Region::Stack => self.hram[MMU::convert(address)],
            Region::Interrupt => self.interrupt
        }
    }
    pub fn read_short(&self, address: u16) -> u16{
        if MMU::validade_short(address){
            self.read_byte(address) as u16 | (self.read_byte(address) as u16) << 8 //>
        } else {
            println!("ILLEGAL ATTEMPT TO READ SHORT\n");
            std::process::exit(1);
        }
    }
    pub fn copy(&mut self, dest: u16, source: u16, lenght: u16){
        for i in 0..lenght {
            self.write_byte(dest + i, self.read_byte(source + i));
        }
    }

    pub fn to_short_lsb(mut x: [u8;2]) -> u16{
        x.reverse();
        x[0] as u16 | (x[1] as u16) << 8 //>
    }


    pub fn push_range(&mut self, data: &Vec<u8>, start: usize, lenght: usize ){

        for i in start..lenght {
            self.write_byte(i as u16, data[i]);
        }
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

