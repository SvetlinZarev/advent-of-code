use std::str::FromStr;
use aoc_shared::input::stdin_line_delimited;

#[derive(Debug, Copy, Clone)]
enum Direction {
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
                    Ok(v) => v
                };

                match dir {
                    "up" => Ok(Direction::Up(value)),
                    "down" => Ok(Direction::Down(value)),
                    "forward" => Ok(Direction::Forward(value)),
                    _ => Err(format!("invalid direction: {}", dir))
                }
            }
        }
    }
}

fn main() {
    let input: Vec<Direction> = stdin_line_delimited();

    let mut f = 0;
    let mut d = 0;

    for dir in input.iter().copied() {
        match dir {
            Direction::Up(v) => d -= v,
            Direction::Down(v) => d += v,
            Direction::Forward(v) => f += v,
        }
    }
    println!("Part 1: {}", f * d);

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
    println!("Part 2: {}", f * d);
}
