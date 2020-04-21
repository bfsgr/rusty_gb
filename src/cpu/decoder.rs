

use crate::cpu::CPU;

#[derive(Clone)] //derives clone trait so we don't have to move references when dealing with instructions
pub struct Instruction {
    pub disassembly: &'static str,
    pub inst: fn(Self, u8, [u8;2] , &mut CPU),
    pub osize: u8,
    pub ticks: u16

}


pub struct Decoder {
    pub inst: [Instruction; 256],
    pub bit_inst: [Instruction; 256]
}

impl Default for Decoder {
    fn default() -> Self {
        Decoder { 
            inst: [
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x1
                Instruction{ 
                    disassembly: "LD BC,nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                },
                //0x2
                Instruction{ 
                    disassembly: "LD (BC),A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                },
                //0x3
                Instruction{ 
                    disassembly: "INC BC",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                },
                //0x4
                Instruction{ 
                    disassembly: "INC B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x5
                Instruction{ 
                    disassembly: "DEC B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x6
                Instruction{ 
                    disassembly: "LD B,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x7
                Instruction{ 
                    disassembly: "RLC A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x8
                Instruction{ 
                    disassembly: "LD (nn),SP",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 20
                }, 
                //0x9
                Instruction{ 
                    disassembly: "ADD HL,BC",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xA
                Instruction{ 
                    disassembly: "LD A,(BC)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xB
                Instruction{ 
                    disassembly: "DEC BC",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xC
                Instruction{ 
                    disassembly: "INC C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xD
                Instruction{ 
                    disassembly: "DEC C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xE
                Instruction{ 
                    disassembly: "LD C,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0xF
                Instruction{ 
                    disassembly: "RRC A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x10
                Instruction{ 
                    disassembly: "STOP",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 4
                }, 
                //0x11
                Instruction{ 
                    disassembly: "LD DE,nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0x12
                Instruction{ 
                    disassembly: "LD (DE),A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x13
                Instruction{ 
                    disassembly: "INC DE",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x14
                Instruction{ 
                    disassembly: "INC D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x15
                Instruction{ 
                    disassembly: "DEC D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x16
                Instruction{ 
                    disassembly: "LD D,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x17
                Instruction{ 
                    disassembly: "RL A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x18
                Instruction{ 
                    disassembly: "JR n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x19
                Instruction{ 
                    disassembly: "ADD HL,DE",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x1A
                Instruction{ 
                    disassembly: "LD A,(DE)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x1B
                Instruction{ 
                    disassembly: "DEC DE",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x1C
                Instruction{ 
                    disassembly: "INC E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x1D
                Instruction{ 
                    disassembly: "DEC E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x1E
                Instruction{ 
                    disassembly: "LD E,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x1F
                Instruction{ 
                    disassembly: "RR A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x20
                Instruction{ 
                    disassembly: "JR NZ,n",
                    inst: CPU::jp_nz_n,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x21
                Instruction{ 
                    disassembly: "LD HL,nn",
                    inst: CPU::ld_hl_nn,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0x22
                Instruction{ 
                    disassembly: "LDI (HL),A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x23
                Instruction{ 
                    disassembly: "INC HL",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                },
                //0x24 
                Instruction{ 
                    disassembly: "INC H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x25
                Instruction{ 
                    disassembly: "DEC H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x26
                Instruction{ 
                    disassembly: "LD H,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x27
                Instruction{ 
                    disassembly: "DAA",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x28
                Instruction{ 
                    disassembly: "JR Z,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x29
                Instruction{ 
                    disassembly: "ADD HL,HL",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x2A
                Instruction{ 
                    disassembly: "LDI A,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x2B
                Instruction{ 
                    disassembly: "DEC HL",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x2C
                Instruction{ 
                    disassembly: "INC L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x2D
                Instruction{ 
                    disassembly: "DEC L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x2E
                Instruction{ 
                    disassembly: "LD L,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x2F
                Instruction{ 
                    disassembly: "CPL",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x30
                Instruction{ 
                    disassembly: "JR NC,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x31
                Instruction{ 
                    disassembly: "LD SP,nn",
                    inst: CPU::ld_sp_nn,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0x32
                Instruction{ 
                    disassembly: "LDD (HL),A",
                    inst: CPU::ldd_HL_A,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x33
                Instruction{ 
                    disassembly: "INC SP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x34
                Instruction{ 
                    disassembly: "INC (HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 12
                }, 
                //0x35
                Instruction{ 
                    disassembly: "DEC (HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 12
                }, 
                //0x36
                Instruction{ 
                    disassembly: "LD (HL),n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 12
                }, 
                //0x37
                Instruction{ 
                    disassembly: "SCF",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x38
                Instruction{ 
                    disassembly: "JR C,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x39
                Instruction{ 
                    disassembly: "ADD HL,SP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x3A
                Instruction{ 
                    disassembly: "LDD A,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x3B
                Instruction{ 
                    disassembly: "DEC SP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x3C
                Instruction{ 
                    disassembly: "INC A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x3D
                Instruction{ 
                    disassembly: "DEC A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x3E
                Instruction{ 
                    disassembly: "LD A,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0x3F
                Instruction{ 
                    disassembly: "CCF",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x40
                Instruction{ 
                    disassembly: "LD B,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x41
                Instruction{ 
                    disassembly: "LD B,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x42
                Instruction{ 
                    disassembly: "LD B,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x43
                Instruction{ 
                    disassembly: "LD B,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x44
                Instruction{ 
                    disassembly: "LD B,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x45
                Instruction{ 
                    disassembly: "LD B,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x46
                Instruction{ 
                    disassembly: "LD B,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x47
                Instruction{ 
                    disassembly: "LD B,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x48
                Instruction{ 
                    disassembly: "LD C,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x49
                Instruction{ 
                    disassembly: "LD C,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x4A
                Instruction{ 
                    disassembly: "LD C,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x4B
                Instruction{ 
                    disassembly: "LD C,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x4C
                Instruction{ 
                    disassembly: "LD C,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x4D
                Instruction{ 
                    disassembly: "LD C,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x4E
                Instruction{ 
                    disassembly: "LD C,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x4F
                Instruction{ 
                    disassembly: "LD C,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x50
                Instruction{ 
                    disassembly: "LD D,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x51
                Instruction{ 
                    disassembly: "LD D,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x52
                Instruction{ 
                    disassembly: "LD D,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x53
                Instruction{ 
                    disassembly: "LD D,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x54
                Instruction{ 
                    disassembly: "LD D,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x55
                Instruction{ 
                    disassembly: "LD D,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x56
                Instruction{ 
                    disassembly: "LD D,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x57
                Instruction{ 
                    disassembly: "LD D,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x58
                Instruction{ 
                    disassembly: "LD E,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x59
                Instruction{ 
                    disassembly: "LD E,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x5A
                Instruction{ 
                    disassembly: "LD E,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x5B
                Instruction{ 
                    disassembly: "LD E,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x5C
                Instruction{ 
                    disassembly: "LD E,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x5D
                Instruction{ 
                    disassembly: "LD E,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x5E
                Instruction{ 
                    disassembly: "LD E,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x5F
                Instruction{ 
                    disassembly: "LD E,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x60
                Instruction{ 
                    disassembly: "LD H,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x61
                Instruction{ 
                    disassembly: "LD H,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x62
                Instruction{ 
                    disassembly: "LD H,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x63
                Instruction{ 
                    disassembly: "LD H,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x64
                Instruction{ 
                    disassembly: "LD H,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x65
                Instruction{ 
                    disassembly: "LD H,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x66
                Instruction{ 
                    disassembly: "LD H,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x67
                Instruction{ 
                    disassembly: "LD H,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x68
                Instruction{ 
                    disassembly: "LD L,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x69
                Instruction{ 
                    disassembly: "LD L,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x6A
                Instruction{ 
                    disassembly: "LD L,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x6B
                Instruction{ 
                    disassembly: "LD L,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x6C
                Instruction{ 
                    disassembly: "LD L,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x6D
                Instruction{ 
                    disassembly: "LD L,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x6E
                Instruction{ 
                    disassembly: "LD L,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x6F
                Instruction{ 
                    disassembly: "LD L,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x70
                Instruction{ 
                    disassembly: "LD (HL),B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x71
                Instruction{ 
                    disassembly: "LD (HL),C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x72
                Instruction{ 
                    disassembly: "LD (HL),D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x73
                Instruction{ 
                    disassembly: "LD (HL),E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x74
                Instruction{ 
                    disassembly: "LD (HL),H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x75
                Instruction{ 
                    disassembly: "LD (HL),L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x76
                Instruction{ 
                    disassembly: "HALT",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x77
                Instruction{ 
                    disassembly: "LD (HL),A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                //0x78
                Instruction{ 
                    disassembly: "LD A,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x79
                Instruction{ 
                    disassembly: "LD A,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x7A
                Instruction{ 
                    disassembly: "LD A,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x7B
                Instruction{ 
                    disassembly: "LD A,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x7C
                Instruction{ 
                    disassembly: "LD A,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x7D
                Instruction{ 
                    disassembly: "LD A,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x7E
                Instruction{ 
                    disassembly: "LD A,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x7F
                Instruction{ 
                    disassembly: "LD A,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x80
                Instruction{ 
                    disassembly: "ADD A,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x81
                Instruction{ 
                    disassembly: "ADD A,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x82
                Instruction{ 
                    disassembly: "ADD A,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x83
                Instruction{ 
                    disassembly: "ADD A,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x84
                Instruction{ 
                    disassembly: "ADD A,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x85
                Instruction{ 
                    disassembly: "ADD A,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x86
                Instruction{ 
                    disassembly: "ADD A,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x87
                Instruction{ 
                    disassembly: "ADD A,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x88
                Instruction{ 
                    disassembly: "ADC A,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x89
                Instruction{ 
                    disassembly: "ADC A,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x8A
                Instruction{ 
                    disassembly: "ADC A,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x8B
                Instruction{ 
                    disassembly: "ADC A,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x8C
                Instruction{ 
                    disassembly: "ADC A,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x8D
                Instruction{ 
                    disassembly: "ADC A,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x8E
                Instruction{ 
                    disassembly: "ADC A,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x8F
                Instruction{ 
                    disassembly: "ADC A,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x90
                Instruction{ 
                    disassembly: "SUB A,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x91
                Instruction{ 
                    disassembly: "SUB A,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x92
                Instruction{ 
                    disassembly: "SUB A,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x93
                Instruction{ 
                    disassembly: "SUB A,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x94
                Instruction{ 
                    disassembly: "SUB A,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x95
                Instruction{ 
                    disassembly: "SUB A,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x96
                Instruction{ 
                    disassembly: "SUB A,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                },
                //0x97
                Instruction{ 
                    disassembly: "SUB A,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x98
                Instruction{ 
                    disassembly: "SBC A,B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x99
                Instruction{ 
                    disassembly: "SBC A,C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x9A
                Instruction{ 
                    disassembly: "SBC A,D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x9B
                Instruction{ 
                    disassembly: "SBC A,E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x9C
                Instruction{ 
                    disassembly: "SBC A,H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x9D
                Instruction{ 
                    disassembly: "SBC A,L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0x9E
                Instruction{ 
                    disassembly: "SBC A,(HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0x9F
                Instruction{ 
                    disassembly: "SBC A,A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xA0
                Instruction{ 
                    disassembly: "AND B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xA1
                Instruction{ 
                    disassembly: "AND C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xA2
                Instruction{ 
                    disassembly: "AND D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xA3
                Instruction{ 
                    disassembly: "AND E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xA4
                Instruction{ 
                    disassembly: "AND H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xA5
                Instruction{ 
                    disassembly: "AND L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xA6
                Instruction{ 
                    disassembly: "AND (HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xA7
                Instruction{ 
                    disassembly: "AND A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xA8
                Instruction{ 
                    disassembly: "XOR B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xA9
                Instruction{ 
                    disassembly: "XOR C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xAA
                Instruction{ 
                    disassembly: "XOR D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xAB
                Instruction{ 
                    disassembly: "XOR E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xAC
                Instruction{ 
                    disassembly: "XOR H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xAD
                Instruction{ 
                    disassembly: "XOR L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xAE
                Instruction{ 
                    disassembly: "XOR (HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xAF
                Instruction{ 
                    disassembly: "XOR A",
                    inst: CPU::xor_a,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xB0
                Instruction{ 
                    disassembly: "OR B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xB1
                Instruction{ 
                    disassembly: "OR C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xB2
                Instruction{ 
                    disassembly: "OR D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xB3
                Instruction{ 
                    disassembly: "OR E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xB4
                Instruction{ 
                    disassembly: "OR H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xB5
                Instruction{ 
                    disassembly: "OR L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xB6
                Instruction{ 
                    disassembly: "OR (HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xB7
                Instruction{ 
                    disassembly: "OR A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xB8
                Instruction{ 
                    disassembly: "CP B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xB9
                Instruction{ 
                    disassembly: "CP C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xBA
                Instruction{ 
                    disassembly: "CP D",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xBB
                Instruction{ 
                    disassembly: "CP E",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xBC
                Instruction{ 
                    disassembly: "CP H",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xBD
                Instruction{ 
                    disassembly: "CP L",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xBE
                Instruction{ 
                    disassembly: "CP (HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xBF
                Instruction{ 
                    disassembly: "CP A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xC0
                Instruction{ 
                    disassembly: "RET NZ",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                },
                //0xC1
                Instruction{ 
                    disassembly: "POP BC{}",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 12
                }, 
                //0xC2
                Instruction{ 
                    disassembly: "JP NZ,nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0xC3
                Instruction{ 
                    disassembly: "JP nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0xC4
                Instruction{ 
                    disassembly: "CALL NZ,nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0xC5
                Instruction{ 
                    disassembly: "PUSH BC",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 16
                }, 
                //0xC6
                Instruction{ 
                    disassembly: "ADD A,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0xC7
                Instruction{ 
                    disassembly: "RST 0",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 32
                }, 
                //0xC8
                Instruction{ 
                    disassembly: "RET Z",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xC9
                Instruction{ 
                    disassembly: "RET",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xCA
                Instruction{ 
                    disassembly: "JP Z,nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0xCB
                Instruction{ 
                    disassembly: "CB prefix",
                    inst: CPU::nop, //redirect to bits instruction table
                    
                    osize: 3,
                    ticks: 0
                }, 
                //0xCC
                Instruction{ 
                    disassembly: "CALL Z,nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0xCD
                Instruction{ 
                    disassembly: "CALL nn",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 12
                }, 
                //0xCE
                Instruction{ 
                    disassembly: "ADC A,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0xCF
                Instruction{ 
                    disassembly: "RST 8",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 32
                }, 
                //0xD0
                Instruction{ 
                    disassembly: "RET NC",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xD1
                Instruction{ 
                    disassembly: "POP DE",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 12
                }, 
                //0xD2
                Instruction{ 
                    disassembly: "JP NC,nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0xD3
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                //0xD4
                Instruction{ 
                    disassembly: "CALL NC,nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0xD5
                Instruction{ 
                    disassembly: "PUSH DE",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 16
                }, 
                //0xD6
                Instruction{ 
                    disassembly: "SUB A,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0xD7
                Instruction{ 
                    disassembly: "RST 10",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 32
                }, 
                //0xD8
                Instruction{ 
                    disassembly: "RET C",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xD9
                Instruction{ 
                    disassembly: "RETI",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xDA
                Instruction{ 
                    disassembly: "JP C,nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0xDB
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                //0xDC
                Instruction{ 
                    disassembly: "CALL C,nn",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 12
                }, 
                //0xDD
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                //0xDE
                Instruction{ 
                    disassembly: "SBC A,n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0xDF
                Instruction{ 
                    disassembly: "RST 18",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 32
                }, 
                //0xE0
                Instruction{ 
                    disassembly: "LDH (n),A",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 12
                }, 
                //0xE1
                Instruction{ 
                    disassembly: "POP HL",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 12
                }, 
                //0xE2
                Instruction{ 
                    disassembly: "LDH (C),A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xE3
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                //0xE4
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                //0xE5
                Instruction{ 
                    disassembly: "PUSH HL",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 16
                }, 
                //0xE6
                Instruction{ 
                    disassembly: "AND n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0xE7
                Instruction{ 
                    disassembly: "RST 20",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 32
                }, 
                //OxE8
                Instruction{ 
                    disassembly: "ADD SP,d",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 16
                }, 
                //0xE9
                Instruction{ 
                    disassembly: "JP (HL)",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xEA
                Instruction{ 
                    disassembly: "LD (nn),A",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 16
                }, 
                //0xEB
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                //0xEE
                Instruction{ 
                    disassembly: "XOR n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //OxEF
                Instruction{ 
                    disassembly: "RST 28",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 32
                }, 
                //0xF0
                Instruction{ 
                    disassembly: "LDH A,(n)",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 12
                }, 
                //0xF1
                Instruction{ 
                    disassembly: "POP AF",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 12
                }, 
                //0xF2
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                //0xF3
                Instruction{ 
                    disassembly: "DI",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xF4
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                //0xF5
                Instruction{ 
                    disassembly: "PUSH AF",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 16
                }, 
                //0xF6
                Instruction{ 
                    disassembly: "OR n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0xF7
                Instruction{ 
                    disassembly: "RST 30",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 32
                }, 
                //0xF8
                Instruction{ 
                    disassembly: "LDHL SP,d",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 12
                }, 
                //0xF9
                Instruction{ 
                    disassembly: "LD SP,HL",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 8
                }, 
                //0xFA
                Instruction{ 
                    disassembly: "LD A,(nn)",
                    inst: CPU::nop,
                    
                    osize: 2,
                    ticks: 16
                }, 
                //0xFB
                Instruction{ 
                    disassembly: "EI",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                }, 
                //0xFC
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                Instruction{ 
                    disassembly: "REMOVED",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 0
                }, 
                //0xFE
                Instruction{ 
                    disassembly: "CP n",
                    inst: CPU::nop,
                    
                    osize: 1,
                    ticks: 8
                }, 
                //0xFF
                Instruction{
                    disassembly: "RST 38",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 32
                }
            ],

            // ===========================================================================
            //                           Two-Bytes Instructions
            // ===========================================================================


            bit_inst: [
                //0x0
                Instruction{ 
                    disassembly: "RLC B",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x1
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x2
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x3
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x4
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x5
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x6
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x7
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x8
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x9
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0xA
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0xB
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0xC
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0xD
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0xE
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0xF
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x10
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x11
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x12
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x13
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x14
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x15
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x16
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x17
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x18
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x19
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x1A
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x1B
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x1C
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x1D
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x1E
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x1F
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x20
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x21
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x22
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x23
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x24
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x25
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x26
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x27
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x28
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x29
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x2A
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x2B
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x2C
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x2D
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x2E
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x2F
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x30
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x31
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x32
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x33
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x34
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x35
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x36
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x37
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x38
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x39
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x3A
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x3B
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x3C
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x3D
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x3E
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x3F
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x40
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0X41
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x42
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x43
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x44
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x45
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x46
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x47
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x48
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x49
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x4A
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x4B
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x4C
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x4D
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x4E
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x4F
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x50
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x51
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x52
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x53
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x54
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x55
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x56
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x57
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x58
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x59
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x5A
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x5B
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x5C
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x5D
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x5E
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x5F
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x60
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x61
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x62
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x63
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x64
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x65
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x66
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x67
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x68
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x69
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x6A
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x6B
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x6C
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x6D
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x6E
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x6F
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x70
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x71
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x72
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x73
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x74
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x75
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x76
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x77
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x78
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x79
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x7A
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x7B
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x7C
                Instruction{ 
                    disassembly: "BIT 7,H",
                    inst: CPU::bit_7h,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                //0x0
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
                Instruction{ 
                    disassembly: "NOP",
                    inst: CPU::nop,
                    
                    osize: 0,
                    ticks: 4
                },
            ]
        }
    }
}

