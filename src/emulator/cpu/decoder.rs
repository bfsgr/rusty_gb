use super::instructions;
use instructions::Instruction;
use std::collections::VecDeque;
use super::registers::Registers;
use super::Bus;
pub struct Decoder;

impl Decoder {
    pub fn decode(opcode: u8, subset: bool) -> Result<Instruction, String> {
        if !subset {

            match (opcode & 0xC0) >> 6 {
                0 => {
                    match opcode & 7 {
                        0 => Self::x0z0((opcode & 0x38) >> 3),
                        1 => Self::x0z1((opcode & 0x38) >> 3),
                        2 => Self::x0z2( (opcode & 0x38) >> 3),
                        3 => Self::x0z3( (opcode & 0x38) >> 3),
                        4 => Self::x0z4( (opcode & 0x38) >> 3),
                        5 => Self::x0z5( (opcode & 0x38) >> 3),
                        6 => Self::x0z6( (opcode & 0x38) >> 3),
                        7 => Self::x0z7( (opcode & 0x38) >> 3),
                        _ => Err("Opcode not found".to_owned())
                    }
                },
                1 => Self::match_load(((opcode & 0x38) >> 3, opcode & 7)),
                2 => Self::match_alu(((opcode & 0x38) >> 3, opcode & 7)),
                3 => {
                    match opcode & 7 {
                        0 => Self::x3z0( (opcode & 0x38) >> 3),
                        1 => Self::x3z1( (opcode & 0x38) >> 3),
                        2 => Self::x3z2( (opcode & 0x38) >> 3),
                        3 => Self::x3z3( (opcode & 0x38) >> 3),
                        4 => Self::x3z4( (opcode & 0x38) >> 3),
                        5 => Self::x3z5( (opcode & 0x38) >> 3),
                        6 => Self::x3z6( (opcode & 0x38) >> 3),
                        7 => Self::x3z7( (opcode & 0x38) >> 3),
                        _ => Err("Opcode not found".to_owned())
                    }
                },
                _ => Err("Opcode not found".to_owned())
            }

        } else {
            match (opcode & 0xC0) >> 6 {
                0 => Self::decode_rss(((opcode & 0x38) >> 3, opcode & 7)), //test bit
                1 => Self::decode_bit(((opcode & 0x38) >> 3, opcode & 7)), //test bit
                2 => Self::decode_res(((opcode & 0x38) >> 3, opcode & 7)), //test bit
                3 => Self::decode_set(((opcode & 0x38) >> 3, opcode & 7)), //test bit
                _ => Err("Opcode not found".to_owned())
            }
        }

        // return Ok(Instruction::holder());
    }

    fn match_load(data: (u8,u8)) -> Result<Instruction, String> {
        match data.0 {
            0 => {
                //LD_B_r
                match data.1 {
                    0 => Ok(atomic!("LD B,B", LD_B_B)),
                    1 => Ok(atomic!("LD B,C", LD_B_C)),
                    2 => Ok(atomic!("LD B,D", LD_B_D)),
                    3 => Ok(atomic!("LD B,E", LD_B_E)),
                    4 => Ok(atomic!("LD B,H", LD_B_H)),
                    5 => Ok(atomic!("LD B,L", LD_B_L)),
                    6 => {
                        Ok(Instruction::new(
                            "LD B, (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::write_b8_in_B);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("LD B,A", LD_B_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            1 => {
                //LD_C_r
                match data.1 {
                    0 => Ok(atomic!("LD C,B", LD_C_B)),
                    1 => Ok(atomic!("LD C,C", LD_C_C)),
                    2 => Ok(atomic!("LD C,D", LD_C_D)),
                    3 => Ok(atomic!("LD C,E", LD_C_E)),
                    4 => Ok(atomic!("LD C,H", LD_C_H)),
                    5 => Ok(atomic!("LD C,L", LD_C_L)),
                    6 => {
                        Ok(Instruction::new(
                            "LD C, (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::write_b8_in_C);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("LD C,A", LD_C_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            2 => {
                //LD_D_r
                match data.1 {
                    0 => Ok(atomic!("LD D,B", LD_D_B)),
                    1 => Ok(atomic!("LD D,C", LD_D_C)),
                    2 => Ok(atomic!("LD D,D", LD_D_D)),
                    3 => Ok(atomic!("LD D,E", LD_D_E)),
                    4 => Ok(atomic!("LD D,H", LD_D_H)),
                    5 => Ok(atomic!("LD D,L", LD_D_L)),
                    6 => {
                        Ok(Instruction::new(
                            "LD D, (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::write_b8_in_D);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("LD D,A", LD_D_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            3 => {
                //LD_E_r
                match data.1 {
                    0 => Ok(atomic!("LD E,B", LD_E_B)),
                    1 => Ok(atomic!("LD E,C", LD_E_C)),
                    2 => Ok(atomic!("LD E,D", LD_E_D)),
                    3 => Ok(atomic!("LD E,E", LD_E_E)),
                    4 => Ok(atomic!("LD E,H", LD_E_H)),
                    5 => Ok(atomic!("LD E,L", LD_E_L)),
                    6 => {
                        Ok(Instruction::new(
                            "LD E, (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::write_b8_in_E);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("LD E,A", LD_E_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            4 => {
                //LD_H_r
                match data.1 {
                    0 => Ok(atomic!("LD H,B", LD_H_B)),
                    1 => Ok(atomic!("LD H,C", LD_H_C)),
                    2 => Ok(atomic!("LD H,D", LD_H_D)),
                    3 => Ok(atomic!("LD H,E", LD_H_E)),
                    4 => Ok(atomic!("LD H,H", LD_H_H)),
                    5 => Ok(atomic!("LD H,L", LD_H_L)),
                    6 => {
                        Ok(Instruction::new(
                            "LD E, (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::write_b8_in_H);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("LD H,A", LD_H_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            5 => {
                //LD_L_r
                match data.1 {
                    0 => Ok(atomic!("LD L,B", LD_L_B)),
                    1 => Ok(atomic!("LD L,C", LD_L_C)),
                    2 => Ok(atomic!("LD L,D", LD_L_D)),
                    3 => Ok(atomic!("LD L,E", LD_L_E)),
                    4 => Ok(atomic!("LD L,H", LD_L_H)),
                    5 => Ok(atomic!("LD L,L", LD_L_L)),
                    6 => {
                        Ok(Instruction::new(
                            "LD L, (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::write_b8_in_L);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("LD L,A", LD_L_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            6 => {
                //LD_L_r
                match data.1 {
                    0 => {
                        Ok(Instruction::new(
                            "LD (HL), B",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_B_in_dHL);
                                o
                            },
                            2
                        ))
                    },
                    1 => {
                        Ok(Instruction::new(
                            "LD (HL), C",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_C_in_dHL);
                                o
                            },
                            2
                        ))
                    },
                    2 => {
                        Ok(Instruction::new(
                            "LD (HL), D",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_D_in_dHL);
                                o
                            },
                            2
                        ))
                    },
                    3 => {
                        Ok(Instruction::new(
                            "LD (HL), E",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_E_in_dHL);
                                o
                            },
                            2
                        ))
                    },
                    4 => {
                        Ok(Instruction::new(
                            "LD (HL), H",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_H_in_dHL);
                                o
                            },
                            2
                        ))
                    },
                    5 => {
                        Ok(Instruction::new(
                            "LD (HL), L",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_L_in_dHL);
                                o
                            },
                            2
                        ))
                    },
                    6 => {
                        Ok(Instruction::new(
                            "HALT",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::halt);
                                o
                            },
                            1
                        ))
                    },
                    7 => {
                        Ok(Instruction::new(
                            "LD (HL), A",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_A_in_dHL);
                                o
                            },
                            2
                        ))
                    },
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            7 => {
                //LD_A_r
                match data.1 {
                    0 => Ok(atomic!("LD A,B", LD_A_B)),
                    1 => Ok(atomic!("LD A,C", LD_A_C)),
                    2 => Ok(atomic!("LD A,D", LD_A_D)),
                    3 => Ok(atomic!("LD A,E", LD_A_E)),
                    4 => Ok(atomic!("LD A,H", LD_A_H)),
                    5 => Ok(atomic!("LD A,L", LD_A_L)),
                    6 => {
                        Ok(Instruction::new(
                            "LD A, (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::write_b8_in_A);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("LD A,A", LD_A_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            _ => Err("Instruction not found".to_owned()),
        }
    }

    fn match_alu(data: (u8,u8)) -> Result<Instruction, String> {
        match data.0 {
            0 => {
                //ADD A,r
                match data.1 {
                    0 => Ok(atomic!("ADD A,B", ADD_A_B)),
                    1 => Ok(atomic!("ADD A,C", ADD_A_C)),
                    2 => Ok(atomic!("ADD A,D", ADD_A_D)),
                    3 => Ok(atomic!("ADD A,E", ADD_A_E)),
                    4 => Ok(atomic!("ADD A,H", ADD_A_H)),
                    5 => Ok(atomic!("ADD A,L", ADD_A_L)),
                    6 => {
                        Ok(Instruction::new(
                            "ADD A,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::add_with_buffer);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("ADD A,A", ADD_A_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            1 => {
                //ADC A,r
                match data.1 {
                    0 => Ok(atomic!("ADC A,B", ADC_A_B)),
                    1 => Ok(atomic!("ADC A,C", ADC_A_C)),
                    2 => Ok(atomic!("ADC A,D", ADC_A_D)),
                    3 => Ok(atomic!("ADC A,E", ADC_A_E)),
                    4 => Ok(atomic!("ADC A,H", ADC_A_H)),
                    5 => Ok(atomic!("ADC A,L", ADC_A_L)),
                    6 => {
                        Ok(Instruction::new(
                            "ADC A,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::adc_with_buffer);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("ADC A,A", ADC_A_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            2 => {
                //SUB A,r
                match data.1 {
                    0 => Ok(atomic!("SUB A,B", SUB_A_B)),
                    1 => Ok(atomic!("SUB A,C", SUB_A_C)),
                    2 => Ok(atomic!("SUB A,D", SUB_A_D)),
                    3 => Ok(atomic!("SUB A,E", SUB_A_E)),
                    4 => Ok(atomic!("SUB A,H", SUB_A_H)),
                    5 => Ok(atomic!("SUB A,L", SUB_A_L)),
                    6 => {
                        Ok(Instruction::new(
                            "SUB A,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::sub_with_buffer);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("SUB A,A", SUB_A_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            3 => {
                //SBC A,r
                match data.1 {
                    0 => Ok(atomic!("SBC A,B", SBC_A_B)),
                    1 => Ok(atomic!("SBC A,C", SBC_A_C)),
                    2 => Ok(atomic!("SBC A,D", SBC_A_D)),
                    3 => Ok(atomic!("SBC A,E", SBC_A_E)),
                    4 => Ok(atomic!("SBC A,H", SBC_A_H)),
                    5 => Ok(atomic!("SBC A,L", SBC_A_L)),
                    6 => {
                        Ok(Instruction::new(
                            "SBC A,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::sbc_with_buffer);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("SBC A,A", SBC_A_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            4 => {
                //AND A,r
                match data.1 {
                    0 => Ok(atomic!("AND B", AND_B)),
                    1 => Ok(atomic!("AND C", AND_C)),
                    2 => Ok(atomic!("AND D", AND_D)),
                    3 => Ok(atomic!("AND E", AND_E)),
                    4 => Ok(atomic!("AND H", AND_H)),
                    5 => Ok(atomic!("AND L", AND_L)),
                    6 => {
                        Ok(Instruction::new(
                            "AND (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::and_with_buffer);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("AND A", AND_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            5 => {
                //XOR A,r
                match data.1 {
                    0 => Ok(atomic!("XOR B", XOR_B)),
                    1 => Ok(atomic!("XOR C", XOR_C)),
                    2 => Ok(atomic!("XOR D", XOR_D)),
                    3 => Ok(atomic!("XOR E", XOR_E)),
                    4 => Ok(atomic!("XOR H", XOR_H)),
                    5 => Ok(atomic!("XOR L", XOR_L)),
                    6 => {
                        Ok(Instruction::new(
                            "XOR (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::xor_with_buffer);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("XOR A", XOR_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            6 => {
                //OR A,r
                match data.1 {
                    0 => Ok(atomic!("OR B", OR_B)),
                    1 => Ok(atomic!("OR C", OR_C)),
                    2 => Ok(atomic!("OR D", OR_D)),
                    3 => Ok(atomic!("OR E", OR_E)),
                    4 => Ok(atomic!("OR H", OR_H)),
                    5 => Ok(atomic!("OR L", OR_L)),
                    6 => {
                        Ok(Instruction::new(
                            "OR (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::or_with_buffer);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("OR A", OR_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            7 => {
                //CP A,r
                match data.1 {
                    0 => Ok(atomic!("CP B", CP_B)),
                    1 => Ok(atomic!("CP C", CP_C)),
                    2 => Ok(atomic!("CP D", CP_D)),
                    3 => Ok(atomic!("CP E", CP_E)),
                    4 => Ok(atomic!("CP H", CP_H)),
                    5 => Ok(atomic!("CP L", CP_L)),
                    6 => {
                        Ok(Instruction::new(
                            "CP (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::cp_with_buffer);
                                o
                            },
                            2
                        ))
                    },
                    7 => Ok(atomic!("CP A", CP_A)),
                    _ => Err("Instruction not found".to_owned()),
                }
            },
            _ => Err("Instruction not found".to_owned()),
        }
    }

    fn x0z0(data: u8) -> Result<Instruction, String> {
        match data {
            0 => {
                //NOP
                Ok(Instruction::new(
                    "NOP",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o
                    },
                    1
                ))
            },
            1 => {
                //LD (nn),SP
                Ok(Instruction::new(
                    "LD (nn),SP",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::write_sp_low);
                        o.push_back(Instruction::inc_buffer_u16);
                        o.push_back(Instruction::write_sp_high);
                        o
                    },
                    5
                ))
            },
            2 => {
                Ok(Instruction::new(
                    "STOP",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::stop);
                        o
                    },
                    1
                ))
            },
            3 => {
                Ok(Instruction::new(
                    "JR n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::jr_n);
                        o
                    },
                    3
                ))
            },
            4 => {
                Ok(Instruction::new(
                    "JR NZ,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::compare_nz);
                        o.push_back(Instruction::jr_if);
                        o
                    },
                    3
                ))
            },
            5 => {
                Ok(Instruction::new(
                    "JR Z,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::compare_z);
                        o.push_back(Instruction::jr_if);
                        o
                    },
                    3
                ))
            },
            6 => {
                Ok(Instruction::new(
                    "JR NC,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::compare_nc);
                        o.push_back(Instruction::jr_if);
                        o
                    },
                    3
                ))
            },
            7 => {
                Ok(Instruction::new(
                    "JR C,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::compare_c);
                        o.push_back(Instruction::jr_if);
                        o
                    },
                    3
                ))
            },
            _ => { Err("Instruction not found".to_owned()) }
        }

    }

    fn x0z1(data: u8) -> Result<Instruction, String> {

        match data {
            0 => {
                //NOP
                Ok(Instruction::new(
                    "LD BC,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::write_b16_to_BC);
                        o
                    },
                    3
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "ADD HL,BC",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::add_bc);
                        o
                    },
                    2
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "LD DE,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::write_b16_to_DE);
                        o
                    },
                    3
                )) 
            },
            3 => {
                Ok(Instruction::new(
                    "ADD HL,DE",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::add_de);
                        o
                    },
                    2
                )) 
            },
            4 => {
                Ok(Instruction::new(
                    "LD HL,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::write_b16_to_HL);
                        o
                    },
                    3
                )) 
            },
            5 => {
                Ok(Instruction::new(
                    "ADD HL,HL",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::add_hl);
                        o
                    },
                    2
                )) 
            },
            6 => {
                Ok(Instruction::new(
                    "LD SP,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::write_b16_to_SP);
                        o
                    },
                    3
                )) 
            },
            7 => {
                Ok(Instruction::new(
                    "ADD HL,SP",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::add_sp);
                        o
                    },
                    2
                )) 
            },
            _ => { Err("Instruction not found".to_owned()) },
        }

    }

    fn x0z2(data: u8) -> Result<Instruction, String> {

        match data {
            0 => {
                //NOP
                Ok(Instruction::new(
                    "LD (BC),A",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::ld_dBC_A);
                        o
                    },
                    2
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "LD A,(BC)",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::read_bus_with_BC);
                        o.push_back(Instruction::write_b8_in_A);
                        o
                    },
                    2
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "LD (DE),A",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::ld_dDE_A);
                        o
                    },
                    2
                )) 
            },
            3 => {
                Ok(Instruction::new(
                    "LD A,(DE)",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::read_bus_with_DE);
                        o.push_back(Instruction::write_b8_in_A);
                        o
                    },
                    2
                )) 
            },
            4 => {
                Ok(Instruction::new(
                    "LD (HL+),A",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::write_A_in_dHL);
                        o.push_back(Instruction::INC_HL);
                        o
                    },
                    2
                )) 
            },
            5 => {
                Ok(Instruction::new(
                    "LD A,(HL+)",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::write_dHL_to_A);
                        o.push_back(Instruction::INC_HL);
                        o
                    },
                    2
                )) 
            },
            6 => {
                Ok(Instruction::new(
                    "LD (HL-),A",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::write_A_in_dHL);
                        o.push_back(Instruction::DEC_HL);
                        o
                    },
                    2
                )) 
            },
            7 => {
                Ok(Instruction::new(
                    "LD A,(HL-)",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::write_dHL_to_A);
                        o.push_back(Instruction::DEC_HL);
                        o
                    },
                    2
                )) 
            },
            _ => { Err("Instruction not found".to_owned()) },
        }

    }

    fn x0z3(data: u8) -> Result<Instruction, String> {

        match data {
            0 => {
                //NOP
                Ok(Instruction::new(
                    "INC BC",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::INC_BC);
                        o
                    },
                    2
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "DEC BC",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::DEC_BC);
                        o
                    },
                    2
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "INC DE",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::INC_DE);
                        o
                    },
                    2
                )) 
            },
            3 => {
                Ok(Instruction::new(
                    "DEC DE",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::DEC_DE);
                        o
                    },
                    2
                )) 
            },
            4 => {
                Ok(Instruction::new(
                    "INC HL",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::INC_HL);
                        o
                    },
                    2
                )) 
            },
            5 => {
                Ok(Instruction::new(
                    "DEC HL",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::DEC_HL);
                        o
                    },
                    2
                )) 
            },
            6 => {
                Ok(Instruction::new(
                    "INC SP",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::INC_SP);
                        o
                    },
                    2
                )) 
            },
            7 => {
                Ok(Instruction::new(
                    "DEC SP",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::DEC_SP);
                        o
                    },
                    2
                )) 
            },
            _ => { Err("Instruction not found".to_owned()) },
        }

    }
  
    fn x0z4(data: u8) -> Result<Instruction, String> {

        match data {
            0 => Ok(atomic!("INC B", INC_B)),
            1 => Ok(atomic!("INC C", INC_C)),
            2 => Ok(atomic!("INC D", INC_D)),
            3 => Ok(atomic!("INC E", INC_E)),
            4 => Ok(atomic!("INC H", INC_H)),
            5 => Ok(atomic!("INC L", INC_L)),
            6 => {
                Ok(Instruction::new(
                    "INC (HL)",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::read_bus_with_HL);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::inc_buffer_u8);
                        o
                    },
                    3
                )) 
            },
            7 => Ok(atomic!("INC A", INC_A)),
            _ => { Err("Instruction not found".to_owned()) },
        }

    }
  
    fn x0z5(data: u8) -> Result<Instruction, String> {

        match data {
            0 => Ok(atomic!("DEC B", DEC_B)),
            1 => Ok(atomic!("DEC C", DEC_C)),
            2 => Ok(atomic!("DEC D", DEC_D)),
            3 => Ok(atomic!("DEC E", DEC_E)),
            4 => Ok(atomic!("DEC H", DEC_H)),
            5 => Ok(atomic!("DEC L", DEC_L)),
            6 => {
                Ok(Instruction::new(
                    "DEC (HL)",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::read_bus_with_HL);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::dec_buffer_u8);
                        o
                    },
                    3
                )) 
            },
            7 => Ok(atomic!("DEC A", DEC_A)),
            _ => { Err("Instruction not found".to_owned()) },
        }

    }

    fn x0z6(data: u8) -> Result<Instruction, String> {

        match data {
            0 => {
                //NOP
                Ok(Instruction::new(
                    "LD B,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::write_b8_in_B);
                        o
                    },
                    2
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "LD C,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::write_b8_in_C);
                        o
                    },
                    2
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "LD D,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::write_b8_in_D);
                        o
                    },
                    2
                )) 
            },
            3 => {
                Ok(Instruction::new(
                    "LD E,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::write_b8_in_E);
                        o
                    },
                    2
                )) 
            },
            4 => {
                Ok(Instruction::new(
                    "LD H,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::write_b8_in_H);
                        o
                    },
                    2
                )) 
            },
            5 => {
                Ok(Instruction::new(
                    "LD L,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::write_b8_in_L);
                        o
                    },
                    2
                )) 
            },
            6 => {
                Ok(Instruction::new(
                    "LD (HL),n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::write_buffer_to_dHL);
                        o
                    },
                    3
                )) 
            },
            7 => {
                Ok(Instruction::new(
                    "LD A,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::write_b8_in_A);
                        o
                    },
                    2
                )) 
            },
            _ => { Err("Instruction not found".to_owned()) },
        }

    }

    fn x0z7(data: u8) -> Result<Instruction, String> {
        match data {
            0 => Ok(atomic!("RLC A", RLC_A)),
            1 => Ok(atomic!("RRC A", RRC_A)),
            2 => Ok(atomic!("RLA", RL_A)),
            3 => Ok(atomic!("RRA", RR_A)),
            4 => Ok(atomic!("DAA", DAA)),
            5 => Ok(atomic!("CPL", NOT_A)),
            6 => Ok(atomic!("SCF", SCF)),
            7 => Ok(atomic!("CCF", CCF)),
            _ => { Err("Instruction not found".to_owned()) },
        }

    }

    fn x3z0(data: u8) -> Result<Instruction, String> {
        match data {
            0 => {
                //NOP
                Ok(Instruction::new(
                    "RET NZ",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::compare_nz);
                        o.push_back(Instruction::read_bus_with_SP);
                        o.push_back(Instruction::INC_SP);
                        o.push_back(Instruction::finish_ret);
                        o
                    },
                    5
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "RET Z",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::compare_z);
                        o.push_back(Instruction::read_bus_with_SP);
                        o.push_back(Instruction::INC_SP);
                        o.push_back(Instruction::finish_ret);
                        o
                    },
                    5
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "RET NC",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::compare_nc);
                        o.push_back(Instruction::read_bus_with_SP);
                        o.push_back(Instruction::INC_SP);
                        o.push_back(Instruction::finish_ret);
                        o
                    },
                    5
                )) 
            },
            3 => {
                Ok(Instruction::new(
                    "RET C",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::compare_c);
                        o.push_back(Instruction::read_bus_with_SP);
                        o.push_back(Instruction::INC_SP);
                        o.push_back(Instruction::finish_ret);
                        o
                    },
                    5
                )) 
            },
            4 => {
                Ok(Instruction::new(
                    "LD (FF00+n),A",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::sum_ff00_to_b8);
                        o.push_back(Instruction::write_A_to_b16);
                        o
                    },
                    3
                )) 

            },
            5 => {
                Ok(Instruction::new(
                    "ADD SP,dd",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::add_sp_dd);
                        o
                    },
                    4
                )) 
            },
            6 => {
                Ok(Instruction::new(
                    "LD A,(FF00+n)",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::sum_ff00_to_b8);
                        o.push_back(Instruction::read_b16_write_A);
                        o
                    },
                    3
                )) 
            },
            7 => {
                Ok(Instruction::new(
                    "LD HL,SP+dd",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::ldhl_sp_dd);
                        o
                    },
                    3
                )) 
            },
            _ => { Err("Instruction not found".to_owned()) },
        }

    }

    fn x3z1(data: u8) -> Result<Instruction, String> {
        match data {
            0 => {
                Ok(Instruction::new(
                    "POP BC",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::read_bus_with_SP);
                        o.push_back(Instruction::write_b8_in_C);
                        o.push_back(Instruction::finish_pop_B);
                        o
                    },
                    3
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "RET",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::read_bus_with_SP);
                        o.push_back(Instruction::INC_SP);
                        o.push_back(Instruction::finish_ret);
                        o
                    },
                    4
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "POP DE",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::read_bus_with_SP);
                        o.push_back(Instruction::write_b8_in_E);
                        o.push_back(Instruction::finish_pop_D);
                        o
                    },
                    3
                )) 
            },
            3 => {
                Ok(Instruction::new(
                    "RETI",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::read_bus_with_SP);
                        o.push_back(Instruction::INC_SP);
                        o.push_back(Instruction::finish_ret);
                        o.push_back(Instruction::ei);
                        o
                    },
                    4
                )) 
            },
            4 => {
                Ok(Instruction::new(
                    "POP HL",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::read_bus_with_SP);
                        o.push_back(Instruction::write_b8_in_L);
                        o.push_back(Instruction::finish_pop_H);
                        o
                    },
                    3
                )) 
            },
            5 => Ok(atomic!("JP HL", JP_HL)),
            6 => {
                Ok(Instruction::new(
                    "POP AF",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::read_bus_with_SP);
                        o.push_back(Instruction::write_b8_in_F);
                        o.push_back(Instruction::finish_pop_A);
                        o
                    },
                    3
                )) 
            },
            7 => {
                Ok(Instruction::new(
                    "LD HL,SP",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::ld_sp_hl);
                        o
                    },
                    2
                )) 
            },
            _ => { Err("Instruction not found".to_owned()) },
        }

    }

    fn x3z2(data: u8) -> Result<Instruction, String> {
        match data {
            0 => {
                Ok(Instruction::new(
                    "JP NZ,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::compare_nz);
                        o.push_back(Instruction::jp_nn);
                        o
                    },
                    4
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "JP Z,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::compare_z);
                        o.push_back(Instruction::jp_nn);
                        o
                    },
                    4
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "JP NC,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::compare_nc);
                        o.push_back(Instruction::jp_nn);
                        o
                    },
                    4
                )) 
            },
            3 => {
                Ok(Instruction::new(
                    "JP C,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::compare_c);
                        o.push_back(Instruction::jp_nn);
                        o
                    },
                    4
                )) 
            },
            4 => {
                Ok(Instruction::new(
                    "LD (FF00+C),A",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::sum_ff00_to_C);
                        o.push_back(Instruction::write_A_to_b16);
                        o
                    },
                    2
                )) 
            },
            5 => {
                Ok(Instruction::new(
                    "LD (nn),A",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::write_A_to_b16);
                        o
                    },
                    4
                )) 
            },
            6 => {
                Ok(Instruction::new(
                    "LD A,(FF00+C)",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::sum_ff00_to_C);
                        o.push_back(Instruction::read_b16_write_A);
                        o
                    },
                    2
                )) 
            },
            7 => {
                Ok(Instruction::new(
                    "LD A,(nn)",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::read_b16_write_A);
                        o
                    },
                    4
                )) 
            },
            _ => { Err("Instruction not found".to_owned()) },
        }

    }
    
    fn x3z3(data: u8) -> Result<Instruction, String> {
        match data {
            0 => {
                Ok(Instruction::new(
                    "JP nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::jp_nn);
                        o
                    },
                    4
                )) 
            },
            1 => Err("0xCB prefix".to_owned()),
            2 => Err("Removed Opcode".to_owned()),
            3 => Err("Removed Opcode".to_owned()),
            4 => Err("Removed Opcode".to_owned()),
            5 => Err("Removed Opcode".to_owned()),
            6 => Ok(atomic!("DI", disable_interrupts)),
            7 => Ok(atomic!("EI", enable_interrupts)),
            _ => { Err("Instruction not found".to_owned()) },
        }

    }

    fn x3z4(data: u8) -> Result<Instruction, String> {
        match data {
            0 => {
                Ok(Instruction::new(
                    "CALL NZ,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::compare_nz);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::finish_call);
                        o
                    },
                    6
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "CALL Z,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::compare_z);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::finish_call);
                        o
                    },
                    6
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "CALL NC,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::compare_nc);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::finish_call);
                        o
                    },
                    6
                )) 
            },
            3 => {
                Ok(Instruction::new(
                    "CALL C,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::compare_c);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::finish_call);
                        o
                    },
                    6
                )) 
            },
            4 => Err("Removed Opcode".to_owned()),
            5 => Err("Removed Opcode".to_owned()),
            6 => Err("Removed Opcode".to_owned()),
            7 => Err("Removed Opcode".to_owned()),
            _ => Err("Instruction not found".to_owned()),
        }

    }

    fn x3z5(data: u8) -> Result<Instruction, String> {
        match data {
            0 => {
                Ok(Instruction::new(
                    "PUSH BC",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_B_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_C_in_dSP);
                        o
                    },
                    4
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "CALL nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::nop);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::finish_call);
                        o
                    },
                    6
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "PUSH DE",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_D_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_E_in_dSP);
                        o
                    },
                    4
                )) 
            },
            3 => Err("Removed Opcode".to_owned()),
            4 => {
                Ok(Instruction::new(
                    "PUSH HL",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_H_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_L_in_dSP);
                        o
                    },
                    4
                )) 
            },
            5 => Err("Removed Opcode".to_owned()),
            6 => {
                Ok(Instruction::new(
                    "PUSH AF",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_A_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_F_in_dSP);
                        o
                    },
                    4
                )) 
            },
            7 => Err("Removed Opcode".to_owned()),
            _ => Err("Instruction not found".to_owned()),
        }

    }
    
    fn x3z6(data: u8) -> Result<Instruction, String> {
        match data {
            0 => {
                Ok(Instruction::new(
                    "ADD A,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::add_with_buffer);

                        o
                    },
                    2
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "ADC A,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::adc_with_buffer);

                        o
                    },
                    2
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "SUB A,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::sub_with_buffer);

                        o
                    },
                    2
                )) 
            },
            3 => {
                Ok(Instruction::new(
                    "SBC A,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::sbc_with_buffer);

                        o
                    },
                    2
                )) 
            },
            4 => {
                Ok(Instruction::new(
                    "AND A,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::and_with_buffer);

                        o
                    },
                    2
                )) 
            },
            5 => {
                Ok(Instruction::new(
                    "XOR A,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::xor_with_buffer);

                        o
                    },
                    2
                )) 
            },
            6 => {
                Ok(Instruction::new(
                    "OR A,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::or_with_buffer);

                        o
                    },
                    2
                )) 
            },
            7 => {
                Ok(Instruction::new(
                    "CP A,n",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::cp_with_buffer);

                        o
                    },
                    2
                )) 
            },
            _ => Err("Instruction not found".to_owned()),
        }

    }

    fn x3z7(data: u8) -> Result<Instruction, String> {
        match data {
            0 => {
                Ok(Instruction::new(
                    "RST 0",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::rst_0);

                        o
                    },
                    4
                )) 
            },
            1 => {
                Ok(Instruction::new(
                    "RST 8",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::rst_8);

                        o
                    },
                    4
                )) 
            },
            2 => {
                Ok(Instruction::new(
                    "RST 10",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::rst_10);

                        o
                    },
                    4
                )) 
            },
            3 => {
                Ok(Instruction::new(
                    "RST 18",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::rst_18);

                        o
                    },
                    4
                )) 
            },
            4 => {
                Ok(Instruction::new(
                    "RST 20",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::rst_20);

                        o
                    },
                    4
                )) 
            },
            5 => {
                Ok(Instruction::new(
                    "RST 28",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::rst_28);

                        o
                    },
                    4
                )) 
            },
            6 => {
                Ok(Instruction::new(
                    "RST 30",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::rst_30);

                        o
                    },
                    4
                ))  
            },
            7 => {
                Ok(Instruction::new(
                    "RST 38",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::write_P_in_dSP);
                        o.push_back(Instruction::DEC_SP);
                        o.push_back(Instruction::rst_38);

                        o
                    },
                    4
                )) 
            },
            _ => Err("Instruction not found".to_owned()),
        }

    }

    fn decode_bit(data: (u8,u8)) -> Result<Instruction, String> {
        match data.0 {
            //bit 0,r
            0 => {
                match data.1 {
                    0 => Ok(subset_atomic!("BIT 0,B", BIT_0B)),
                    1 => Ok(subset_atomic!("BIT 0,C", BIT_0C)),
                    2 => Ok(subset_atomic!("BIT 0,D", BIT_0D)),
                    3 => Ok(subset_atomic!("BIT 0,E", BIT_0E)),
                    4 => Ok(subset_atomic!("BIT 0,H", BIT_0H)),
                    5 => Ok(subset_atomic!("BIT 0,L", BIT_0L)),
                    6 => {
                        Ok(Instruction::new(
                            "BIT 0,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::bit_0_buffer);
        
                                o
                            },
                            3
                        ))
                    },
                    7 => Ok(subset_atomic!("BIT 0,A", BIT_0A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            1 => {
                match data.1 {
                    0 => Ok(subset_atomic!("BIT 1,B", BIT_1B)),
                    1 => Ok(subset_atomic!("BIT 1,C", BIT_1C)),
                    2 => Ok(subset_atomic!("BIT 1,D", BIT_1D)),
                    3 => Ok(subset_atomic!("BIT 1,E", BIT_1E)),
                    4 => Ok(subset_atomic!("BIT 1,H", BIT_1H)),
                    5 => Ok(subset_atomic!("BIT 1,L", BIT_1L)),
                    6 => {
                        Ok(Instruction::new(
                            "BIT 1,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::bit_1_buffer);
        
                                o
                            },
                            3
                        ))
                    },
                    7 => Ok(subset_atomic!("BIT 1,A", BIT_1A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            2 => {
                match data.1 {
                    0 => Ok(subset_atomic!("BIT 2,B", BIT_2B)),
                    1 => Ok(subset_atomic!("BIT 2,C", BIT_2C)),
                    2 => Ok(subset_atomic!("BIT 2,D", BIT_2D)),
                    3 => Ok(subset_atomic!("BIT 2,E", BIT_2E)),
                    4 => Ok(subset_atomic!("BIT 2,H", BIT_2H)),
                    5 => Ok(subset_atomic!("BIT 2,L", BIT_2L)),
                    6 => {
                        Ok(Instruction::new(
                            "BIT 2,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::bit_2_buffer);
        
                                o
                            },
                            3
                        ))
                    },
                    7 => Ok(subset_atomic!("BIT 2,A", BIT_2A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            3 => {
                match data.1 {
                    0 => Ok(subset_atomic!("BIT 3,B", BIT_3B)),
                    1 => Ok(subset_atomic!("BIT 3,C", BIT_3C)),
                    2 => Ok(subset_atomic!("BIT 3,D", BIT_3D)),
                    3 => Ok(subset_atomic!("BIT 3,E", BIT_3E)),
                    4 => Ok(subset_atomic!("BIT 3,H", BIT_3H)),
                    5 => Ok(subset_atomic!("BIT 3,L", BIT_3L)),
                    6 => {
                        Ok(Instruction::new(
                            "BIT 3,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::bit_3_buffer);
        
                                o
                            },
                            3
                        ))
                    },
                    7 => Ok(subset_atomic!("BIT 3,A", BIT_3A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            4 => {
                match data.1 {
                    0 => Ok(subset_atomic!("BIT 4,B", BIT_4B)),
                    1 => Ok(subset_atomic!("BIT 4,C", BIT_4C)),
                    2 => Ok(subset_atomic!("BIT 4,D", BIT_4D)),
                    3 => Ok(subset_atomic!("BIT 4,E", BIT_4E)),
                    4 => Ok(subset_atomic!("BIT 4,H", BIT_4H)),
                    5 => Ok(subset_atomic!("BIT 4,L", BIT_4L)),
                    6 => {
                        Ok(Instruction::new(
                            "BIT 4,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::bit_4_buffer);
        
                                o
                            },
                            3
                        ))
                    },
                    7 => Ok(subset_atomic!("BIT 4,A", BIT_4A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            5 => {
                match data.1 {
                    0 => Ok(subset_atomic!("BIT 5,B", BIT_5B)),
                    1 => Ok(subset_atomic!("BIT 5,C", BIT_5C)),
                    2 => Ok(subset_atomic!("BIT 5,D", BIT_5D)),
                    3 => Ok(subset_atomic!("BIT 5,E", BIT_5E)),
                    4 => Ok(subset_atomic!("BIT 5,H", BIT_5H)),
                    5 => Ok(subset_atomic!("BIT 5,L", BIT_5L)),
                    6 => {
                        Ok(Instruction::new(
                            "BIT 5,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::bit_5_buffer);
        
                                o
                            },
                            3
                        ))
                    },
                    7 => Ok(subset_atomic!("BIT 5,A", BIT_5A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            6 => {
                match data.1 {
                    0 => Ok(subset_atomic!("BIT 6,B", BIT_6B)),
                    1 => Ok(subset_atomic!("BIT 6,C", BIT_6C)),
                    2 => Ok(subset_atomic!("BIT 6,D", BIT_6D)),
                    3 => Ok(subset_atomic!("BIT 6,E", BIT_6E)),
                    4 => Ok(subset_atomic!("BIT 6,H", BIT_6H)),
                    5 => Ok(subset_atomic!("BIT 6,L", BIT_6L)),
                    6 => {
                        Ok(Instruction::new(
                            "BIT 6,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::bit_6_buffer);
        
                                o
                            },
                            3
                        ))
                    },
                    7 => Ok(subset_atomic!("BIT 6,A", BIT_6A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            7 => {
                match data.1 {
                    0 => Ok(subset_atomic!("BIT 7,B", BIT_7B)),
                    1 => Ok(subset_atomic!("BIT 7,C", BIT_7C)),
                    2 => Ok(subset_atomic!("BIT 7,D", BIT_7D)),
                    3 => Ok(subset_atomic!("BIT 7,E", BIT_7E)),
                    4 => Ok(subset_atomic!("BIT 7,H", BIT_7H)),
                    5 => Ok(subset_atomic!("BIT 7,L", BIT_7L)),
                    6 => {
                        Ok(Instruction::new(
                            "BIT 7,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::bit_7_buffer);
        
                                o
                            },
                            3
                        ))
                    },
                    7 => Ok(subset_atomic!("BIT 7,A", BIT_7A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            _ => Err("Instruction not found".to_owned()),
        }

    }
    
    fn decode_rss(data: (u8,u8)) -> Result<Instruction, String> {
        match data.0 {
            //bit 0,r
            0 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RLC B", RLC_B)),
                    1 => Ok(subset_atomic!("RLC C", RLC_C)),
                    2 => Ok(subset_atomic!("RLC D", RLC_D)),
                    3 => Ok(subset_atomic!("RLC E", RLC_E)),
                    4 => Ok(subset_atomic!("RLC H", RLC_H)),
                    5 => Ok(subset_atomic!("RLC L", RLC_L)),
                    6 => {
                        Ok(Instruction::new(
                            "RLC (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::rlc_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RLC A", RLC_A_CB)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            1 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RRC B", RRC_B)),
                    1 => Ok(subset_atomic!("RRC C", RRC_C)),
                    2 => Ok(subset_atomic!("RRC D", RRC_D)),
                    3 => Ok(subset_atomic!("RRC E", RRC_E)),
                    4 => Ok(subset_atomic!("RRC H", RRC_H)),
                    5 => Ok(subset_atomic!("RRC L", RRC_L)),
                    6 => {
                        Ok(Instruction::new(
                            "RRC (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::rrc_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RRC A", RRC_A_CB)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            2 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RL B", RL_B)),
                    1 => Ok(subset_atomic!("RL C", RL_C)),
                    2 => Ok(subset_atomic!("RL D", RL_D)),
                    3 => Ok(subset_atomic!("RL E", RL_E)),
                    4 => Ok(subset_atomic!("RL H", RL_H)),
                    5 => Ok(subset_atomic!("RL L", RL_L)),
                    6 => {
                        Ok(Instruction::new(
                            "RL (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::rl_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RL A", RL_A_CB)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            3 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RR B", RR_B)),
                    1 => Ok(subset_atomic!("RR C", RR_C)),
                    2 => Ok(subset_atomic!("RR D", RR_D)),
                    3 => Ok(subset_atomic!("RR E", RR_E)),
                    4 => Ok(subset_atomic!("RR H", RR_H)),
                    5 => Ok(subset_atomic!("RR L", RR_L)),
                    6 => {
                        Ok(Instruction::new(
                            "RR (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::rr_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RR A", RR_A_CB)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            4 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SLA B", SLA_B)),
                    1 => Ok(subset_atomic!("SLA C", SLA_C)),
                    2 => Ok(subset_atomic!("SLA D", SLA_D)),
                    3 => Ok(subset_atomic!("SLA E", SLA_E)),
                    4 => Ok(subset_atomic!("SLA H", SLA_H)),
                    5 => Ok(subset_atomic!("SLA L", SLA_L)),
                    6 => {
                        Ok(Instruction::new(
                            "SLA (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::sla_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SLA A", SLA_A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            5 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SRA B", SRA_B)),
                    1 => Ok(subset_atomic!("SRA C", SRA_C)),
                    2 => Ok(subset_atomic!("SRA D", SRA_D)),
                    3 => Ok(subset_atomic!("SRA E", SRA_E)),
                    4 => Ok(subset_atomic!("SRA H", SRA_H)),
                    5 => Ok(subset_atomic!("SRA L", SRA_L)),
                    6 => {
                        Ok(Instruction::new(
                            "SRA (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::sra_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SRA A", SRA_A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            6 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SWAP B", SWAP_B)),
                    1 => Ok(subset_atomic!("SWAP C", SWAP_C)),
                    2 => Ok(subset_atomic!("SWAP D", SWAP_D)),
                    3 => Ok(subset_atomic!("SWAP E", SWAP_E)),
                    4 => Ok(subset_atomic!("SWAP H", SWAP_H)),
                    5 => Ok(subset_atomic!("SWAP L", SWAP_L)),
                    6 => {
                        Ok(Instruction::new(
                            "SWAP (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::swap_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SWAP A", SWAP_A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            7 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SRL B", SRL_B)),
                    1 => Ok(subset_atomic!("SRL C", SRL_C)),
                    2 => Ok(subset_atomic!("SRL D", SRL_D)),
                    3 => Ok(subset_atomic!("SRL E", SRL_E)),
                    4 => Ok(subset_atomic!("SRL H", SRL_H)),
                    5 => Ok(subset_atomic!("SRL L", SRL_L)),
                    6 => {
                        Ok(Instruction::new(
                            "SRL (HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::srl_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SRL A", SRL_A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            _ => { Err("Instruction not found".to_owned()) },
        }

    }

    fn decode_res(data: (u8,u8)) -> Result<Instruction, String> {
        match data.0 {
            //bit 0,r
            0 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RES 0,B", RES_0B)),
                    1 => Ok(subset_atomic!("RES 0,C", RES_0C)),
                    2 => Ok(subset_atomic!("RES 0,D", RES_0D)),
                    3 => Ok(subset_atomic!("RES 0,E", RES_0E)),
                    4 => Ok(subset_atomic!("RES 0,H", RES_0H)),
                    5 => Ok(subset_atomic!("RES 0,L", RES_0L)),
                    6 => {
                        Ok(Instruction::new(
                            "RES 0,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::res_0_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RES 0,A", RES_0A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            1 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RES 1,B", RES_1B)),
                    1 => Ok(subset_atomic!("RES 1,C", RES_1C)),
                    2 => Ok(subset_atomic!("RES 1,D", RES_1D)),
                    3 => Ok(subset_atomic!("RES 1,E", RES_1E)),
                    4 => Ok(subset_atomic!("RES 1,H", RES_1H)),
                    5 => Ok(subset_atomic!("RES 1,L", RES_1L)),
                    6 => {
                        Ok(Instruction::new(
                            "RES 1,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::res_1_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RES 1,A", RES_1A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            2 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RES 2,B", RES_2B)),
                    1 => Ok(subset_atomic!("RES 2,C", RES_2C)),
                    2 => Ok(subset_atomic!("RES 2,D", RES_2D)),
                    3 => Ok(subset_atomic!("RES 2,E", RES_2E)),
                    4 => Ok(subset_atomic!("RES 2,H", RES_2H)),
                    5 => Ok(subset_atomic!("RES 2,L", RES_2L)),
                    6 => {
                        Ok(Instruction::new(
                            "RES 2,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::res_2_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RES 2,A", RES_2A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            3 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RES 3,B", RES_3B)),
                    1 => Ok(subset_atomic!("RES 3,C", RES_3C)),
                    2 => Ok(subset_atomic!("RES 3,D", RES_3D)),
                    3 => Ok(subset_atomic!("RES 3,E", RES_3E)),
                    4 => Ok(subset_atomic!("RES 3,H", RES_3H)),
                    5 => Ok(subset_atomic!("RES 3,L", RES_3L)),
                    6 => {
                        Ok(Instruction::new(
                            "RES 3,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::res_3_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RES 3,A", RES_3A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            4 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RES 4,B", RES_4B)),
                    1 => Ok(subset_atomic!("RES 4,C", RES_4C)),
                    2 => Ok(subset_atomic!("RES 4,D", RES_4D)),
                    3 => Ok(subset_atomic!("RES 4,E", RES_4E)),
                    4 => Ok(subset_atomic!("RES 4,H", RES_4H)),
                    5 => Ok(subset_atomic!("RES 4,L", RES_4L)),
                    6 => {
                        Ok(Instruction::new(
                            "RES 4,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::res_4_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RES 4,A", RES_4A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            5 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RES 5,B", RES_5B)),
                    1 => Ok(subset_atomic!("RES 5,C", RES_5C)),
                    2 => Ok(subset_atomic!("RES 5,D", RES_5D)),
                    3 => Ok(subset_atomic!("RES 5,E", RES_5E)),
                    4 => Ok(subset_atomic!("RES 5,H", RES_5H)),
                    5 => Ok(subset_atomic!("RES 5,L", RES_5L)),
                    6 => {
                        Ok(Instruction::new(
                            "RES 5,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::res_5_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RES 5,A", RES_5A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            6 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RES 6,B", RES_6B)),
                    1 => Ok(subset_atomic!("RES 6,C", RES_6C)),
                    2 => Ok(subset_atomic!("RES 6,D", RES_6D)),
                    3 => Ok(subset_atomic!("RES 6,E", RES_6E)),
                    4 => Ok(subset_atomic!("RES 6,H", RES_6H)),
                    5 => Ok(subset_atomic!("RES 6,L", RES_6L)),
                    6 => {
                        Ok(Instruction::new(
                            "RES 0,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::res_6_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RES 6,A", RES_6A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            7 => {
                match data.1 {
                    0 => Ok(subset_atomic!("RES 7,B", RES_7B)),
                    1 => Ok(subset_atomic!("RES 7,C", RES_7C)),
                    2 => Ok(subset_atomic!("RES 7,D", RES_7D)),
                    3 => Ok(subset_atomic!("RES 7,E", RES_7E)),
                    4 => Ok(subset_atomic!("RES 7,H", RES_7H)),
                    5 => Ok(subset_atomic!("RES 7,L", RES_7L)),
                    6 => {
                        Ok(Instruction::new(
                            "RES 7,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::res_7_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("RES 7,A", RES_7A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            _ => { Err("Instruction not found".to_owned()) },
        }

    }

    fn decode_set(data: (u8,u8)) -> Result<Instruction, String> {
        match data.0 {
            //bit 0,r
            0 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SET 0,B", SET_0B)),
                    1 => Ok(subset_atomic!("SET 0,C", SET_0C)),
                    2 => Ok(subset_atomic!("SET 0,D", SET_0D)),
                    3 => Ok(subset_atomic!("SET 0,E", SET_0E)),
                    4 => Ok(subset_atomic!("SET 0,H", SET_0H)),
                    5 => Ok(subset_atomic!("SET 0,L", SET_0L)),
                    6 => {
                        Ok(Instruction::new(
                            "SET 0,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::set_0_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SET 0,A", SET_0A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            1 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SET 1,B", SET_1B)),
                    1 => Ok(subset_atomic!("SET 1,C", SET_1C)),
                    2 => Ok(subset_atomic!("SET 1,D", SET_1D)),
                    3 => Ok(subset_atomic!("SET 1,E", SET_1E)),
                    4 => Ok(subset_atomic!("SET 1,H", SET_1H)),
                    5 => Ok(subset_atomic!("SET 1,L", SET_1L)),
                    6 => {
                        Ok(Instruction::new(
                            "SET 1,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::set_1_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SET 1,A", SET_1A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            2 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SET 2,B", SET_2B)),
                    1 => Ok(subset_atomic!("SET 2,C", SET_2C)),
                    2 => Ok(subset_atomic!("SET 2,D", SET_2D)),
                    3 => Ok(subset_atomic!("SET 2,E", SET_2E)),
                    4 => Ok(subset_atomic!("SET 2,H", SET_2H)),
                    5 => Ok(subset_atomic!("SET 2,L", SET_2L)),
                    6 => {
                        Ok(Instruction::new(
                            "SET 2,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::set_2_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SET 2,A", SET_2A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            3 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SET 3,B", SET_3B)),
                    1 => Ok(subset_atomic!("SET 3,C", SET_3C)),
                    2 => Ok(subset_atomic!("SET 3,D", SET_3D)),
                    3 => Ok(subset_atomic!("SET 3,E", SET_3E)),
                    4 => Ok(subset_atomic!("SET 3,H", SET_3H)),
                    5 => Ok(subset_atomic!("SET 3,L", SET_3L)),
                    6 => {
                        Ok(Instruction::new(
                            "SET 3,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::set_3_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SET 3,A", SET_3A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            4 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SET 4,B", SET_4B)),
                    1 => Ok(subset_atomic!("SET 4,C", SET_4C)),
                    2 => Ok(subset_atomic!("SET 4,D", SET_4D)),
                    3 => Ok(subset_atomic!("SET 4,E", SET_4E)),
                    4 => Ok(subset_atomic!("SET 4,H", SET_4H)),
                    5 => Ok(subset_atomic!("SET 4,L", SET_4L)),
                    6 => {
                        Ok(Instruction::new(
                            "SET 4,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::set_4_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SET 4,A", SET_4A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            5 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SET 5,B", SET_5B)),
                    1 => Ok(subset_atomic!("SET 5,C", SET_5C)),
                    2 => Ok(subset_atomic!("SET 5,D", SET_5D)),
                    3 => Ok(subset_atomic!("SET 5,E", SET_5E)),
                    4 => Ok(subset_atomic!("SET 5,H", SET_5H)),
                    5 => Ok(subset_atomic!("SET 5,L", SET_5L)),
                    6 => {
                        Ok(Instruction::new(
                            "SET 5,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::set_5_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SET 5,A", SET_5A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            6 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SET 6,B", SET_6B)),
                    1 => Ok(subset_atomic!("SET 6,C", SET_6C)),
                    2 => Ok(subset_atomic!("SET 6,D", SET_6D)),
                    3 => Ok(subset_atomic!("SET 6,E", SET_6E)),
                    4 => Ok(subset_atomic!("SET 6,H", SET_6H)),
                    5 => Ok(subset_atomic!("SET 6,L", SET_6L)),
                    6 => {
                        Ok(Instruction::new(
                            "SET 6,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::set_6_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SET 6,A", SET_6A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            7 => {
                match data.1 {
                    0 => Ok(subset_atomic!("SET 7,B", SET_7B)),
                    1 => Ok(subset_atomic!("SET 7,C", SET_7C)),
                    2 => Ok(subset_atomic!("SET 7,D", SET_7D)),
                    3 => Ok(subset_atomic!("SET 7,E", SET_7E)),
                    4 => Ok(subset_atomic!("SET 7,H", SET_7H)),
                    5 => Ok(subset_atomic!("SET 7,L", SET_7L)),
                    6 => {
                        Ok(Instruction::new(
                            "SET 7,(HL)",
                            {
                                let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                                o.push_back(Instruction::read_bus_with_HL);
                                o.push_back(Instruction::set_7_buffer);
                                o.push_back(Instruction::nop);
                                o.push_back(Instruction::write_b8_to_b16);
        
                                o
                            },
                            4
                        ))
                    },
                    7 => Ok(subset_atomic!("SET 7,A", SET_7A)),
                    _ => { Err("Instruction not found".to_owned()) },
                }
            },
            _ => { Err("Instruction not found".to_owned()) },
        }

    }
}