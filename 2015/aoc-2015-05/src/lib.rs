use std::path::Path;

use aoc_2015_common::input::load_input;
use aoc_2015_common::output::measure_solution;

pub mod part_one;
pub mod part_two;

pub const DAY: &'static str = "day-05";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);

    measure_solution(5, 1, "", || part_one::solve(&input));
    measure_solution(5, 2, "", || part_two::solve(&input));
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let solution = part_one::solve(&input);
        assert_eq!(236, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let solution = part_two::solve(&input);
        assert_eq!(51, solution);
    }
}
