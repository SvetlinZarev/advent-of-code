use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-17.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);

    let mut data = part_one::parse_input(&input);
    measure_solution(17, 1, "", || part_one::solve(&mut data));

    let mut data = part_two::parse_input(&input);
    measure_solution(17, 2, "", || part_two::solve(&mut data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);

        let mut data = part_one::parse_input(&input);
        let solution = part_one::solve(&mut data);
        assert_eq!(240, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);

        let mut data = part_two::parse_input(&input);
        let solution = part_two::solve(&mut data);
        assert_eq!(1180, solution);
    }
}
