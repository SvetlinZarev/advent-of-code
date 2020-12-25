use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-12.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);

    let commands = part_one::parse_input_data(&input);
    measure_solution(12, 1, "", || part_one::solve(&commands));

    let directions = part_two::parse_input_data(&input);
    measure_solution(12, 2, "", || part_two::solve(&directions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);

        let commands = part_one::parse_input_data(&input);
        let solution = part_one::solve(&commands);
        assert_eq!(508, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);

        let directions = part_two::parse_input_data(&input);
        let solution = part_two::solve(&directions);
        assert_eq!(30761, solution);
    }
}
