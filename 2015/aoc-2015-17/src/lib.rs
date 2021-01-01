use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::parsing::parse_line_delimited;
use aoc_2015_common::timing::measure;

pub const DAY: usize = 17;
const TARGET: usize = 150;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let (dp, parsed) = measure(DAY, "parsing", || parse_line_delimited(&input));
    let (d1, _) = measure(DAY, "part 1", || solve_part_one(&parsed, TARGET));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(&parsed, TARGET));

    dp + d1 + d2
}

fn solve_part_one(containers: &[usize], target_liters: usize) -> usize {
    let cols = containers.len() + 1;
    let counts = count_containers(containers, target_liters);

    // The columns represent the number of containers.  As we are interested
    // in how many ways we can store the eggnog regardless of the number of
    // containers, we have to sum in how many ways we can do it with 0
    // containers, with 1 container, ..with 20 containers, etc
    counts.iter().copied().skip(target_liters * cols).sum()
}

fn solve_part_two(containers: &[usize], target_liters: usize) -> usize {
    let cols = containers.len() + 1;
    let counts = count_containers(containers, target_liters);

    // The columns represent the number of containers. As we are interested
    // in the minimum number of containers, we must find the minimum column
    // index which has a non-zero value of combinations
    counts
        .iter()
        .copied()
        .enumerate()
        // Skip the columns which represent less than 150 liters
        .skip(target_liters * cols)
        .filter(|&(_, x)| x > 0)
        .min_by(|&(ia, _), (ib, _)| ia.cmp(ib))
        .map(|(_, value)| value)
        .unwrap()
}

fn count_containers(containers: &[usize], target_liters: usize) -> Vec<usize> {
    // Each column represents a number of containers: 0 containers, 1, 2,... containers.len() inclusive
    let cols = containers.len() + 1;

    // Each rows represents an amount of eggnog in liters: 0 liters, 1, 2, ... 150 liters inclusive
    let rows = target_liters + 1;

    let mut cache = vec![0; rows * cols];
    // The base case: there is only one way to store 0 liters with 0 containers
    cache[0 * cols + 0] = 1;

    for &container in containers {
        // The counter should go from larger to lower value, otherwise it will
        // count the container multiple times
        for liters_remaining in (0..=target_liters - container).rev() {
            for n in 0..containers.len() {
                let r_idx = (liters_remaining + container) * cols;
                let prev_r_idx = liters_remaining * cols;

                // As we might have already computed some value, we have to use
                // += in order to not lose the previous computation
                cache[r_idx + n + 1] += cache[prev_r_idx + n];
            }
        }
    }

    cache
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_line_delimited(&input);
        let answer = solve_part_one(&parsed, TARGET);
        assert_eq!(654, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_line_delimited(&input);
        let answer = solve_part_two(&parsed, TARGET);
        assert_eq!(57, answer);
    }
}
