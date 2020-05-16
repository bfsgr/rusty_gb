pub const JOYP: u16   = 0x00; // Joypad info

pub const SB: u16   = 0x01; // Serial transfer data
pub const SC: u16   = 0x02; // Serial transfer Control

pub const DIV: u16  = 0x04; // Divider Register
pub const TIMA: u16 = 0x05; // Timer counter
pub const TMA: u16  = 0x06; // Timer Modulo
pub const TAC: u16  = 0x07; // Timer Control

pub const IF: u16   = 0x0F; // Interrupt Flag

pub const NR10: u16 = 0x10; // Sound Mode 1 sweep
pub const NR11: u16 = 0x11; // Sound Mode 1 wave pattern
pub const NR12: u16 = 0x12; // Sound Mode 1 envelope
pub const NR13: u16 = 0x13; // Sound Mode 1 frequency low
pub const NR14: u16 = 0x14; // Sound Mode 1 frequency high
pub const NR21: u16 = 0x16; // Sound Mode 2 wave pattern
pub const NR22: u16 = 0x17; // Sound Mode 2 envelope
pub const NR23: u16 = 0x18; // Sound Mode 2 frequency low
pub const NR24: u16 = 0x19; // Sound Mode 2 frequency high
pub const NR30: u16 = 0x1A; // Sound Mode 3 sound on/off
pub const NR31: u16 = 0x1B; // Sound Mode 3 sound length
pub const NR32: u16 = 0x1C; // Sound Mode 3 output level
pub const NR33: u16 = 0x1D; // Sound Mode 3 frequency low
pub const NR34: u16 = 0x1E; // Sound Mode 3 frequency high
pub const NR41: u16 = 0x20; // Sound Mode 4 sound length
pub const NR42: u16 = 0x21; // Sound Mode 4 envelope
pub const NR43: u16 = 0x22; // Sound Mode 4 polynomial counter
pub const NR44: u16 = 0x23; // Sound Mode 4 counter/consecutive
pub const NR50: u16 = 0x24; // Channel Control/Volume
pub const NR51: u16 = 0x25; // Sound output terminal
pub const NR52: u16 = 0x26; // Sound on/off

// FF30 - FF3F Wave pattern RAM

pub const LCDC: u16    = 0x40; // LCD Control
pub const STAT: u16    = 0x41; // LCD Status
pub const SCY: u16     = 0x42; // Scroll Y
pub const SCX: u16     = 0x43; // Scroll X
pub const LY: u16      = 0x44; // LCD Y coordinate
pub const LYC: u16     = 0x45; // LY Compare
pub const OAM_DMA: u16 = 0x46; // DMA transfer/start address
pub const BGP: u16     = 0x47; // Background/Window palette data
pub const OBP0: u16    = 0x48; // Object Palette 0 data
pub const OBP1: u16    = 0x49; // Object Palette 1 data
pub const WY: u16      = 0x4A; // Window Y position
pub const WX: u16      = 0x4B; // Window X position

pub const IE: u16      = 0xFF; // Interrupt Enable

