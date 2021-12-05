use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.rfind(' ') {
            None => Err(format!("missing separator: {}", s)),
            Some(idx) => {
                let (dir, val) = s.split_at(idx);
                let value = match val[1..].parse() {
                    Err(_) => return Err(format!("invalid value: {}", val)),
                    Ok(v) => v,
                };

                match dir {
                    "up" => Ok(Direction::Up(value)),
                    "down" => Ok(Direction::Down(value)),
                    "forward" => Ok(Direction::Forward(value)),
                    _ => Err(format!("invalid direction: {}", dir)),
                }
            }
        }
    }
}

pub fn part_one(input: &[Direction]) -> i32 {
    let mut f = 0;
    let mut d = 0;

    for dir in input.iter().copied() {
        match dir {
            Direction::Up(v) => d -= v,
            Direction::Down(v) => d += v,
            Direction::Forward(v) => f += v,
        }
    }
    f * d
}

pub fn part_two(input: &[Direction]) -> i32 {
    let mut a = 0;
    let mut f = 0;
    let mut d = 0;

    for dir in input {
        match dir {
            Direction::Up(v) => a -= v,
            Direction::Down(v) => a += v,
            Direction::Forward(v) => {
                f += v;
                d += a * v;
            }
        }
    }

    f * d
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(1499229, answer)
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(1340836560, answer)
    }
}
