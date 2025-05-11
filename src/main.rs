use std::env;
use std::path::Path;
use std::fs::read_to_string;

fn print_error(error: &str) {
    println!("\x1b[31merror:\x1b[0m {}", error);
    std::process::exit(1);
}

fn parse_register(register: &str, line: usize) -> u8 {
    match register {
        "x0" | "zero" => 0,
        "x1" | "ra" => 1,
        "x2" | "sp" => 2,
        "x3" | "gp" => 3,
        "x4" | "tp" => 4,
        "x5" | "t0" => 5,
        "x6" | "t1" => 6,
        "x7" | "t2" => 7,
        "x8" | "s0" | "fp" => 8,
        "x9" | "s1" => 9,
        "x10" | "a0" => 10,
        "x11" | "a1" => 11,
        "x12" | "a2" => 12,
        "x13" | "a3" => 13,
        "x14" | "a4" => 14,
        "x15" | "a5" => 15,
        "x16" | "a6" => 16,
        "x17" | "a7" => 17,
        "x18" | "s2" => 18,
        "x19" | "s3" => 19,
        "x20" | "s4" => 20,
        "x21" | "s5" => 21,
        "x22" | "s6" => 22,
        "x23" | "s7" => 23,
        "x24" | "s8" => 24,
        "x25" | "s9" => 25,
        "x26" | "s10" => 26,
        "x27" | "s11" => 27,
        "x28" | "t3" => 28,
        "x29" | "t4" => 29,
        "x30" | "t5" => 30,
        "x31" | "t6" => 31,
        _ => {
            print_error(&format!("Invalid register name: {} on line {}", register, line));
            32
        }
    }
}

fn parse_12_bit_immediate(imm: &str, line: usize) -> i16 {
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

fn parse_i_type(instruction: &str, mut imm: i16, rs1: u8, rd: u8, opcode: u8, line: usize) {
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
                        print_error(&format!("Invalid immediate value at line {}", line));
                    }
                    1
                }
                "srli" => {
                    if imm > 31 {
                        print_error(&format!("Invalid immediate value at line {}", line));
                    }
                    5
                }
                "srai" => {
                    if imm > 31 {
                        print_error(&format!("Invalid immediate value at line {}", line));
                    }
                    imm += 1024;
                    5
                }
                _ => {
                    print_error(&format!("Invalid instruction '{}' at line {}", instruction, line));
                    0
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
                    print_error(&format!("Invalid instruction '{}' at line {}", instruction, line));
                    0
                }
            }
        }
        115 => { //111011
            0
        }
        _ => {
            print_error(&format!("Invalid opcode '{}' at line {}", opcode, line));
            0
        }
    };

    println!("{:08x}", u32::from_str_radix(&format!("{:012b}{:05b}{:03b}{:05b}{:07b}", (imm as u32) & 0x0FFF, rs1, func3, rd, opcode), 2).expect("Invalid binary string"));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // make sure a file is being input as an argument
    if args.len() < 2 {
        print_error(&format!("Usage: {} <file name>", &args[0]));
    }

    // make sure the file exists
    if !Path::new(&args[1]).is_file() {
        print_error("File does not exist.");
    }

    //parse instructions
    for (i, line) in read_to_string(&args[1]).unwrap().lines().enumerate() {
        let instruction: &str = &line.trim().split("#").next().unwrap().trim();
        let instruction_parts: Vec<&str> = instruction.split_whitespace().map(|s| s.trim_end_matches(",")).collect();
        if instruction_parts.len() == 0 {
            continue;
        }
        match instruction_parts[0] {
            "addi" | "slti" | "sltiu" | "xori" | "ori" | "andi" | "slli" | "srli" | "srai" => {
                if instruction_parts.len() != 4 {
                    print_error(&format!("Invalid syntax on line {}", i+1));
                }
                let imm: i16 = parse_12_bit_immediate(instruction_parts[3], i+1);
                let rs1: u8 = parse_register(instruction_parts[2], i+1);
                let rd: u8 = parse_register(instruction_parts[1], i+1);
                parse_i_type(instruction_parts[0], imm, rs1, rd, 19, i+1);
            }
            "lb" | "lh" | "lw" | "lbu" | "lhu" => {
                if instruction_parts.len() != 3 {
                    print_error(&format!("Invalid syntax on line {}", i+1));
                }
                let rd: u8 = parse_register(instruction_parts[1], i+1);
                let imm_rs1_split: Vec<&str> = instruction_parts[2].split("(").collect();
                if imm_rs1_split.len() != 2 {
                    print_error(&format!("Invalid syntax on line {}", i+1));
                }
                let imm: i16 = parse_12_bit_immediate(&imm_rs1_split[0], i+1);
                let rs1: u8 = parse_register(imm_rs1_split[1].strip_suffix(")").unwrap(), i + 1);
                parse_i_type(instruction_parts[0], imm, rs1, rd, 3, i+1);
            }
            "ecall" => {
                println!("00000073")
            }
            "ebreak" => {
                println!("00100073")
            }
            unknown => {
                print_error(&format!("Unknown instruction {} on line {}.", unknown, i+1));
            }
        }
    }
}
