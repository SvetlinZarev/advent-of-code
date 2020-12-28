use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::parsing::parse_line_delimited;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 10;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let input = parse_line_delimited(&input);

    let (d1a, _) = measure(DAY, "part 1: v1", || part_one::solve_v1(&mut input.clone()));
    let (d1b, _) = measure(DAY, "part 1: v2", || part_one::solve_v2(&mut input.clone()));

    let (d2a, _) = measure(DAY, "part 2: O(N) mem", || {
        part_two::solve_v1(&mut input.clone())
    });
    let (d2b, _) = measure(DAY, "part 2: O(1) mem", || {
        part_two::solve_v2_const_mem(&mut input.clone())
    });

    d1a.min(d1b).add(d2a.min(d2b))
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let input = parse_line_delimited(&input);

        let solution = part_one::solve_v1(&mut input.clone());
        assert_eq!(Some(2343), solution);

        let solution = part_one::solve_v1(&mut input.clone());
        assert_eq!(Some(2343), solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let input = parse_line_delimited(&input);

        let solution = part_two::solve_v1(&mut input.clone());
        assert_eq!(Some(31581162962944), solution);

        let solution = part_two::solve_v2_const_mem(&mut input.clone());
        assert_eq!(Some(31581162962944), solution);
    }
}
