const ZERO_FLAG: u8 = 0b10000000;
const NEGATIVE_FLAG: u8 = 0b01000000;
const HALFCARRY_FLAG: u8 = 0b00100000;
const CARRY_FLAG: u8 = 0b00010000;

#[derive(Copy, Clone)] //needed so it can be an union field
struct Pair{
    lsb: u8,
    msb: u8
}

union Register{
    Pair: Pair,
    all: u16
}

impl Default for Register {
    fn default() -> Register{
        Register{ all: 0x0 }
    }
}
pub struct Registers{
    AF: Register,
    BC: Register,
    DE: Register,
    HL: Register,
    SP: u16,
    PC: u16
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f,
           "AF: {:#10x}
            \rBC: {:#10x}
            \rDE: {:#10x}
            \rHL: {:#10x}
            \rSP: {:#10x}
            \rPC: {:#10x}\n", 
                self.AF.all,
                self.BC.all,
                self.DE.all,
                self.HL.all,
                self.SP,
                self.PC,
            
        )
        }
    }
}


impl Default for Registers {
    fn default() -> Registers{
        Registers{ 
            AF: Register::default(),
            BC: Register::default(),
            DE: Register::default(),
            HL: Register::default(),
            SP: 0,
            PC: 0x0
        }
    }
}

pub enum Action {
    Write(u16),
    Read,
    Decrement(u16),
    Increment(u16),
}
pub enum Response {
    None,
    Byte(u8),
    Short(u16),
}

pub trait Value<T> {
    fn value(self) -> T;
}

impl Value<u8> for Response {
    fn value(self) -> u8 {
        match self {
            Response::Byte(x) => x,
            _ => panic!("Error on consume response")
        }
    }
}

impl Value<u16> for Response {
    fn value(self) -> u16 {
        match self {
            Response::Short(x) => x,
            _ => panic!("Error on consume response")
        }
    }
}

impl Registers {
    pub fn AF(&mut self, wr: Action) -> Response {
        match wr {
            Action::Write(x) => {
                self.AF.all = x;
                return Response::None;
            },
            Action::Read => unsafe { Response::Short(self.AF.all) },
            Action::Decrement(x) => { unsafe { self.AF.all = self.AF.all.wrapping_sub(x); }; return Response::None; } ,
            Action::Increment(x) => { unsafe { self.AF.all = self.AF.all.wrapping_add(x); }; return Response::None; } ,

        }  
    }
    pub fn BC(&mut self, wr: Action) -> Response {
        match wr {
            Action::Write(x) => {
                self.BC.all = x;
                return Response::None;
            },
            Action::Read => unsafe { Response::Short(self.BC.all) },
            Action::Decrement(x) => { unsafe { self.BC.all = self.BC.all.wrapping_sub(x); }; return Response::None; } ,
            Action::Increment(x) => { unsafe { self.BC.all = self.BC.all.wrapping_add(x); }; return Response::None; } ,

        }
    }
    pub fn DE(&mut self, wr: Action) -> Response {
        match wr {
            Action::Write(x) => {
                self.DE.all = x;
                return Response::None;
            },
            Action::Read => unsafe { Response::Short(self.DE.all) },
            Action::Decrement(x) => { unsafe { self.DE.all = self.DE.all.wrapping_sub(x); }; return Response::None; } ,
            Action::Increment(x) => { unsafe { self.DE.all = self.DE.all.wrapping_add(x); }; return Response::None; } ,

        }
    }
    pub fn HL(&mut self, wr: Action) -> Response{
        match wr {
            Action::Write(x) => {
                self.HL.all = x;
                return Response::None;
            },
            Action::Read => unsafe { Response::Short(self.HL.all) },
            Action::Decrement(x) => { unsafe { self.HL.all = self.HL.all.wrapping_sub(x); }; return Response::None; } ,
            Action::Increment(x) => { unsafe { self.HL.all = self.HL.all.wrapping_add(x); }; return Response::None; } ,

        }
    }

    pub fn PC(&mut self, wr: Action) -> Response{
        match wr {
            Action::Write(x) => {
                self.PC = x;
                return Response::None;
            },
            Action::Read => Response::Short(self.PC),
            Action::Decrement(x) => { self.PC = self.PC.wrapping_sub(x); Response::None} ,
            Action::Increment(x) => { self.PC = self.PC.wrapping_add(x); Response::None } ,
        }
    }

    pub fn SP(&mut self, wr: Action) -> Response{
        match wr {
            Action::Write(x) => {
                self.SP = x;
                return Response::None;
            },
            Action::Read => Response::Short(self.SP),
            Action::Decrement(x) => { self.SP = self.SP.wrapping_sub(x); Response::None},
            Action::Increment(x) => { self.SP = self.SP.wrapping_add(x); Response::None },
        }
    }

    pub fn A(&mut self, wr: Action) -> Response{
        match wr {
            Action::Write(x) => {
                self.AF.Pair.msb = x as u8;
                return Response::None;
            },
            Action::Read => unsafe { Response::Byte(self.AF.Pair.msb) },
            Action::Decrement(_) => { panic!("Tried to decrement 8 bit register through registers module") } ,
            Action::Increment(_) => { panic!("Tried to increment 8 bit register through registers module") } ,

        }
    }

    pub fn B(&mut self, wr: Action) -> Response{
        match wr {
            Action::Write(x) => {
                self.BC.Pair.msb = x as u8;
                return Response::None;
            },
            Action::Read => unsafe { Response::Byte(self.BC.Pair.msb) },
            Action::Decrement(_) => { panic!("Tried to decrement 8 bit register through registers module") } ,
            Action::Increment(_) => { panic!("Tried to increment 8 bit register through registers module") } ,

        }
    }

    pub fn C(&mut self, wr: Action) -> Response{
        match wr {
            Action::Write(x) => {
                self.BC.Pair.lsb = x as u8;
                return Response::None;
            },
            Action::Read => unsafe { Response::Byte(self.BC.Pair.lsb) },
            Action::Decrement(_) => { panic!("Tried to decrement 8 bit register through registers module") } ,
            Action::Increment(_) => { panic!("Tried to increment 8 bit register through registers module") } ,

        }
    }

    pub fn D(&mut self, wr: Action) -> Response{
        match wr {
            Action::Write(x) => {
                self.DE.Pair.msb = x as u8;
                return Response::None;
            },
            Action::Read => unsafe { Response::Byte(self.DE.Pair.msb) },
            Action::Decrement(_) => { panic!("Tried to decrement 8 bit register through registers module") } ,
            Action::Increment(_) => { panic!("Tried to increment 8 bit register through registers module") } ,

        }
    }

    pub fn E(&mut self, wr: Action) -> Response{
        match wr {
            Action::Write(x) => {
                self.DE.Pair.lsb = x as u8;
                return Response::None;
            },
            Action::Read => unsafe { Response::Byte(self.DE.Pair.lsb) },
            Action::Decrement(_) => { panic!("Tried to decrement 8 bit register through registers module") } ,
            Action::Increment(_) => { panic!("Tried to increment 8 bit register through registers module") } ,

        }
    }

    pub fn H(&mut self, wr: Action) -> Response{
        match wr {
            Action::Write(x) => {
                self.HL.Pair.msb = x as u8;
                return Response::None;
            },
            Action::Read => unsafe { Response::Byte(self.HL.Pair.msb) },
            Action::Decrement(_) => { panic!("Tried to decrement 8 bit register through registers module") } ,
            Action::Increment(_) => { panic!("Tried to increment 8 bit register through registers module") } ,

        }
    }

    pub fn L(&mut self, wr: Action) -> Response{
        match wr {
            Action::Write(x) => {
                self.HL.Pair.lsb = x as u8;
                return Response::None;
            },
            Action::Read => unsafe { Response::Byte(self.HL.Pair.lsb) },
            Action::Decrement(_) => { panic!("Tried to decrement 8 bit register through registers module") } ,
            Action::Increment(_) => { panic!("Tried to increment 8 bit register through registers module") } ,

        }
    }

    pub fn set_flag(&mut self, flag: u8){
        match flag {
            0 => { self.AF.Pair.lsb = unsafe { self.AF.Pair.lsb } | ZERO_FLAG; },
            1 => { self.AF.Pair.lsb = unsafe { self.AF.Pair.lsb } | NEGATIVE_FLAG; },
            2 => { self.AF.Pair.lsb = unsafe { self.AF.Pair.lsb } | HALFCARRY_FLAG; },
            3 => { self.AF.Pair.lsb = unsafe { self.AF.Pair.lsb } | CARRY_FLAG; },
            _ => { panic!("Wrong flag number") },
        }
    }
    pub fn clear_flag(&mut self, flag: u8){
        match flag {
            0 => { self.AF.Pair.lsb = unsafe { self.AF.Pair.lsb } & !ZERO_FLAG; },
            1 => { self.AF.Pair.lsb = unsafe { self.AF.Pair.lsb } & !NEGATIVE_FLAG; },
            2 => { self.AF.Pair.lsb = unsafe { self.AF.Pair.lsb } & !HALFCARRY_FLAG; },
            3 => { self.AF.Pair.lsb = unsafe { self.AF.Pair.lsb } & !CARRY_FLAG; },
            _ => { panic!("Wrong flag number") },
        }
    }
    pub fn test_flag(&mut self, flag: u8) -> bool{
        match flag {
            0 => { (unsafe { self.AF.Pair.lsb } & ZERO_FLAG) == ZERO_FLAG },
            1 => { (unsafe { self.AF.Pair.lsb } & NEGATIVE_FLAG) == NEGATIVE_FLAG  },
            2 => { (unsafe { self.AF.Pair.lsb } & HALFCARRY_FLAG) == HALFCARRY_FLAG  },
            3 => { (unsafe { self.AF.Pair.lsb } & CARRY_FLAG) == CARRY_FLAG  },
            _ => { panic!("Wrong flag number") },
        }
    }
}

