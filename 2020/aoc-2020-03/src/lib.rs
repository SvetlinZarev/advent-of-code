use std::path::Path;

use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-03.txt";

const WIDTH: usize = 32;
const COLUMNS: usize = WIDTH - 1;
const MARK_TREE: u8 = b'#';
const NEW_LINE: u8 = b'\n';

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);

    measure_solution(3, 1, "", || part_one::solve(input.as_bytes()));
    measure_solution(3, 2, "", || part_two::solve(input.as_bytes()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let solution = part_one::solve(input.as_bytes());
        assert_eq!(211, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let solution = part_two::solve(input.as_bytes());
        assert_eq!(3584591857, solution);
    }
}
