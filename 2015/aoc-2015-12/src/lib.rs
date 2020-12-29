use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

mod part_one_regex;
mod part_two_serde;

pub const DAY: usize = 12;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d1, _) = measure(DAY, "part 1: regex", || solve_p1_regex(&input));
    let (d2, _) = measure(DAY, "part 2: serde", || solve_p2_serde(&input));

    d1 + d2
}

pub fn solve_p1_regex(input: &str) -> i64 {
    part_one_regex::solve(input)
}

pub fn solve_p2_serde(input: &str) -> i64 {
    part_two_serde::solve(input)
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let answer = solve_p1_regex(&input);
        assert_eq!(191164, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let answer = solve_p2_serde(&input);
        assert_eq!(87842, answer);
    }
}
