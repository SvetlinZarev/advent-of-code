use aoc_shared::hashing::FnvHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, BuildHasherDefault};

const INFECTED: u8 = b'#';
const DIR: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part_one(input: &str) -> usize {
    let (mut grid, mut r, mut c) = load_input(input);
    let mut dir = 0;
    let mut infections = 0;

    for _ in 0..10_000 {
        if grid.remove(&(r, c)) {
            dir = rotr(dir);
        } else {
            dir = rotl(dir);
            infections += 1;
            grid.insert((r, c));
        }

        r += DIR[dir].0;
        c += DIR[dir].1;
    }

    infections
}

pub fn part_two(input: &str) -> usize {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum State {
        Weakened,
        Infected,
        Flagged,
    }

    let (mut grid, mut r, mut c) = load_input(input);
    let mut grid = grid
        .into_iter()
        .map(|key| (key, State::Infected))
        .collect::<HashMap<_, _, BuildHasherDefault<FnvHasher>>>();

    let mut dir = 0;
    let mut infections = 0;

    for _ in 0..10_000_000 {
        match grid.remove(&(r, c)) {
            None => {
                dir = rotl(dir);
                grid.insert((r, c), State::Weakened);
            }

            Some(state) => match state {
                State::Weakened => {
                    infections += 1;
                    grid.insert((r, c), State::Infected);
                }
                State::Infected => {
                    dir = rotr(dir);
                    grid.insert((r, c), State::Flagged);
                }

                State::Flagged => {
                    dir = rotr(dir);
                    dir = rotr(dir);
                }
            },
        }

        r += DIR[dir].0;
        c += DIR[dir].1;
    }

    infections
}

fn load_input(input: &str) -> (HashSet<(i32, i32)>, i32, i32) {
    let mut grid = HashSet::new();
    let (mut r, mut c) = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.bytes().enumerate() {
            if ch == INFECTED {
                grid.insert((row as i32, col as i32));
            }
        }

        r = row as i32 / 2;
        c = line.len() as i32 / 2;
    }

    (grid, r, c)
}

fn rotl(dir: usize) -> usize {
    if dir == 0 {
        3
    } else {
        dir - 1
    }
}

fn rotr(dir: usize) -> usize {
    (dir + 1) % 4
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(5259, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(2_511_722, answer);
    }
}
