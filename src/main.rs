mod register;
mod intermediate;
mod instructions;

use std::env;
use std::path::Path;
use std::fs::read_to_string;
use crate::instructions::i_type::parse_i_type;
use instructions::r_type::parse_r_type;
use register::parse_register;
use intermediate::parse_12_bit_immediate;

fn print_error(error: &str) {
    println!("\x1b[31merror:\x1b[0m {}", error);
    std::process::exit(1);
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
        let instruction: &str = &line.trim().split("#").next().unwrap().trim().to_lowercase();
        let instruction_parts: Vec<&str> = instruction.split_whitespace().map(|s| s.trim_end_matches(",")).collect();
        if instruction_parts.is_empty() {
            continue;
        }

        let mut parts = instruction_parts[0].split('.');
        let first = parts.next().unwrap_or("");
        let second = parts.next();

        let base: &str = match second {
            Some(s) => &instruction_parts[0][..first.len() + 1 + s.len()],
            None => first,
        };

        match base {
            "addi" | "slti" | "sltiu" | "xori" | "ori" | "andi" | "slli" | "srli" | "srai" => {
                if instruction_parts.len() != 4 {
                    print_error(&format!("Invalid syntax on line {}", i+1));
                }
                let imm: i16 = parse_12_bit_immediate(instruction_parts[3], i+1);
                let rs1: u8 = parse_register(instruction_parts[2], i+1);
                let rd: u8 = parse_register(instruction_parts[1], i+1);
                parse_i_type(&instruction_parts[0], imm, rs1, rd, 19, i+1);
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
                let rs1_str = imm_rs1_split[1]
                    .strip_suffix(")")
                    .unwrap_or_else(|| {
                        print_error(&format!("Missing ')' on line {}", i + 1));
                        ""
                    });
                let rs1: u8 = parse_register(rs1_str, i + 1);
                parse_i_type(&instruction_parts[0], imm, rs1, rd, 3, i+1);
            }
            "ecall" => {
                println!("00000073")
            }
            "ebreak" => {
                println!("00100073")
            }
            "add" | "sub" | "xor" | "or" | "and" | "sll" | "srl" | "sra" | "slt" | "sltu" | "mul" | "mulh" | "mulsu" | "mulu" | "div" | "divu" | "rem" | "remu" => {
                if instruction_parts.len() != 4 {
                    print_error(&format!("Invalid syntax on line {}", i+1));
                }
                let rs2: u8 = parse_register(instruction_parts[3], i+1);
                let rs1: u8 = parse_register(instruction_parts[2], i+1);
                let rd: u8 = parse_register(instruction_parts[1], i+1);
                parse_r_type(&instruction_parts[0], rs2, rs1, rd, 51, i+1);
            }
            "lr.w" => {
                if instruction_parts.len() == 3 {
                    let rd: u8 = parse_register(instruction_parts[1], i+1);
                    let rs1_str = instruction_parts[2].strip_prefix("(").and_then(|s| s.strip_suffix(")")).unwrap_or(instruction_parts[2]);
                    let rs1: u8 = parse_register(rs1_str, i+1);
                    parse_r_type(&instruction_parts[0], 0, rs1, rd, 47, i+1); 
                }
                else if instruction_parts.len() == 2 {
                    let rd_rs1_split: Vec<&str> = instruction_parts[2].split("(").collect();
                    if rd_rs1_split.len() != 2 {
                        print_error(&format!("Invalid syntax on line {}", i+1));
                    }
                    let rd: u8 = parse_register(&rd_rs1_split[0], i+1);
                    let rs1_str = rd_rs1_split[1]
                        .strip_suffix(")")
                        .unwrap_or_else(|| {
                            print_error(&format!("Missing ')' on line {}", i + 1));
                            ""
                        });
                    let rs1: u8 = parse_register(rs1_str, i + 1);
                    parse_r_type(&instruction_parts[0], 0, rs1, rd, 47, i+1);
                }
                else {
                    print_error(&format!("Invalid syntax on line {}", i+1));
                }
            }
            "sc.w" | "amoswap.w" | "amoadd.w" | "amoand.w" | "amoor.w" | "amoxor.w" | "amomax.w" | "amomin.w" => {
                if instruction_parts.len() != 4 {
                    print_error(&format!("Invalid syntax on line {}", i+1));
                }
                let rs2: u8 = parse_register(instruction_parts[2], i+1);
                let rs1_str = instruction_parts[3].strip_prefix("(").and_then(|s| s.strip_suffix(")")).unwrap_or(instruction_parts[2]);
                let rs1: u8 = parse_register(rs1_str, i+1);
                let rd: u8 = parse_register(instruction_parts[1], i+1);
                parse_r_type(&instruction_parts[0], rs2, rs1, rd, 47, i+1); 
            }
            unknown => {
                print_error(&format!("Unknown instruction {} on line {}.", unknown, i+1));
            }
        }
    }
}
