use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_INSTR: Regex = Regex::new(r#"(?<dst>[a-z]+) (?<op>(inc)|(dec)) (?<increment>\-?[0-9]+) if (?<src>[a-z]+) (?<cmp>((<|>)=?)|((!|=)=)) (?<query>\-?[0-9]+)"#).unwrap();
}

#[derive(Debug)]
pub struct Instruction<'l> {
    op: Op,
    cmp: Jump,
    dst: &'l str,
    src: &'l str,
    increment: i32,
    query: i32,
}

impl<'l> TryFrom<&'l str> for Instruction<'l> {
    type Error = Box<dyn Error>;

    fn try_from(value: &'l str) -> Result<Self, Self::Error> {
        let Some(cap) = REGEX_INSTR.captures(value) else {
            return Err(format!("invalid instruction string: {}", value).into());
        };

        let Some(dst) = cap.name("dst").map(|x| x.as_str()) else {
            return Err(format!("invalid instruction string: {}", value).into());
        };
        let Some(src) = cap.name("src").map(|x| x.as_str()) else {
            return Err(format!("invalid instruction string: {}", value).into());
        };
        let Some(op_name) = cap.name("op").map(|x| x.as_str()) else {
            return Err(format!("invalid instruction string: {}", value).into());
        };
        let Some(increment) = cap.name("increment").map(|x| x.as_str()) else {
            return Err(format!("invalid instruction string: {}", value).into());
        };
        let Some(query) = cap.name("query").map(|x| x.as_str()) else {
            return Err(format!("invalid instruction string: {}", value).into());
        };
        let Some(cmp) = cap.name("cmp").map(|x| x.as_str()) else {
            return Err(format!("invalid instruction string: {}", value).into());
        };

        Ok(Instruction {
            dst,
            src,
            op: op_name.parse()?,
            cmp: cmp.parse()?,
            increment: increment.parse()?,
            query: query.parse()?,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Op {
    Inc,
    Dec,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "inc" => Op::Inc,
            "dec" => Op::Dec,
            _ => return Err(format!("invalid operation: {}", s)),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Jump {
    EQ,
    NEQ,
    ELT,
    EGT,
    LT,
    GT,
}

impl FromStr for Jump {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "==" => Jump::EQ,
            "!=" => Jump::NEQ,
            ">=" => Jump::EGT,
            "<=" => Jump::ELT,
            ">" => Jump::GT,
            "<" => Jump::LT,
            _ => return Err(format!("invalid comparison operator: {}", s)),
        })
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.try_into().unwrap()).collect()
}

pub fn part_one(input: &[Instruction]) -> i32 {
    let mut registers = HashMap::new();

    for instr in input {
        let src_val = *registers.entry(instr.src).or_insert(0);
        let diff = match instr.cmp {
            Jump::EQ if src_val != instr.query => continue,
            Jump::NEQ if src_val == instr.query => continue,
            Jump::ELT if src_val > instr.query => continue,
            Jump::EGT if src_val < instr.query => continue,
            Jump::LT if src_val >= instr.query => continue,
            Jump::GT if src_val <= instr.query => continue,
            _ => match instr.op {
                Op::Inc => instr.increment,
                Op::Dec => -instr.increment,
            },
        };
        *registers.entry(instr.dst).or_insert(0) += diff;
    }

    registers.values().copied().max().unwrap()
}

pub fn part_two(input: &[Instruction]) -> i32 {
    let mut registers = HashMap::new();
    let mut answer = i32::MIN;

    for instr in input {
        let src_val = *registers.entry(instr.src).or_insert(0);
        let diff = match instr.cmp {
            Jump::EQ if src_val != instr.query => continue,
            Jump::NEQ if src_val == instr.query => continue,
            Jump::ELT if src_val > instr.query => continue,
            Jump::EGT if src_val < instr.query => continue,
            Jump::LT if src_val >= instr.query => continue,
            Jump::GT if src_val <= instr.query => continue,
            _ => match instr.op {
                Op::Inc => instr.increment,
                Op::Dec => -instr.increment,
            },
        };
        let value = registers.entry(instr.dst).or_insert(0);
        *value += diff;
        answer = answer.max(*value);
    }

    answer
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);

        let answer = part_one(&parsed);
        assert_eq!(5966, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);

        let answer = part_two(&parsed);
        assert_eq!(6347, answer);
    }
}
