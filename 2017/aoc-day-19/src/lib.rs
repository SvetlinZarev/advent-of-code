const DIR: &[Bearing] = &[Bearing::Up, Bearing::Left, Bearing::Right, Bearing::Down];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Bearing {
    Up,
    Down,
    Left,
    Right,
}

impl Bearing {
    fn rev(self) -> Self {
        match self {
            Bearing::Up => Bearing::Down,
            Bearing::Down => Bearing::Up,
            Bearing::Left => Bearing::Right,
            Bearing::Right => Bearing::Left,
        }
    }

    fn apply(self, r: usize, c: usize) -> (usize, usize) {
        match self {
            Bearing::Up => (r - 1, c),
            Bearing::Down => (r + 1, c),
            Bearing::Left => (r, c - 1),
            Bearing::Right => (r, c + 1),
        }
    }
}

pub fn solve(grid: &Vec<Vec<u8>>) -> (String, u32) {
    let mut answer = String::new();
    let mut steps = 0;

    let (mut r, mut c) = find_start(grid);
    let mut bearing = match (r, c) {
        (0, _) => Bearing::Down,
        (_, 0) => Bearing::Right,
        (r, _) => {
            if r == grid.len() - 1 {
                Bearing::Up
            } else {
                Bearing::Left
            }
        }
    };

    'next: loop {
        if grid[r][c] == b'+' {
            for dir in DIR.iter().copied() {
                if dir == bearing.rev() {
                    continue;
                }

                let (y, x) = dir.apply(r, c);
                if grid[y][x] == b' ' {
                    continue;
                }

                bearing = dir;
                (r, c) = (y, x);
                steps += 1;
                continue 'next;
            }
        }

        if grid[r][c] == b' ' {
            break;
        }

        if grid[r][c].is_ascii_uppercase() {
            answer.push(grid[r][c] as char);
        }

        (r, c) = bearing.apply(r, c);
        steps += 1;
    }

    (answer, steps)
}

fn find_start(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    for c in 0..grid[0].len() {
        if grid[0][c] == b'|' {
            return (0, c);
        }
    }

    for c in 0..grid[grid.len() - 1].len() {
        if grid[grid.len() - 1][c] == b'|' {
            return (grid.len() - 1, c);
        }
    }

    for r in 0..grid.len() {
        if grid[r][0] == b'-' {
            return (r, 0);
        }
    }

    for r in 0..grid.len() {
        if grid[r][grid[r].len() - 1] == b'-' {
            return (r, grid[r].len() - 1);
        }
    }

    panic!("failed to find the starting point")
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_u8_grid;

    use super::*;

    #[test]
    fn test_solve() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_u8_grid(input);

        let (part_one, part_two) = solve(&input);
        assert_eq!("LIWQYKMRP", part_one);
        assert_eq!(16_764, part_two);
    }
}
