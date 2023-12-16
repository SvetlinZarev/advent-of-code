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

pub fn part_one(input: &Vec<&[u8]>) -> usize {
    energize(input, 0, 0, Direction::Right)
}

pub fn part_two(input: &Vec<&[u8]>) -> usize {
    let rows = input.len();
    let cols = input[0].len();
    for r in 0..rows {
        assert_eq!(cols, input[r].len());
    }

    let mut answer = 0;
    for r in 0..rows {
        answer = answer.max(energize(input, r, 0, Direction::Right));
        answer = answer.max(energize(input, r, cols - 1, Direction::Left));
    }

    for c in 0..cols {
        answer = answer.max(energize(input, 0, c, Direction::Down));
        answer = answer.max(energize(input, rows - 1, c, Direction::Up));
    }

    answer
}

pub fn part_two_rayon(input: &Vec<&[u8]>) -> usize {
    let rows = input.len();
    let cols = input[0].len();

    (0..rows)
        .par_bridge()
        .map(|r| energize(input, r, 0, Direction::Right))
        .chain(
            (0..rows)
                .par_bridge()
                .map(|r| energize(input, r, cols - 1, Direction::Left)),
        )
        .chain(
            (0..cols)
                .par_bridge()
                .map(|c| energize(input, 0, c, Direction::Down)),
        )
        .chain(
            (0..cols)
                .par_bridge()
                .map(|c| energize(input, rows - 1, c, Direction::Up)),
        )
        .reduce(|| 0, |a, b| a.max(b))
}

fn energize(input: &Vec<&[u8]>, r: usize, c: usize, d: Direction) -> usize {
    let rows = input.len();
    let cols = input[0].len();

    let mut seen = vec![0u8; rows * cols];
    seen[r * cols + c] |= 1 << (d as u32);

    let mut stack = vec![];
    stack.push((r, c, d));

    while let Some((r, c, dir)) = stack.pop() {
        let next = match input[r][c] {
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

            _ => unreachable!(),
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

    if r >= rows || c >= cols {
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
    use aoc_shared::parsing::parse_u8_grid_borrowed;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_u8_grid_borrowed(&input);

        let answer = part_one(&input);
        assert_eq!(7_608, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_u8_grid_borrowed(&input);

        let answer = part_two(&input);
        assert_eq!(8_221, answer);
    }
}
