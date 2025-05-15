mod register;
mod intermediate;
mod instructions;

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::read_to_string;
use crate::instructions::i_type::parse_i_type;
use instructions::{b_type::parse_b_type, r_type::parse_r_type, u_type::parse_u_type};
use register::parse_register;
use intermediate::{parse_12_bit_immediate, parse_13_bit_immediate, parse_20_bit_immediate};

fn print_error(error: &str) {
    println!("\x1b[91merror:\x1b[0m {}", error);
    std::process::exit(1);
}

fn print_warning(error: &str) {
    println!("\x1b[93mwarning:\x1b[0m {}", error);
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

    let mut current_memory_address: u32 = 0;
    let mut label_map: HashMap<String, u32> = HashMap::new();

    //parse instructions
    for (i, line) in read_to_string(&args[1]).unwrap().lines().enumerate() {
        //ignore any comments
        let mut trimmed_line: &str = line.trim().split(['#', ';']).next().unwrap().trim();
        if trimmed_line.is_empty() {
            continue;
        }

        if trimmed_line.starts_with(".") {
            //handle directives
            match trimmed_line {
                ".text" => {}
                //TODO: add .global
                _ => {
                    print_warning(&format!("Unsupported directive '{}' on line {}", trimmed_line, i+1));
                }
            }
            continue
        } else if let Some(label_index) = trimmed_line.find(":") {
            //handle labels
            if label_index == 0 {
                print_error(&format!("Invalid label on line {}", i+1));
            }
            let (label_raw, instruction) = trimmed_line.split_at(label_index);
            let label = label_raw.trim();
            if label_map.contains_key(label) {
                print_error(&format!("Label '{}' redefined on line {}", label, i+1));
            }
            if label.chars().next().expect("This shouldn't happen").is_numeric() || !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                print_error(&format!("Invalid label '{}' on line {}", label, i+1));
            }
            //TODO: possible warning if there is a label without an instruction?
            label_map.insert(label.to_string(), current_memory_address);
            if instruction.trim() == ":" {
                continue;
            }
            trimmed_line = &instruction[1..].trim();
        }
        //parse instruction
        let lowered_line: String = trimmed_line.to_lowercase();
        let instruction_line: &str = &lowered_line.replace(",", " ").replace("(", " ").replace(")", "");
        let instruction_args: Vec<&str> = instruction_line.trim().split_whitespace().collect();
        match instruction_args[0] {
            "add" | "sub" | "xor" | "or" | "and" | "sll" | "srl" | "sra" | "slt" | "sltu" => { //base R-types
                if instruction_args.len() != 4 {
                    print_error(&format!("Invalid instruction '{}' on line {}", line, i+1));
                }
                let rd: u8 = parse_register(instruction_args[1], i+1);
                let rs1: u8 = parse_register(instruction_args[2], i+1);
                let rs2: u8 = parse_register(instruction_args[3], i+1);
                parse_r_type(instruction_args[0], rd, rs1, rs2, 51, i+1);
            },
            "addi" | "xori" | "ori" | "andi" | "slli" | "srli" | "srai" | "slti" | "sltiu" => { //base I-types w/o offset
                if instruction_args.len() != 4 {
                    print_error(&format!("Invalid instruction '{}' on line {}", line, i+1));
                }
                let rd: u8 = parse_register(instruction_args[1], i+1);
                //TODO: warn about registers that shouldn't be written to
                let rs1: u8 = parse_register(instruction_args[2], i+1);
                let imm: i16 = parse_12_bit_immediate(instruction_args[3], i+1);
                parse_i_type(instruction_args[0], rd, rs1, imm, 19, i+1);
            },
            "lb" | "lh" | "lw" | "lbu" | "lhu" | "jalr" => { //base I-types w/ offset
                let opcode: u8 = match instruction_args[0] {
                    "lb" | "lh" | "lw" | "lbu" | "lhu" => 3,
                    "jalr" => 103,
                    _ => unreachable!()
                };
                if instruction_args.len() != 4 {
                    print_error(&format!("Invalid instruction '{}' on line {}", line, i+1));
                }
                let rd: u8 = parse_register(instruction_args[1], i+1);
                let offset: i16 = parse_12_bit_immediate(instruction_args[2], i+1);
                let rs1: u8 = parse_register(instruction_args[3], i+1);
                parse_i_type(instruction_args[0], rd, rs1, offset, opcode, i+1);
            },
            "ecall" => {
                println!("00000073")
            },
            "ebreak" => {
                println!("00100073")
            },
            "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu" => {
                if instruction_args.len() != 4 {
                    print_error(&format!("Invalid instruction '{}' on line {}", line, i+1));
                }
                let rs1: u8 = parse_register(instruction_args[1], i+1);
                let rs2: u8 = parse_register(instruction_args[2], i+1);
                let offset: i16 = match label_map.get(instruction_args[3]) {
                    Some(&n) => {
                        let offset_bytes = n as i32 - current_memory_address as i32;
                        offset_bytes as i16
                    },
                    _ => parse_13_bit_immediate(instruction_args[3], i+1)
                };
                parse_b_type(instruction_args[0], rs1, rs2, offset, 99, i+1);
            }
            "lui" | "auipc" => {
                if instruction_args.len() != 3 {
                    print_error(&format!("Invalid instruction '{}' on line {}", line, i+1)); 
                }
                let opcode: u8 = match instruction_args[0] {
                    "lui" => 55,
                    "auipc" => 23,
                    _ => unreachable!()
                };
                let imm: i32 = parse_20_bit_immediate(instruction_args[2], i+1);
                let rd: u8 = parse_register(instruction_args[1], i+1);
                parse_u_type(imm, rd, opcode);
            }
            _ => {
                print_error(&format!("Invalid instruction '{}' on line {}", instruction_args[0], i+1));
            }
        }
        current_memory_address += 4;
    }
}
