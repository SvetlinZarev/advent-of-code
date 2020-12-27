use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one_dfs;
pub mod part_one_recursive;
pub mod part_two;

pub const DAY: usize = 7;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d1ap, parsed) = measure(DAY, "parsing", || part_one_recursive::parse_input(&input));
    let (d1a, _) = measure(DAY, "part 1: recursive", || {
        part_one_recursive::solve_v1(&parsed)
    });
    let dur_v1 = d1a.add(d1ap);

    let (d1bp, parsed) = measure(DAY, "parsing", || part_one_dfs::parse_input(&input));
    let (d1b, _) = measure(DAY, "part 1: DFS", || part_one_dfs::solve_dfs(&parsed));
    let dur_v2 = d1b.add(d1bp);

    let (d2p, parsed) = measure(DAY, "parsing", || part_two::parse_input(&input));
    let (d2, _) = measure(DAY, "part 2", || part_two::solve(&parsed));

    d2.add(d2p).add(dur_v1.min(dur_v2))
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));

        let parsed = part_one_recursive::parse_input(&input);
        let solution = part_one_recursive::solve_v1(&parsed);
        assert_eq!(332, solution);

        let parsed = part_one_dfs::parse_input(&input);
        let solution = part_one_dfs::solve_dfs(&parsed);
        assert_eq!(332, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));

        let parsed = part_two::parse_input(&input);
        let solution = part_two::solve(&parsed);
        assert_eq!(10875, solution);
    }
}
