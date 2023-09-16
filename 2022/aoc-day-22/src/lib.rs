use std::num::NonZeroU32;

pub mod part_one;
pub mod part_two;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    Move(NonZeroU32),
    RotR,
    RotL,
}

pub fn parse_input(input: impl AsRef<str>) -> (Vec<Vec<u8>>, Vec<Instruction>) {
    let mut map = vec![];
    let mut instructions = vec![];

    let mut parse_map = true;

    for line in input.as_ref().lines() {
        if parse_map {
            if line.is_empty() {
                parse_map = false;
                continue;
            }

            map.push(line.as_bytes().to_vec());
            continue;
        }

        instructions = parse_instructions(line);
        break;
    }

    (map, instructions)
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let mut instr = vec![];
    let mut n = 0;

    for ch in instructions.bytes() {
        match ch {
            b'L' => {
                if n != 0 {
                    instr.push(Instruction::Move(unsafe { NonZeroU32::new_unchecked(n) }));
                    n = 0;
                }
                instr.push(Instruction::RotL);
            }

            b'R' => {
                if n != 0 {
                    instr.push(Instruction::Move(unsafe { NonZeroU32::new_unchecked(n) }));
                    n = 0;
                }
                instr.push(Instruction::RotR);
            }

            b'0'..=b'9' => {
                n *= 10;
                n += (ch - b'0') as u32;
            }

            _ => panic!("unexpected character: {}", ch as char),
        }
    }

    if n != 0 {
        instr.push(Instruction::Move(unsafe { NonZeroU32::new_unchecked(n) }));
    }

    instr
}

fn find_start_column(map: &[Vec<u8>]) -> usize {
    for c in 0..map[0].len() {
        if map[0][c] == b'.' {
            return c;
        }
    }

    panic!("there is no starting column")
}
