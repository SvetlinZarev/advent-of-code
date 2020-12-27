use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 12;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d1p, commands) = measure(DAY, "parsing", || part_one::parse_input_data(&input));
    let (d1, _) = measure(DAY, "part 1", || part_one::solve(&commands));

    let (d2p, directions) = measure(DAY, "parsing", || part_two::parse_input_data(&input));
    let (d2, _) = measure(DAY, "part 2", || part_two::solve(&directions));

    d1.add(d1p).add(d2).add(d2p)
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));

        let commands = part_one::parse_input_data(&input);
        let solution = part_one::solve(&commands);
        assert_eq!(508, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));

        let directions = part_two::parse_input_data(&input);
        let solution = part_two::solve(&directions);
        assert_eq!(30761, solution);
    }
}
