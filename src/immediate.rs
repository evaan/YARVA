use crate::{print_error};

pub fn parse_12_bit_immediate(imm: &str, line: usize) -> u16 {
  let parsed = if imm.starts_with("0x") {
    i16::from_str_radix(&imm[2..], 16)
  } else if imm.starts_with("0o") {
    i16::from_str_radix(&imm[2..], 8)
  } else if imm.starts_with("0b") {
    i16::from_str_radix(&imm[2..], 2)
  } else {
    imm.parse::<i16>()
  };

  match parsed {
    Ok(mut n) => {
      n = (n & 0x7FF) - (n & 0x800);
      if n >= 2048 || n < -2048 {
        print_error(&format!("Out of range immediate '{}' on line {}", imm, line));
        unreachable!();
      } else {
        (n & 0xFFF) as u16
      }
    }
    Err(_) => {
      print_error(&format!("Invalid immediate '{}' on line {}", imm, line));
      unreachable!();
    }
  }
}

pub fn parse_13_bit_immediate(imm: &str, line: usize) -> u16 {
  let parsed = if imm.starts_with("0x") {
    i16::from_str_radix(&imm[2..], 16)
  } else if imm.starts_with("0o") {
    i16::from_str_radix(&imm[2..], 8)
  } else if imm.starts_with("0b") {
    i16::from_str_radix(&imm[2..], 2)
  } else {
    imm.parse::<i16>()
  };

  match parsed {
    Ok(mut n) => {
      n = (n & 0xFFF) - (n & 0x1000);
      if n % 2 == 1 {
        print_error(&format!("Non two-byte aligned immediate '{}' on line {}", imm, line));
        unreachable!();
      } else if n >= 4096 || n < -4096 {
        print_error(&format!("Out of range immediate '{}' on line {}", imm, line));
        unreachable!();
      } else {
        (n & 0x1FFE) as u16
      }
    }
    Err(_) => {
      print_error(&format!("Invalid immediate '{}' on line {}", imm, line));
      unreachable!();
    }
  }
}

pub fn parse_32_bit_immediate(imm: &str, line: usize) -> u32 {
  let parsed = if imm.starts_with("0x") {
    i32::from_str_radix(&imm[2..], 16)
  } else if imm.starts_with("0o") {
    i32::from_str_radix(&imm[2..], 8)
  } else if imm.starts_with("0b") {
    i32::from_str_radix(&imm[2..], 2)
  } else {
    imm.parse::<i32>()
  };

  match parsed {
    Ok(mut n) => {
      n = (n & 0x7FFFF) - (n & 0x80000);
      if n >= 1048575 || n < -1048575 {
        print_error(&format!("Out of range immediate '{}' on line {}", imm, line));
        unreachable!();
      } else {
        (n & 0xFFFFF) as u32
      }
    }
    Err(_) => {
      print_error(&format!("Invalid immediate '{}' on line {}", imm, line));
      unreachable!();
    }
  }
}

pub fn parse_21_bit_immediate(imm: &str, line: usize) -> u32 {
  let parsed = if imm.starts_with("0x") {
    i32::from_str_radix(&imm[2..], 16)
  } else if imm.starts_with("0o") {
    i32::from_str_radix(&imm[2..], 8)
  } else if imm.starts_with("0b") {
    i32::from_str_radix(&imm[2..], 2)
  } else {
    imm.parse::<i32>()
  };

  match parsed {
    Ok(mut n) => {
      n = (n & 0xFFFFE) - (n & 0x100000);
      if n % 2 == 1 {
        print_error(&format!("Non two-byte aligned immediate '{}' on line {}", imm, line));
        unreachable!();
      } else if n > 1048576 || n < -1048576 {
        print_error(&format!("Out of range immediate '{}' on line {}", imm, line));
        unreachable!();
      } else {
        (n & 0x1FFFFE) as u32
      }
    }
    Err(_) => {
      print_error(&format!("Invalid immediate '{}' on line {}", imm, line));
      unreachable!();
    }
  }
}
