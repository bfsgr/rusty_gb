#![allow(non_snake_case)]

use std::fmt;

const ZERO_FLAG: u8 = 0b10000000;
const NEGATIVE_FLAG: u8 = 0b01000000;
const HALFCARRY_FLAG: u8 = 0b00100000;
const CARRY_FLAG: u8 = 0b00010000;

pub struct Node { 
    pub T: u8,  //Top half of 16 bit register
    pub L: u8   //Lower half of 16 bit register
}

pub trait Operations {
    fn r16b(&self) -> u16;
    fn w16b(&mut self, mut ax: u16){}
    fn decrement(&mut self);
}

pub trait BitTest {
    fn test(&self, bit: u8) -> bool;
}

impl BitTest for u8 {
    //test if bit is zero
    fn test(&self, bit: u8) -> bool {
        let b = 1 << bit; //>
        (self & b) != b
    }
}
impl BitTest for u16 {
    //test if bit is zero
    fn test(&self, bit: u8) -> bool {
        let b = 1 << bit; //>
        (self & b) != b
    }
}

impl BitTest for Node {
    fn test(&self, bit: u8) -> bool {
        let short = self.r16b(); 
        let b = 1 << bit; //>
        (short & b) != b
    }
}

impl Operations for Node {
    fn r16b(&self) -> u16{
        return (self.T as u16) << 8 | self.L as u16; //> 
    }
    fn w16b(&mut self, mut ax: u16){
        self.L = ax as u8;
        ax = ax >> 8; 
        self.T = ax as u8;
    }
    fn decrement(&mut self){
        self.w16b(self.r16b() - 1);
    }
}
pub struct Registers {
    pub AF: Node, //F should be FLAGS register FIX
    pub BC: Node, 
    pub DE: Node,
    pub HL: Node,
    pub SP: u16,
    pub PC: u16
}

impl fmt::Display for Registers{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A: {:#04x}\nB: {:#04x}\nC: {:#04x}\nD: {:#04x}\nE: {:#04x}\nF: {:#04x}\nH: {:#04x}\nL: {:#04x}\nAF: {:#04x}\nBC: {:#04x}\nDE: {:#04x}\nHL: {:#04x}\nSP: {:#04x}\nPC: {:#04x}\nFLAGS: {:#04x}", self.AF.T, self.BC.T, self.BC.L, self.DE.T, self.DE.L, self.AF.L, self.HL.T, self.HL.L, self.AF.r16b(), self.BC.r16b(), self.DE.r16b(), self.HL.r16b(), self.SP, self.PC, self.AF.L)
    }
}
impl fmt::Debug for Registers{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A: {:#010b}\nB: {:#010b}\nC: {:#010b}\nD: {:#010b}\nE: {:#010b}\nF: {:#010b}\nH: {:#010b}\nL: {:#010b}\nAF: {:#018b}\nBC: {:#018b}\nDE: {:#018b}\nHL: {:#018b}\nSP: {:#018b}\nPC: {:#018b}\nFLAGS: {:#010b}", self.AF.T, self.BC.T, self.BC.L, self.DE.T, self.DE.L, self.AF.L, self.HL.T, self.HL.L, self.AF.r16b(), self.BC.r16b(), self.DE.r16b(), self.HL.r16b(), self.SP, self.PC, self.AF.L)
    }
}

impl Default for Registers{
    fn default() -> Registers{
        Registers{
            AF: Node{T: 0, L:0},
            BC: Node{T: 0, L:0}, 
            DE: Node{T: 0, L:0},
            HL: Node{T: 0, L:0},
            SP: 0,
            PC: 0
        }
    }
}

impl Registers {
    pub fn set_flag(&mut self, bit: u8) -> bool{
        match bit {
            1 => self.AF.L = self.AF.L | ZERO_FLAG, //Zero Flag
            2 => self.AF.L = self.AF.L | NEGATIVE_FLAG, //Negative Flag
            3 => self.AF.L = self.AF.L | HALFCARRY_FLAG, //Halfcarry Flag
            4 => self.AF.L = self.AF.L | CARRY_FLAG, //Carry Flag
            _ => return false
        }
        return true;
    }
    
    pub fn clear_flag(&mut self, bit: u8) -> bool{
        match bit {
            1 => self.AF.L = self.AF.L & !ZERO_FLAG, //Zero Flag
            2 => self.AF.L = self.AF.L & !NEGATIVE_FLAG, //Negative Flag
            3 => self.AF.L = self.AF.L & !HALFCARRY_FLAG, //Halfcarry Flag
            4 => self.AF.L = self.AF.L & !CARRY_FLAG, //Carry Flag
            _ => return false
        }
        return true;
    }



}
