use std::path::Path;

use md5::{compute, Digest};

use aoc_2015_common::input::load_input;
use aoc_2015_common::output::measure_solution;

pub const DAY: &'static str = "day-04";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path).trim().to_owned();

    measure_solution(4, 1, "", || solve_p1(&input));
    measure_solution(4, 2, "", || solve_p2(&input));
}

pub fn solve_p1(input: &str) -> Option<u32> {
    solve(input, is_part_one_digest)
}

pub fn solve_p2(input: &str) -> Option<u32> {
    solve(input, is_part_two_digest)
}

fn solve(input: &str, check_digest: fn(&Digest) -> bool) -> Option<u32> {
    for key in 1..=u32::max_value() {
        let digest = compute(format!("{}{}", input, key));
        if check_digest(&digest) {
            return Some(key);
        }
    }

    None
}

fn is_part_one_digest(digest: &Digest) -> bool {
    let be_bytes = [digest.0[0], digest.0[1], digest.0[2], digest.0[3]];

    let num = u32::from_be_bytes(be_bytes);
    num <= 0x00_00_0F_FF
}

fn is_part_two_digest(digest: &Digest) -> bool {
    let be_bytes = [digest.0[0], digest.0[1], digest.0[2], digest.0[3]];

    let num = u32::from_be_bytes(be_bytes);
    num <= 0x00_00_00_FF
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_is_part_one_digest_ok() {
        let digest = compute("abcdef609043");
        assert!(is_part_one_digest(&digest));
    }

    #[test]
    fn test_is_part_two_digest_ok() {
        let digest = compute("bgvyzdsv1038736");
        assert!(is_part_two_digest(&digest));
    }

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY)).trim().to_owned();
        let solution = solve_p1(&input);
        assert_eq!(Some(254575), solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY)).trim().to_owned();
        let solution = solve_p2(&input);
        assert_eq!(Some(1038736), solution);
    }
}
