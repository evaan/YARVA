use crate::print_error;

pub fn parse_b_type(instruction: &str, rs1: u8, rs2: u8, offset: i16, opcode: u8, line: usize) -> u32 {
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

    let instruction = (((offset as u32 >> 12) & 0x1) << 31)
        | (((offset as u32 >> 5) & 0x3F) << 25)
        | ((rs2 as u32) << 20)
        | ((rs1 as u32) << 15)
        | ((func3 as u32) << 12)
        | (((offset as u32 >> 1) & 0xF) << 8)
        | (((offset as u32 >> 11) & 0x1) << 7)
        | (opcode as u32);

    return instruction;
}