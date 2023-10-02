use std::iter::once;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_DISC: Regex =
        Regex::new(r#"^Disc #(?<id>\d+) has (?<positions>\d+) positions; at time=(?<time>\d+), it is at position (?<position>\d+)\.$"#).unwrap();
}

#[derive(Debug, Copy, Clone)]
pub struct Disc {
    positions: u32,
    position: u32,
}

impl FromStr for Disc {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(cap) = REGEX_DISC.captures(s) else {
            return Err(format!(
                "The string {:?} does not match the Disc regex: {}",
                s,
                REGEX_DISC.as_str()
            ));
        };

        let positions = cap
            .name("positions")
            .ok_or_else(|| {
                format!(
                    "The string {:?} does not match the Disc regex: {}",
                    s,
                    REGEX_DISC.as_str()
                )
            })?
            .as_str()
            .parse()
            .expect("failed to parse number");

        let position = cap
            .name("position")
            .ok_or_else(|| {
                format!(
                    "The string {:?} does not match the Disc regex: {}",
                    s,
                    REGEX_DISC.as_str()
                )
            })?
            .as_str()
            .parse()
            .expect("failed to parse number");

        Ok(Disc {
            positions,
            position,
        })
    }
}

pub fn part_one(input: &[Disc]) -> u32 {
    let mut time = 0;
    loop {
        let mut increment = 1;
        let mut discs = input.len();

        for (delay, disc) in input.iter().copied().enumerate() {
            if (disc.position + time + 1 + delay as u32) % disc.positions != 0 {
                break;
            }

            // works, because the positions are co-prime
            increment *= disc.positions;
            discs -= 1;
        }

        if discs == 0 {
            break time;
        }

        time += increment;
    }
}

pub fn part_two(input: &[Disc]) -> u32 {
    let mut time = 0;
    let total_discs = input.len() + 1;

    loop {
        let input = input
            .iter()
            .copied()
            .chain(once(Disc {
                positions: 11,
                position: 0,
            }))
            .enumerate();

        let mut increment = 1;
        let mut discs = total_discs;

        for (delay, disc) in input {
            if (disc.position + time + 1 + delay as u32) % disc.positions != 0 {
                break;
            }

            // works, because the positions are co-prime
            increment *= disc.positions;
            discs -= 1;
        }

        if discs == 0 {
            break time;
        }

        time += increment;
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(16824, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(3543984, answer);
    }
}
