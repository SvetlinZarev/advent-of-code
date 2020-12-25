use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use aoc_2020_common::parsing::parse_lines_as_usize;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-10.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let input = parse_lines_as_usize(&input);

    measure_solution(10, 1, "v1", || part_one::solve_v1(&mut input.clone()));
    measure_solution(10, 1, "v2", || part_one::solve_v2(&mut input.clone()));

    measure_solution(10, 2, "O(N) mem", || {
        part_two::solve_v1(&mut input.clone()) //
    });
    measure_solution(10, 2, "O(1) mem", || {
        part_two::solve_v2_const_mem(&mut input.clone()) //
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let input = parse_lines_as_usize(&input);

        let solution = part_one::solve_v1(&mut input.clone());
        assert_eq!(Some(2343), solution);

        let solution = part_one::solve_v1(&mut input.clone());
        assert_eq!(Some(2343), solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let input = parse_lines_as_usize(&input);

        let solution = part_two::solve_v1(&mut input.clone());
        assert_eq!(Some(31581162962944), solution);

        let solution = part_two::solve_v2_const_mem(&mut input.clone());
        assert_eq!(Some(31581162962944), solution);
    }
}
