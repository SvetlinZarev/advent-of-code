use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 23;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let cups = parse_input(&input);

    let (d1, _) = measure(DAY, "part 1", || part_one::solve(&cups));
    let (d2, _) = measure(DAY, "part 2", || part_two::solve(&cups));

    d1.add(d2)
}

pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .as_bytes()
        .iter()
        .copied()
        .map(|b| b - b'0')
        .map(|b| b as _)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let cups = parse_input(&input);

        let solution = part_one::solve(&cups);
        assert_eq!("39564287", solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let cups = parse_input(&input);

        let solution = part_two::solve(&cups);
        assert_eq!(404431096944, solution);
    }
}
