use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
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
            },

            Direction::E => match rot {
                Rot::L(_) => Direction::N,
                Rot::R(_) => Direction::S,
            },

            Direction::S => match rot {
                Rot::L(_) => Direction::E,
                Rot::R(_) => Direction::W,
            },

            Direction::W => match rot {
                Rot::L(_) => Direction::S,
                Rot::R(_) => Direction::N,
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Rot {
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rot = &s[..1];
        let steps = s[1..]
            .parse()
            .map_err(|_| format!("cannot parse: {:?}", &s[1..]))?;

        return match rot {
            "L" => Ok(Rot::L(steps)),
            "R" => Ok(Rot::R(steps)),
            _ => Err(format!("Invalid rotation: {}", rot)),
        };
    }
}

pub fn part_one(input: &[Rot]) -> i32 {
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

pub fn part_two(input: &[Rot]) -> Option<i32> {
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
    use super::*;
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_csv;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_csv(&input);
        let answer = part_one(&parsed);

        assert_eq!(181, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_csv(&input);
        let answer = part_two(&parsed);

        assert_eq!(Some(140), answer);
    }
}
