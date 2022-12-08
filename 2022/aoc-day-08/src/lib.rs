use std::collections::HashSet;

pub fn part_one(grid: &[Vec<u8>]) -> usize {
    let mut visible = HashSet::new();

    for r in 1..grid.len() - 1 {
        let row = &grid[r];

        let mut tallest = row[0];
        for c in 1..row.len() - 1 {
            let height = row[c];
            if height > tallest {
                tallest = height;
                visible.insert((r, c));
            }
        }

        tallest = row[row.len() - 1];
        for c in (1..row.len() - 1).rev() {
            let height = row[c];
            if height > tallest {
                tallest = height;
                visible.insert((r, c));
            }
        }
    }

    for c in 1..grid[0].len() - 1 {
        let mut tallest = grid[0][c];
        for r in 1..grid.len() - 1 {
            let height = grid[r][c];
            if height > tallest {
                tallest = height;
                visible.insert((r, c));
            }
        }

        tallest = grid[grid.len() - 1][c];
        for r in (1..grid.len() - 1).rev() {
            let height = grid[r][c];
            if height > tallest {
                tallest = height;
                visible.insert((r, c));
            }
        }
    }

    visible.len() + grid.len() * 2 + grid[0].len() * 2 - 4
}

pub fn part_two(grid: &[Vec<u8>]) -> usize {
    let mut map = vec![vec![(0usize, 0usize, 0usize, 0usize); grid[0].len()]; grid.len()];
    let mut stack: Vec<usize> = vec![];

    for r in 0..grid.len() {
        let row = &grid[r];

        // to the left
        for c in (0..row.len()).rev() {
            let h = row[c];
            while let Some(x) = stack.last().copied() {
                if h < row[x] {
                    break;
                }

                stack.pop();
                map[r][x].0 = x - c;
            }

            stack.push(c);
        }
        for x in stack.drain(..) {
            map[r][x].0 = x;
        }

        // to the right
        for c in 0..row.len() {
            let h = row[c];
            while let Some(x) = stack.last().copied() {
                if h < row[x] {
                    break;
                }

                stack.pop();
                map[r][x].1 = c - x;
            }

            stack.push(c);
        }
        for x in stack.drain(..) {
            map[r][x].1 = grid[r].len() - x - 1;
        }
    }

    for c in 0..grid[0].len() {
        // to the bottom
        for r in 0..grid.len() {
            let h = grid[r][c];
            while let Some(x) = stack.last().copied() {
                if h < grid[x][c] {
                    break;
                }

                stack.pop();
                map[x][c].2 = r - x;
            }

            stack.push(r);
        }
        for x in stack.drain(..) {
            map[x][c].2 = grid[0].len() - x - 1;
        }

        // to the top
        for r in (0..grid.len()).rev() {
            let h = grid[r][c];
            while let Some(x) = stack.last().copied() {
                if h < grid[x][c] {
                    break;
                }

                stack.pop();
                map[x][c].3 = x - r;
            }

            stack.push(r);
        }
        for x in stack.drain(..) {
            map[x][c].3 = x;
        }
    }

    map.into_iter()
        .flatten()
        .map(|(a, b, c, d)| a * b * c * d)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_u8_numeric_grid;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let grid = parse_u8_numeric_grid(&input);
        let answer = part_one(&grid);
        assert_eq!(1672, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let grid = parse_u8_numeric_grid(&input);
        let answer = part_two(&grid);
        assert_eq!(327180, answer);
    }
}
