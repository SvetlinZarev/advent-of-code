use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::parsing::parse_line_delimited;
use aoc_2015_common::timing::measure;

const DAY: usize = 23;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let instructions = parse_line_delimited(&input);

    let (d1, _) = measure(DAY, "part 1", || solve_part_one(&instructions));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(&instructions));

    d1 + d2
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Half(u8),
    Triple(u8),
    Increment(u8),
    Jump(i8),
    JumpIfEven(u8, i8),
    JumpIfOne(u8, i8),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hlf a" => Ok(Instruction::Half(0)),
            "hlf b" => Ok(Instruction::Half(1)),
            "tpl a" => Ok(Instruction::Triple(0)),
            "tpl b" => Ok(Instruction::Triple(1)),
            "inc a" => Ok(Instruction::Increment(0)),
            "inc b" => Ok(Instruction::Increment(1)),
            _ => match &s[0..3] {
                "jmp" => {
                    let offset = s[4..].parse().map_err(|_| ())?;
                    Ok(Instruction::Jump(offset))
                }

                "jie" => {
                    let r = s.as_bytes()[4] - b'a';
                    let offset = s[7..].parse().map_err(|_| ())?;
                    Ok(Instruction::JumpIfEven(r, offset))
                }

                "jio" => {
                    let r = s.as_bytes()[4] - b'a';
                    let offset = s[7..].parse().map_err(|_| ())?;
                    Ok(Instruction::JumpIfOne(r, offset))
                }

                _ => Err(()),
            },
        }
    }
}

fn solve_part_one(input: &[Instruction]) -> u64 {
    solve(0, 0, input)
}

fn solve_part_two(input: &[Instruction]) -> u64 {
    solve(1, 0, input)
}

fn solve(a: u64, b: u64, inst: &[Instruction]) -> u64 {
    let mut reg = [a, b];
    let mut pc = 0isize;

    loop {
        if pc < 0 || pc as usize >= inst.len() {
            return reg[1];
        }

        let ins = inst[pc as usize];
        pc += 1;

        match ins {
            Instruction::Half(r) => {
                reg[r as usize] /= 2;
            }
            Instruction::Triple(r) => {
                reg[r as usize] *= 3;
            }
            Instruction::Increment(r) => {
                reg[r as usize] += 1;
            }
            Instruction::Jump(o) => {
                pc = pc + o as isize - 1;
            }
            Instruction::JumpIfEven(r, o) => {
                if reg[r as usize] & 1 == 0 {
                    pc = pc + o as isize - 1;
                }
            }
            Instruction::JumpIfOne(r, o) => {
                if reg[r as usize] == 1 {
                    pc = pc + o as isize - 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let instructions = parse_line_delimited(&input);
        let answer = solve_part_one(&instructions);
        assert_eq!(170, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let instructions = parse_line_delimited(&input);
        let answer = solve_part_two(&instructions);
        assert_eq!(247, answer);
    }
}
