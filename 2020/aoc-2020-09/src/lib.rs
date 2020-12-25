use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use aoc_2020_common::parsing::parse_lines_as_u64;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const PART_ONE_KEY: u64 = 507622668;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-09.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let input = parse_lines_as_u64(&input);

    let key = measure_solution(9, 1, "", || part_one::solve(&input));
    if let Some(key) = key {
        measure_solution(9, 2, "", || part_two::solve(&input, key));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let input = parse_lines_as_u64(&input);

        let solution = part_one::solve(&input);
        assert_eq!(Some(507622668), solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let input = parse_lines_as_u64(&input);

        let solution = part_two::solve(&input, PART_ONE_KEY);
        assert_eq!(Some(76688505), solution);
    }
}
