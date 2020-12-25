use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use aoc_2020_common::parsing::parse_lines_as_usize;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-22.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let (first, second) = parse_input(&input);

    measure_solution(22, 1, "", || part_one::solve(&first, &second));
    measure_solution(22, 2, "", || part_two::solve(&first, &second));
}

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let separator = input.find("\n\n").unwrap();
    let (a, b) = input.split_at(separator);

    let player_one = parse_lines_as_usize(a[9..].trim());
    let player_two = parse_lines_as_usize(b[9 + 2..].trim());

    (player_one, player_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let (a, b) = parse_input(&input);

        let solution = part_one::solve(&a, &b);
        assert_eq!(32401, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let (a, b) = parse_input(&input);

        let solution = part_two::solve(&a, &b);
        assert_eq!(31436, solution);
    }
}
