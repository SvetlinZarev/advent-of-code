use std::collections::HashMap;
use std::str::FromStr;

use aoc_shared_2019::hashing::{FxHasher, HashBuilder};

type Map<K, V> = HashMap<K, V, HashBuilder<FxHasher>>;
type Grid = Map<u64, u64>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instr {
    L(i32),
    R(i32),
    U(i32),
    D(i32),
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, val) = s.split_at(1);
        let val = val
            .parse()
            .map_err(|e| format!("Failed to parse '{:?}': {:?}", val, e))?;

        Ok(match dir {
            "L" => Instr::L(val),
            "R" => Instr::R(val),
            "U" => Instr::U(val),
            "D" => Instr::D(val),
            _ => return Err(format!("Invalid direction: {:?}", s)),
        })
    }
}

pub fn parse_input<S: AsRef<str>>(input: S) -> (Vec<Instr>, Vec<Instr>) {
    let input = input.as_ref();
    let (la, lb) = input.split_once('\n').unwrap();

    let a = la
        .split(',')
        .map(|s| s.trim())
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let b = lb
        .split(',')
        .map(|s| s.trim())
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    (a, b)
}

pub fn part_one(a: &[Instr], b: &[Instr]) -> u64 {
    let mut grid = Map::default();

    draw(&mut grid, a, |_, _, k, s, g| {
        g.insert(k, s);
    });

    let mut manhattan = u64::MAX;
    draw(&mut grid, b, |x, y, k, _, g| {
        if g.contains_key(&k) {
            let dist = (x.abs() + y.abs()) as u64;
            if dist != 0 && dist < manhattan {
                manhattan = dist;
            }
        }
    });

    manhattan
}

pub fn part_two(a: &[Instr], b: &[Instr]) -> u64 {
    let mut grid = Map::default();

    draw(&mut grid, a, |_x, _y, k, s, g| {
        g.entry(k).or_insert(s);
    });

    let mut result = u64::MAX;
    draw(&mut grid, b, |_, _, k, s, g| {
        if let Some(&steps) = g.get(&k) {
            let total = steps + s;

            if total > 0 && total < result {
                result = total;
            }
        }
    });

    result
}

fn draw<F: FnMut(i32, i32, u64, u64, &mut Grid)>(grid: &mut Grid, instr: &[Instr], mut action: F) {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;

    for &dir in instr.iter() {
        let mut xx = x;
        let mut yy = y;

        match dir {
            Instr::L(v) => x -= v,
            Instr::R(v) => x += v,
            Instr::U(v) => y += v,
            Instr::D(v) => y -= v,
        }

        // draw a horizontal line, without the last point
        let dx = if xx < x { 1 } else { -1 };
        while xx != x {
            action(xx, y, as_hash_key(xx, y), steps, grid);
            xx += dx;
            steps += 1;
        }

        // draw a vertical line, without the last point
        let dy = if yy < y { 1 } else { -1 };
        while yy != y {
            action(x, yy, as_hash_key(x, yy), steps, grid);
            yy += dy;
            steps += 1;
        }
    }

    // The very last point
    if !instr.is_empty() {
        action(x, y, as_hash_key(x, y), steps, grid);
    }
}

#[inline(always)]
fn as_hash_key(a: i32, b: i32) -> u64 {
    ((a as u32) as u64) << 32 | (b as u32) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared_2019::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let (a, b) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&a, &b);
        assert_eq!(403, answer);
    }

    #[test]
    fn test_part_two() {
        let (a, b) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&a, &b);
        assert_eq!(4158, answer);
    }
}
