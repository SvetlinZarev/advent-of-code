use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

const DIR: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

pub fn part_one_v1(grid: Vec<Vec<u8>>) -> u32 {
    let (mut sr, mut sc) = (0, 0);
    let (mut er, mut ec) = (0, 0);

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'S' {
                (sr, sc) = (r, c);
            } else if grid[r][c] == b'E' {
                (er, ec) = (r, c);
            }
        }
    }

    solve_v1(grid, &[(sr, sc)], er, ec).unwrap()
}

pub fn part_two_v1(grid: Vec<Vec<u8>>) -> u32 {
    let (mut er, mut ec) = (0, 0);
    let mut start = vec![];

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'a' || grid[r][c] == b'S' {
                start.push((r, c));
            } else if grid[r][c] == b'E' {
                (er, ec) = (r, c);
            }
        }
    }

    solve_v1(grid, &start, er, ec).unwrap()
}

// A* => move from start to end
fn solve_v1(mut grid: Vec<Vec<u8>>, start: &[(usize, usize)], er: usize, ec: usize) -> Option<u32> {
    let mut pq = BinaryHeap::new();
    start.iter().copied().for_each(|(sr, sc)| {
        pq.push((Reverse(manhattan(sr, sc, er, ec)), Reverse(0), b'a', sr, sc));
    });

    while let Some((Reverse(_score), Reverse(steps), h, r, c)) = pq.pop() {
        if (r, c) == (er, ec) {
            return Some(steps);
        }

        // skip already visited
        if grid[r][c] == 0 {
            continue;
        }

        // mark visited
        grid[r][c] = 0;

        for (dr, dc) in DIR.iter().copied() {
            let rx = r as isize + dr;
            let cx = c as isize + dc;
            if rx < 0 || cx < 0 {
                continue;
            }

            let (rx, cx) = (rx as usize, cx as usize);
            if rx >= grid.len() || cx >= grid[rx].len() {
                continue;
            }

            let new_h = grid[rx][cx];

            // skip visited nodes as early as possible
            if new_h == 0 {
                continue;
            }

            // skip invalid moves
            if new_h > h && new_h - h > 1 && !(h == b'z' && new_h == b'E') {
                continue;
            }

            let new_score = manhattan(rx, cx, er, ec) + steps + 1;
            pq.push((Reverse(new_score), Reverse(steps + 1), new_h, rx, cx));
        }
    }

    None
}

fn manhattan(sr: usize, sc: usize, er: usize, ec: usize) -> u32 {
    (sr.abs_diff(er) + sc.abs_diff(ec)) as u32
}

pub fn part_one_v2(grid: Vec<Vec<u8>>) -> u32 {
    solve_v2_all_parts(grid, b'S')
}

pub fn part_two_v2(grid: Vec<Vec<u8>>) -> u32 {
    solve_v2_all_parts(grid, b'a')
}

fn solve_v2_all_parts(grid: Vec<Vec<u8>>, target: u8) -> u32 {
    let (mut sr, mut sc) = (0, 0);

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'E' {
                (sr, sc) = (r, c);
                break;
            }
        }
    }

    solve_v2(grid, sr, sc, target).unwrap()
}

// BFS => move from end to start
fn solve_v2(mut grid: Vec<Vec<u8>>, sr: usize, sc: usize, target: u8) -> Option<u32> {
    grid[sr][sc] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((b'z', sr, sc));

    let mut steps = 0;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            if let Some((h, r, c)) = queue.pop_front() {
                if target == h {
                    return Some(steps);
                }

                for (dr, dc) in DIR.iter().copied() {
                    let rx = r as isize + dr;
                    let cx = c as isize + dc;
                    if rx < 0 || cx < 0 {
                        continue;
                    }

                    let (rx, cx) = (rx as usize, cx as usize);
                    if rx >= grid.len() || cx >= grid[rx].len() {
                        continue;
                    }

                    let mut new_h = grid[rx][cx];
                    new_h = if new_h == b'S' { b'a' } else { new_h };

                    // skip already visited
                    // skip paths that violate the condition of maximum 1 diff in the steepness increase
                    if new_h == 0 || (new_h < h && (h - new_h) > 1) {
                        continue;
                    }

                    queue.push_back((grid[rx][cx], rx, cx));

                    // mark as visited
                    grid[rx][cx] = 0;
                }
            }
        }
        steps += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_u8_grid;

    use crate::{part_one_v1, part_one_v2, part_two_v1, part_two_v2};

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let grid = parse_u8_grid(input);
        let answer = part_one_v1(grid);
        assert_eq!(517, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let grid = parse_u8_grid(input);
        let answer = part_two_v1(grid);
        assert_eq!(512, answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let grid = parse_u8_grid(input);
        let answer = part_one_v2(grid);
        assert_eq!(517, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let grid = parse_u8_grid(input);
        let answer = part_two_v2(grid);
        assert_eq!(512, answer);
    }
}
