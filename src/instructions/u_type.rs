pub fn parse_u_type(imm: i32, rd: u8, opcode: u8) -> u32 {
    let instruction = ((imm as u32 & 0xFFFFF) << 12)
        | ((rd as u32) << 7)
        | (opcode as u32);

    return instruction;
}