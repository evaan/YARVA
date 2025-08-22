mod register;
mod instructions;
mod immediate;

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::read_to_string;

use crate::immediate::{parse_12_bit_immediate, parse_13_bit_immediate, parse_21_bit_immediate, parse_32_bit_immediate};
use crate::instructions::b_type::parse_b_type;
use crate::instructions::i_type::parse_i_type;
use crate::instructions::j_type::parse_j_type;
use crate::instructions::r_type::parse_r_type;
use crate::instructions::s_type::parse_s_type;
use crate::instructions::u_type::parse_u_type;
use crate::register::{parse_dest_register, parse_register};

fn print_error(error: &str) {
  println!("\x1b[91merror:\x1b[0m {}", error);
  std::process::exit(1);
}

fn print_warning(error: &str) {
  println!("\x1b[93mwarning:\x1b[0m {}", error);
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
  let mut instructions: Vec<u32> = Vec::new();
  
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
        //TODO: data parsing, constants for values
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
    let instruction_line: &str = &lowered_line.replace(",", " ").replace("(", " ").replace(")", "").replace("_", "");
    let instruction_args: Vec<&str> = instruction_line.trim().split_whitespace().collect();
    //TODO: make sure in .text before parsing instructions
    match instruction_args[0] {
      "add" | "sub" | "xor" | "or" | "and" | "sll" | "srl" | "sra" | "slt" | "sltu" | "mul" | "mulh" | "mulhsu" | "mulhu" | "div" | "divu" | "rem" | "remu" => { //RV32IM R-types
        if instruction_args.len() != 4 {
          print_error(&format!("Invalid instruction '{}' on line {}", line, i+1));
        }
        let rd: u8 = parse_dest_register(instruction_args[1], i+1);
        let rs1: u8 = parse_register(instruction_args[2], i+1);
        let rs2: u8 = parse_register(instruction_args[3], i+1);
        instructions.push(parse_r_type(instruction_args[0], rd, rs1, rs2));
      }
      "addi" | "xori" | "ori" | "andi" | "slli" | "srli" | "srai" | "slti" | "sltiu" | "lb" | "lh" | "lw" | "lbu" | "lhu" | "jalr" => { //RV32I I-types
        if instruction_args.len() != 4 {
          print_error(&format!("Invalid instruction '{}' on line {}", line, i+1));
        }
        let rd: u8 = parse_dest_register(instruction_args[1], i+1);
        let rs1: u8 = parse_register(instruction_args[if instruction_args[0].starts_with("l") {3} else {2}], i+1); //Load instructions are backwards
        let imm: u16 = parse_12_bit_immediate(instruction_args[if instruction_args[0].starts_with("l") {2} else {3}], i+1);
        instructions.push(parse_i_type(instruction_args[0], rd, rs1, imm, i+1));
      }
      "ebreak" | "ecall" => {
        if instruction_args.len() != 1 {
          print_error(&format!("Invalid instruction '{}' on line {}", line, i+1));
        }
        instructions.push(parse_i_type(instruction_args[0], 0x0, 0x0, instruction_args[0].eq("ebreak") as u16, i+1));
      }
      "sb" | "sh" | "sw" => { //RV32I S-types
        if instruction_args.len() != 4 {
          print_error(&format!("Invalid instruction '{}' on line {}", line, i+1));
        }
        let rs1: u8 = parse_register(instruction_args[3], i+1);
        let rs2: u8 = parse_register(instruction_args[1], i+1);
        let imm: u16 = parse_12_bit_immediate(instruction_args[2], i+1);
        instructions.push(parse_s_type(instruction_args[0], rs1, rs2, imm));
      }
      "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu" => { //RV32I B-types
        if instruction_args.len() != 4 {
          print_error(&format!("Invalid instruction '{}' on line {}", line, i+1));
        }
        let rs1: u8 = parse_register(instruction_args[1], i+1);
        let rs2: u8 = parse_register(instruction_args[2], i+1);
        let imm: u16 = parse_13_bit_immediate(instruction_args[3], i+1);
        instructions.push(parse_b_type(instruction_args[0], rs1, rs2, imm));
      }
      "lui" | "auipc" => { //RV32I U-types
        let rd: u8 = parse_dest_register(instruction_args[1], i+1);
        let imm: u32 = parse_32_bit_immediate(instruction_args[2], i+1);
        instructions.push(parse_u_type(instruction_args[0], rd, imm));
      }
      "jal" => { //RV32I J-type
        let rd: u8 = parse_dest_register(instruction_args[1], i+1);
        let imm: u32 = parse_21_bit_immediate(instruction_args[2], i+1);
        instructions.push(parse_j_type(rd, imm));
      }
      _ => {
        print_error(&format!("Invalid instruction '{}' on line {}", instruction_args[0], i+1));
      }
    }
    current_memory_address += 4;
  }
  for instruction in instructions.iter() {
    println!("{:08x}", instruction);
  }
}
