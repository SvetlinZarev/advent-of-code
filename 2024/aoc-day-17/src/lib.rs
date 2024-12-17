use std::error::Error;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

mod compiled;
mod interpreted;

pub struct Input {
    rom: Vec<u8>,
    reg: [u64; 3],
}

pub fn parse_input(input: &str) -> Result<Input, Box<dyn Error>> {
    let mut input = input.as_bytes();
    while !input[0].is_ascii_digit() {
        input = &input[1..];
    }

    let mut a = 0;
    while input[0].is_ascii_digit() {
        a *= 10;
        a += (input[0] - b'0') as u64;
        input = &input[1..];
    }

    while !input[0].is_ascii_digit() {
        input = &input[1..];
    }

    let mut b = 0;
    while input[0].is_ascii_digit() {
        b *= 10;
        b += (input[0] - b'0') as u64;
        input = &input[1..];
    }

    while !input[0].is_ascii_digit() {
        input = &input[1..];
    }

    let mut c = 0;
    while input[0].is_ascii_digit() {
        c *= 10;
        c += (input[0] - b'0') as u64;
        input = &input[1..];
    }

    while !input[0].is_ascii_digit() {
        input = &input[1..];
    }

    let mut rom = vec![];
    while !input.is_empty() {
        rom.push(input[0] - b'0');
        input = &input[2..];
    }

    Ok(Input {
        rom,
        reg: [a, b, c],
    })
}

pub fn part_one_v1(input: &Input) -> String {
    interpreted::part_one(input)
}

pub fn part_two_v1(input: &Input) -> u64 {
    interpreted::part_two(input)
}

pub fn part_one_v2(input: &Input) -> String {
    compiled::part_one(input)
}

pub fn part_two_v2(input: &Input) -> u64 {
    compiled::part_two(input)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_one_v1(&parsed);
        assert_eq!("6,5,4,7,1,6,0,3,1", answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_one_v2(&parsed);
        assert_eq!("6,5,4,7,1,6,0,3,1", answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two_v1(&parsed);
        assert_eq!(106086382266778, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two_v2(&parsed);
        assert_eq!(106086382266778, answer);
    }
}
