use crate::print_error;

pub fn parse_r_type(instruction: &str, rd: u8, rs1: u8, rs2: u8, opcode: u8, line: usize) -> u32 {
	let mut parts = instruction.split('.');
	let first = parts.next().unwrap_or("");
	let second = parts.next();

	let base: &str = match second {
			Some(s) => &instruction[..first.len() + 1 + s.len()],
			None => first,
	};

	let (func3, mut func7): (u8, u8) = match opcode {
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
					unreachable!();
				}
			}
		}
		47 => {
			match base {
				"lr.w"      => (2, 8),
				"sc.w"      => (2, 12),
				"amoswap.w" => (2, 4),
				"amoadd.w"  => (2, 0),
				"amoand.w"  => (2, 48),
				"amoor.w"   => (2, 40),
				"amoxor.w"  => (2, 16),
				"amomax.w"  => (2, 80),
				"amomin.w"  => (2, 64),
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

	if opcode == 47 {
		if instruction.contains(".aq") {
			func7 += 2;
		}
		if instruction.contains(".rl") {
			func7 += 1;
		}
	}

	let instruction = ((func7 as u32) << 25)
		| ((rs2 as u32) << 20)
		| ((rs1 as u32) << 15)
		| ((func3 as u32) << 12)
		| ((rd as u32) << 7)
		| (opcode as u32);

	return instruction;
}