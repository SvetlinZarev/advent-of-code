use std::str::FromStr;

use aoc_shared::hashing::HashSet;

const DIR: &[(isize, isize, isize)] = &[
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, rest) = s
            .split_once(',')
            .ok_or_else(|| format!("cannot extract coordinates from: {}", s))?;

        let (y, z) = rest
            .split_once(',')
            .ok_or_else(|| format!("cannot extract coordinates from: {}", s))?;

        Ok(Point::new(
            x.parse().map_err(|_| format!("cannot parse: {}", x))?,
            y.parse().map_err(|_| format!("cannot parse: {}", x))?,
            z.parse().map_err(|_| format!("cannot parse: {}", x))?,
        ))
    }
}

pub fn part_one(input: &[Point]) -> u32 {
    let (mut x, mut y, mut z) = (0, 0, 0);
    for point in input.iter().copied() {
        x = x.max(point.x);
        y = y.max(point.y);
        z = z.max(point.z);
    }

    let mut grid = vec![vec![vec![false; z + 1]; y + 1]; x + 1];
    for point in input.iter().copied() {
        grid[point.x][point.y][point.z] = true;
    }

    let mut seen = HashSet::default();
    let mut stack = vec![];
    let mut area = 0;

    for point in input.iter().copied() {
        if !seen.insert(point) {
            continue;
        }
        stack.push(point);

        while let Some(p) = stack.pop() {
            area += 6;

            for &(dx, dy, dz) in DIR {
                let Some(x) = p.x.checked_add_signed(dx) else{
                    continue;
                };

                let Some(y) = p.y.checked_add_signed(dy) else{
                    continue;
                };

                let Some(z) = p.z.checked_add_signed(dz) else{
                    continue;
                };

                if x >= grid.len() || y >= grid[x].len() || z >= grid[x][y].len() {
                    continue;
                }

                if !grid[x][y][z] {
                    continue;
                }

                area -= 1;
                let next = Point::new(x, y, z);
                if seen.insert(next) {
                    stack.push(next);
                }
            }
        }
    }

    area
}

pub fn part_two(input: &[Point]) -> usize {
    let (mut x, mut y, mut z) = (0, 0, 0);
    for point in input.iter().copied() {
        x = x.max(point.x);
        y = y.max(point.y);
        z = z.max(point.z);
    }

    let mut grid = vec![vec![vec![false; z + 1]; y + 1]; x + 1];
    for point in input.iter().copied() {
        grid[point.x][point.y][point.z] = true;
    }

    let mut seen = HashSet::default();
    let mut stack = vec![];
    let mut area = 0;

    // Collect the sides of the bounding cube
    let mut surface = vec![];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            surface.push(Point::new(x, y, 0));
            surface.push(Point::new(x, y, grid[0][0].len() - 1));
        }
    }
    for x in 0..grid.len() {
        for z in 0..grid[0][0].len() {
            surface.push(Point::new(x, 0, z));
            surface.push(Point::new(x, grid[0].len() - 1, z));
        }
    }
    for y in 0..grid[0].len() {
        for z in 0..grid[0][0].len() {
            surface.push(Point::new(0, y, z));
            surface.push(Point::new(grid.len() - 1, y, z));
        }
    }

    for point in surface {
        if !seen.insert(point) {
            continue;
        }
        stack.push(point);

        while let Some(p) = stack.pop() {
            // Handle external surface on the surface of the bounding cube
            if grid[p.x][p.y][p.z] {
                area += (p.x == 0) as usize
                    + (p.y == 0) as usize
                    + (p.z == 0) as usize
                    + (p.x == grid.len() - 1) as usize
                    + (p.y == grid[p.x].len() - 1) as usize
                    + (p.z == grid[p.x][p.y].len() - 1) as usize;
                continue;
            }

            for &(dx, dy, dz) in DIR {
                let Some(x) = p.x.checked_add_signed(dx) else{
                    continue;
                };

                let Some(y) = p.y.checked_add_signed(dy) else{
                    continue;
                };

                let Some(z) = p.z.checked_add_signed(dz) else{
                    continue;
                };

                if x >= grid.len() || y >= grid[x].len() || z >= grid[x][y].len() {
                    continue;
                }

                if grid[x][y][z] {
                    area += 1;
                    continue;
                }

                let next = Point::new(x, y, z);
                if seen.insert(next) {
                    stack.push(next);
                }
            }
        }
    }

    area
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(3494, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(2062, answer);
    }
}
