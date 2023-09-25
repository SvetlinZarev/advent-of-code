use std::fmt::Write;

use md5::digest::FixedOutputReset;
use md5::{Digest, Md5};

const HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

const EMPTY: char = ' ';
const PASS_LEN: usize = 8;

pub fn part_one(input: &str) -> String {
    let mut hasher = Md5::new();
    let mut buffer = String::with_capacity(8);
    let mut answer = String::with_capacity(PASS_LEN);

    for idx in 0u32.. {
        buffer.clear();
        write!(buffer, "{}", idx).unwrap();

        hasher.update(input);
        hasher.update(&buffer);

        let hash = hasher.finalize_fixed_reset();
        if hash[0] == 0 && hash[1] == 0 && hash[2] <= 0x0F {
            answer.push(HEX[hash[2] as usize]);
            if answer.len() == PASS_LEN {
                break;
            }
        }
    }

    answer
}

pub fn part_two(input: &str) -> String {
    let mut hasher = Md5::new();
    let mut buffer = String::with_capacity(8);
    let mut answer = [EMPTY; PASS_LEN];
    let mut remaining = PASS_LEN;

    for idx in 0u32.. {
        buffer.clear();
        write!(buffer, "{}", idx).unwrap();

        hasher.update(input);
        hasher.update(&buffer);

        let hash = hasher.finalize_fixed_reset();
        if hash[0] == 0 && hash[1] == 0 && hash[2] <= 0x07 && answer[hash[2] as usize] == EMPTY {
            answer[hash[2] as usize] = HEX[(hash[3] >> 4) as usize];

            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }

    String::from_iter(answer)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(input.trim());
        assert_eq!("1A3099AA", answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(input.trim());
        assert_eq!("694190CD", answer);
    }
}
