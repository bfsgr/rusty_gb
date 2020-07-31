#![allow(dead_code)] //unused constants will be used when sound is implemented

pub const JOYP: u16   = 0xFF00; // Joypad info

pub const SB: u16   = 0xFF01; // Serial transfer data
pub const SC: u16   = 0xFF02; // Serial transfer Control

pub const DIV: u16  = 0xFF04; // Divider Register
pub const TIMA: u16 = 0xFF05; // Timer counter
pub const TMA: u16  = 0xFF06; // Timer Modulo
pub const TAC: u16  = 0xFF07; // Timer Control

pub const IF: u16   = 0xFF0F; // Interrupt Flag

pub const NR10: u16 = 0xFF10; // Sound Mode 1 sweep
pub const NR11: u16 = 0xFF11; // Sound Mode 1 wave pattern
pub const NR12: u16 = 0xFF12; // Sound Mode 1 envelope
pub const NR13: u16 = 0xFF13; // Sound Mode 1 frequency low
pub const NR14: u16 = 0xFF14; // Sound Mode 1 frequency high
pub const NR21: u16 = 0xFF16; // Sound Mode 2 wave pattern
pub const NR22: u16 = 0xFF17; // Sound Mode 2 envelope
pub const NR23: u16 = 0xFF18; // Sound Mode 2 frequency low
pub const NR24: u16 = 0xFF19; // Sound Mode 2 frequency high
pub const NR30: u16 = 0xFF1A; // Sound Mode 3 sound on/off
pub const NR31: u16 = 0xFF1B; // Sound Mode 3 sound length
pub const NR32: u16 = 0xFF1C; // Sound Mode 3 output level
pub const NR33: u16 = 0xFF1D; // Sound Mode 3 frequency low
pub const NR34: u16 = 0xFF1E; // Sound Mode 3 frequency high
pub const NR41: u16 = 0xFF20; // Sound Mode 4 sound length
pub const NR42: u16 = 0xFF21; // Sound Mode 4 envelope
pub const NR43: u16 = 0xFF22; // Sound Mode 4 polynomial counter
pub const NR44: u16 = 0xFF23; // Sound Mode 4 counter/consecutive
pub const NR50: u16 = 0xFF24; // Channel Control/Volume
pub const NR51: u16 = 0xFF25; // Sound output terminal
pub const NR52: u16 = 0xFF26; // Sound on/off

// FF30 - FF3F Wave pattern RAM

pub const LCDC: u16    = 0xFF40; // LCD Control
pub const STAT: u16    = 0xFF41; // LCD Status
pub const SCY: u16     = 0xFF42; // Scroll Y
pub const SCX: u16     = 0xFF43; // Scroll X
pub const LY: u16      = 0xFF44; // LCD Y coordinate
pub const LYC: u16     = 0xFF45; // LY Compare
pub const OAM_DMA: u16 = 0xFF46; // DMA transfer/start address
pub const BGP: u16     = 0xFF47; // Background/Window palette data
pub const OBP0: u16    = 0xFF48; // Object Palette 0 data
pub const OBP1: u16    = 0xFF49; // Object Palette 1 data
pub const WY: u16      = 0xFF4A; // Window Y position
pub const WX: u16      = 0xFF4B; // Window X position

pub const BROM: u16    = 0xFF50; //Boot Rom enable register

pub const IE: u16      = 0xFFFF; // Interrupt Enable

