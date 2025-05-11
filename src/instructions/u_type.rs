pub fn parse_u_type(imm: i32, rd: u8, opcode: u8) {
    println!("{:08x}", u32::from_str_radix(&format!("{:020b}{:05b}{:07b}", (imm as u32) & 0xFFFFF, rd, opcode), 2).expect("Invalid binary string"));
}