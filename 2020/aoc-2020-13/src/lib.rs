use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-13.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let raw_input = load_input(path);

    let (arrival_time, schedule) = part_one::parse_input_data(&raw_input);
    measure_solution(13, 1, "", || part_one::solve(arrival_time, &schedule));

    let schedule = part_two::parse_input_data(&raw_input);
    measure_solution(13, 2, "", || part_two::solve(&schedule));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let raw_input = load_input(DEFAULT_INPUT_PATH);

        let (arrival_time, schedule) = part_one::parse_input_data(&raw_input);
        let solution = part_one::solve(arrival_time, &schedule);
        assert_eq!(2215, solution);
    }

    #[test]
    fn test_part_two() {
        let raw_input = load_input(DEFAULT_INPUT_PATH);

        let schedule = part_two::parse_input_data(&raw_input);
        let solution = part_two::solve(&schedule);
        assert_eq!(1058443396696792, solution);
    }
}
