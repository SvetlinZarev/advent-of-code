use std::path::Path;
use std::time::Duration;

use aoc_2019_common::input::load_input;
use aoc_2019_common::parsing::parse_line_delimited;
use aoc_2019_common::timing::measure;

pub const DAY: usize = 1;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let input = parse_line_delimited(&input);

    let (d_1, _) = measure(DAY, "part 1", || part_one::solve(&input));
    let (d_2, _) = measure(DAY, "part 2", || part_two::solve(&input));

    d_1 + d_2
}

mod part_one {
    pub fn solve(input: &[usize]) -> usize {
        input.iter().map(|m| (m / 3).saturating_sub(2)).sum()
    }
}

mod part_two {
    pub fn solve(input: &[usize]) -> usize {
        let mut fuel = 0;
        for mass in input.iter().copied() {
            let mut additional_mass = mass;
            while additional_mass > 0 {
                additional_mass = (additional_mass / 3).saturating_sub(2);
                fuel += additional_mass;
            }
        }
        fuel
    }
}

#[cfg(test)]
mod tests {
    use aoc_2019_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let input = parse_line_delimited(&input);

        assert_eq!(3427947, part_one::solve(&input));
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let input = parse_line_delimited(&input);

        assert_eq!(5139037, part_two::solve(&input));
    }
}
