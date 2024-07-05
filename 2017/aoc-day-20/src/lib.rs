use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use std::sync::LazyLock;

use regex::Regex;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$"#).unwrap()
});

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct P3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl P3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn manhattan(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Add for P3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for P3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for P3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for P3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for P3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div for P3D {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<i64> for P3D {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<i64> for P3D {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

pub fn parse_input(input: &str) -> (Vec<P3D>, Vec<P3D>, Vec<P3D>) {
    let mut positions = vec![];
    let mut velocities = vec![];
    let mut accelerations = vec![];

    for line in input.lines() {
        let Some(captures) = REGEX.captures(line.trim_end()) else {
            panic!("failed to parse line: {:?}", line);
        };

        let p = P3D::new(
            captures[1].parse().unwrap(),
            captures[2].parse().unwrap(),
            captures[3].parse().unwrap(),
        );

        let v = P3D::new(
            captures[4].parse().unwrap(),
            captures[5].parse().unwrap(),
            captures[6].parse().unwrap(),
        );

        let a = P3D::new(
            captures[7].parse().unwrap(),
            captures[8].parse().unwrap(),
            captures[9].parse().unwrap(),
        );

        positions.push(p);
        velocities.push(v);
        accelerations.push(a);
    }

    (positions, velocities, accelerations)
}

pub fn part_one(p: &[P3D], v: &[P3D], a: &[P3D]) -> usize {
    const T: i64 = i32::MAX as i64;

    let min_accel = a
        .iter()
        .map(|x| x.manhattan(&P3D::default()))
        .min()
        .unwrap();

    let mut min_dist = i64::MAX;
    let mut answer = 0;

    a.iter()
        .map(|x| x.manhattan(&P3D::default()))
        .enumerate()
        .filter(|&(_idx, val)| val == min_accel)
        .map(|(idx, _val)| idx)
        .for_each(|idx| {
            // p(t) = p + v*t + (a/2)*t²
            let distance = p[idx] + v[idx] * T + a[idx] * T.pow(2) / 2;

            if distance.manhattan(&P3D::default()) < min_dist {
                min_dist = distance.manhattan(&P3D::default());
                answer = idx;
            }
        });

    answer
}

pub fn part_two(p: &[P3D], v: &[P3D], a: &[P3D]) -> usize {
    const ITER_WAIT_FOR_COLLISION: i32 = 100;

    assert_eq!(p.len(), v.len());
    assert_eq!(v.len(), a.len());

    let mut p = p.to_owned();
    let mut v = v.to_owned();
    let mut a = a.to_owned();

    let mut prev_len = p.len();
    let mut counter = 0;

    while prev_len != p.len() || counter < ITER_WAIT_FOR_COLLISION {
        for idx in 0..v.len() {
            v[idx] += a[idx];
        }

        let mut seen = HashMap::new();
        for idx in 0..v.len() {
            p[idx] += v[idx];
            seen.entry(p[idx]).or_insert(HashSet::new()).insert(idx);
        }

        let to_remove = seen
            .into_values()
            .filter(|x| x.len() > 1)
            .flatten()
            .collect::<HashSet<_>>();

        p = p
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| !to_remove.contains(idx))
            .map(|(_, val)| val)
            .collect();

        v = v
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| !to_remove.contains(idx))
            .map(|(_, val)| val)
            .collect();

        a = a
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| !to_remove.contains(idx))
            .map(|(_, val)| val)
            .collect();

        if prev_len != p.len() {
            prev_len = p.len();
            counter = 0;
        } else {
            counter += 1;
        }
    }

    p.len()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (p, v, a) = parse_input(&input);

        let answer = part_one(&p, &v, &a);

        assert_eq!(308, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (p, v, a) = parse_input(&input);

        let answer = part_two(&p, &v, &a);

        assert_eq!(504, answer);
    }
}
