use super::instructions::Instruction;

pub struct Decoder;

impl Decoder {
    pub fn decode( opcode: u8, subset: bool ) -> Instruction{
        if !subset {
            INSTRUCTION_SET[opcode as usize]
        } else {
            INSTRUCTION_SUBSET[opcode as usize]
        }
    }
}



const INSTRUCTION_SET: [Instruction; 256] = [
    //0x0
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP_R,
        args: 0,
        cycles: 4
    },
    //0x1
    Instruction{ 
        disassembly: "LD BC,nn",
        function: Instruction::LD_BC_nn,
        args: 2,
        cycles: 12
    },
    //0x2
    Instruction{ 
        disassembly: "LD (BC),A",
        function: Instruction::LD_dBC_A,
        args: 0,
        cycles: 8
    },
    //0x3
    Instruction{ 
        disassembly: "INC BC",
        function: Instruction::INC_BC,
        args: 0,
        cycles: 8
    },
    //0x4
    Instruction{ 
        disassembly: "INC B",
        function: Instruction::INC_B,
        args: 0,
        cycles: 4
    }, 
    //0x5
    Instruction{ 
        disassembly: "DEC B",
        function: Instruction::DEC_B,
        args: 0,
        cycles: 4
    }, 
    //0x6
    Instruction{ 
        disassembly: "LD B,n",
        function: Instruction::LD_B_n,
        args: 1,
        cycles: 8
    }, 
    //0x7
    Instruction{ 
        disassembly: "RLC A",
        function: Instruction::RLC_A,
        args: 0,
        cycles: 4
    }, 
    //0x8
    Instruction{ 
        disassembly: "LD (nn),SP",
        function: Instruction::LD_dnn_SP,
        args: 2,
        cycles: 20
    }, 
    //0x9
    Instruction{ 
        disassembly: "ADD HL,BC",
        function: Instruction::ADD_HL_BC,
        args: 0,
        cycles: 8
    }, 
    //0xA
    Instruction{ 
        disassembly: "LD A,(BC)",
        function: Instruction::LD_A_dBC,
        args: 0,
        cycles: 8
    }, 
    //0xB
    Instruction{ 
        disassembly: "DEC BC",
        function: Instruction::DEC_BC,
        args: 0,
        cycles: 8
    }, 
    //0xC
    Instruction{ 
        disassembly: "INC C",
        function: Instruction::INC_C,
        args: 0,
        cycles: 4
    }, 
    //0xD
    Instruction{ 
        disassembly: "DEC C",
        function: Instruction::DEC_C,
        args: 0,
        cycles: 4
    }, 
    //0xE
    Instruction{ 
        disassembly: "LD C,n",
        function: Instruction::LD_C_n,
        args: 1,
        cycles: 8
    }, 
    //0xF
    Instruction{ 
        disassembly: "RRC A",
        function: Instruction::RRC_A,
        args: 0,
        cycles: 4
    }, 
    //0x10
    Instruction{ 
        disassembly: "STOP",
        function: Instruction::STOP,
        args: 1,
        cycles: 4
    }, 
    //0x11
    Instruction{ 
        disassembly: "LD DE,nn",
        function: Instruction::LD_DE_nn,
        args: 2,
        cycles: 12
    }, 
    //0x12
    Instruction{ 
        disassembly: "LD (DE),A",
        function: Instruction::LD_dDE_A,
        args: 0,
        cycles: 8
    }, 
    //0x13
    Instruction{ 
        disassembly: "INC DE",
        function: Instruction::INC_DE,
        args: 0,
        cycles: 8
    }, 
    //0x14
    Instruction{ 
        disassembly: "INC D",
        function: Instruction::INC_D,
        args: 0,
        cycles: 4
    }, 
    //0x15
    Instruction{ 
        disassembly: "DEC D",
        function: Instruction::DEC_D,
        args: 0,
        cycles: 4
    }, 
    //0x16
    Instruction{ 
        disassembly: "LD D,n",
        function: Instruction::LD_D_n,
        args: 1,
        cycles: 8
    }, 
    //0x17
    Instruction{ 
        disassembly: "RL A",
        function: Instruction::RL_A,
        args: 0,
        cycles: 4
    }, 
    //0x18
    Instruction{ 
        disassembly: "JR n",
        function: Instruction::JR_n,
        args: 1,
        cycles: 8
    }, 
    //0x19
    Instruction{ 
        disassembly: "ADD HL,DE",
        function: Instruction::ADD_HL_DE,
        args: 0,
        cycles: 8
    }, 
    //0x1A
    Instruction{ 
        disassembly: "LD A,(DE)",
        function: Instruction::LD_A_dDE,
        args: 0,
        cycles: 8
    }, 
    //0x1B
    Instruction{ 
        disassembly: "DEC DE",
        function: Instruction::DEC_DE,
        args: 0,
        cycles: 8
    }, 
    //0x1C
    Instruction{ 
        disassembly: "INC E",
        function: Instruction::INC_E,
        args: 0,
        cycles: 4
    }, 
    //0x1D
    Instruction{ 
        disassembly: "DEC E",
        function: Instruction::DEC_E,
        args: 0,
        cycles: 4
    }, 
    //0x1E
    Instruction{ 
        disassembly: "LD E,n",
        function: Instruction::LD_E_n,
        args: 1,
        cycles: 8
    }, 
    //0x1F
    Instruction{ 
        disassembly: "RR A",
        function: Instruction::RR_A,
        args: 0,
        cycles: 4
    }, 
    //0x20
    Instruction{ 
        disassembly: "JR NZ,n",
        function: Instruction::JR_NZ_n,
        args: 1,
        cycles: 8
    }, 
    //0x21
    Instruction{ 
        disassembly: "LD HL,nn",
        function: Instruction::LD_HL_nn,
        args: 2,
        cycles: 12
    }, 
    //0x22
    Instruction{ 
        disassembly: "LD (HL+),A",
        function: Instruction::LDI_HL_A,
        args: 0,
        cycles: 8
    }, 
    //0x23
    Instruction{ 
        disassembly: "INC HL",
        function: Instruction::INC_HL,
        args: 0,
        cycles: 8
    },
    //0x24 
    Instruction{ 
        disassembly: "INC H",
        function: Instruction::INC_H,
        args: 0,
        cycles: 4
    }, 
    //0x25
    Instruction{ 
        disassembly: "DEC H",
        function: Instruction::DEC_H,
        args: 0,
        cycles: 4
    }, 
    //0x26
    Instruction{ 
        disassembly: "LD H,n",
        function: Instruction::LD_H_n,
        args: 1,
        cycles: 8
    }, 
    //0x27
    Instruction{ 
        disassembly: "DAA",
        function: Instruction::DAA,
        args: 0,
        cycles: 4
    }, 
    //0x28
    Instruction{ 
        disassembly: "JR Z,n",
        function: Instruction::JR_Z_n,
        args: 1,
        cycles: 8
    }, 
    //0x29
    Instruction{ 
        disassembly: "ADD HL,HL",
        function: Instruction::ADD_HL_HL,
        args: 0,
        cycles: 8
    }, 
    //0x2A
    Instruction{ 
        disassembly: "LD A,(HL+)",
        function: Instruction::LDI_A_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x2B
    Instruction{ 
        disassembly: "DEC HL",
        function: Instruction::DEC_HL,
        args: 0,
        cycles: 8
    }, 
    //0x2C
    Instruction{ 
        disassembly: "INC L",
        function: Instruction::INC_L,
        args: 0,
        cycles: 4
    }, 
    //0x2D
    Instruction{ 
        disassembly: "DEC L",
        function: Instruction::DEC_L,
        args: 0,
        cycles: 4
    }, 
    //0x2E
    Instruction{ 
        disassembly: "LD L,n",
        function: Instruction::LD_L_n,
        args: 1,
        cycles: 8
    }, 
    //0x2F
    Instruction{ 
        disassembly: "CPL",
        function: Instruction::NOT_A,
        args: 0,
        cycles: 4
    }, 
    //0x30
    Instruction{ 
        disassembly: "JR NC,n",
        function: Instruction::JR_NC_n,
        args: 1,
        cycles: 8
    }, 
    //0x31
    Instruction{ 
        disassembly: "LD SP,nn",
        function: Instruction::LD_SP_nn,
        args: 2,
        cycles: 12
    }, 
    //0x32
    Instruction{ 
        disassembly: "LD (HL-),A",
        function: Instruction::LDD_HL_A,
        args: 0,
        cycles: 8
    }, 
    //0x33
    Instruction{ 
        disassembly: "INC SP",
        function: Instruction::INC_SP,
        args: 0,
        cycles: 8
    }, 
    //0x34
    Instruction{ 
        disassembly: "INC (HL)",
        function: Instruction::INC_dHL,
        args: 0,
        cycles: 12
    }, 
    //0x35
    Instruction{ 
        disassembly: "DEC (HL)",
        function: Instruction::DEC_dHL,
        args: 0,
        cycles: 12
    }, 
    //0x36
    Instruction{ 
        disassembly: "LD (HL),n",
        function: Instruction::LD_dHL_n,
        args: 1,
        cycles: 12
    }, 
    //0x37
    Instruction{ 
        disassembly: "SCF",
        function: Instruction::SCF,
        args: 0,
        cycles: 4
    }, 
    //0x38
    Instruction{ 
        disassembly: "JR C,n",
        function: Instruction::JR_C_n,
        args: 1,
        cycles: 8
    }, 
    //0x39
    Instruction{ 
        disassembly: "ADD HL,SP",
        function: Instruction::ADD_HL_SP,
        args: 0,
        cycles: 8
    }, 
    //0x3A
    Instruction{ 
        disassembly: "LD A,(HL-)",
        function: Instruction::LDD_A_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x3B
    Instruction{ 
        disassembly: "DEC SP",
        function: Instruction::DEC_SP,
        args: 0,
        cycles: 8
    }, 
    //0x3C
    Instruction{ 
        disassembly: "INC A",
        function: Instruction::INC_A,
        args: 0,
        cycles: 4
    }, 
    //0x3D
    Instruction{ 
        disassembly: "DEC A",
        function: Instruction::DEC_A,
        args: 0,
        cycles: 4
    }, 
    //0x3E
    Instruction{ 
        disassembly: "LD A,n",
        function: Instruction::LD_A_n,
        args: 1,
        cycles: 8
    }, 
    //0x3F
    Instruction{ 
        disassembly: "CCF",
        function: Instruction::CCF,
        args: 0,
        cycles: 4
    }, 
    //0x40
    Instruction{ 
        disassembly: "LD B,B",
        function: Instruction::LD_B_B,
        args: 0,
        cycles: 4
    },
    //0x41
    Instruction{ 
        disassembly: "LD B,C",
        function: Instruction::LD_B_C,
        args: 0,
        cycles: 4
    }, 
    //0x42
    Instruction{ 
        disassembly: "LD B,D",
        function: Instruction::LD_B_D,
        args: 0,
        cycles: 4
    }, 
    //0x43
    Instruction{ 
        disassembly: "LD B,E",
        function: Instruction::LD_B_E,
        args: 0,
        cycles: 4
    }, 
    //0x44
    Instruction{ 
        disassembly: "LD B,H",
        function: Instruction::LD_B_H,
        args: 0,
        cycles: 4
    }, 
    //0x45
    Instruction{ 
        disassembly: "LD B,L",
        function: Instruction::LD_B_L,
        args: 0,
        cycles: 4
    }, 
    //0x46
    Instruction{ 
        disassembly: "LD B,(HL)",
        function: Instruction::LD_B_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x47
    Instruction{ 
        disassembly: "LD B,A",
        function: Instruction::LD_B_A,
        args: 0,
        cycles: 4
    }, 
    //0x48
    Instruction{ 
        disassembly: "LD C,B",
        function: Instruction::LD_C_B,
        args: 0,
        cycles: 4
    }, 
    //0x49
    Instruction{ 
        disassembly: "LD C,C",
        function: Instruction::LD_C_C,
        args: 0,
        cycles: 4
    }, 
    //0x4A
    Instruction{ 
        disassembly: "LD C,D",
        function: Instruction::LD_C_D,
        args: 0,
        cycles: 4
    }, 
    //0x4B
    Instruction{ 
        disassembly: "LD C,E",
        function: Instruction::LD_C_E,
        args: 0,
        cycles: 4
    }, 
    //0x4C
    Instruction{ 
        disassembly: "LD C,H",
        function: Instruction::LD_C_H,
        args: 0,
        cycles: 4
    }, 
    //0x4D
    Instruction{ 
        disassembly: "LD C,L",
        function: Instruction::LD_C_L,
        args: 0,
        cycles: 4
    }, 
    //0x4E
    Instruction{ 
        disassembly: "LD C,(HL)",
        function: Instruction::LD_C_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x4F
    Instruction{ 
        disassembly: "LD C,A",
        function: Instruction::LD_C_A,
        args: 0,
        cycles: 4
    }, 
    //0x50
    Instruction{ 
        disassembly: "LD D,B",
        function: Instruction::LD_D_B,
        args: 0,
        cycles: 4
    }, 
    //0x51
    Instruction{ 
        disassembly: "LD D,C",
        function: Instruction::LD_D_C,
        args: 0,
        cycles: 4
    }, 
    //0x52
    Instruction{ 
        disassembly: "LD D,D",
        function: Instruction::LD_D_D,
        args: 0,
        cycles: 4
    }, 
    //0x53
    Instruction{ 
        disassembly: "LD D,E",
        function: Instruction::LD_D_E,
        args: 0,
        cycles: 4
    }, 
    //0x54
    Instruction{ 
        disassembly: "LD D,H",
        function: Instruction::LD_D_H,
        args: 0,
        cycles: 4
    }, 
    //0x55
    Instruction{ 
        disassembly: "LD D,L",
        function: Instruction::LD_D_L,
        args: 0,
        cycles: 4
    }, 
    //0x56
    Instruction{ 
        disassembly: "LD D,(HL)",
        function: Instruction::LD_D_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x57
    Instruction{ 
        disassembly: "LD D,A",
        function: Instruction::LD_D_A,
        args: 0,
        cycles: 4
    }, 
    //0x58
    Instruction{ 
        disassembly: "LD E,B",
        function: Instruction::LD_E_B,
        args: 0,
        cycles: 4
    }, 
    //0x59
    Instruction{ 
        disassembly: "LD E,C",
        function: Instruction::LD_E_C,
        args: 0,
        cycles: 4
    }, 
    //0x5A
    Instruction{ 
        disassembly: "LD E,D",
        function: Instruction::LD_E_D,
        args: 0,
        cycles: 4
    }, 
    //0x5B
    Instruction{ 
        disassembly: "LD E,E",
        function: Instruction::LD_E_E,
        args: 0,
        cycles: 4
    }, 
    //0x5C
    Instruction{ 
        disassembly: "LD E,H",
        function: Instruction::LD_E_H,
        args: 0,
        cycles: 4
    }, 
    //0x5D
    Instruction{ 
        disassembly: "LD E,L",
        function: Instruction::LD_E_L,
        args: 0,
        cycles: 4
    }, 
    //0x5E
    Instruction{ 
        disassembly: "LD E,(HL)",
        function: Instruction::LD_E_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x5F
    Instruction{ 
        disassembly: "LD E,A",
        function: Instruction::LD_E_A,
        args: 0,
        cycles: 4
    }, 
    //0x60
    Instruction{ 
        disassembly: "LD H,B",
        function: Instruction::LD_H_B,
        args: 0,
        cycles: 4
    }, 
    //0x61
    Instruction{ 
        disassembly: "LD H,C",
        function: Instruction::LD_H_C,
        args: 0,
        cycles: 4
    }, 
    //0x62
    Instruction{ 
        disassembly: "LD H,D",
        function: Instruction::LD_H_D,
        args: 0,
        cycles: 4
    }, 
    //0x63
    Instruction{ 
        disassembly: "LD H,E",
        function: Instruction::LD_H_E,
        args: 0,
        cycles: 4
    }, 
    //0x64
    Instruction{ 
        disassembly: "LD H,H",
        function: Instruction::LD_H_H,
        args: 0,
        cycles: 4
    }, 
    //0x65
    Instruction{ 
        disassembly: "LD H,L",
        function: Instruction::LD_H_L,
        args: 0,
        cycles: 4
    }, 
    //0x66
    Instruction{ 
        disassembly: "LD H,(HL)",
        function: Instruction::LD_H_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x67
    Instruction{ 
        disassembly: "LD H,A",
        function: Instruction::LD_H_A,
        args: 0,
        cycles: 4
    }, 
    //0x68
    Instruction{ 
        disassembly: "LD L,B",
        function: Instruction::LD_L_B,
        args: 0,
        cycles: 4
    }, 
    //0x69
    Instruction{ 
        disassembly: "LD L,C",
        function: Instruction::LD_L_C,
        args: 0,
        cycles: 4
    }, 
    //0x6A
    Instruction{ 
        disassembly: "LD L,D",
        function: Instruction::LD_L_D,
        args: 0,
        cycles: 4
    }, 
    //0x6B
    Instruction{ 
        disassembly: "LD L,E",
        function: Instruction::LD_L_E,
        args: 0,
        cycles: 4
    }, 
    //0x6C
    Instruction{ 
        disassembly: "LD L,H",
        function: Instruction::LD_L_H,
        args: 0,
        cycles: 4
    }, 
    //0x6D
    Instruction{ 
        disassembly: "LD L,L",
        function: Instruction::LD_L_L,
        args: 0,
        cycles: 4
    }, 
    //0x6E
    Instruction{ 
        disassembly: "LD L,(HL)",
        function: Instruction::LD_L_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x6F
    Instruction{ 
        disassembly: "LD L,A",
        function: Instruction::LD_L_A,
        args: 0,
        cycles: 4
    }, 
    //0x70
    Instruction{ 
        disassembly: "LD (HL),B",
        function: Instruction::LD_dHL_B,
        args: 0,
        cycles: 8
    }, 
    //0x71
    Instruction{ 
        disassembly: "LD (HL),C",
        function: Instruction::LD_dHL_C,
        args: 0,
        cycles: 8
    }, 
    //0x72
    Instruction{ 
        disassembly: "LD (HL),D",
        function: Instruction::LD_dHL_D,
        args: 0,
        cycles: 8
    }, 
    //0x73
    Instruction{ 
        disassembly: "LD (HL),E",
        function: Instruction::LD_dHL_E,
        args: 0,
        cycles: 8
    }, 
    //0x74
    Instruction{ 
        disassembly: "LD (HL),H",
        function: Instruction::LD_dHL_H,
        args: 0,
        cycles: 8
    }, 
    //0x75
    Instruction{ 
        disassembly: "LD (HL),L",
        function: Instruction::LD_dHL_L,
        args: 0,
        cycles: 8
    }, 
    //0x76
    Instruction{ 
        disassembly: "HALT",
        function: Instruction::HALT,
        args: 0,
        cycles: 4
    }, 
    //0x77
    Instruction{ 
        disassembly: "LD (HL),A",
        function: Instruction::LD_dHL_A,
        args: 0,
        cycles: 8
    }, 
    //0x78
    Instruction{ 
        disassembly: "LD A,B",
        function: Instruction::LD_A_B,
        args: 0,
        cycles: 4
    }, 
    //0x79
    Instruction{ 
        disassembly: "LD A,C",
        function: Instruction::LD_A_C,
        args: 0,
        cycles: 4
    }, 
    //0x7A
    Instruction{ 
        disassembly: "LD A,D",
        function: Instruction::LD_A_D,
        args: 0,
        cycles: 4
    }, 
    //0x7B
    Instruction{ 
        disassembly: "LD A,E",
        function: Instruction::LD_A_E,
        args: 0,
        cycles: 4
    }, 
    //0x7C
    Instruction{ 
        disassembly: "LD A,H",
        function: Instruction::LD_A_H,
        args: 0,
        cycles: 4
    }, 
    //0x7D
    Instruction{ 
        disassembly: "LD A,L",
        function: Instruction::LD_A_L,
        args: 0,
        cycles: 4
    }, 
    //0x7E
    Instruction{ 
        disassembly: "LD A,(HL)",
        function: Instruction::LD_A_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x7F
    Instruction{ 
        disassembly: "LD A,A",
        function: Instruction::LD_A_A,
        args: 0,
        cycles: 4
    }, 
    //0x80
    Instruction{ 
        disassembly: "ADD A,B",
        function: Instruction::ADD_A_B,
        args: 0,
        cycles: 4
    },
    //0x81
    Instruction{ 
        disassembly: "ADD A,C",
        function: Instruction::ADD_A_C,
        args: 0,
        cycles: 4
    }, 
    //0x82
    Instruction{ 
        disassembly: "ADD A,D",
        function: Instruction::ADD_A_D,
        args: 0,
        cycles: 4
    }, 
    //0x83
    Instruction{ 
        disassembly: "ADD A,E",
        function: Instruction::ADD_A_E,
        args: 0,
        cycles: 4
    }, 
    //0x84
    Instruction{ 
        disassembly: "ADD A,H",
        function: Instruction::ADD_A_H,
        args: 0,
        cycles: 4
    }, 
    //0x85
    Instruction{ 
        disassembly: "ADD A,L",
        function: Instruction::ADD_A_L,
        args: 0,
        cycles: 4
    }, 
    //0x86
    Instruction{ 
        disassembly: "ADD A,(HL)",
        function: Instruction::ADD_A_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x87
    Instruction{ 
        disassembly: "ADD A,A",
        function: Instruction::ADD_A_A,
        args: 0,
        cycles: 4
    }, 
    //0x88
    Instruction{ 
        disassembly: "ADC A,B",
        function: Instruction::ADC_A_B,
        args: 0,
        cycles: 4
    }, 
    //0x89
    Instruction{ 
        disassembly: "ADC A,C",
        function: Instruction::ADC_A_C,
        args: 0,
        cycles: 4
    }, 
    //0x8A
    Instruction{ 
        disassembly: "ADC A,D",
        function: Instruction::ADC_A_D,
        args: 0,
        cycles: 4
    }, 
    //0x8B
    Instruction{ 
        disassembly: "ADC A,E",
        function: Instruction::ADC_A_E,
        args: 0,
        cycles: 4
    }, 
    //0x8C
    Instruction{ 
        disassembly: "ADC A,H",
        function: Instruction::ADC_A_H,
        args: 0,
        cycles: 4
    }, 
    //0x8D
    Instruction{ 
        disassembly: "ADC A,L",
        function: Instruction::ADC_A_L,
        args: 0,
        cycles: 4
    }, 
    //0x8E
    Instruction{ 
        disassembly: "ADC A,(HL)",
        function: Instruction::ADC_A_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x8F
    Instruction{ 
        disassembly: "ADC A,A",
        function: Instruction::ADC_A_A,
        args: 0,
        cycles: 4
    }, 
    //0x90
    Instruction{ 
        disassembly: "SUB A,B",
        function: Instruction::SUB_A_B,
        args: 0,
        cycles: 4
    }, 
    //0x91
    Instruction{ 
        disassembly: "SUB A,C",
        function: Instruction::SUB_A_C,
        args: 0,
        cycles: 4
    }, 
    //0x92
    Instruction{ 
        disassembly: "SUB A,D",
        function: Instruction::SUB_A_D,
        args: 0,
        cycles: 4
    }, 
    //0x93
    Instruction{ 
        disassembly: "SUB A,E",
        function: Instruction::SUB_A_E,
        args: 0,
        cycles: 4
    }, 
    //0x94
    Instruction{ 
        disassembly: "SUB A,H",
        function: Instruction::SUB_A_H,
        args: 0,
        cycles: 4
    }, 
    //0x95
    Instruction{ 
        disassembly: "SUB A,L",
        function: Instruction::SUB_A_L,
        args: 0,
        cycles: 4
    }, 
    //0x96
    Instruction{ 
        disassembly: "SUB A,(HL)",
        function: Instruction::SUB_A_dHL,
        args: 0,
        cycles: 8
    },
    //0x97
    Instruction{ 
        disassembly: "SUB A,A",
        function: Instruction::SUB_A_A,
        args: 0,
        cycles: 4
    }, 
    //0x98
    Instruction{ 
        disassembly: "SBC A,B",
        function: Instruction::SBC_A_B,
        args: 0,
        cycles: 4
    }, 
    //0x99
    Instruction{ 
        disassembly: "SBC A,C",
        function: Instruction::SBC_A_C,
        args: 0,
        cycles: 4
    }, 
    //0x9A
    Instruction{ 
        disassembly: "SBC A,D",
        function: Instruction::SBC_A_D,
        args: 0,
        cycles: 4
    }, 
    //0x9B
    Instruction{ 
        disassembly: "SBC A,E",
        function: Instruction::SBC_A_E,
        args: 0,
        cycles: 4
    }, 
    //0x9C
    Instruction{ 
        disassembly: "SBC A,H",
        function: Instruction::SBC_A_H,
        args: 0,
        cycles: 4
    }, 
    //0x9D
    Instruction{ 
        disassembly: "SBC A,L",
        function: Instruction::SBC_A_L,
        args: 0,
        cycles: 4
    }, 
    //0x9E
    Instruction{ 
        disassembly: "SBC A,(HL)",
        function: Instruction::SBC_A_dHL,
        args: 0,
        cycles: 8
    }, 
    //0x9F
    Instruction{ 
        disassembly: "SBC A,A",
        function: Instruction::SBC_A_A,
        args: 0,
        cycles: 4
    }, 
    //0xA0
    Instruction{ 
        disassembly: "AND B",
        function: Instruction::AND_B,
        args: 0,
        cycles: 4
    }, 
    //0xA1
    Instruction{ 
        disassembly: "AND C",
        function: Instruction::AND_C,
        args: 0,
        cycles: 4
    }, 
    //0xA2
    Instruction{ 
        disassembly: "AND D",
        function: Instruction::AND_D,
        args: 0,
        cycles: 4
    }, 
    //0xA3
    Instruction{ 
        disassembly: "AND E",
        function: Instruction::AND_E,
        args: 0,
        cycles: 4
    }, 
    //0xA4
    Instruction{ 
        disassembly: "AND H",
        function: Instruction::AND_H,
        args: 0,
        cycles: 4
    }, 
    //0xA5
    Instruction{ 
        disassembly: "AND L",
        function: Instruction::AND_L,
        args: 0,
        cycles: 4
    }, 
    //0xA6
    Instruction{ 
        disassembly: "AND (HL)",
        function: Instruction::AND_dHL,
        args: 0,
        cycles: 8
    }, 
    //0xA7
    Instruction{ 
        disassembly: "AND A",
        function: Instruction::AND_A,
        args: 0,
        cycles: 4
    }, 
    //0xA8
    Instruction{ 
        disassembly: "XOR B",
        function: Instruction::XOR_B,
        args: 0,
        cycles: 4
    }, 
    //0xA9
    Instruction{ 
        disassembly: "XOR C",
        function: Instruction::XOR_C,
        args: 0,
        cycles: 4
    }, 
    //0xAA
    Instruction{ 
        disassembly: "XOR D",
        function: Instruction::XOR_D,
        args: 0,
        cycles: 4
    }, 
    //0xAB
    Instruction{ 
        disassembly: "XOR E",
        function: Instruction::XOR_E,
        args: 0,
        cycles: 4
    }, 
    //0xAC
    Instruction{ 
        disassembly: "XOR H",
        function: Instruction::XOR_H,
        args: 0,
        cycles: 4
    }, 
    //0xAD
    Instruction{ 
        disassembly: "XOR L",
        function: Instruction::XOR_L,
        args: 0,
        cycles: 4
    }, 
    //0xAE
    Instruction{ 
        disassembly: "XOR (HL)",
        function: Instruction::XOR_dHL,
        args: 0,
        cycles: 8
    }, 
    //0xAF
    Instruction{ 
        disassembly: "XOR A",
        function: Instruction::XOR_A,
        args: 0,
        cycles: 4
    }, 
    //0xB0
    Instruction{ 
        disassembly: "OR B",
        function: Instruction::OR_B,
        args: 0,
        cycles: 4
    }, 
    //0xB1
    Instruction{ 
        disassembly: "OR C",
        function: Instruction::OR_C,
        args: 0,
        cycles: 4
    }, 
    //0xB2
    Instruction{ 
        disassembly: "OR D",
        function: Instruction::OR_D,
        args: 0,
        cycles: 4
    }, 
    //0xB3
    Instruction{ 
        disassembly: "OR E",
        function: Instruction::OR_E,
        args: 0,
        cycles: 4
    }, 
    //0xB4
    Instruction{ 
        disassembly: "OR H",
        function: Instruction::OR_H,
        args: 0,
        cycles: 4
    }, 
    //0xB5
    Instruction{ 
        disassembly: "OR L",
        function: Instruction::OR_L,
        args: 0,
        cycles: 4
    }, 
    //0xB6
    Instruction{ 
        disassembly: "OR (HL)",
        function: Instruction::OR_dHL,
        args: 0,
        cycles: 8
    }, 
    //0xB7
    Instruction{ 
        disassembly: "OR A",
        function: Instruction::OR_A,
        args: 0,
        cycles: 4
    }, 
    //0xB8
    Instruction{ 
        disassembly: "CP B",
        function: Instruction::CP_B,
        args: 0,
        cycles: 4
    }, 
    //0xB9
    Instruction{ 
        disassembly: "CP C",
        function: Instruction::CP_C,
        args: 0,
        cycles: 4
    }, 
    //0xBA
    Instruction{ 
        disassembly: "CP D",
        function: Instruction::CP_D,
        args: 0,
        cycles: 4
    }, 
    //0xBB
    Instruction{ 
        disassembly: "CP E",
        function: Instruction::CP_E,
        args: 0,
        cycles: 4
    }, 
    //0xBC
    Instruction{ 
        disassembly: "CP H",
        function: Instruction::CP_H,
        args: 0,
        cycles: 4
    }, 
    //0xBD
    Instruction{ 
        disassembly: "CP L",
        function: Instruction::CP_L,
        args: 0,
        cycles: 4
    }, 
    //0xBE
    Instruction{ 
        disassembly: "CP (HL)",
        function: Instruction::CP_dHL,
        args: 0,
        cycles: 8
    }, 
    //0xBF
    Instruction{ 
        disassembly: "CP A",
        function: Instruction::CP_A,
        args: 0,
        cycles: 4
    }, 
    //0xC0
    Instruction{ 
        disassembly: "RET NZ",
        function: Instruction::RET_NZ,
        args: 0,
        cycles: 8
    },
    //0xC1
    Instruction{ 
        disassembly: "POP BC",
        function: Instruction::POP_BC,
        args: 0,
        cycles: 12
    }, 
    //0xC2
    Instruction{ 
        disassembly: "JP NZ,nn",
        function: Instruction::JP_NZ_nn,
        args: 2,
        cycles: 12
    }, 
    //0xC3
    Instruction{ 
        disassembly: "JP nn",
        function: Instruction::JP_nn,
        args: 2,
        cycles: 12
    }, 
    //0xC4
    Instruction{ 
        disassembly: "CALL NZ,nn",
        function: Instruction::CALL_NZ_nn,
        args: 2,
        cycles: 12
    }, 
    //0xC5
    Instruction{ 
        disassembly: "PUSH BC",
        function: Instruction::PUSH_BC,
        args: 0,
        cycles: 16
    }, 
    //0xC6
    Instruction{ 
        disassembly: "ADD A,n",
        function: Instruction::ADD_A_n,
        args: 1,
        cycles: 8
    }, 
    //0xC7
    Instruction{ 
        disassembly: "RST 0",
        function: Instruction::RST_0,
        args: 0,
        cycles: 32
    }, 
    //0xC8
    Instruction{ 
        disassembly: "RET Z",
        function: Instruction::RET_Z,
        args: 0,
        cycles: 8
    }, 
    //0xC9
    Instruction{ 
        disassembly: "RET",
        function: Instruction::RET,
        args: 0,
        cycles: 8
    }, 
    //0xCA
    Instruction{ 
        disassembly: "JP Z,nn",
        function: Instruction::JP_Z_nn,
        args: 2,
        cycles: 12
    }, 
    //0xCB
    Instruction{ 
        disassembly: "CB prefix",
        function: Instruction::NOP,
        args: 3,
        cycles: 0
    }, 
    //0xCC
    Instruction{ 
        disassembly: "CALL Z,nn",
        function: Instruction::CALL_Z_nn,
        args: 2,
        cycles: 12
    }, 
    //0xCD
    Instruction{ 
        disassembly: "CALL nn",
        function: Instruction::CALL_nn,
        args: 2,
        cycles: 12
    }, 
    //0xCE
    Instruction{ 
        disassembly: "ADC A,n",
        function: Instruction::ADC_A_n,
        args: 1,
        cycles: 8
    }, 
    //0xCF
    Instruction{ 
        disassembly: "RST 8",
        function: Instruction::RST_8,
        args: 0,
        cycles: 32
    }, 
    //0xD0
    Instruction{ 
        disassembly: "RET NC",
        function: Instruction::RET_NC,
        args: 0,
        cycles: 8
    }, 
    //0xD1
    Instruction{ 
        disassembly: "POP DE",
        function: Instruction::POP_DE,
        args: 0,
        cycles: 12
    }, 
    //0xD2
    Instruction{ 
        disassembly: "JP NC,nn",
        function: Instruction::JP_NC_nn,
        args: 2,
        cycles: 12
    }, 
    //0xD3
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    //0xD4
    Instruction{ 
        disassembly: "CALL NC,nn",
        function: Instruction::CALL_NC_nn,
        args: 2,
        cycles: 12
    }, 
    //0xD5
    Instruction{ 
        disassembly: "PUSH DE",
        function: Instruction::PUSH_DE,
        args: 0,
        cycles: 16
    }, 
    //0xD6
    Instruction{ 
        disassembly: "SUB A,n",
        function: Instruction::SUB_A_n,
        args: 1,
        cycles: 8
    }, 
    //0xD7
    Instruction{ 
        disassembly: "RST 10",
        function: Instruction::RST_10,
        args: 0,
        cycles: 32
    }, 
    //0xD8
    Instruction{ 
        disassembly: "RET C",
        function: Instruction::RET_C,
        args: 0,
        cycles: 8
    }, 
    //0xD9
    Instruction{ 
        disassembly: "RETI",
        function: Instruction::RETI,
        args: 0,
        cycles: 8
    }, 
    //0xDA
    Instruction{ 
        disassembly: "JP C,nn",
        function: Instruction::JP_C_nn,
        args: 2,
        cycles: 12
    }, 
    //0xDB
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    //0xDC
    Instruction{ 
        disassembly: "CALL C,nn",
        function: Instruction::CALL_C_nn,
        args: 2,
        cycles: 12
    }, 
    //0xDD
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    //0xDE
    Instruction{ 
        disassembly: "SBC A,n",
        function: Instruction::SBC_A_n,
        args: 1,
        cycles: 8
    }, 
    //0xDF
    Instruction{ 
        disassembly: "RST 18",
        function: Instruction::RST_18,
        args: 0,
        cycles: 32
    }, 
    //0xE0
    Instruction{ 
        disassembly: "LDH (n),A",
        function: Instruction::LDH_dn_A,
        args: 1,
        cycles: 12
    }, 
    //0xE1
    Instruction{ 
        disassembly: "POP HL",
        function: Instruction::POP_HL,
        args: 0,
        cycles: 12
    }, 
    //0xE2
    Instruction{ 
        disassembly: "LDH (C),A",
        function: Instruction::LDH_dC,
        args: 0,
        cycles: 8
    }, 
    //0xE3
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    //0xE4
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    //0xE5
    Instruction{ 
        disassembly: "PUSH HL",
        function: Instruction::PUSH_HL,
        args: 0,
        cycles: 16
    }, 
    //0xE6
    Instruction{ 
        disassembly: "AND n",
        function: Instruction::AND_n,
        args: 1,
        cycles: 8
    }, 
    //0xE7
    Instruction{ 
        disassembly: "RST 20",
        function: Instruction::RST_20,
        args: 0,
        cycles: 32
    }, 
    //OxE8
    Instruction{ 
        disassembly: "ADD SP,n",
        function: Instruction::ADD_SP_n,
        args: 1,
        cycles: 16
    }, 
    //0xE9
    Instruction{ 
        disassembly: "JP (HL)",
        function: Instruction::JP_dHL,
        args: 0,
        cycles: 4
    }, 
    //0xEA
    Instruction{ 
        disassembly: "LD (nn),A",
        function: Instruction::LD_dnn_A,
        args: 2,
        cycles: 16
    }, 
    //0xEB
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    //0xEE
    Instruction{ 
        disassembly: "XOR n",
        function: Instruction::XOR_n,
        args: 1,
        cycles: 8
    }, 
    //OxEF
    Instruction{ 
        disassembly: "RST 28",
        function: Instruction::RST_28,
        args: 0,
        cycles: 32
    }, 
    //0xF0
    Instruction{ 
        disassembly: "LDH A,(n)",
        function: Instruction::LDH_A_dn,
        args: 1,
        cycles: 12
    }, 
    //0xF1
    Instruction{ 
        disassembly: "POP AF",
        function: Instruction::POP_AF,
        args: 0,
        cycles: 12
    }, 
    //0xF2
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    //0xF3
    Instruction{ 
        disassembly: "DI",
        function: Instruction::DI,
        args: 0,
        cycles: 4
    }, 
    //0xF4
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    //0xF5
    Instruction{ 
        disassembly: "PUSH AF",
        function: Instruction::PUSH_AF,
        args: 0,
        cycles: 16
    }, 
    //0xF6
    Instruction{ 
        disassembly: "OR n",
        function: Instruction::OR_n,
        args: 1,
        cycles: 8
    }, 
    //0xF7
    Instruction{ 
        disassembly: "RST 30",
        function: Instruction::RST_30,
        args: 0,
        cycles: 32
    }, 
    //0xF8
    Instruction{ 
        disassembly: "LDHL SP,d",
        function: Instruction::LDHL_SP_d,
        args: 1,
        cycles: 12
    }, 
    //0xF9
    Instruction{ 
        disassembly: "LD SP,HL",
        function: Instruction::LD_SP_HL,
        args: 0,
        cycles: 8
    }, 
    //0xFA
    Instruction{ 
        disassembly: "LD A,(nn)",
        function: Instruction::LD_A_dnn,
        args: 2,
        cycles: 16
    }, 
    //0xFB
    Instruction{ 
        disassembly: "EI",
        function: Instruction::EI,
        args: 0,
        cycles: 4
    }, 
    //0xFC
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    Instruction{ 
        disassembly: "REMOVED",
        function: Instruction::NOP,
        args: 0,
        cycles: 0
    }, 
    //0xFE
    Instruction{ 
        disassembly: "CP n",
        function: Instruction::CP_n,
        args: 1,
        cycles: 8
    }, 
    //0xFF
    Instruction{
        disassembly: "RST 38",
        function: Instruction::RST_38,
        args: 0,
        cycles: 32
    }
];

// ===========================================================================
//                           Two-Bytes Instructions
// ===========================================================================


const INSTRUCTION_SUBSET: [Instruction; 256] = [
    //0x0
    Instruction{ 
        disassembly: "RLC B",
        function: Instruction::RLC_B,
        args: 0,
        cycles: 8
    },
    //0x1
    Instruction{ 
        disassembly: "RLC C",
        function: Instruction::RLC_C,
        args: 0,
        cycles: 8
    },
    //0x2
    Instruction{ 
        disassembly: "RLC D",
        function: Instruction::RLC_D,
        args: 0,
        cycles: 8
    },
    //0x3
    Instruction{ 
        disassembly: "RLC E",
        function: Instruction::RLC_E,
        args: 0,
        cycles: 8
    },
    //0x4
    Instruction{ 
        disassembly: "RLC H",
        function: Instruction::RLC_H,
        args: 0,
        cycles: 8
    },
    //0x5
    Instruction{ 
        disassembly: "RLC L",
        function: Instruction::RLC_L,
        args: 0,
        cycles: 8
    },
    //0x6
    Instruction{ 
        disassembly: "RLC (HL)",
        function: Instruction::RLC_dHL,
        args: 0,
        cycles: 16
    },
    //0x7
    Instruction{ 
        disassembly: "RLC A",
        function: Instruction::RLC_A_CB,
        args: 0,
        cycles: 8
    },
    //0x8
    Instruction{ 
        disassembly: "RRC B",
        function: Instruction::RRC_B,
        args: 0,
        cycles: 8
    },
    //0x9
    Instruction{ 
        disassembly: "RRC_C",
        function: Instruction::RRC_C,
        args: 0,
        cycles: 8
    },
    //0xA
    Instruction{ 
        disassembly: "RRC_D",
        function: Instruction::RRC_D,
        args: 0,
        cycles: 8
    },
    //0xB
    Instruction{ 
        disassembly: "RRC E",
        function: Instruction::RRC_E,
        args: 0,
        cycles: 8
    },
    //0xC
    Instruction{ 
        disassembly: "RRC H",
        function: Instruction::RRC_H,
        args: 0,
        cycles: 8
    },
    //0xD
    Instruction{ 
        disassembly: "RRC L",
        function: Instruction::RRC_L,
        args: 0,
        cycles: 8
    },
    //0xE
    Instruction{ 
        disassembly: "RRC (HL)",
        function: Instruction::RRC_dHL,
        args: 0,
        cycles: 16
    },
    //0xF
    Instruction{ 
        disassembly: "RRC A",
        function: Instruction::RRC_A_CB,
        args: 0,
        cycles: 8
    },
    //0x10
    Instruction{ 
        disassembly: "RL B",
        function: Instruction::RL_B,
        args: 0,
        cycles: 8
    },
    //0x11
    Instruction{ 
        disassembly: "RL C",
        function: Instruction::RL_C,
        args: 0,
        cycles: 4
    },
    //0x12
    Instruction{ 
        disassembly: "RL D",
        function: Instruction::RL_D,
        args: 0,
        cycles: 8
    },
    //0x13
    Instruction{ 
        disassembly: "RL E",
        function: Instruction::RL_E,
        args: 0,
        cycles: 8
    },
    //0x14
    Instruction{ 
        disassembly: "RL H",
        function: Instruction::RL_H,
        args: 0,
        cycles: 8
    },
    //0x15
    Instruction{ 
        disassembly: "RL L",
        function: Instruction::RL_L,
        args: 0,
        cycles: 8
    },
    //0x16
    Instruction{ 
        disassembly: "RL (HL)",
        function: Instruction::RL_dHL,
        args: 0,
        cycles: 16
    },
    //0x17
    Instruction{ 
        disassembly: "RL A",
        function: Instruction::RL_A_CB,
        args: 0,
        cycles: 8
    },
    //0x18
    Instruction{ 
        disassembly: "RR_B",
        function: Instruction::RR_B,
        args: 0,
        cycles: 8
    },
    //0x19
    Instruction{ 
        disassembly: "RR C",
        function: Instruction::RR_C,
        args: 0,
        cycles: 8
    },
    //0x1A
    Instruction{ 
        disassembly: "RR D",
        function: Instruction::RR_D,
        args: 0,
        cycles: 8
    },
    //0x1B
    Instruction{ 
        disassembly: "RR E",
        function: Instruction::RR_E,
        args: 0,
        cycles: 8
    },
    //0x1C
    Instruction{ 
        disassembly: "RR H",
        function: Instruction::RR_H,
        args: 0,
        cycles: 8
    },
    //0x1D
    Instruction{ 
        disassembly: "RR L",
        function: Instruction::RR_L,
        args: 0,
        cycles: 8
    },
    //0x1E
    Instruction{ 
        disassembly: "RR (HL)",
        function: Instruction::RR_dHL,
        args: 0,
        cycles: 16
    },
    //0x1F
    Instruction{ 
        disassembly: "RR A",
        function: Instruction::RR_A_CB,
        args: 0,
        cycles: 8
    },
    //0x20
    Instruction{ 
        disassembly: "SLA B",
        function: Instruction::SLA_B,
        args: 0,
        cycles: 8
    },
    //0x21
    Instruction{ 
        disassembly: "SLA C",
        function: Instruction::SLA_C,
        args: 0,
        cycles: 8
    },
    //0x22
    Instruction{ 
        disassembly: "SLA D",
        function: Instruction::SLA_D,
        args: 0,
        cycles: 8
    },
    //0x23
    Instruction{ 
        disassembly: "SLA E",
        function: Instruction::SLA_E,
        args: 0,
        cycles: 8
    },
    //0x24
    Instruction{ 
        disassembly: "SLA H",
        function: Instruction::SLA_H,
        args: 0,
        cycles: 8
    },
    //0x25
    Instruction{ 
        disassembly: "SLA L",
        function: Instruction::SLA_L,
        args: 0,
        cycles: 8
    },
    //0x26
    Instruction{ 
        disassembly: "SLA (HL)",
        function: Instruction::SLA_dHL,
        args: 0,
        cycles: 16
    },
    //0x27
    Instruction{ 
        disassembly: "SLA A",
        function: Instruction::SLA_A,
        args: 0,
        cycles: 8
    },
    //0x28
    Instruction{ 
        disassembly: "SRA B",
        function: Instruction::SRA_B,
        args: 0,
        cycles: 8
    },
    //0x29
    Instruction{ 
        disassembly: "SRA C",
        function: Instruction::SRA_C,
        args: 0,
        cycles: 8
    },
    //0x2A
    Instruction{ 
        disassembly: "SRA D",
        function: Instruction::SRA_D,
        args: 0,
        cycles: 8
    },
    //0x2B
    Instruction{ 
        disassembly: "SRA E",
        function: Instruction::SRA_E,
        args: 0,
        cycles: 8
    },
    //0x2C
    Instruction{ 
        disassembly: "SRA H",
        function: Instruction::SRA_H,
        args: 0,
        cycles: 8
    },
    //0x2D
    Instruction{ 
        disassembly: "SRA L",
        function: Instruction::SRA_L,
        args: 0,
        cycles: 8
    },
    //0x2E
    Instruction{ 
        disassembly: "SRA (HL)",
        function: Instruction::SRA_dHL,
        args: 0,
        cycles: 16
    },
    //0x2F
    Instruction{ 
        disassembly: "SRA A",
        function: Instruction::SRA_A,
        args: 0,
        cycles: 8
    },
    //0x30
    Instruction{ 
        disassembly: "SWAP_B",
        function: Instruction::SWAP_B,
        args: 0,
        cycles: 8
    },
    //0x31
    Instruction{ 
        disassembly: "SWAP C",
        function: Instruction::SWAP_C,
        args: 0,
        cycles: 8
    },
    //0x32
    Instruction{ 
        disassembly: "SWAP D",
        function: Instruction::SWAP_D,
        args: 0,
        cycles: 8
    },
    //0x33
    Instruction{ 
        disassembly: "SWAP E",
        function: Instruction::SWAP_E,
        args: 0,
        cycles: 8
    },
    //0x34
    Instruction{ 
        disassembly: "SWAP H",
        function: Instruction::SWAP_H,
        args: 0,
        cycles: 8
    },
    //0x35
    Instruction{ 
        disassembly: "SWAP L",
        function: Instruction::SWAP_L,
        args: 0,
        cycles: 8
    },
    //0x36
    Instruction{ 
        disassembly: "SWAP (HL)",
        function: Instruction::SWAP_dHL,
        args: 0,
        cycles: 16
    },
    //0x37
    Instruction{ 
        disassembly: "SWAP A",
        function: Instruction::SWAP_A,
        args: 0,
        cycles: 8
    },
    //0x38
    Instruction{ 
        disassembly: "SRL B",
        function: Instruction::SRL_B,
        args: 0,
        cycles: 8
    },
    //0x39
    Instruction{ 
        disassembly: "SRL C",
        function: Instruction::SRL_C,
        args: 0,
        cycles: 8
    },
    //0x3A
    Instruction{ 
        disassembly: "SRL D",
        function: Instruction::SRL_D,
        args: 0,
        cycles: 8
    },
    //0x3B
    Instruction{ 
        disassembly: "SRL E",
        function: Instruction::SRL_E,
        args: 0,
        cycles: 8
    },
    //0x3C
    Instruction{ 
        disassembly: "SRL_H",
        function: Instruction::SRL_H,
        args: 0,
        cycles: 8
    },
    //0x3D
    Instruction{ 
        disassembly: "SRL L",
        function: Instruction::SRL_L,
        args: 0,
        cycles: 8
    },
    //0x3E
    Instruction{ 
        disassembly: "SRL (HL)",
        function: Instruction::SRL_dHL,
        args: 0,
        cycles: 16
    },
    //0x3F
    Instruction{ 
        disassembly: "SRL A",
        function: Instruction::SRL_A,
        args: 0,
        cycles: 8
    },
    //0x40
    Instruction{ 
        disassembly: "BIT 0,B",
        function: Instruction::BIT_0B,
        args: 0,
        cycles: 8
    },
    //0X41
    Instruction{ 
        disassembly: "BIT 0,C",
        function: Instruction::BIT_0C,
        args: 0,
        cycles: 8
    },
    //0x42
    Instruction{ 
        disassembly: "BIT 0,D",
        function: Instruction::BIT_0D,
        args: 0,
        cycles: 8
    },
    //0x43
    Instruction{ 
        disassembly: "BIT 0,E",
        function: Instruction::BIT_0E,
        args: 0,
        cycles: 8
    },
    //0x44
    Instruction{ 
        disassembly: "BIT 0,H",
        function: Instruction::BIT_0H,
        args: 0,
        cycles: 8
    },
    //0x45
    Instruction{ 
        disassembly: "BIT 0,L",
        function: Instruction::BIT_0L,
        args: 0,
        cycles: 8
    },
    //0x46
    Instruction{ 
        disassembly: "BIT 0,(HL)",
        function: Instruction::BIT_0dHL,
        args: 0,
        cycles: 16
    },
    //0x47
    Instruction{ 
        disassembly: "BIT 0,A",
        function: Instruction::BIT_0A,
        args: 0,
        cycles: 8
    },
    //0x48
    Instruction{ 
        disassembly: "BIT 1,B",
        function: Instruction::BIT_1B,
        args: 0,
        cycles: 8
    },
    //0x49
    Instruction{ 
        disassembly: "BIT 1,C",
        function: Instruction::BIT_1C,
        args: 0,
        cycles: 8
    },
    //0x4A
    Instruction{ 
        disassembly: "BIT 1,D",
        function: Instruction::BIT_1D,
        args: 0,
        cycles: 8
    },
    //0x4B
    Instruction{ 
        disassembly: "BIT 1,E",
        function: Instruction::BIT_1E,
        args: 0,
        cycles: 8
    },
    //0x4C
    Instruction{ 
        disassembly: "BIT 1,H",
        function: Instruction::BIT_1H,
        args: 0,
        cycles: 8
    },
    //0x4D
    Instruction{ 
        disassembly: "BIT 1,L",
        function: Instruction::BIT_1L,
        args: 0,
        cycles: 8
    },
    //0x4E
    Instruction{ 
        disassembly: "BIT 1,(HL)",
        function: Instruction::BIT_1dHL,
        args: 0,
        cycles: 16
    },
    //0x4F
    Instruction{ 
        disassembly: "BIT 1,A",
        function: Instruction::BIT_1A,
        args: 0,
        cycles: 8
    },
    //0x50
    Instruction{ 
        disassembly: "BIT 2,B",
        function: Instruction::BIT_2B,
        args: 0,
        cycles: 8
    },
    //0x51
    Instruction{ 
        disassembly: "BIT 2,C",
        function: Instruction::BIT_2C,
        args: 0,
        cycles: 8
    },
    //0x52
    Instruction{ 
        disassembly: "BIT 2,D",
        function: Instruction::BIT_2D,
        args: 0,
        cycles: 8
    },
    //0x53
    Instruction{ 
        disassembly: "BIT 2,E",
        function: Instruction::BIT_2E,
        args: 0,
        cycles: 8
    },
    //0x54
    Instruction{ 
        disassembly: "BIT 2,H",
        function: Instruction::BIT_2H,
        args: 0,
        cycles: 8
    },
    //0x55
    Instruction{ 
        disassembly: "BIT 2,L",
        function: Instruction::BIT_2L,
        args: 0,
        cycles: 8
    },
    //0x56
    Instruction{ 
        disassembly: "BIT 2,(HL)",
        function: Instruction::BIT_2dHL,
        args: 0,
        cycles: 16
    },
    //0x57
    Instruction{ 
        disassembly: "BIT 2,A",
        function: Instruction::BIT_2A,
        args: 0,
        cycles: 8
    },
    //0x58
    Instruction{ 
        disassembly: "BIT 3,B",
        function: Instruction::BIT_3B,
        args: 0,
        cycles: 8
    },
    //0x59
    Instruction{ 
        disassembly: "BIT 3,C",
        function: Instruction::BIT_3C,
        args: 0,
        cycles: 8
    },
    //0x5A
    Instruction{ 
        disassembly: "BIT 3,D",
        function: Instruction::BIT_3D,
        args: 0,
        cycles: 8
    },
    //0x5B
    Instruction{ 
        disassembly: "BIT 3,E",
        function: Instruction::BIT_3E,
        args: 0,
        cycles: 8
    },
    //0x5C
    Instruction{ 
        disassembly: "BIT 3,H",
        function: Instruction::BIT_3H,
        args: 0,
        cycles: 8
    },
    //0x5D
    Instruction{ 
        disassembly: "BIT 3,L",
        function: Instruction::BIT_3L,
        args: 0,
        cycles: 8
    },
    //0x5E
    Instruction{ 
        disassembly: "BIT 3,(HL)",
        function: Instruction::BIT_3dHL,
        args: 0,
        cycles: 16
    },
    //0x5F
    Instruction{ 
        disassembly: "BIT 3,A",
        function: Instruction::BIT_3A,
        args: 0,
        cycles: 8
    },
    //0x60
    Instruction{ 
        disassembly: "BIT 4,B",
        function: Instruction::BIT_4B,
        args: 0,
        cycles: 8
    },
    //0x61
    Instruction{ 
        disassembly: "BIT 4,C",
        function: Instruction::BIT_4C,
        args: 0,
        cycles: 8
    },
    //0x62
    Instruction{ 
        disassembly: "BIT 4,D",
        function: Instruction::BIT_4D,
        args: 0,
        cycles: 8
    },
    //0x63
    Instruction{ 
        disassembly: "BIT 4,E",
        function: Instruction::BIT_4E,
        args: 0,
        cycles: 8
    },
    //0x64
    Instruction{ 
        disassembly: "BIT 4,H",
        function: Instruction::BIT_4H,
        args: 0,
        cycles: 8
    },
    //0x65
    Instruction{ 
        disassembly: "BIT 4,L",
        function: Instruction::BIT_4L,
        args: 0,
        cycles: 8
    },
    //0x66
    Instruction{ 
        disassembly: "BIT 4,(HL)",
        function: Instruction::BIT_4dHL,
        args: 0,
        cycles: 16
    },
    //0x67
    Instruction{ 
        disassembly: "BIT 4,A",
        function: Instruction::BIT_4A,
        args: 0,
        cycles: 8
    },
    //0x68
    Instruction{ 
        disassembly: "BIT 5,B",
        function: Instruction::BIT_5B,
        args: 0,
        cycles: 8
    },
    //0x69
    Instruction{ 
        disassembly: "BIT 5,C",
        function: Instruction::BIT_5C,
        args: 0,
        cycles: 8
    },
    //0x6A
    Instruction{ 
        disassembly: "BIT 5,D",
        function: Instruction::BIT_5D,
        args: 0,
        cycles: 8
    },
    //0x6B
    Instruction{ 
        disassembly: "BIT 5,E",
        function: Instruction::BIT_5E,
        args: 0,
        cycles: 8
    },
    //0x6C
    Instruction{ 
        disassembly: "BIT 5,H",
        function: Instruction::BIT_5H,
        args: 0,
        cycles: 8
    },
    //0x6D
    Instruction{ 
        disassembly: "BIT 5,L",
        function: Instruction::BIT_5L,
        args: 0,
        cycles: 8
    },
    //0x6E
    Instruction{ 
        disassembly: "BIT 5,(HL)",
        function: Instruction::BIT_5dHL,
        args: 0,
        cycles: 16
    },
    //0x6F
    Instruction{ 
        disassembly: "BIT 5,A",
        function: Instruction::BIT_5A,
        args: 0,
        cycles: 8
    },
    //0x70
    Instruction{ 
        disassembly: "BIT 6,B",
        function: Instruction::BIT_6B,
        args: 0,
        cycles: 8
    },
    //0x71
    Instruction{ 
        disassembly: "BIT 6,C",
        function: Instruction::BIT_6C,
        args: 0,
        cycles: 8
    },
    //0x72
    Instruction{ 
        disassembly: "BIT 6,D",
        function: Instruction::BIT_6D,
        args: 0,
        cycles: 8
    },
    //0x73
    Instruction{ 
        disassembly: "BIT 6,E",
        function: Instruction::BIT_6E,
        args: 0,
        cycles: 8
    },
    //0x74
    Instruction{ 
        disassembly: "BIT 6,H",
        function: Instruction::BIT_6H,
        args: 0,
        cycles: 8
    },
    //0x75
    Instruction{ 
        disassembly: "BIT 6,L",
        function: Instruction::BIT_6L,
        args: 0,
        cycles: 8
    },
    //0x76
    Instruction{ 
        disassembly: "BIT 6,(HL)",
        function: Instruction::BIT_6dHL,
        args: 0,
        cycles: 16
    },
    //0x77
    Instruction{ 
        disassembly: "BIT 6,A",
        function: Instruction::BIT_6A,
        args: 0,
        cycles: 8
    },
    //0x78
    Instruction{ 
        disassembly: "BIT 7,B",
        function: Instruction::BIT_7B,
        args: 0,
        cycles: 8
    },
    //0x79
    Instruction{ 
        disassembly: "BIT 7,C",
        function: Instruction::BIT_7C,
        args: 0,
        cycles: 8
    },
    //0x7A
    Instruction{ 
        disassembly: "BIT 7,D",
        function: Instruction::BIT_7D,
        args: 0,
        cycles: 8
    },
    //0x7B
    Instruction{ 
        disassembly: "BIT 7,E",
        function: Instruction::BIT_7E,
        args: 0,
        cycles: 8
    },
    //0x7C
    Instruction{ 
        disassembly: "BIT 7,H",
        function: Instruction::BIT_7H,
        args: 0,
        cycles: 8
    },
    //0x7D
    Instruction{ 
        disassembly: "BIT 7,L",
        function: Instruction::BIT_7L,
        args: 0,
        cycles: 8
    },
    //0x7E
    Instruction{ 
        disassembly: "BIT 7,(HL)",
        function: Instruction::BIT_7dHL,
        args: 0,
        cycles: 16
    },
    //0x7F
    Instruction{ 
        disassembly: "BIT 7,A",
        function: Instruction::BIT_7A,
        args: 0,
        cycles: 8
    },
    //0x80
    Instruction{ 
        disassembly: "RES 0,B",
        function: Instruction::RES_0_B,
        args: 0,
        cycles: 8
    },
    //0x81
    Instruction{ 
        disassembly: "RES 0,C",
        function: Instruction::RES_0_C,
        args: 0,
        cycles: 8
    },
    //0x82
    Instruction{ 
        disassembly: "RES 0,D",
        function: Instruction::RES_0_D,
        args: 0,
        cycles: 8
    },
    //0x83
    Instruction{ 
        disassembly: "RES 0,E",
        function: Instruction::RES_0_E,
        args: 0,
        cycles: 8
    },
    //0x84
    Instruction{ 
        disassembly: "RES 0,H",
        function: Instruction::RES_0_H,
        args: 0,
        cycles: 8
    },
    //0x85
    Instruction{ 
        disassembly: "RES 0,L",
        function: Instruction::RES_0_L,
        args: 0,
        cycles: 8
    },
    //0x86
    Instruction{ 
        disassembly: "RES 0,(HL)",
        function: Instruction::RES_0_dHL,
        args: 0,
        cycles: 16
    },
    //0x87
    Instruction{ 
        disassembly: "RES 0,A",
        function: Instruction::RES_0_A,
        args: 0,
        cycles: 8
    },
    //0x88
    Instruction{ 
        disassembly: "RES 1,B",
        function: Instruction::RES_1_B,
        args: 0,
        cycles: 8
    },
    //0x89
    Instruction{ 
        disassembly: "RES 1,C",
        function: Instruction::RES_1_C,
        args: 0,
        cycles: 8
    },
    //0x8A
    Instruction{ 
        disassembly: "RES 1,D",
        function: Instruction::RES_1_D,
        args: 0,
        cycles: 8
    },
    //0x8B
    Instruction{ 
        disassembly: "RES 1,E",
        function: Instruction::RES_1_E,
        args: 0,
        cycles: 8
    },
    //0x8C
    Instruction{ 
        disassembly: "RES 1,H",
        function: Instruction::RES_1_H,
        args: 0,
        cycles: 8
    },
    //0x8D
    Instruction{ 
        disassembly: "RES 1,L",
        function: Instruction::RES_1_L,
        args: 0,
        cycles: 8
    },
    //0x8E
    Instruction{ 
        disassembly: "RES 1,(HL)",
        function: Instruction::RES_1_dHL,
        args: 0,
        cycles: 16
    },
    //0x8F
    Instruction{ 
        disassembly: "RES 1,A",
        function: Instruction::RES_1_A,
        args: 0,
        cycles: 8
    },
    //0x90
    Instruction{ 
        disassembly: "RES 2,B",
        function: Instruction::RES_2_B,
        args: 0,
        cycles: 8
    },
    //0x91
    Instruction{ 
        disassembly: "RES 2,C",
        function: Instruction::RES_2_C,
        args: 0,
        cycles: 8
    },
    //0X92
    Instruction{ 
        disassembly: "RES 2,D",
        function: Instruction::RES_2_D,
        args: 0,
        cycles: 8
    },
    //0x93
    Instruction{ 
        disassembly: "RES 2,E",
        function: Instruction::RES_2_E,
        args: 0,
        cycles: 8
    },
    //0x94
    Instruction{ 
        disassembly: "RES 2,H",
        function: Instruction::RES_2_H,
        args: 0,
        cycles: 8
    },
    //0x95
    Instruction{ 
        disassembly: "RES 2,L",
        function: Instruction::RES_2_L,
        args: 0,
        cycles: 8
    },
    //0x96
    Instruction{ 
        disassembly: "RES 2,(HL)",
        function: Instruction::RES_2_dHL,
        args: 0,
        cycles: 16
    },
    //0x97
    Instruction{ 
        disassembly: "RES 2,A",
        function: Instruction::RES_2_A,
        args: 0,
        cycles: 8
    },
    //0x98
    Instruction{ 
        disassembly: "RES 3,B",
        function: Instruction::RES_3_B,
        args: 0,
        cycles: 8
    },
    //0x99
    Instruction{ 
        disassembly: "RES 3,C",
        function: Instruction::RES_3_C,
        args: 0,
        cycles: 8
    },
    //0x9A
    Instruction{ 
        disassembly: "RES 3,D",
        function: Instruction::RES_3_D,
        args: 0,
        cycles: 8
    },
    //0x9B
    Instruction{ 
        disassembly: "RES 3,E",
        function: Instruction::RES_3_E,
        args: 0,
        cycles: 8
    },
    //0x9C
    Instruction{ 
        disassembly: "RES 3,H",
        function: Instruction::RES_3_H,
        args: 0,
        cycles: 8
    },
    //0x9D
    Instruction{ 
        disassembly: "RES 3,L",
        function: Instruction::RES_3_L,
        args: 0,
        cycles: 8
    },
    //0x9E
    Instruction{ 
        disassembly: "RES 3,(HL)",
        function: Instruction::RES_3_dHL,
        args: 0,
        cycles: 16
    },
    //0x9F
    Instruction{ 
        disassembly: "RES 3,A",
        function: Instruction::RES_3_A,
        args: 0,
        cycles: 8
    },
    //0xA0
    Instruction{ 
        disassembly: "RES 4,B",
        function: Instruction::RES_4_B,
        args: 0,
        cycles: 8
    },
    //0xA1
    Instruction{ 
        disassembly: "RES 4,C",
        function: Instruction::RES_4_C,
        args: 0,
        cycles: 8
    },
    //0xA2
    Instruction{ 
        disassembly: "RES 4,D",
        function: Instruction::RES_4_D,
        args: 0,
        cycles: 8
    },
    //0xA3
    Instruction{ 
        disassembly: "RES 4,E",
        function: Instruction::RES_4_E,
        args: 0,
        cycles: 8
    },
    //0xA4
    Instruction{ 
        disassembly: "RES 4,H",
        function: Instruction::RES_4_H,
        args: 0,
        cycles: 8
    },
    //0xA5
    Instruction{ 
        disassembly: "RES 4,L",
        function: Instruction::RES_4_L,
        args: 0,
        cycles: 8
    },
    //0xA6
    Instruction{ 
        disassembly: "RES 4,(HL)",
        function: Instruction::RES_4_dHL,
        args: 0,
        cycles: 16
    },
    //0xA7
    Instruction{ 
        disassembly: "RES 4,A",
        function: Instruction::RES_4_A,
        args: 0,
        cycles: 8
    },
    //0xA8
    Instruction{ 
        disassembly: "RES 5,B",
        function: Instruction::RES_5_B,
        args: 0,
        cycles: 8
    },
    //0xA9
    Instruction{ 
        disassembly: "RES 5,C",
        function: Instruction::RES_5_C,
        args: 0,
        cycles: 8
    },
    //0xAA
    Instruction{ 
        disassembly: "RES 5,D",
        function: Instruction::RES_5_D,
        args: 0,
        cycles: 8
    },
    //0xAB
    Instruction{ 
        disassembly: "RES 5,E",
        function: Instruction::RES_5_E,
        args: 0,
        cycles: 8
    },
    //0xAC
    Instruction{ 
        disassembly: "RES 5,H",
        function: Instruction::RES_5_H,
        args: 0,
        cycles: 8
    },
    //0xAD
    Instruction{ 
        disassembly: "RES 5,L",
        function: Instruction::RES_5_L,
        args: 0,
        cycles: 8
    },
    //0xAE
    Instruction{ 
        disassembly: "RES 5,(HL)",
        function: Instruction::RES_5_dHL,
        args: 0,
        cycles: 16
    },
    //0xAF
    Instruction{ 
        disassembly: "RES 5,A",
        function: Instruction::RES_5_A,
        args: 0,
        cycles: 8
    },
    //0xB0
    Instruction{ 
        disassembly: "RES 6,B",
        function: Instruction::RES_6_B,
        args: 0,
        cycles: 8
    },
    //0xB1
    Instruction{ 
        disassembly: "RES 6,C",
        function: Instruction::RES_6_C,
        args: 0,
        cycles: 8
    },
    //0xB2
    Instruction{ 
        disassembly: "RES 6,D",
        function: Instruction::RES_6_D,
        args: 0,
        cycles: 8
    },
    //0xB3
    Instruction{ 
        disassembly: "RES 6,E",
        function: Instruction::RES_6_E,
        args: 0,
        cycles: 8
    },
    //0xB4
    Instruction{ 
        disassembly: "RES 6,H",
        function: Instruction::RES_6_H,
        args: 0,
        cycles: 8
    },
    //0xB5
    Instruction{ 
        disassembly: "RES 6,L",
        function: Instruction::RES_6_L,
        args: 0,
        cycles: 8
    },
    //0xB6
    Instruction{ 
        disassembly: "RES 6,(HL)",
        function: Instruction::RES_6_dHL,
        args: 0,
        cycles: 16
    },
    //0xB7
    Instruction{ 
        disassembly: "RES 6,A",
        function: Instruction::RES_6_A,
        args: 0,
        cycles: 8
    },
    //0xB8
    Instruction{ 
        disassembly: "RES 7,B",
        function: Instruction::RES_7_B,
        args: 0,
        cycles: 8
    },
    //0xB9
    Instruction{ 
        disassembly: "RES 7,C",
        function: Instruction::RES_7_C,
        args: 0,
        cycles: 8
    },
    //0xBA
    Instruction{ 
        disassembly: "RES 7,D",
        function: Instruction::RES_7_D,
        args: 0,
        cycles: 8
    },
    //0xBB
    Instruction{ 
        disassembly: "RES_7_E",
        function: Instruction::RES_7_E,
        args: 0,
        cycles: 8
    },
    //0xBC
    Instruction{ 
        disassembly: "RES 7,H",
        function: Instruction::RES_7_H,
        args: 0,
        cycles: 8
    },
    //0xBD
    Instruction{ 
        disassembly: "RES 7,L",
        function: Instruction::RES_7_L,
        args: 0,
        cycles: 8
    },
    //0xBE
    Instruction{ 
        disassembly: "RES 7,(HL)",
        function: Instruction::RES_7_dHL,
        args: 0,
        cycles: 16
    },
    //0xBF
    Instruction{ 
        disassembly: "RES 7,A",
        function: Instruction::RES_7_A,
        args: 0,
        cycles: 8
    },
    //0xC0
    Instruction{ 
        disassembly: "SET 0,B",
        function: Instruction::SET_0B,
        args: 0,
        cycles: 8
    },
    //0xC1
    Instruction{ 
        disassembly: "SET 0,C",
        function: Instruction::SET_0C,
        args: 0,
        cycles: 8
    },
    //0xC2
    Instruction{ 
        disassembly: "SET 0,D",
        function: Instruction::SET_0D,
        args: 0,
        cycles: 8
    },
    //0xC3
    Instruction{ 
        disassembly: "SET 0,E",
        function: Instruction::SET_0E,
        args: 0,
        cycles: 8
    },
    //0xC4
    Instruction{ 
        disassembly: "SET 0,H",
        function: Instruction::SET_0H,
        args: 0,
        cycles: 8
    },
    //0xC5
    Instruction{ 
        disassembly: "SET 0,L",
        function: Instruction::SET_0L,
        args: 0,
        cycles: 8
    },
    //0xC6
    Instruction{ 
        disassembly: "SET 0,(HL)",
        function: Instruction::SET_0dHL,
        args: 0,
        cycles: 16
    },
    //0xC7
    Instruction{ 
        disassembly: "SET 0,A",
        function: Instruction::SET_0A,
        args: 0,
        cycles: 8
    },
    //0xC8
    Instruction{ 
        disassembly: "SET 1,B",
        function: Instruction::SET_1B,
        args: 0,
        cycles: 8
    },
    //0xC9
    Instruction{ 
        disassembly: "SET 1,C",
        function: Instruction::SET_1C,
        args: 0,
        cycles: 8
    },
    //0xCA
    Instruction{ 
        disassembly: "SET 1,D",
        function: Instruction::SET_1D,
        args: 0,
        cycles: 8
    },
    //0xCB
    Instruction{ 
        disassembly: "SET 1,E",
        function: Instruction::SET_1E,
        args: 0,
        cycles: 8
    },
    //0xCC
    Instruction{ 
        disassembly: "SET 1,H",
        function: Instruction::SET_1H,
        args: 0,
        cycles: 8
    },
    //0xCD
    Instruction{ 
        disassembly: "SET 1,L",
        function: Instruction::SET_1L,
        args: 0,
        cycles: 8
    },
    //0xCE
    Instruction{ 
        disassembly: "SET 1,(HL)",
        function: Instruction::SET_1dHL,
        args: 0,
        cycles: 16
    },
    //0xCF
    Instruction{ 
        disassembly: "SET 1,A",
        function: Instruction::SET_1A,
        args: 0,
        cycles: 8
    },
    //0xD0
    Instruction{ 
        disassembly: "SET 2,B",
        function: Instruction::SET_2B,
        args: 0,
        cycles: 8
    },
    //0xD1
    Instruction{ 
        disassembly: "SET 2,C",
        function: Instruction::SET_2C,
        args: 0,
        cycles: 8
    },
    //0xD2
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xD3
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xD4
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xD5
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xD6
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xD7
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xD8
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xD9
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xDA
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xDB
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xDC
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xDD
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xDE
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xDF
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xE0
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xE1
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xE2
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xE3
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xE4
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xE5
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xE6
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xE7
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xE8
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xE9
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xEA
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xEB
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xEC
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xED
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xEE
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xEF
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xF0
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xF1
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xF2
    Instruction{ 
        disassembly: "SET 6,D",
        function: Instruction::SET_6D,
        args: 0,
        cycles: 8
    },
    //0xF3
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xF4
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xF5
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xF6
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xF7
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xF8
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xF9
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xFA
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xFB
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xFC
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xFD
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xFE
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
    //0xFF
    Instruction{ 
        disassembly: "NOP",
        function: Instruction::NOP,
        args: 0,
        cycles: 4
    },
];
        

