use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Dir {
    N,
    S,
    NE,
    NW,
    SE,
    SW,
}

impl FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "n" => Dir::N,
            "s" => Dir::S,
            "ne" => Dir::NE,
            "nw" => Dir::NW,
            "se" => Dir::SE,
            "sw" => Dir::SW,
            _ => return Err(format!("invalid direction: {}", s)),
        })
    }
}

pub fn part_one(input: &[Dir]) -> u32 {
    let (mut r, mut c) = (0i32, 0i32);
    for dir in input {
        match dir {
            Dir::N => r -= 2,
            Dir::S => r += 2,
            Dir::NE => {
                r -= 1;
                c += 1;
            }
            Dir::NW => {
                r -= 1;
                c -= 1;
            }
            Dir::SE => {
                r += 1;
                c += 1;
            }
            Dir::SW => {
                r += 1;
                c -= 1;
            }
        }
    }

    dist(0, 0, r, c)
}

pub fn part_two(input: &[Dir]) -> u32 {
    let (mut r, mut c) = (0i32, 0i32);
    let mut answer = 0;

    for dir in input {
        match dir {
            Dir::N => r -= 2,
            Dir::S => r += 2,
            Dir::NE => {
                r -= 1;
                c += 1;
            }
            Dir::NW => {
                r -= 1;
                c -= 1;
            }
            Dir::SE => {
                r += 1;
                c += 1;
            }
            Dir::SW => {
                r += 1;
                c -= 1;
            }
        }

        answer = answer.max(dist(0, 0, r, c))
    }

    answer
}

// Distance on "doubled rows" coordinate system: https://www.redblobgames.com/grids/hexagons/#distances
fn dist(r0: i32, c0: i32, r1: i32, c1: i32) -> u32 {
    let dc = c0.abs_diff(c1);
    let dr = r0.abs_diff(r1);

    dc + dr.saturating_sub(dc) / 2
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_csv;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_csv(input);

        let answer = part_one(&parsed);
        assert_eq!(805, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_csv(input);

        let answer = part_two(&parsed);
        assert_eq!(1535, answer);
    }
}
