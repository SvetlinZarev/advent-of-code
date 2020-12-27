use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_two;

pub const DAY: usize = 4;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d_2, _) = measure(DAY, "part 2: with nom", || part_two::solve(&input));
    d_2
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let solution = part_two::solve(&input);
        assert_eq!(188, solution);
    }
}
