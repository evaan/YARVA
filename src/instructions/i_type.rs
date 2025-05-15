use crate::print_error;

pub fn parse_i_type(instruction: &str, rd: u8, rs1: u8, mut imm: i16, opcode: u8, line: usize) {
    let func3: u8 = match opcode {
        19 => { //0010011
            match instruction {
                "addi" => 0,
                "xori" => 4,
                "ori" => 6,
                "andi" => 7,
                "slti" => 2,
                "sltiu" => 3,
                "slli" => {
                    if imm > 31 {
                        print_error(&format!("Invalid immediate value at line {}, should be 0-31", line));
                    }
                    1
                }
                "srli" => {
                    if imm > 31 {
                        print_error(&format!("Invalid immediate value at line {}, should be 0-31", line));
                    }
                    5
                }
                "srai" => {
                    if imm > 31 {
                        print_error(&format!("Invalid immediate value at line {}, should be 0-31", line));
                    }
                    imm += 1024;
                    5
                }
                _ => {
                    unreachable!();
                }
            }
        }
        3 => { //0000011
            match instruction {
                "lb" => 0,
                "lh" => 1,
                "lw" => 2,
                "lbu" => 4,
                "lhu" => 5,
                _ => {
                    unreachable!();
                }
            }
        }
        115 => { //111011
            0
        }
        103 => { //1100111
            0
        }
        _ => {
            unreachable!();
        }
    };

    println!("{:08x}", u32::from_str_radix(&format!("{:012b}{:05b}{:03b}{:05b}{:07b}", (imm as u32) & 0x0FFF, rs1, func3, rd, opcode), 2).expect("Invalid binary string"));
}