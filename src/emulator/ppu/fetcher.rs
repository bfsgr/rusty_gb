use super::pixelFIFO::PixelFIFO;

pub const MAP_ADDR_BASE: u16 = 0x9800;
pub const MAP_ADDR_UPPER: u16 = 0x9C00;
pub const DATA_ADDR_UPPER: u16 = 0x9000;
pub const DATA_ADDR_BASE: u16 = 0x8000;

pub struct Fetcher {
    tile_num: usize,
    tile_data_0: u8,
    tile_data_1: u8,
    //address registers
    pub tile_map_addr: u16,
    pub tile_data_addr: u16,
    //pixel coordinates
    px: u8,
    py: u8,                // => LY register
    //public registers

    pub bgp: u8,     //0xFF47     (R/W)
    pub obp0: u8,    //0xFF48     (R/W)
    pub obp1: u8,    //0xFF49     (R/W)
    
    pub dma: u8,        //0xFF46     (R/W)
    //memory
    pub vram: [u8; 0x2000],
    pub oam: [u8; 0xA0],

    display: Vec<u32>
}

impl Default for Fetcher {
    fn default() -> Self {
        Self {
            tile_num: 0,
            tile_data_0: 0,
            tile_data_1: 0,
            tile_map_addr: MAP_ADDR_BASE,
            tile_data_addr: DATA_ADDR_UPPER,
            px: 0,
            py: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            dma: 0,
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            display: vec![0; 160*144]
        }
    }
}

impl Fetcher {
    pub fn line(&self) -> &u8 {
        return &self.py;
    }


}