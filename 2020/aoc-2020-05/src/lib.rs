use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 5;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d_1, _) = measure(DAY, "part 1", || part_one::solve(input.as_bytes()));

    measure(DAY, "part 2: naive", || {
        part_two::solve_v1(input.as_bytes())
    });
    let (d_2, _) = measure(DAY, "part 2: xor", || {
        part_two::solve_v2_xor(input.as_bytes())
    });
    measure(DAY, "part 2: bitwise", || {
        part_two::solve_v3_bitwise(input.as_bytes())
    });

    d_1.add(d_2)
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let solution = part_one::solve(input.as_bytes());
        assert_eq!(906, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));

        let solution = part_two::solve_v1(input.as_bytes());
        assert_eq!(Some(519), solution);

        let solution = part_two::solve_v2_xor(input.as_bytes());
        assert_eq!(519, solution);

        let solution = part_two::solve_v3_bitwise(input.as_bytes());
        assert_eq!(519, solution);
    }
}
