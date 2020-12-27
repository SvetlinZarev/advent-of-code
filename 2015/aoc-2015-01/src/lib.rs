use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 1;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let input = input.trim().as_bytes();

    let (d_1, _) = measure(DAY, "part 1", || part_one::solve(input));
    let (d_2, _) = measure(DAY, "part 2", || part_two::solve(input));

    d_1.add(d_2)
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let input = input.trim().as_bytes();

        let solution = part_one::solve(input);
        assert_eq!(138, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let input = input.trim().as_bytes();

        let solution = part_two::solve(input);
        assert_eq!(Some(1771), solution);
    }
}
