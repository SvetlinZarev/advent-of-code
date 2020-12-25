use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-02.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);

    measure_solution(2, 1, "", || part_one::solve(&input));
    measure_solution(2, 2, "", || part_two::solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let solution = part_one::solve(&input);
        assert_eq!(548, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let solution = part_two::solve(&input);
        assert_eq!(502, solution);
    }
}
