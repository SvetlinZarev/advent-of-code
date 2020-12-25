use std::path::Path;

use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use aoc_2020_common::parsing::parse_lines_as_usize;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-25.txt";

const SUBJECT_NUMBER: usize = 7;
const KEY_DIV: usize = 20201227;

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let data = parse_lines_as_usize(&input);

    measure_solution(25, 1, "two loops", || solve_v1(&data));
    measure_solution(25, 1, "one loop", || solve_v2(&data));
}

pub fn solve_v2(input: &[usize]) -> usize {
    assert_eq!(2, input.len());

    let pub_key_a = input[0];
    let pub_key_b = input[1];

    let mut pkey = 1;
    let mut encryption_key = 1;

    while pkey != pub_key_a {
        pkey = (pkey * SUBJECT_NUMBER) % KEY_DIV;
        encryption_key = (encryption_key * pub_key_b) % KEY_DIV;
    }

    encryption_key
}

pub fn solve_v1(input: &[usize]) -> usize {
    assert_eq!(2, input.len());

    let pkey_a = input[0];
    let pkey_b = input[1];

    let ls = derive_loop_size(pkey_a, SUBJECT_NUMBER);
    derive_encryption_key(ls, pkey_b)
}

fn derive_loop_size(pkey: usize, sn: usize) -> usize {
    let mut public_key = 1;
    let mut loop_size = 0;

    while public_key != pkey {
        public_key *= sn;
        public_key %= KEY_DIV;
        loop_size += 1;
    }

    loop_size
}

fn derive_encryption_key(ls: usize, sn: usize) -> usize {
    let mut encryption_key = 1;

    for _ in 0..ls {
        encryption_key *= sn;
        encryption_key %= KEY_DIV;
    }

    encryption_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_loop_size() {
        let ls = derive_loop_size(5764801, SUBJECT_NUMBER);
        assert_eq!(8, ls);

        let ls = derive_loop_size(17807724, SUBJECT_NUMBER);
        assert_eq!(11, ls);
    }

    #[test]
    fn test_derive_encryption_key() {
        let key_a = derive_encryption_key(8, 17807724);
        let key_b = derive_encryption_key(11, 5764801);
        assert_eq!(key_a, key_b);
    }

    #[test]
    fn test_part_one_v1() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let data = parse_lines_as_usize(&input);

        let solution = solve_v1(&data);
        assert_eq!(11328376, solution);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let data = parse_lines_as_usize(&input);

        let solution = solve_v2(&data);
        assert_eq!(11328376, solution);
    }
}
