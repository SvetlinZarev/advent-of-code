use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use aoc_2020_common::parsing::parse_lines_as_i32;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-01.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let input = parse_lines_as_i32(&input);

    measure_solution(1, 1, "bruteforce", || {
        part_one::solve_bruteforce(&input) //
    });
    measure_solution(1, 1, "sorting", || {
        part_one::solve_with_sorting(&mut input.clone()) //
    });

    measure_solution(1, 2, "bruteforce", || {
        part_two::solve_with_bruteforce(&input) //
    });
    measure_solution(1, 2, "sorting", || {
        part_two::solve_with_quadratic_alg(&mut input.clone()) //
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let input = parse_lines_as_i32(&input);

        let solution = part_one::solve_bruteforce(&input);
        assert_eq!(Some(1020036), solution);

        let solution = part_one::solve_with_sorting(&mut input.clone());
        assert_eq!(Some(1020036), solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let input = parse_lines_as_i32(&input);

        let solution = part_two::solve_with_bruteforce(&input);
        assert_eq!(Some(286977330), solution);

        let solution = part_two::solve_with_quadratic_alg(&mut input.clone());
        assert_eq!(Some(286977330), solution);
    }
}
