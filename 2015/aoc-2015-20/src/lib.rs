use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

const DAY: usize = 20;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let parsed = input.trim().parse().unwrap();

    let (d1, _) = measure(DAY, "part 1", || solve_part_one(parsed));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(parsed));

    d1 + d2
}

fn solve_part_one(presents: usize) -> usize {
    let mut houses = vec![0; presents / 10];

    for elf in 1..houses.len() {
        for house in (elf..houses.len()).step_by(elf) {
            houses[house] += elf;
        }
    }

    for (idx, p) in houses.iter().copied().enumerate() {
        if p >= presents / 10 {
            return idx;
        }
    }

    0
}

fn solve_part_two(presents: usize) -> usize {
    let mut houses = vec![0; presents / 11];

    for elf in 1..houses.len() {
        let max_house = houses.len().min(elf * 50 + 1);
        for house in (elf..max_house).step_by(elf) {
            houses[house] += elf;
        }
    }

    for (idx, p) in houses.iter().copied().enumerate() {
        if p >= presents / 11 {
            return idx;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let parsed = input.trim().parse().unwrap();
        let answer = solve_part_one(parsed);
        assert_eq!(831600, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let parsed = input.trim().parse().unwrap();
        let answer = solve_part_two(parsed);
        assert_eq!(884520, answer);
    }
}
