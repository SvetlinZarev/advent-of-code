mod parsing;
mod part_one_naive;
mod solver;

use crate::solver::solve;
pub use parsing::{parse_input, Rule};
pub use part_one_naive::part_one_naive;

pub fn part_one(polymer: &str, rules: &[Rule]) -> u64 {
    solve(polymer, rules, 10)
}

pub fn part_two(polymer: &str, rules: &[Rule]) -> u64 {
    solve(polymer, rules, 40)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let (polymer, rules) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&polymer, &rules);
        assert_eq!(3411, answer);
    }

    #[test]
    fn test_part_two() {
        let (polymer, rules) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&polymer, &rules);
        assert_eq!(7477815755570, answer);
    }
}
