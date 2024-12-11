use aoc_shared::grid::DIR4;
use aoc_shared::util::BitSet;

pub fn part_one_v1(input: &str) -> u32 {
    let input = input.as_bytes();
    let width = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let height = input.len() / width;

    let mut visited = BitSet::new(input.len());
    let mut queue = vec![];
    let mut answer = 0;

    for r in 0..height {
        for c in 0..width - 1 {
            if visited.mark(r * width + c) {
                let expected = input[r * width + c];

                let mut area = 0;
                let mut perimeter = 0;

                queue.push((r as isize, c as isize));
                while let Some((y, x)) = queue.pop() {
                    area += 1;
                    perimeter += 4;

                    for (dy, dx) in DIR4.into_iter().rev() {
                        let row = y + dy;
                        let col = x + dx;

                        if !(0..height as isize).contains(&row) {
                            continue;
                        }

                        if !(0..width as isize - 1).contains(&col) {
                            continue;
                        }

                        let pos = row as usize * width + col as usize;
                        if input[pos] != expected {
                            continue;
                        }

                        perimeter -= 1;

                        if !visited.mark(pos) {
                            continue;
                        }

                        queue.push((row, col));
                    }
                }

                answer += area * perimeter;
            }
        }
    }

    answer
}

pub fn part_one_v2(input: &str) -> u32 {
    let input = input.as_bytes();
    let width = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let height = input.len() / width;

    let mut visited = BitSet::new(input.len());
    let mut answer = 0;

    for r in 0..height {
        for c in 0..width - 1 {
            if visited.mark(r * width + c) {
                let (area, perimeter) = mark_and_count(
                    input,
                    &mut visited,
                    height as isize,
                    width as isize,
                    r as isize,
                    c as isize,
                );

                answer += area * perimeter;
            }
        }
    }

    answer
}

fn mark_and_count(
    grid: &[u8],
    visited: &mut BitSet,
    rows: isize,
    cols: isize,
    row: isize,
    col: isize,
) -> (u32, u32) {
    let from = (row * cols + col) as usize;
    let expected = grid[from];

    let (mut a, mut p) = (1, 4);

    for (dr, dc) in DIR4.into_iter().rev() {
        let r = row + dr;
        let c = col + dc;

        if !(0..rows).contains(&r) {
            continue;
        }

        if !(0..cols - 1).contains(&c) {
            continue;
        }

        let to = (r * cols + c) as usize;
        if grid[to] != expected {
            continue;
        }

        p -= 1;
        if !visited.mark(to) {
            continue;
        }

        let (m, n) = mark_and_count(grid, visited, rows, cols, r, c);
        a += m;
        p += n;
    }

    (a, p)
}

pub fn part_two(input: &str) -> u32 {
    const UP: usize = 0;
    const LEFT: usize = 1;
    const RIGHT: usize = 2;
    const DOWN: usize = 3;

    let input = input.as_bytes();
    let cols = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = input.len() / cols;

    let mut seen = BitSet::new(input.len());
    let mut queue = vec![];
    let mut answer = 0;

    for r in 0..rows {
        for c in 0..cols - 1 {
            let expected = input[r * cols + c];
            let mut area = 0;
            let mut sides = 0;

            if seen.mark(r * cols + c) {
                queue.push((r as isize, c as isize));

                while let Some((r, c)) = queue.pop() {
                    area += 1;

                    // up, left, right, down
                    let mut s = [false; 4];
                    for (idx, dr, dc) in [(UP, -1, 0), (LEFT, 0, -1), (RIGHT, 0, 1), (DOWN, 1, 0)] {
                        let row = r + dr;
                        let col = c + dc;

                        if !(0..rows as isize).contains(&row) {
                            continue;
                        }

                        if !(0..cols as isize - 1).contains(&col) {
                            continue;
                        }

                        let pos = row as usize * cols + col as usize;
                        if input[pos] == expected {
                            s[idx] = true;

                            if !seen.mark(pos) {
                                continue;
                            }

                            queue.push((row, col));
                        }
                    }

                    // check the 4 corners
                    //
                    // we assume that all cell letters with the same value
                    // belong to the same group, otherwise we'll have to do
                    // it in two stages:
                    // 1. mark all zones using flood fill with a unique zone id
                    // 2. work on the corners using the zone ids, instead of the cell letters from the input

                    // top-left corner
                    if !s[UP] && !s[LEFT] {
                        sides += 1;
                    } else if s[UP] && s[LEFT] {
                        if cell(input, rows as isize, cols as isize, r - 1, c - 1) != expected {
                            sides += 1;
                        }
                    }

                    // top-right corner
                    if !s[UP] && !s[RIGHT] {
                        sides += 1;
                    } else if s[UP] && s[RIGHT] {
                        if cell(input, rows as isize, cols as isize, r - 1, c + 1) != expected {
                            sides += 1;
                        }
                    }

                    // bottom-left corner
                    if !s[DOWN] && !s[LEFT] {
                        sides += 1;
                    } else if s[DOWN] && s[LEFT] {
                        if cell(input, rows as isize, cols as isize, r + 1, c - 1) != expected {
                            sides += 1;
                        }
                    }

                    // bottom-right corner
                    if !s[DOWN] && !s[RIGHT] {
                        sides += 1;
                    } else if s[DOWN] && s[RIGHT] {
                        if cell(input, rows as isize, cols as isize, r + 1, c + 1) != expected {
                            sides += 1;
                        }
                    }
                }

                answer += area * sides;
            }
        }
    }

    answer
}

fn cell(grid: &[u8], rows: isize, cols: isize, row: isize, col: isize) -> u8 {
    if row < 0 || col < 0 {
        return 0;
    }

    if row >= rows || col >= cols - 1 {
        return 0;
    }

    grid[(row * cols + col) as usize]
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v1(&input);
        assert_eq!(1415378, answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v2(&input);
        assert_eq!(1415378, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(862714, answer);
    }
}
