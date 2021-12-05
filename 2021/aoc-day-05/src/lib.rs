use crate::fnvhash::{FnvHasher, HashBuilder};
use std::collections::HashMap;
use std::str::FromStr;

mod fnvhash;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    pub fn is_straight(&self) -> bool {
        self.a.x == self.b.x || self.a.y == self.b.y
    }

    pub fn steps(&self) -> usize {
        let x = self.a.x.max(self.b.x) - self.a.x.min(self.b.x);
        let y = self.a.y.max(self.b.y) - self.a.y.min(self.b.y);
        x.max(y) as usize
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(" -> ")
            .unwrap_or_else(|| panic!("invalid input (missing points separator): {}", s));

        let first = a
            .split_once(',')
            .and_then(|(x, y)| {
                let x = x.parse().ok()?;
                let y = y.parse().ok()?;
                Some((x, y))
            })
            .and_then(|(x, y)| Some(Point::new(x, y)));

        let first = match first {
            None => return Err(format!("failed to parse first point: {:?}", a)),
            Some(point) => point,
        };

        let second = b
            .split_once(',')
            .and_then(|(x, y)| {
                let x = x.parse().ok()?;
                let y = y.parse().ok()?;
                Some((x, y))
            })
            .and_then(|(x, y)| Some(Point::new(x, y)));

        let second = match second {
            None => return Err(format!("failed to parse first point: {:?}", b)),
            Some(point) => point,
        };

        Ok(Line::new(first, second))
    }
}

pub fn part_one(input: &[Line]) -> usize {
    let mut field = HashMap::with_hasher(HashBuilder::<FnvHasher>::default());

    input.iter().filter(|&l| l.is_straight()).for_each(|l| {
        let (mut x, mut y) = (l.a.x, l.a.y);

        for _ in 0..=l.steps() {
            field.entry((x, y)).and_modify(|v| *v += 1).or_insert(1u32);

            x += (x < l.b.x) as u16;
            x -= (x > l.b.x) as u16;

            y += (y < l.b.y) as u16;
            y -= (y > l.b.y) as u16;
        }
    });

    field.values().filter(|&&v| v > 1).count()
}

pub fn part_two(input: &[Line]) -> usize {
    let mut field = HashMap::with_hasher(HashBuilder::<FnvHasher>::default());

    input.iter().for_each(|l| {
        let (mut x, mut y) = (l.a.x, l.a.y);

        for _ in 0..=l.steps() {
            field.entry((x, y)).and_modify(|v| *v += 1).or_insert(1u32);

            x += (x < l.b.x) as u16;
            x -= (x > l.b.x) as u16;

            y += (y < l.b.y) as u16;
            y -= (y > l.b.y) as u16;
        }
    });

    field.values().filter(|&&v| v > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(6267, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(20196, answer);
    }
}
