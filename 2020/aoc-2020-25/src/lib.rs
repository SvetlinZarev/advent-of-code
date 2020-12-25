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
    measure_solution(25, 1, "both keys", || solve_v3(&data));
}

pub fn solve_v3(input: &[usize]) -> usize {
    assert_eq!(2, input.len());

    let pub_key_a = input[0].max(input[1]);
    let pub_key_b = input[0].min(input[1]);

    let mut pub_key = 1;
    let mut enc_key_a = 1;
    let mut enc_key_b = 1;

    loop {
        pub_key = (pub_key * SUBJECT_NUMBER) % KEY_DIV;
        enc_key_a = (enc_key_a * pub_key_a) % KEY_DIV;
        enc_key_b = (enc_key_b * pub_key_b) % KEY_DIV;

        if pub_key == pub_key_a {
            break enc_key_b;
        }

        if pub_key == pub_key_b {
            break enc_key_a;
        }
    }
}

pub fn solve_v2(input: &[usize]) -> usize {
    assert_eq!(2, input.len());

    let pub_key_a = input[0].max(input[1]);
    let pub_key_b = input[0].min(input[1]);

    let mut pub_key = 1;
    let mut encryption_key = 1;

    while pub_key != pub_key_a {
        pub_key = (pub_key * SUBJECT_NUMBER) % KEY_DIV;
        encryption_key = (encryption_key * pub_key_b) % KEY_DIV;
    }

    encryption_key
}

pub fn solve_v1(input: &[usize]) -> usize {
    assert_eq!(2, input.len());

    let pub_key_a = input[0].max(input[1]);
    let pub_key_b = input[0].min(input[1]);

    let ls = derive_loop_size(pub_key_a, SUBJECT_NUMBER);
    derive_encryption_key(ls, pub_key_b)
}

fn derive_loop_size(pkey: usize, sn: usize) -> usize {
    let mut public_key = 1;
    let mut loop_size = 0;

    while public_key != pkey {
        public_key = (public_key * sn) % KEY_DIV;
        loop_size += 1;
    }

    loop_size
}

fn derive_encryption_key(ls: usize, sn: usize) -> usize {
    let mut encryption_key = 1;

    for _ in 0..ls {
        encryption_key = (encryption_key * sn) % KEY_DIV;
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

    #[test]
    fn test_part_one_v3() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let data = parse_lines_as_usize(&input);

        let solution = solve_v3(&data);
        assert_eq!(11328376, solution);
    }
}
