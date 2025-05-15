use crate::print_error;

pub fn parse_b_type(instruction: &str, rs1: u8, rs2: u8, offset: i16, opcode: u8, line: usize) {
    let func3: u8 = match opcode {
        99 => { //1100011
            match instruction {
                "beq" => 0,
                "bne" => 1,
                "blt" => 4,
                "bge" => 5,
                "bltu" => 6,
                "bgeu" => 7,
                _ => {
                    unreachable!();
                }
            }
        }
        _ => {
            unreachable!();
        }
    };

    if offset & 1 == 1 {
        print_error(&format!("Offset {} is not 2-byte aligned on line {}", offset, line));
    }

    let imm_12 = offset >> 12 & 1;
    let imm_11 = offset >> 11 & 1;
    let imm_10_5 = offset >> 5 & 63;
    let imm_4_1 = (offset & 31) >> 1;

    println!("{:08x}", u32::from_str_radix(&format!("{:01b}{:06b}{:05b}{:05b}{:03b}{:04b}{:01b}{:07b}", imm_12, imm_10_5, rs2, rs1, func3, imm_4_1, imm_11, opcode), 2).expect("Invalid binary string"));
}