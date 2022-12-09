use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    L(i32),
    R(i32),
    U(i32),
    D(i32),
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s[2..]
            .parse()
            .map_err(|_| format!("cannot parse steps: {}", s))?;

        match &s[..1] {
            "L" => Ok(Direction::L(steps)),
            "R" => Ok(Direction::R(steps)),
            "U" => Ok(Direction::U(steps)),
            "D" => Ok(Direction::D(steps)),
            _ => Err(format!("invalid direction: {}", s)),
        }
    }
}

pub fn part_one(input: &[Direction]) -> usize {
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let (mut tr, mut tc) = (0, 0);
    let (mut hr, mut hc) = (0, 0);

    for d in input.iter().copied() {
        match d {
            Direction::L(s) => {
                for _ in 0..s {
                    hc -= 1;
                    (tr, tc) = move_tail(hr, hc, tr, tc);
                    visited.insert((tr, tc));
                }
            }

            Direction::R(s) => {
                for _ in 0..s {
                    hc += 1;
                    (tr, tc) = move_tail(hr, hc, tr, tc);
                    visited.insert((tr, tc));
                }
            }

            Direction::U(s) => {
                for _ in 0..s {
                    hr += 1;
                    (tr, tc) = move_tail(hr, hc, tr, tc);
                    visited.insert((tr, tc));
                }
            }

            Direction::D(s) => {
                for _ in 0..s {
                    hr -= 1;
                    (tr, tc) = move_tail(hr, hc, tr, tc);
                    visited.insert((tr, tc));
                }
            }
        }
    }

    visited.len()
}

pub fn part_two(input: &[Direction]) -> usize {
    let mut rope = [(0, 0); 10];

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for d in input.iter().copied() {
        match d {
            Direction::L(s) => {
                for _ in 0..s {
                    rope[0].1 -= 1;
                    move_rope(&mut rope, &mut visited);
                }
            }

            Direction::R(s) => {
                for _ in 0..s {
                    rope[0].1 += 1;
                    move_rope(&mut rope, &mut visited);
                }
            }

            Direction::U(s) => {
                for _ in 0..s {
                    rope[0].0 += 1;
                    move_rope(&mut rope, &mut visited);
                }
            }

            Direction::D(s) => {
                for _ in 0..s {
                    rope[0].0 -= 1;
                    move_rope(&mut rope, &mut visited);
                }
            }
        }
    }

    visited.len()
}

fn move_rope(rope: &mut [(i32, i32); 10], visited: &mut HashSet<(i32, i32)>) {
    let old_tail = rope[9];

    for h in 0..9 {
        let (hr, hc) = rope[h + 0];
        let (tr, tc) = rope[h + 1];

        let (rx, cx) = move_tail(hr, hc, tr, tc);
        if (rx, cx) == (tr, tc) {
            break;
        }

        rope[h + 1] = (rx, cx);
    }

    if old_tail != rope[9] {
        visited.insert(rope[9]);
    }
}

fn move_tail(hr: i32, hc: i32, mut tr: i32, mut tc: i32) -> (i32, i32) {
    if hc.abs_diff(tc) > 1 || hr.abs_diff(tr) > 1 {
        tr += (hr - tr).signum();
        tc += (hc - tc).signum();
    }

    (tr, tc)
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(6197, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(2562, answer);
    }
}
