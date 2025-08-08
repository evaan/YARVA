use crate::print_error;

pub fn parse_j_type(imm: i32, rd: u8, line: usize) -> u32 { //only jal
  if imm & 1 == 1 {
    print_error(&format!("Invalid immediate on line {}", line));
  }

  let instruction = (((imm as u32 >> 20) & 0x1) << 31)
    | (((imm as u32 >> 1) & 0x3FF) << 21)
    | (((imm as u32 >> 11) & 0x1) << 20)
    | (((imm as u32 >> 12) & 0xFF) << 12)
    | ((rd as u32) << 7)
    | 0b1101111;

  return instruction
}