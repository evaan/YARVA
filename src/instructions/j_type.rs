pub fn parse_j_type(rd: u8, imm: u32) -> u32 {
  ((imm & 0x100000) << 11)
    | ((imm & 0x7FE) << 20)
    | ((imm & 0x800) <<  9)
    | ((imm & 0xFF000))
    | (((rd as u32) & 0x1F) << 7)
    | 0b1101111u32
}