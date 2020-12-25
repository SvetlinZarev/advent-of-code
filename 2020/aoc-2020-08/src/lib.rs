use std::path::Path;

use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-08.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);

    let input_v1 = part_one::parse_input(&input);
    measure_solution(8, 1, "", || part_one::solve(input_v1));

    let input_v2 = part_two::parse_input(&input);
    measure_solution(8, 2, "", || part_two::solve(input_v2));

    let mut input_v2 = part_two::parse_input(&input);
    part_two::preprocess_opcodes(&mut input_v2);
    measure_solution(8, 2, "processed", || part_two::solve(input_v2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);

        let input_v1 = part_one::parse_input(&input);
        let solution = part_one::solve(input_v1);
        assert_eq!(1939, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);

        let input_v2 = part_two::parse_input(&input);
        let solution = part_two::solve(input_v2);
        assert_eq!(Some(2212), solution);

        let mut input_v2 = part_two::parse_input(&input);
        part_two::preprocess_opcodes(&mut input_v2);
        let solution = part_two::solve(input_v2);
        assert_eq!(Some(2212), solution);
    }
}
