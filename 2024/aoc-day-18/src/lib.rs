use aoc_shared::algo::UnionFind;
use aoc_shared::grid::DIR4;
use std::collections::VecDeque;
use std::error::Error;

const SIDE: usize = 71;
const TAKE: usize = 1024;

const UNVISITED: u8 = 0u8;
const WALL: u8 = 1u8;
const VISITED: u8 = 2u8;

pub fn parse_input(input: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let mut parsed = vec![];
    for line in input.lines() {
        let Some((a, b)) = line.split_once(',') else {
            return Err(format!("Invalid input: {}", line).into());
        };

        parsed.push((b.trim().parse()?, a.trim().parse()?));
    }

    Ok(parsed)
}

pub fn part_one(input: &[(usize, usize)]) -> u32 {
    min_cost(&input, SIDE, TAKE).unwrap()
}

// Binary search with BFS
pub fn part_two_v1(input: &[(usize, usize)]) -> (usize, usize) {
    let mut lo = 0;
    let mut hi = input.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        // We use `mid+1` here because we specify how many elements to take,
        // instead of up to which element (inclusive) to go. Thus if `mid=0`
        // we'll take no elements, be we want to take the 0th element instead.
        if min_cost(input, SIDE, mid + 1).is_some() {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    let block = input[hi];
    (block.1, block.0)
}

fn min_cost(blocks: &[(usize, usize)], side: usize, take: usize) -> Option<u32> {
    let mut grid = vec![UNVISITED; side * side];

    for (r, c) in blocks.iter().copied().take(take) {
        grid[r * SIDE + c] = WALL;
    }

    let mut queue = VecDeque::new();
    queue.push_back((0isize, 0isize));

    let mut steps = 0u32;
    while !queue.is_empty() {
        steps += 1;

        for _ in 0..queue.len() {
            let (r, c) = queue.pop_front().unwrap();

            for (dr, dc) in DIR4 {
                let y = r + dr;
                let x = c + dc;
                if !(0..side as isize).contains(&x) || !(0..side as isize).contains(&y) {
                    continue;
                }

                if grid[y as usize * side + x as usize] != UNVISITED {
                    continue;
                }
                grid[y as usize * side + x as usize] = VISITED;

                if (y as usize, x as usize) == (side - 1, side - 1) {
                    return Some(steps);
                }

                queue.push_back((y, x));
            }
        }
    }

    None
}

// Binary Search with DFS instead of BFS
pub fn part_two_v2(input: &[(usize, usize)]) -> (usize, usize) {
    let mut lo = 0;
    let mut hi = input.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        // We use `mid+1` here because we specify how many elements to take,
        // instead of up to which element (inclusive) to go. Thus if `mid=0`
        // we'll take no elements, be we want to take the 0th element instead.
        if is_reachable(input, SIDE, mid + 1) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    let block = input[hi];
    (block.1, block.0)
}

fn is_reachable(blocks: &[(usize, usize)], side: usize, take: usize) -> bool {
    let mut grid = vec![UNVISITED; side * side];

    for (r, c) in blocks.iter().copied().take(take) {
        grid[r * SIDE + c] = WALL;
    }

    let mut stack = vec![];
    stack.push((0isize, 0isize));

    while let Some((r, c)) = stack.pop() {
        for (dr, dc) in DIR4 {
            let y = r + dr;
            let x = c + dc;
            if !(0..side as isize).contains(&x) || !(0..side as isize).contains(&y) {
                continue;
            }

            if grid[y as usize * side + x as usize] != UNVISITED {
                continue;
            }
            grid[y as usize * side + x as usize] = VISITED;

            if (y as usize, x as usize) == (side - 1, side - 1) {
                return true;
            }

            stack.push((y, x));
        }
    }

    false
}

// UF, going backwards, trying to undo a WALL fom the input in reverse
pub fn part_two_v3(input: &[(usize, usize)]) -> (usize, usize) {
    let mut uf = UnionFind::new(SIDE * SIDE);

    let mut grid = vec![UNVISITED; SIDE * SIDE];
    for &(r, c) in input {
        grid[r * SIDE + c] = WALL;
    }

    let mut queue = VecDeque::new();
    queue.push_back((0isize, 0isize));
    queue.push_back(((SIDE - 1) as isize, (SIDE - 1) as isize));

    flood_union(&grid, &mut queue, &mut uf);

    for &(r, c) in input.iter().rev() {
        grid[r * SIDE + c] = UNVISITED;

        queue.push_back((r as isize, c as isize));
        flood_union(&grid, &mut queue, &mut uf);

        if uf.find(0) == uf.find(SIDE * SIDE - 1) {
            return (c, r);
        }
    }

    unreachable!()
}

#[inline(always)]
fn flood_union(grid: &[u8], queue: &mut VecDeque<(isize, isize)>, uf: &mut UnionFind) {
    while let Some((r, c)) = queue.pop_front() {
        let a = r as usize * SIDE + c as usize;

        for (dr, dc) in DIR4 {
            let y = r + dr;
            let x = c + dc;
            if !(0..SIDE as isize).contains(&x) || !(0..SIDE as isize).contains(&y) {
                continue;
            }

            let b = y as usize * SIDE + x as usize;
            if grid[b] == UNVISITED && uf.union(a, b) {
                queue.push_back((y, x));
            }
        }
    }
}

// UF going forward
pub fn part_two_v4(input: &[(usize, usize)]) -> (usize, usize) {
    let mut uf = UnionFind::new((SIDE + 1) * (SIDE + 1));
    let index = |y: usize, x: usize| y * (SIDE + 1) + x;

    for i in 0..SIDE - 1 {
        uf.union(index(0, i + 1), index(0, i + 2)); // top edge
        uf.union(index(SIDE, i), index(SIDE, i + 1)); // bottom edge
        uf.union(index(i + 1, 0), index(i + 2, 0)); // left edge
        uf.union(index(i, SIDE), index(i + 1, SIDE)); // right edge
    }

    // We are going only DOWN, RIGHT, AND DOWN_RIGHT,
    // That;s why we don;t need to track if there is wall
    // at the target position
    for &(y, x) in input {
        uf.union(index(y, x), index(y, x + 1));
        uf.union(index(y, x), index(y + 1, x));
        uf.union(index(y, x), index(y + 1, x + 1));

        if uf.find(index(1, 0)) == uf.find(index(0, 1)) {
            return (x, y);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_one(&parsed);
        assert_eq!(232, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two_v1(&parsed);
        assert_eq!((44, 64), answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two_v2(&parsed);
        assert_eq!((44, 64), answer);
    }

    #[test]
    fn test_part_two_v3() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two_v3(&parsed);
        assert_eq!((44, 64), answer);
    }

    #[test]
    fn test_part_two_v4() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two_v4(&parsed);
        assert_eq!((44, 64), answer);
    }
}
