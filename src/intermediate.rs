use crate::print_error;

pub fn parse_12_bit_immediate(imm: &str, line: usize) -> i16 {
    let parsed = if imm.to_lowercase().starts_with("0x") {
        u16::from_str_radix(&imm[2..], 16).map(|val| {
            if val & 0x800 != 0 {
                (val as i32 - 0x1000) as i16
            } else {
                val as i16
            }
        })
    } else {
        imm.parse::<i16>()
    };

    match parsed {
        Ok(n) => {
            if n >= -2048 && n <= 2047 {
                n
            } else {
                print_error(&format!("Immediate {} out of range (-2048 to 2047) on line {}", imm, line));
                0
            }
        }
        Err(_) => {
            print_error(&format!("Invalid immediate '{}' on line {}", imm, line));
            0
        }
    }
}