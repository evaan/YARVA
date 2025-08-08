pub fn parse_s_type(instruction: &str, rs1: u8, rs2: u8, imm: i16, opcode: u8) -> u32 {
	let func3: u8 = match instruction {
		"sb" => 0,
		"sh" => 1,
		"sw" => 2,
		_ => {
			unreachable!();
		}
	};

	let instruction = (((imm as u32 >> 5) & 0x7F) << 25)
    | ((rs2 as u32) << 20)
    | ((rs1 as u32) << 15)
    | ((func3 as u32) << 12)
    | ((imm as u32 & 0x1F) << 7)
    | (opcode as u32);
	
	return instruction
}