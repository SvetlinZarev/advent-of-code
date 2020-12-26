use std::path::Path;

use aoc_2015_common::input::load_input;
use aoc_2015_common::output::measure_solution;

pub mod part_one;
pub mod part_two;

pub const DAY: &'static str = "day-03";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let input = parse_input(&input);

    measure_solution(3, 1, "", || part_one::solve(&input));
    measure_solution(3, 2, "", || part_two::solve(&input));
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
