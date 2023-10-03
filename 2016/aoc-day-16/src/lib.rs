use std::ops::BitXor;

use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;

pub const PART_1_LEN: usize = 272;
pub const PART_2_LEN: usize = 35_651_584;

pub fn part_one(input: impl AsRef<[u8]>) -> String {
    solve_v1(input, PART_1_LEN)
}

pub fn part_two(input: impl AsRef<[u8]>) -> String {
    solve_v1(input, PART_2_LEN)
}

pub fn solve_v1(input: impl AsRef<[u8]> + Sized, length: usize) -> String {
    let mut buffer = input.as_ref().to_vec();

    while buffer.len() < length {
        buffer.push(b'0');
        for idx in (0..buffer.len() - 1).rev() {
            let value = match buffer[idx] {
                b'0' => b'1',
                b'1' => b'0',
                _ => unreachable!(),
            };
            buffer.push(value);
        }
    }
    buffer.truncate(length);

    while buffer.len() % 2 == 0 {
        for idx in 0..buffer.len() / 2 {
            buffer[idx] = (buffer[idx * 2] == buffer[idx * 2 + 1]) as u8 + b'0';
        }
        buffer.truncate(buffer.len() / 2);
    }

    String::from_utf8(buffer).unwrap()
}

pub fn solve_v2(input: impl AsRef<[u8]> + Sized, length: usize) -> String {
    let mut bits = BitVec::<usize, Msb0>::new();
    for bit in input.as_ref().iter().copied() {
        bits.push(bit == b'1');
    }

    while bits.len() < length {
        bits.push(false);
        for idx in (0..bits.len() - 1).rev() {
            let value = unsafe { bits.get_unchecked_mut(idx).bitxor(true) };
            bits.push(value);
        }
    }
    bits.truncate(length);

    while bits.len() % 2 == 0 {
        for idx in 0..bits.len() / 2 {
            let a = unsafe { bits.get_unchecked(idx * 2) }.as_bool();
            let b = unsafe { bits.get_unchecked(idx * 2 + 1) }.as_bool();
            bits.set(idx, a == b);
        }

        bits.truncate(bits.len() / 2);
    }

    bits.iter()
        .by_vals()
        .map(|x| if x { '1' } else { '0' })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{solve_v1, solve_v2, PART_1_LEN, PART_2_LEN};

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = solve_v1(input.trim_end(), PART_1_LEN);
        assert_eq!("10010010110011010", answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = solve_v1(input.trim_end(), PART_2_LEN);
        assert_eq!("01010100101011100", answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = solve_v2(input.trim_end(), PART_1_LEN);
        assert_eq!("10010010110011010", answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = solve_v2(input.trim_end(), PART_2_LEN);
        assert_eq!("01010100101011100", answer);
    }
}
