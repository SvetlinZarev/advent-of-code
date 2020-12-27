use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::parsing::parse_lines_as_u64;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const PART_ONE_KEY: u64 = 507622668;

pub const DAY: usize = 9;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let input = parse_lines_as_u64(&input);

    let (d1, key) = measure(DAY, "part 1", || part_one::solve(&input));

    let mut d2 = Duration::default();
    if let Some(key) = key {
        let (d2x, _) = measure(DAY, "part 2", || part_two::solve(&input, key));
        d2 = d2x;
    }

    d1.add(d2)
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let input = parse_lines_as_u64(&input);

        let solution = part_one::solve(&input);
        assert_eq!(Some(507622668), solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let input = parse_lines_as_u64(&input);

        let solution = part_two::solve(&input, PART_ONE_KEY);
        assert_eq!(Some(76688505), solution);
    }
}
