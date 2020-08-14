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
                        0 => Self::table0((opcode & 0x38) >> 3),
                        1 => Self::table1((opcode & 0x38) >> 3),
                        2 => Self::table2( (opcode & 0x38) >> 3),
                        3 => { Ok(Instruction::holder()) },
                        4 => { Ok(Instruction::holder()) },
                        5 => { Ok(Instruction::holder()) },
                        6 => { Ok(Instruction::holder()) },
                        7 => { Ok(Instruction::holder()) },
                        _ => Err("Opcode not found".to_owned())
                    }
                },
                1 => {
                    //LD and HALT
                    Self::match_load(((opcode & 0x38) >> 3, opcode & 7))
                },
                2 => {
                    //ALU
                    Self::match_alu(((opcode & 0x38) >> 3, opcode & 7))
                },
                3 => {
                    match opcode & 7 {
                        0 => { Ok(Instruction::holder()) },
                        1 => { Ok(Instruction::holder()) },
                        2 => { Ok(Instruction::holder()) },
                        3 => { Ok(Instruction::holder()) },
                        4 => { Ok(Instruction::holder()) },
                        5 => { Ok(Instruction::holder()) },
                        6 => { Ok(Instruction::holder()) },
                        7 => { Ok(Instruction::holder()) },
                        _ => Err("Opcode not found".to_owned())
                    }
                },
                _ => Err("Opcode not found".to_owned())
            }

        } else {
            Err("0xCB not implemented".to_owned())
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
                                o.push_back(Instruction::write_B_with_buffer_u8);
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
                                o.push_back(Instruction::write_C_with_buffer_u8);
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
                                o.push_back(Instruction::write_D_with_buffer_u8);
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
                                o.push_back(Instruction::write_E_with_buffer_u8);
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
                                o.push_back(Instruction::write_H_with_buffer_u8);
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
                                o.push_back(Instruction::write_L_with_buffer_u8);
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
                                o.push_back(Instruction::write_A_with_buffer_u8);
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

    fn table0(data: u8) -> Result<Instruction, String> {
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

    fn table1(data: u8) -> Result<Instruction, String> {

        match data {
            0 => {
                //NOP
                Ok(Instruction::new(
                    "LD BC,nn",
                    {
                        let mut o: VecDeque<fn(&mut Instruction, &mut Registers, &mut Bus)> = VecDeque::new();
                        o.push_back(Instruction::load_immediate);
                        o.push_back(Instruction::load_short);
                        o.push_back(Instruction::write_buffer_to_BC);
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
                        o.push_back(Instruction::write_buffer_to_DE);
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
                        o.push_back(Instruction::write_buffer_to_HL);
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
                        o.push_back(Instruction::write_buffer_to_SP);
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

    fn table2(data: u8) -> Result<Instruction, String> {

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
                        o.push_back(Instruction::write_A_with_buffer_u8);
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
                        o.push_back(Instruction::write_A_with_buffer_u8);
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
}