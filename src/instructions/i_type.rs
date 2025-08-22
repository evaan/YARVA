use crate::print_error;

pub fn parse_i_type(instruction: &str, rd: u8, rs1: u8, mut imm: u16, line: usize) -> u32 {
  if (instruction == "slli" || instruction == "srli" || instruction == "srai") && imm > 0b11111 {
    print_error(&format!("Invalid immediate '{}' on line {}", imm, line));
  } else if instruction == "srai" {
    imm = imm | 0x400;
  }

  let (opcode, func3): (u8, u8) = match instruction {
    "addi"   => (0b0010011, 0b000),
    "xori"   => (0b0010011, 0b100),
    "ori"    => (0b0010011, 0b110),
    "andi"   => (0b0010011, 0b111),
    "slli"   => (0b0010011, 0b001),
    "srli"   => (0b0010011, 0b101),
    "srai"   => (0b0010011, 0b101),
    "slti"   => (0b0010011, 0b010),
    "sltiu"  => (0b0010011, 0b011),
    "lb"     => (0b0000011, 0b000),
    "lh"     => (0b0000011, 0b001),
    "lw"     => (0b0000011, 0b010),
    "lbu"    => (0b0000011, 0b100),
    "lhu"    => (0b0000011, 0b101),
    "jalr"   => (0b1100111, 0b111),
    "ecall"  => (0b1110011, 0b000),
    "ebreak" => (0b1110011, 0b000),
    _ => unreachable!()
  };

  ((imm as u32) << 20)
    | ((rs1 as u32) << 15)
    | ((func3 as u32) << 12)
    | ((rd as u32) << 7)
    | (opcode as u32)
}
