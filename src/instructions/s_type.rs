pub fn parse_s_type(instruction: &str, rs1: u8, rs2: u8, imm: u16) -> u32 {
  let (opcode, func3): (u8, u8) = match instruction {
    "sb" => (0b0100011, 0b00),
    "sh" => (0b0100011, 0b01),
    "sw" => (0b0100011, 0b10),
    _ => unreachable!()
  };

  ((imm as u32 >> 5) << 25)
    | (rs2 as u32) << 20
    | (rs1 as u32) << 15
    | (func3 as u32) << 12
    | ((imm as u32) & 0b11111) << 7
    | opcode as u32
}
