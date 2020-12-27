use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 24;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (dp, tile_directions) = measure(DAY, "parsing", || parse_input(&input));

    let mut tiles = None;
    let (d1, _) = measure(DAY, "part 1", || {
        let (solution, part_two_input) = part_one::solve(&tile_directions);
        tiles = Some(part_two_input);
        solution
    });

    let (d2, _) = measure(DAY, "part 2", || part_two::solve(&tiles.unwrap()));

    dp.add(d1).add(d2)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn flip(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    E,
    W,
    SE,
    SW,
    NE,
    NW,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coordinate {
    q: i32,
    r: i32,
}

impl Coordinate {
    pub fn new(q: i32, r: i32) -> Coordinate {
        Coordinate { q, r }
    }

    pub fn on_direction(self, direction: Direction) -> Coordinate {
        match direction {
            Direction::E => Coordinate::new(self.q + 1, self.r),
            Direction::W => Coordinate::new(self.q - 1, self.r),
            Direction::SE => Coordinate::new(self.q, self.r + 1),
            Direction::SW => Coordinate::new(self.q - 1, self.r + 1),
            Direction::NE => Coordinate::new(self.q + 1, self.r - 1),
            Direction::NW => Coordinate::new(self.q, self.r - 1),
        }
    }

    pub fn iter(self) -> CoordinateIter {
        CoordinateIter::new(self)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CoordinateIter {
    base: Coordinate,
    next: Option<Direction>,
}

impl CoordinateIter {
    pub fn new(base: Coordinate) -> CoordinateIter {
        CoordinateIter {
            base,
            next: Some(Direction::W),
        }
    }
}

impl Iterator for CoordinateIter {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(direction) = self.next {
            self.next = match direction {
                Direction::W => Some(Direction::NW),
                Direction::NW => Some(Direction::NE),
                Direction::NE => Some(Direction::E),
                Direction::E => Some(Direction::SE),
                Direction::SE => Some(Direction::SW),
                Direction::SW => None,
            };

            return Some(self.base.on_direction(direction));
        }

        None
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    let mut tiles = vec![];

    for line in input.lines() {
        let line = line.as_bytes();
        let mut directions = vec![];

        let mut idx = 0;
        while idx < line.len() {
            let (dir, step) = if line.len() - idx > 1 {
                match (line[idx], line[idx + 1]) {
                    (b'n', b'e') => (Direction::NE, 2),
                    (b'n', b'w') => (Direction::NW, 2),
                    (b's', b'e') => (Direction::SE, 2),
                    (b's', b'w') => (Direction::SW, 2),
                    (b'e', _) => (Direction::E, 1),
                    (b'w', _) => (Direction::W, 1),
                    (x, y) => panic!("Unexpected pattern: {}{}", x as char, y as char),
                }
            } else {
                match line[idx] {
                    b'e' => (Direction::E, 1),
                    b'w' => (Direction::W, 1),
                    x => panic!("Unexpected pattern: {}", x as char),
                }
            };

            idx += step;
            directions.push(dir);
        }

        tiles.push(directions);
    }

    tiles
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_parse_input_sample_1() {
        let parsed = parse_input("esenee");
        assert_eq!(1, parsed.len());

        let parsed = &parsed[0];
        assert_eq!(4, parsed.len(), "{:?}", parsed);

        assert_eq!(Direction::E, parsed[0]);
        assert_eq!(Direction::SE, parsed[1]);
        assert_eq!(Direction::NE, parsed[2]);
        assert_eq!(Direction::E, parsed[3]);
    }

    #[test]
    fn test_parse_input_sample_2() {
        let parsed = parse_input("nwwswee");
        assert_eq!(1, parsed.len());

        let parsed = &parsed[0];
        assert_eq!(5, parsed.len(), "{:?}", parsed);

        assert_eq!(Direction::NW, parsed[0]);
        assert_eq!(Direction::W, parsed[1]);
        assert_eq!(Direction::SW, parsed[2]);
        assert_eq!(Direction::E, parsed[3]);
        assert_eq!(Direction::E, parsed[4]);
    }

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let tile_directions = parse_input(&input);

        let (solution, _) = part_one::solve(&tile_directions);
        assert_eq!(488, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let tile_directions = parse_input(&input);

        let (_, tiles) = part_one::solve(&tile_directions);
        let solution = part_two::solve(&tiles);
        assert_eq!(4118, solution);
    }
}
