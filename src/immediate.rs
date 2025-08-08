use crate::print_error;

pub fn parse_12_bit_immediate(imm: &str, line: usize, unsigned: bool) -> i16 {
    let parsed = if imm.to_lowercase().starts_with("0x") {
        i16::from_str_radix(&imm[2..], 16)
    } else if imm.to_lowercase().starts_with("0b") {
        i16::from_str_radix(&imm[2..], 2)
    } else if imm.to_lowercase().starts_with("0o") {
        i16::from_str_radix(&imm[2..], 8)
    } else {
        imm.parse::<i16>()
    };

    match parsed {
        Ok(mut n) => {
            if n & 0x800 != 0 && !unsigned {
                n = (n | 0xF000u16 as i16) as i16
            }
            if (!unsigned && n >= -2048 && n <= 2047) || (unsigned && n >= 0 && n <= 4095) {
                n
            } else {
                if unsigned {
                    print_error(&format!("Immediate {} out of range (0 to 4095) on line {}", imm, line));
                } else {
                    print_error(&format!("Immediate {} out of range (-2048 to 2047) on line {}", imm, line));
                }
                0
            }
        }
        Err(s) => {
            println!("{}", s);
            print_error(&format!("Invalid immediate '{}' on line {}", imm, line));
            0
        }
    }
}

pub fn parse_13_bit_immediate(imm: &str, line: usize) -> i16 {
    let parsed = if imm.to_lowercase().starts_with("0x") {
        u16::from_str_radix(&imm[2..], 16).map(|val| {
            if val & 0x1000 != 0 {
                // sign-extend from 13 bits
                (val as i32 - 0x2000) as i16
            } else {
                val as i16
            }
        })
    } else {
        imm.parse::<i16>()
    };
    // TODO: binary and octo

    match parsed {
        Ok(n) => {
            if n >= -4096 && n <= 4095 {
                n
            } else {
                print_error(&format!("Immediate {} out of range (-4096 to 4095) on line {}", imm, line));
                0
            }
        }
        Err(_) => {
            print_error(&format!("Invalid immediate '{}' on line {}", imm, line));
            0
        }
    }
}

pub fn parse_20_bit_immediate(imm: &str, line: usize) -> i32 {
    let parsed = if imm.to_lowercase().starts_with("0x") {
        i32::from_str_radix(&imm[2..], 16)
    } else if imm.to_lowercase().starts_with("0b") {
        i32::from_str_radix(&imm[2..], 2)
    } else if imm.to_lowercase().starts_with("0o") {
        i32::from_str_radix(&imm[2..], 8)
    } else {
        imm.parse::<i32>()
    };

    match parsed {
        Ok(mut n) => {
            if n & 0x80000 != 0 {
                n = (n | 0xFFF00000u32 as i32) as i32
            }
            if n >= -524288 && n <= 524287 {
                n
            } else {
                print_error(&format!("Immediate {} out of range (-524288 to 524287) on line {}", imm, line));
                0
            }
        }
        Err(_) => {
            print_error(&format!("Invalid immediate '{}' on line {}", imm, line));
            0
        }
    }
}