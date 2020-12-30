use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

mod part_one;
mod part_two;

pub const DAY: usize = 14;
const SECONDS: u32 = 2503;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let (dp, parsed) = measure(DAY, "parsing", || parse_input(&input));
    let (d1, _) = measure(DAY, "part 1", || solve_part_one(&parsed));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(&parsed));

    dp + d1 + d2
}

pub fn solve_part_one(data: &[Reindeer]) -> u32 {
    part_one::solve(data)
}

pub fn solve_part_two(data: &[Reindeer]) -> u32 {
    part_two::solve(data)
}

#[derive(Debug, Copy, Clone)]
pub struct Reindeer {
    speed: u32,
    flight_duration: u32,
    rest_duration: u32,
}

pub fn parse_input(input: &str) -> Vec<Reindeer> {
    let mut reindeers = vec![];

    for line in input.lines() {
        let mut line = line;

        let mut end = line.find(" fly ").unwrap();
        line = &line[end + 5..];
        end = line.find(' ').unwrap();
        let speed = line[..end].parse().unwrap();

        line = &line[end + 10..];
        end = line.find(' ').unwrap();
        let fly = line[..end].parse().unwrap();

        end = line.rfind(' ').unwrap();
        line = &line[..end];
        end = line.rfind(' ').unwrap();
        let rest = line[end + 1..].parse().unwrap();

        reindeers.push(Reindeer {
            flight_duration: fly,
            rest_duration: rest,
            speed,
        })
    }

    reindeers
}

#[cfg(tests)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_input(&input);

        let answer = solve_part_one(&parsed);
        assert_eq!(2696, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_input(&input);

        let answer = solve_part_two(&parsed);
        assert_eq!(1084, answer);
    }
}
