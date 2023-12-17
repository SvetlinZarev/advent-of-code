use rayon::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn apply(self, r: usize, c: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => r.checked_sub(1).and_then(|r| Some((r, c))),
            Direction::Down => Some((r + 1, c)),
            Direction::Left => c.checked_sub(1).and_then(|c| Some((r, c))),
            Direction::Right => Some((r, c + 1)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Next {
    Follow(Direction),
    Split(Direction, Direction),
}

pub fn part_one(input: &[u8]) -> usize {
    let cols = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = input.len() / cols;

    energize(input, rows, cols, 0, 0, Direction::Right)
}

pub fn part_two(input: &[u8]) -> usize {
    // We have to adjust all functions to subtract "1" when dealing with the columns due to that newline character
    let cols = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = input.len() / cols;

    let mut answer = 0;
    for r in 0..rows {
        answer = answer.max(energize(input, rows, cols, r, 0, Direction::Right));
        answer = answer.max(energize(input, rows, cols, r, cols - 2, Direction::Left));
    }

    // Use "cols - 1" because of the parasitic newline character in each column, which we must skip
    for c in 0..cols - 1 {
        answer = answer.max(energize(input, rows, cols, 0, c, Direction::Down));
        answer = answer.max(energize(input, rows, cols, rows - 1, c, Direction::Up));
    }

    answer
}

pub fn part_two_rayon(input: &[u8]) -> usize {
    let cols = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = input.len() / cols;

    (0..rows)
        .par_bridge()
        .map(|r| energize(input, rows, cols, r, 0, Direction::Right))
        .chain(
            (0..rows)
                .par_bridge()
                .map(|r| energize(input, rows, cols, r, cols - 2, Direction::Left)),
        )
        .chain(
            (0..cols - 1)
                .par_bridge()
                .map(|c| energize(input, rows, cols, 0, c, Direction::Down)),
        )
        .chain(
            (0..cols - 1)
                .par_bridge()
                .map(|c| energize(input, rows, cols, rows - 1, c, Direction::Up)),
        )
        .reduce(|| 0, |a, b| a.max(b))
}

fn energize(input: &[u8], rows: usize, cols: usize, r: usize, c: usize, d: Direction) -> usize {
    let mut seen = vec![0u8; rows * cols];
    seen[r * cols + c] |= 1 << (d as u32);

    let mut stack = Vec::with_capacity(64);
    stack.push((r, c, d));

    while let Some((r, c, dir)) = stack.pop() {
        let next = match input[r * cols + c] {
            b'.' => Next::Follow(dir),
            b'-' => match dir {
                Direction::Up | Direction::Down => Next::Split(Direction::Left, Direction::Right),
                Direction::Left | Direction::Right => Next::Follow(dir),
            },
            b'|' => match dir {
                Direction::Up | Direction::Down => Next::Follow(dir),
                Direction::Left | Direction::Right => Next::Split(Direction::Up, Direction::Down),
            },
            b'/' => match dir {
                Direction::Up => Next::Follow(Direction::Right),
                Direction::Down => Next::Follow(Direction::Left),
                Direction::Left => Next::Follow(Direction::Down),
                Direction::Right => Next::Follow(Direction::Up),
            },

            b'\\' => match dir {
                Direction::Up => Next::Follow(Direction::Left),
                Direction::Down => Next::Follow(Direction::Right),
                Direction::Left => Next::Follow(Direction::Up),
                Direction::Right => Next::Follow(Direction::Down),
            },

            _ => unreachable!("{}:{}", r, c),
        };

        match next {
            Next::Follow(d) => {
                follow(rows, cols, r, c, d, &mut seen, &mut stack);
            }
            Next::Split(a, b) => {
                follow(rows, cols, r, c, a, &mut seen, &mut stack);
                follow(rows, cols, r, c, b, &mut seen, &mut stack);
            }
        }
    }

    seen.into_iter().filter(|&x| x != 0).count()
}

fn follow(
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    d: Direction,
    seen: &mut [u8],
    queue: &mut Vec<(usize, usize, Direction)>,
) {
    let Some((r, c)) = d.apply(r, c) else {
        return;
    };

    if r >= rows || c >= cols - 1 {
        return;
    }

    if seen[r * cols + c] & (1 << d as u32) != 0 {
        return;
    }

    seen[r * cols + c] |= 1 << d as u32;
    queue.push((r, c, d));
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(input.as_bytes());
        assert_eq!(7_608, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(input.as_bytes());
        assert_eq!(8_221, answer);
    }

    #[test]
    fn test_part_two_rayon() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_rayon(input.as_bytes());
        assert_eq!(8_221, answer);
    }
}
