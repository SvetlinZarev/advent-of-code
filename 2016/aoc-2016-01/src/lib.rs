use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use aoc_2016_common::input::load_input;
use aoc_2016_common::parsing::parse_csv;
use aoc_2016_common::timing::measure;

const DAY: usize = 1;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let (dp, inst) = measure(DAY, "parsing", || parse_csv(&input));
    let (d1, _) = measure(DAY, "part 1", || solve_part_one(&inst));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(&inst));


    dp + d1 + d2
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn rotate(self, rot: Rot) -> Self {
        match self {
            Direction::N => match rot {
                Rot::L(_) => Direction::W,
                Rot::R(_) => Direction::E,
            }

            Direction::E => match rot {
                Rot::L(_) => Direction::N,
                Rot::R(_) => Direction::S,
            }

            Direction::S => match rot {
                Rot::L(_) => Direction::E,
                Rot::R(_) => Direction::W,
            }

            Direction::W => match rot {
                Rot::L(_) => Direction::S,
                Rot::R(_) => Direction::N,
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Rot {
    L(i32),
    R(i32),
}

impl Rot {
    fn steps(self) -> i32 {
        match self {
            Rot::L(s) => s,
            Rot::R(s) => s,
        }
    }
}

impl FromStr for Rot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rot = &s[..1];
        let steps = s[1..].parse()?;

        return match rot {
            "L" => Ok(Rot::L(steps)),
            "R" => Ok(Rot::R(steps)),
            _ => Err(anyhow::anyhow!("Invalid rotation: {}", rot)),
        };
    }
}

fn solve_part_one(input: &[Rot]) -> i32 {
    let mut direction = Direction::N;
    let (mut north, mut east) = (0, 0);


    for inst in input.iter().copied() {
        direction = direction.rotate(inst);
        match direction {
            Direction::N => north += inst.steps(),
            Direction::E => east += inst.steps(),
            Direction::S => north -= inst.steps(),
            Direction::W => east -= inst.steps(),
        }
    }

    north.abs() + east.abs()
}

fn solve_part_two(input: &[Rot]) -> Option<i32> {
    let mut direction = Direction::N;
    let (mut north, mut east) = (0i32, 0i32);
    let mut visited = HashSet::new();

    for inst in input.iter().copied() {
        direction = direction.rotate(inst);
        match direction {
            Direction::N => {
                for s in 1..inst.steps() {
                    if !visited.insert((north + s, east)) {
                        return Some((north + s).abs() + east.abs());
                    }
                }
                north += inst.steps();
            }
            Direction::E => {
                for s in 1..inst.steps() {
                    if !visited.insert((north, east + s)) {
                        return Some(north.abs() + (east + s).abs());
                    }
                }
                east += inst.steps();
            }
            Direction::S => {
                for s in 1..inst.steps() {
                    if !visited.insert((north - s, east)) {
                        return Some((north - s).abs() + east.abs());
                    }
                }
                north -= inst.steps();
            }
            Direction::W => {
                for s in 1..inst.steps() {
                    if !visited.insert((north, east - s)) {
                        return Some(north.abs() + (east - s).abs());
                    }
                }
                east -= inst.steps();
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use aoc_2016_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_csv(&input);
        let answer = solve_part_one(&parsed);

        assert_eq!(181, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_csv(&input);
        let answer = solve_part_two(&parsed);

        assert_eq!(Some(140), answer);
    }
}
