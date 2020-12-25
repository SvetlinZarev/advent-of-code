use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-05.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);

    measure_solution(5, 1, "", || {
        part_one::solve(input.as_bytes()) //
    });
    measure_solution(5, 2, "naive", || {
        part_two::solve_v1(input.as_bytes()) //
    });
    measure_solution(5, 2, "xor", || {
        part_two::solve_v2_xor(input.as_bytes()) //
    });
    measure_solution(5, 2, "bitwise", || {
        part_two::solve_v3_bitwise(input.as_bytes()) //
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let solution = part_one::solve(input.as_bytes());
        assert_eq!(906, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);

        let solution = part_two::solve_v1(input.as_bytes());
        assert_eq!(Some(519), solution);

        let solution = part_two::solve_v2_xor(input.as_bytes());
        assert_eq!(519, solution);

        let solution = part_two::solve_v3_bitwise(input.as_bytes());
        assert_eq!(519, solution);
    }
}
