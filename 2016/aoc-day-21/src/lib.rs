use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Captures;
use regex::Regex;

const TO_SCRAMBLE: &str = "abcdefgh";
const TO_UNSCRAMBLE: &str = "fbgdceah";

lazy_static! {
    static ref REGEX_SWAP_LETTER: Regex =
        Regex::new(r#"^swap letter (?<target>[a-z]) with letter (?<replacement>[a-z])$"#).unwrap();
    static ref REGEX_SWAP_POSITION: Regex =
        Regex::new(r#"^swap position (?<target>\d+) with position (?<replacement>\d+)$"#).unwrap();
    static ref REGEX_REVERSE_RANGE: Regex =
        Regex::new(r#"^reverse positions (?<from>\d+) through (?<to>\d+)$"#).unwrap();
    static ref REGEX_MOVE_POSITION: Regex =
        Regex::new(r#"^move position (?<from>\d+) to position (?<to>\d+)$"#).unwrap();
    static ref REGEX_ROTATE_LEFT: Regex =
        Regex::new(r#"^rotate left (?<amount>\d+) step(s?)$"#).unwrap();
    static ref REGEX_ROTATE_RIGHT: Regex =
        Regex::new(r#"^rotate right (?<amount>\d+) step(s?)$"#).unwrap();
    static ref REGEX_ROTATE_RIGHT_ON_LETTER: Regex =
        Regex::new(r#"^rotate based on position of letter (?<target>[a-z])$"#).unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    SwapLetter(u8, u8),
    SwapPosition(usize, usize),
    ReverseRange(usize, usize),
    MovePosition(usize, usize),
    RotateLeft(usize),
    RotateRight(usize),
    RotateRightOn(u8),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cap) = REGEX_SWAP_LETTER.captures(s) {
            let (target, replacement) = extract_two_captures(s, cap, "target", "replacement")?;

            return Ok(Instruction::SwapLetter(
                target.as_bytes()[0],
                replacement.as_bytes()[0],
            ));
        }

        if let Some(cap) = REGEX_SWAP_POSITION.captures(s) {
            let (target, replacement) = extract_two_captures(s, cap, "target", "replacement")?;
            let target = target
                .parse()
                .map_err(|_| format!("invalid input: {}", s))?;
            let replacement = replacement
                .parse()
                .map_err(|_| format!("invalid input: {}", s))?;

            return Ok(Instruction::SwapPosition(target, replacement));
        }

        if let Some(cap) = REGEX_REVERSE_RANGE.captures(s) {
            let (from, to) = extract_two_captures(s, cap, "from", "to")?;
            let from = from.parse().map_err(|_| format!("invalid input: {}", s))?;
            let to = to.parse().map_err(|_| format!("invalid input: {}", s))?;

            return Ok(Instruction::ReverseRange(from, to));
        }

        if let Some(cap) = REGEX_MOVE_POSITION.captures(s) {
            let (from, to) = extract_two_captures(s, cap, "from", "to")?;
            let from = from.parse().map_err(|_| format!("invalid input: {}", s))?;
            let to = to.parse().map_err(|_| format!("invalid input: {}", s))?;

            return Ok(Instruction::MovePosition(from, to));
        }

        if let Some(cap) = REGEX_ROTATE_LEFT.captures(s) {
            let amount = cap
                .name("amount")
                .ok_or_else(|| format!("invalid input: {}", s))?
                .as_str();
            let amount = amount
                .parse()
                .map_err(|_| format!("invalid input: {}", s))?;

            return Ok(Instruction::RotateLeft(amount));
        }

        if let Some(cap) = REGEX_ROTATE_RIGHT.captures(s) {
            let amount = cap
                .name("amount")
                .ok_or_else(|| format!("invalid input: {}", s))?
                .as_str();
            let amount = amount
                .parse()
                .map_err(|_| format!("invalid input: {}", s))?;

            return Ok(Instruction::RotateRight(amount));
        }

        if let Some(cap) = REGEX_ROTATE_RIGHT_ON_LETTER.captures(s) {
            let target = cap
                .name("target")
                .ok_or_else(|| format!("invalid input: {}", s))?
                .as_str();

            return Ok(Instruction::RotateRightOn(target.as_bytes()[0]));
        }

        Err(format!("invalid input: {}", s))
    }
}

fn extract_two_captures<'c>(
    s: &'c str,
    cap: Captures<'c>,
    first: &str,
    second: &str,
) -> Result<(&'c str, &'c str), String> {
    let one = cap
        .name(first)
        .ok_or_else(|| format!("invalid input: {}", s))?
        .as_str();

    let two = cap
        .name(second)
        .ok_or_else(|| format!("invalid input: {}", s))?
        .as_str();

    Ok((one, two))
}

pub fn part_one(input: &[Instruction]) -> String {
    let mut buf = TO_SCRAMBLE.as_bytes().to_vec();
    let len = buf.len();

    for ins in input.iter().copied() {
        match ins {
            Instruction::SwapLetter(x, y) => {
                for idx in 0..buf.len() {
                    if buf[idx] == x {
                        buf[idx] = y;
                    } else if buf[idx] == y {
                        buf[idx] = x;
                    }
                }
            }
            Instruction::SwapPosition(a, b) => buf.swap(a, b),
            Instruction::ReverseRange(from, to) => buf[from..=to].reverse(),
            Instruction::MovePosition(from, to) => {
                if from < to {
                    buf[from..=to].rotate_left(1);
                } else if from > to {
                    buf[to..=from].rotate_right(1);
                }
            }
            Instruction::RotateLeft(amount) => buf.rotate_left(amount % len),
            Instruction::RotateRight(amount) => buf.rotate_right(amount % len),
            Instruction::RotateRightOn(x) => {
                rotate_based_on(&mut buf, x);
            }
        }
    }

    String::from_utf8(buf).unwrap()
}

pub fn part_two(input: &[Instruction]) -> String {
    let mut buf = TO_UNSCRAMBLE.as_bytes().to_vec();
    let len = buf.len();

    for ins in input.iter().copied().rev() {
        match ins {
            Instruction::SwapLetter(x, y) => {
                for idx in 0..buf.len() {
                    if buf[idx] == x {
                        buf[idx] = y;
                    } else if buf[idx] == y {
                        buf[idx] = x;
                    }
                }
            }
            Instruction::SwapPosition(a, b) => buf.swap(a, b),
            Instruction::ReverseRange(from, to) => buf[from..=to].reverse(),
            Instruction::MovePosition(from, to) => {
                if from < to {
                    buf[from..=to].rotate_right(1);
                } else if from > to {
                    buf[to..=from].rotate_left(1);
                }
            }
            Instruction::RotateLeft(amount) => buf.rotate_right(amount % len),
            Instruction::RotateRight(amount) => buf.rotate_left(amount % len),
            Instruction::RotateRightOn(x) => {
                // Reverse this function by brute-forcing it,
                // as it has only 8 possible answers
                let mut s = vec![0; buf.len()];

                for amount in 0..buf.len() {
                    s.copy_from_slice(&buf);
                    s.rotate_left(amount);

                    rotate_based_on(&mut s, x);
                    if s == buf {
                        buf.rotate_left(amount);
                        break;
                    }
                }
            }
        }
    }

    String::from_utf8(buf).unwrap()
}

fn rotate_based_on(buf: &mut Vec<u8>, letter: u8) {
    let len = buf.len();

    let idx = buf.iter().position(|&l| l == letter).unwrap();
    let amount = (idx + 1 + (idx >= 4) as usize) % len;

    buf.rotate_right(amount);
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!("agcebfdh", answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!("afhdbegc", answer);
    }
}
