use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex; //67

lazy_static! {
    static ref REGEX_INFO: Regex = Regex::new(
        r#"^/dev/grid/node-x(?<x>\d+)-y(?<y>\d+)\s+\d+T\s+(?<used>\d+)T\s+(?<available>\d+)T\s+\d+%$"#
    )
    .unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Info {
    x: u32,
    y: u32,
    used: u32,
    available: u32,
}

impl FromStr for Info {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(cap) = REGEX_INFO.captures(s) else {
            return Err(format!("invalid input: {}", s));
        };

        let x = cap
            .name("x")
            .ok_or_else(|| format!("invalid input: {}", s))?
            .as_str();
        let y = cap
            .name("y")
            .ok_or_else(|| format!("invalid input: {}", s))?
            .as_str();
        let used = cap
            .name("used")
            .ok_or_else(|| format!("invalid input: {}", s))?
            .as_str();
        let avail = cap
            .name("available")
            .ok_or_else(|| format!("invalid input: {}", s))?
            .as_str();

        Ok(Info {
            x: x.parse().map_err(|_| format!("invalid input: {}", s))?,
            y: y.parse().map_err(|_| format!("invalid input: {}", s))?,
            used: used.parse().map_err(|_| format!("invalid input: {}", s))?,
            available: avail.parse().map_err(|_| format!("invalid input: {}", s))?,
        })
    }
}

pub fn part_one_v1(input: &[Info]) -> u32 {
    let mut pairs = 0;

    for i in 0..input.len() {
        let a = &input[i];
        if a.used == 0 {
            continue;
        }

        for j in 0..input.len() {
            if i == j {
                continue;
            }

            let b = &input[j];
            if a.used <= b.available {
                pairs += 1;
            }
        }
    }

    pairs
}

pub fn part_one_v2(input: &mut [Info]) -> usize {
    input.sort_unstable_by_key(|i| i.used);
    let with_no_used = input.partition_point(|info| info.used == 0);

    let mut pairs = 0;
    for idx in 0..input.len() {
        let info = &input[idx];

        let pp = input.partition_point(|x| x.used <= info.available);
        pairs += pp - with_no_used;
    }

    pairs
}

pub fn part_two(input: &[Info]) -> u32 {
    const TARGET: char = '🌟';
    const WALL: char = '🟥';
    const NODE: char = '🟩';
    const HOLE: char = '🟢';

    let max_x = input.iter().map(|info| info.x).max().unwrap() as usize;
    let max_y = input.iter().map(|info| info.y).max().unwrap() as usize;
    println!("ROWS: {}; COLS: {}", max_y + 1, max_x + 1);

    let mut grid = vec![vec![NODE; max_x + 1]; max_y + 1];
    for info in input.iter().copied() {
        grid[info.y as usize][info.x as usize] = match info.used {
            0 => HOLE,
            x if x > 99 => WALL,
            _ => NODE,
        }
    }
    grid[0][max_x] = TARGET;

    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }

    // The answer was found by manually counting the steps on the above visualization
    233
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_line_delimited_after_row;

    use super::*;

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_line_delimited_after_row(input, 2);
        let answer = part_one_v1(&input);
        assert_eq!(981, answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let mut input = parse_line_delimited_after_row(input, 2);
        let answer = part_one_v2(&mut input);
        assert_eq!(981, answer);
    }
}
