use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-23.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let cups = parse_input(&input);

    measure_solution(23, 1, "", || part_one::solve(&cups));
    measure_solution(23, 2, "", || part_two::solve(&cups));
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
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let cups = parse_input(&input);

        let solution = part_one::solve(&cups);
        assert_eq!("39564287", solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let cups = parse_input(&input);

        let solution = part_two::solve(&cups);
        assert_eq!(404431096944, solution);
    }
}
