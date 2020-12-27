use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 3;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d_p, input) = measure(DAY, "parsing", || parse_input(&input));
    let (d_1, _) = measure(DAY, "part 1", || part_one::solve(&input));
    let (d_2, _) = measure(DAY, "part 2", || part_two::solve(&input));

    d_p.add(d_1).add(d_2)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

pub fn parse_input(input: &str) -> Vec<Direction> {
    input
        .as_bytes()
        .iter()
        .copied()
        .map(|b| match b {
            b'^' => Direction::N,
            b'>' => Direction::E,
            b'v' => Direction::S,
            b'<' => Direction::W,
            _ => panic!("Invalid direction: {}", b as char),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(vec![Direction::N], parse_input("^"));
        assert_eq!(vec![Direction::E], parse_input(">"));
        assert_eq!(vec![Direction::S], parse_input("v"));
        assert_eq!(vec![Direction::W], parse_input("<"));
    }

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let input = parse_input(&input);
        let solution = part_one::solve(&input);
        assert_eq!(2565, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let input = parse_input(&input);
        let solution = part_two::solve(&input);
        assert_eq!(2639, solution);
    }
}
