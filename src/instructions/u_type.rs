pub fn parse_u_type(instruction: &str, rd: u8, imm: u32) -> u32 {
  let opcode: u8 = match instruction {
    "lui"   => 0b0110111,
    "auipc" => 0b0010111,
    _ => unreachable!()
  };

  imm << 12
    | (rd as u32) << 7
    | (opcode as u32)
}
