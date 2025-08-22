pub fn parse_b_type(instruction: &str, rs1: u8, rs2: u8, imm: u16) -> u32 {
  let (opcode, func3): (u8, u8) = match instruction {
    "beq"  => (0b1100011, 0b000),
    "bne"  => (0b1100011, 0b001),
    "blt"  => (0b1100011, 0b100),
    "bge"  => (0b1100011, 0b101),
    "bltu" => (0b1100011, 0b110),
    "bgeu" => (0b1100011, 0b111),
    _ => unreachable!()
  };

  (((imm as u32) >> 12) & 1) << 31
    | (((imm as u32) >> 5) & 0x3F) << 25
    | (rs2 as u32) << 20
    | (rs1 as u32) << 15
    | (func3 as u32) << 12
    | (((imm as u32) >> 1) & 0xF) << 8
    | (((imm as u32) >> 11) & 0x1) << 7
    | (opcode as u32)
}
