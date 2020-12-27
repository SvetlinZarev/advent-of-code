use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 13;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let raw_input = load_input(path);

    let (arrival_time, schedule) = part_one::parse_input_data(&raw_input);
    let (d1, _) = measure(DAY, "part 1", || part_one::solve(arrival_time, &schedule));

    let schedule = part_two::parse_input_data(&raw_input);
    let (d2, _) = measure(DAY, "part 2", || part_two::solve(&schedule));

    d1.add(d2)
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let raw_input = load_input(default_test_input(DAY));

        let (arrival_time, schedule) = part_one::parse_input_data(&raw_input);
        let solution = part_one::solve(arrival_time, &schedule);
        assert_eq!(2215, solution);
    }

    #[test]
    fn test_part_two() {
        let raw_input = load_input(default_test_input(DAY));

        let schedule = part_two::parse_input_data(&raw_input);
        let solution = part_two::solve(&schedule);
        assert_eq!(1058443396696792, solution);
    }
}
