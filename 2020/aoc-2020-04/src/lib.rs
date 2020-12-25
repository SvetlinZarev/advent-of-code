use std::path::Path;

use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;

pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-04.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);

    measure_solution(4, 2, "nom", || part_two::solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let solution = part_two::solve(&input);
        assert_eq!(188, solution);
    }
}
