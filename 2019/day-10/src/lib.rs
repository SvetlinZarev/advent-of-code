use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct F64C(f64);
impl Eq for F64C {}
impl Ord for F64C {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord {
    r: u32,
    c: u32,
}

impl Coord {
    pub fn new(r: u32, c: u32) -> Coord {
        Coord { r, c }
    }

    pub fn angle(self, other: Coord) -> f64 {
        let dy = other.r as f64 - self.r as f64;
        let dx = other.c as f64 - self.c as f64;

        // Normalize atan2() to [0; 2*pi)
        (dy.atan2(dx) + std::f64::consts::TAU) % std::f64::consts::TAU
    }

    pub fn dist(self, other: Coord) -> f64 {
        let dr = (other.r as f64 - self.r as f64).powi(2);
        let dc = (other.c as f64 - self.c as f64).powi(2);

        (dr + dc).sqrt()
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.c, self.r)
    }
}

pub fn parse_input<S: AsRef<str>>(s: S) -> Vec<Coord> {
    let mut input = vec![];

    for (row, line) in s.as_ref().lines().enumerate() {
        for (col, &ch) in line.as_bytes().iter().enumerate() {
            if ch == b'#' {
                input.push(Coord::new(row.try_into().unwrap(), col.try_into().unwrap()));
            }
        }
    }

    input
}

pub fn part_one(input: &[Coord]) -> usize {
    select_best_asteroid(input)
        .map(|(_coord, count)| count)
        .unwrap_or(0)
}

pub fn part_two(input: &[Coord]) -> u32 {
    const ASTEROIDS_TO_DESTROY: u32 = 200;

    let station = match select_best_asteroid(input) {
        Some((coord, _count)) => coord,
        None => return 0,
    };

    let mut asteroids = asteroids_by_angle(station, input);

    let mut count = 0;
    let mut last = Coord::new(0, 0);

    'all: loop {
        // used to detect if there are no more asteroids and thus prevent infinite loop
        let start_count = count;

        for by_angle in asteroids.iter_mut() {
            match by_angle.pop() {
                None => continue,
                Some(x) => {
                    last = x;
                    count += 1;

                    if ASTEROIDS_TO_DESTROY == count {
                        break 'all;
                    }
                }
            }
        }

        // We did not destroy any asteroids -> break to prevent infinite loop
        if count == start_count {
            break;
        }
    }

    last.c * 100 + last.r
}

pub fn select_best_asteroid(input: &[Coord]) -> Option<(Coord, usize)> {
    if input.is_empty() {
        return None;
    }

    let mut angles = HashSet::new();
    let mut answer = 0;
    let mut best = input[0];

    for &a in input.iter() {
        angles.clear();

        for &b in input.iter() {
            if a == b {
                continue;
            }

            let angle = a.angle(b);

            // There can be only one visible asteroid on the same line
            // of sight (i.e. same angle between them), so count the
            // number of distinct angles for each asteroid
            angles.insert(angle.to_bits());
        }

        if angles.len() > answer {
            answer = angles.len();
            best = a;
        }
    }

    Some((best, answer))
}

fn asteroids_by_angle(from: Coord, asteroids: &[Coord]) -> Vec<Vec<Coord>> {
    let mut visible = HashMap::new();
    for &x in asteroids.iter() {
        if x == from {
            continue;
        }

        // rotate 90 degrees to the right, because 0 degrees is to the right,
        // but we need to start fom the top.
        let mut angle = from.angle(x).to_degrees();
        angle = (angle + 360.0 + 90.0) % 360.0;

        visible.entry(angle.to_bits()).or_insert(vec![]).push(x);
    }

    let mut angles = visible
        .keys()
        .map(|&x| f64::from_bits(x))
        .map(|x| F64C(x))
        .collect::<Vec<_>>();

    // The laser will be rotation clockwise, so sort in reverse order
    angles.sort_unstable_by(|x, y| x.cmp(y));

    let mut result = vec![];
    for F64C(angle) in angles {
        let mut coords = visible.remove(&angle.to_bits()).unwrap();
        coords.sort_unstable_by(|&x, &y| {
            let a = F64C(from.dist(x));
            let b = F64C(from.dist(y));
            a.cmp(&b).reverse()
        });
        result.push(coords);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared_2019::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);

        let answer = part_one(&parsed);
        assert_eq!(274, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);

        let answer = part_two(&parsed);
        assert_eq!(305, answer);
    }
}
