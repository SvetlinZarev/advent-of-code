use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 14;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let (dp, input) = measure(DAY, "parsing", || parse_input(&input));
    let (d1, _) = measure(DAY, "part 1", || part_one::solve(&input));
    let (d2, _) = measure(DAY, "part 2", || part_two::solve(&input));

    dp.add(d1).add(d2)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Mask {
    /// The AND mask is inverted each set bit is 0
    and: u64,
    or: u64,
}

impl Mask {
    pub fn new(and: u64, or: u64) -> Mask {
        Mask { and, or }
    }

    pub fn no_op() -> Mask {
        Mask::new(u64::max_value(), 0)
    }

    /// The floating mask is inverted - it has 0 set for each X in the input
    pub fn floating(self) -> u64 {
        self.or | !self.and
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Mem {
    address: u64,
    value: u64,
}

impl Mem {
    pub fn new(address: u64, value: u64) -> Mem {
        Mem { address, value }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    Mem(Mem),
    Mask(Mask),
}

pub fn parse_input(input: &str) -> Vec<OpCode> {
    let mut result = vec![];

    for line in input.lines() {
        let opcode = if line.starts_with("mem") {
            parse_opcode_mem(line)
        } else if line.starts_with("mask") {
            parse_opcode_mask(line)
        } else {
            panic!("Unknown instruction in input: {}", line)
        };

        result.push(opcode);
    }

    result
}

fn parse_opcode_mem(line: &str) -> OpCode {
    assert!(line.starts_with("mem"));

    let idx_addr_start = 4;
    let idx_addr_end = line.find(']').unwrap();
    let addr = line[idx_addr_start..idx_addr_end].parse().unwrap();

    let idx_value_start = line.find("= ").unwrap() + 2;
    let value = line[idx_value_start..].parse().unwrap();

    OpCode::Mem(Mem::new(addr, value))
}

fn parse_opcode_mask(line: &str) -> OpCode {
    assert!(line.starts_with("mask"));

    let mask = &line[7..].as_bytes();
    /*
       There are two masks: AND and OR mask:
       ---
       The AND mask is used for setting bits to 0
       The OR mask is used for setting bits to 1

       Bit value 1 means "preserve current bit" for the AND mask
       Bit value 0 means "clear current bit" for the AND mask

       Bit value 1 means "set current bit" for the OR mask
       Bit value 0 means "preserve current bit" for the OR mask
    */

    let mut mask_or = 0;
    let mut mask_and = u64::max_value();

    for b in mask.iter().copied() {
        mask_and <<= 1;
        mask_or <<= 1;

        match b {
            b'X' => {
                mask_and |= 1;
                // LSB on mask_or is already 0
            }

            b'0' => {
                mask_and &= u64::max_value() << 1;
                // LSB on mask_or is already 0
            }

            b'1' => {
                mask_and |= 1;
                mask_or |= 1;
            }

            _ => panic!("Unexpected input: {}", line),
        }
    }

    OpCode::Mask(Mask::new(mask_and, mask_or))
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let input = parse_input(&input);

        let solution = part_one::solve(&input);
        assert_eq!(13476250121721, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let input = parse_input(&input);

        let solution = part_two::solve(&input);
        assert_eq!(4463708436768, solution);
    }
}
