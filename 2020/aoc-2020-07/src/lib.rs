use std::path::Path;

use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;

pub mod part_one_dfs;
pub mod part_one_recursive;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-07.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);

    let parsed = part_one_recursive::parse_input(&input);
    measure_solution(7, 1, "recursive", || part_one_recursive::solve_v1(&parsed));

    let parsed = part_one_dfs::parse_input(&input);
    measure_solution(7, 1, "DFS", || part_one_dfs::solve_dfs(&parsed));

    let parsed = part_two::parse_input(&input);
    measure_solution(7, 2, "", || part_two::solve(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);

        let parsed = part_one_recursive::parse_input(&input);
        let solution = part_one_recursive::solve_v1(&parsed);
        assert_eq!(332, solution);

        let parsed = part_one_dfs::parse_input(&input);
        let solution = part_one_dfs::solve_dfs(&parsed);
        assert_eq!(332, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);

        let parsed = part_two::parse_input(&input);
        let solution = part_two::solve(&parsed);
        assert_eq!(10875, solution);
    }
}
