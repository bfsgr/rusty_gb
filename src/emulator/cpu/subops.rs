use super::Instruction;
use crate::emulator::cpu::registers::*;
use crate::emulator::Bus;
// use crate::emulator::bit_utils::BitUtils;

// pub fn BIT_4H(_operands: [u8; 2], registers: &mut Registers, _mem: &mut Bus) -> u8{
//     let val: u8 = registers.H( Action::Read ).value();
//     Instruction::BIT(val, 4, registers);
//     return 8;
// }

macro_rules! bit_nr {
    ( $( $name:ident,$num:expr,$r:ident ),* ) => {
        $( 
            pub fn $name(_inst: &mut Instruction, registers: &mut Registers, _mem: &mut Bus) {
                let val: u8 = registers.$r( Action::Read ).value();
                Instruction::BIT(val, $num, registers);
            }
        )*
    }
}

impl Instruction {
    bit_nr!(
        BIT_0B, 0, B,  BIT_0C, 0, C, BIT_0D, 0, D,  BIT_0E, 0, E, BIT_0H, 0, H,  BIT_0L, 0, L, BIT_0A, 0, A,
        BIT_1B, 1, B,  BIT_1C, 1, C, BIT_1D, 1, D,  BIT_1E, 1, E, BIT_1H, 1, H,  BIT_1L, 1, L, BIT_1A, 1, A,
        BIT_2B, 2, B,  BIT_2C, 2, C, BIT_2D, 2, D,  BIT_2E, 2, E, BIT_2H, 2, H,  BIT_2L, 2, L, BIT_2A, 2, A,
        BIT_3B, 3, B,  BIT_3C, 3, C, BIT_3D, 3, D,  BIT_3E, 3, E, BIT_3H, 3, H,  BIT_3L, 3, L, BIT_3A, 3, A,
        BIT_4B, 4, B,  BIT_4C, 4, C, BIT_4D, 4, D,  BIT_4E, 4, E, BIT_4H, 4, H,  BIT_4L, 4, L, BIT_4A, 4, A,
        BIT_5B, 5, B,  BIT_5C, 5, C, BIT_5D, 5, D,  BIT_5E, 5, E, BIT_5H, 5, H,  BIT_5L, 5, L, BIT_5A, 5, A,
        BIT_6B, 6, B,  BIT_6C, 6, C, BIT_6D, 6, D,  BIT_6E, 6, E, BIT_6H, 6, H,  BIT_6L, 6, L, BIT_6A, 6, A,
        BIT_7B, 7, B,  BIT_7C, 7, C, BIT_7D, 7, D,  BIT_7E, 7, E, BIT_7H, 7, H,  BIT_7L, 7, L, BIT_7A, 7, A
    );
}