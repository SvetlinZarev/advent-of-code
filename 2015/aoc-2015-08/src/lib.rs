use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

mod part_one;
mod part_two;

pub const DAY: usize = 8;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d1, _) = measure(DAY, "part 1", || solve_part_one(&input));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(&input));
    d1 + d2
}

pub fn solve_part_one(input: &str) -> usize {
    part_one::solve(input)
}

pub fn solve_part_two(input: &str) -> usize {
    part_two::solve(input)
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let count = solve_part_one(&input);
        assert_eq!(1350, count);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let count = solve_part_two(&input);
        assert_eq!(2085, count);
    }
}
