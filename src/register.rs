use crate::print_error;

//bad solution but it works
pub fn parse_register(register: &str, line: usize) -> u8 {
    match register {
        "x0" | "zero" => 0,
        "x1" | "ra" => 1,
        "x2" | "sp" => 2,
        "x3" | "gp" => 3,
        "x4" | "tp" => 4,
        "x5" | "t0" => 5,
        "x6" | "t1" => 6,
        "x7" | "t2" => 7,
        "x8" | "s0" | "fp" => 8,
        "x9" | "s1" => 9,
        "x10" | "a0" => 10,
        "x11" | "a1" => 11,
        "x12" | "a2" => 12,
        "x13" | "a3" => 13,
        "x14" | "a4" => 14,
        "x15" | "a5" => 15,
        "x16" | "a6" => 16,
        "x17" | "a7" => 17,
        "x18" | "s2" => 18,
        "x19" | "s3" => 19,
        "x20" | "s4" => 20,
        "x21" | "s5" => 21,
        "x22" | "s6" => 22,
        "x23" | "s7" => 23,
        "x24" | "s8" => 24,
        "x25" | "s9" => 25,
        "x26" | "s10" => 26,
        "x27" | "s11" => 27,
        "x28" | "t3" => 28,
        "x29" | "t4" => 29,
        "x30" | "t5" => 30,
        "x31" | "t6" => 31,
        _ => {
            print_error(&format!("Invalid register name: {} on line {}", register, line));
            32
        }
    }
}