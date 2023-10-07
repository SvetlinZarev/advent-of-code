use std::collections::{HashSet, VecDeque};

const DIR: &[(isize, isize)] = &[(-1, 0), (0, -1), (0, 1), (1, 0)];
const WALL: u8 = b'#';

pub fn part_one(grid: &[Vec<u8>]) -> usize {
    let mut points = vec![];
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c].is_ascii_digit() {
                points.push((grid[r][c] - b'0', r, c));
            }
        }
    }

    let mut distances = vec![vec![0; points.len()]; points.len()];
    for (src, sr, sc) in points.iter().copied() {
        for (dst, dr, dc) in points.iter().copied() {
            let d = dist(grid, (sr, sc), (dr, dc));
            distances[src as usize][dst as usize] = d;
            distances[dst as usize][src as usize] = d;
        }
    }

    dfs_part_1(&distances, 0, 1)
}

pub fn part_two(grid: &[Vec<u8>]) -> usize {
    let mut points = vec![];
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c].is_ascii_digit() {
                points.push((grid[r][c] - b'0', r, c));
            }
        }
    }

    let mut distances = vec![vec![0; points.len()]; points.len()];
    for (src, sr, sc) in points.iter().copied() {
        for (dst, dr, dc) in points.iter().copied() {
            let d = dist(grid, (sr, sc), (dr, dc));
            distances[src as usize][dst as usize] = d;
            distances[dst as usize][src as usize] = d;
        }
    }

    dfs_part_2(&distances, 0, 1)
}

fn dist(grid: &[Vec<u8>], start: (usize, usize), target: (usize, usize)) -> usize {
    if start == target {
        return 0;
    }

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut seen = HashSet::new();
    seen.insert(start);

    let mut steps = 0;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (r, c) = queue.pop_front().unwrap();
            for (dr, dc) in DIR.iter().copied() {
                let Some(row) = r.checked_add_signed(dr) else {
                    continue;
                };
                let Some(col) = c.checked_add_signed(dc) else {
                    continue;
                };
                if row >= grid.len() || col >= grid[r].len() {
                    continue;
                }
                if grid[row][col] == WALL {
                    continue;
                }
                if !seen.insert((row, col)) {
                    continue;
                }

                if (row, col) == target {
                    return steps + 1;
                }

                queue.push_back((row, col));
            }
        }

        steps += 1;
    }

    usize::MAX
}

fn dfs_part_1(distances: &[Vec<usize>], prev: usize, used: u32) -> usize {
    if used.count_ones() as usize == distances.len() {
        return 0;
    }

    let mut best = usize::MAX;
    for next in 1..distances.len() {
        let mask = 1 << next;
        if used & (mask) != 0 {
            continue;
        }

        let d = dfs_part_1(distances, next, used | (mask));
        best = best.min(d + distances[prev][next]);
    }

    best
}

fn dfs_part_2(distances: &[Vec<usize>], prev: usize, used: u32) -> usize {
    if used.count_ones() as usize == distances.len() {
        return distances[0][prev];
    }

    let mut best = usize::MAX;
    for next in 1..distances.len() {
        let mask = 1 << next;
        if used & mask != 0 {
            continue;
        }

        let d = dfs_part_2(distances, next, used | mask);
        best = best.min(d + distances[prev][next]);
    }

    best
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_u8_grid;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_u8_grid(&input);
        let answer = part_one(&parsed);

        assert_eq!(474, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_u8_grid(&input);
        let answer = part_two(&parsed);

        assert_eq!(696, answer);
    }
}
