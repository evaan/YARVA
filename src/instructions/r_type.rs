use crate::print_error;

pub fn parse_r_type(instruction: &str, rs2: u8, rs1: u8, rd: u8, opcode: u8, line: usize) {
  let (func3, func7): (u8, u8) = match opcode {
    51 => {
      match instruction {
        "add" => (0, 0),
        "sub" => (0, 32),
        "xor" => (4, 0),
        "or" => (6, 0),
        "and" => (7, 0),
        "sll" => (1, 0),
        "srl" => (5, 0),
        "sra" => (5, 32),
        "slt" => (2, 0),
        "sltu" => (3, 0),
        "mul" => (0, 1),
        "mulh" => (1, 1),
        "mulsu" => (2, 1),
        "mulu" => (3, 1),
        "div" => (4, 1),
        "divu" => (5, 1),
        "rem" => (6, 1),
        "remu" => (7, 1),
        _ => {
            print_error(&format!("Invalid instruction '{}' at line {}", instruction, line));
            (0, 0)
        }
      }
    }
    _ => {
      print_error(&format!("Invalid opcode '{}' at line {}", opcode, line));
      (0, 0)
    }
  };

  println!("{:08x}", u32::from_str_radix(&format!("{:07b}{:05b}{:05b}{:03b}{:05b}{:07b}", func7, rs2, rs1, func3, rd, opcode), 2).expect("Invalid binary string"));
}