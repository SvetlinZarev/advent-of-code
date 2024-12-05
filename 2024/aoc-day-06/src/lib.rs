use aoc_shared::grid::Direction;

pub fn part_one(input: &str) -> usize {
    let grid = input.as_bytes();
    let width = grid.iter().position(|&c| c == b'\n').unwrap() as isize + 1;
    let height = grid.len() as isize / width;

    let start_idx = grid.iter().position(|&c| c == b'^').unwrap() as isize;
    let row = start_idx / width;
    let col = start_idx % width;

    let visited = mark_route(grid, height, width, row, col);
    visited.iter().filter(|&&c| c != 0).count()
}

pub fn part_two_v1(input: &str) -> u32 {
    let grid = input.as_bytes();
    let width = grid.iter().position(|&c| c == b'\n').unwrap() as isize + 1;
    let height = grid.len() as isize / width;

    let start_idx = grid.iter().position(|&c| c == b'^').unwrap() as isize;
    let start_row = start_idx / width;
    let start_col = start_idx % width;

    // It makes sense to put obstacles only on the original route of the guard,
    // because otherwise he will never reach them.
    let route = mark_route(grid, height, width, start_row, start_col);

    let mut visited = vec![0u8; grid.len()];
    let mut answer = 0;

    for idx in 0..route.len() as isize {
        if route[idx as usize] == 0 {
            continue;
        }

        let obs_r = idx / width;
        let obs_c = idx % width;
        if (start_row, start_col) == (obs_r, obs_c) {
            continue;
        }

        visited.fill(0);
        let mut row = start_row;
        let mut col = start_col;
        let mut dir = Direction::Up;

        loop {
            if visited[(row * width + col) as usize] & (1 << dir as usize) != 0 {
                answer += 1;
                break;
            }
            visited[(row * width + col) as usize] |= 1 << dir as usize;

            let (r, c) = dir.apply_signed(row, col);
            if r < 0 || c < 0 || r >= height || c + 1 >= width {
                break;
            }

            if grid[(r * width + c) as usize] == b'#' || (r, c) == (obs_r, obs_c) {
                dir = dir.rotr();
            } else {
                row = r;
                col = c;
            }
        }
    }

    answer
}

fn mark_route(
    grid: &[u8],
    height: isize,
    width: isize,
    start_row: isize,
    start_col: isize,
) -> Vec<u8> {
    let mut row = start_row;
    let mut col = start_col;
    let mut dir = Direction::Up;

    let mut visited = vec![0u8; grid.len()];

    loop {
        visited[(row * width + col) as usize] |= 1 << dir as usize;
        let (r, c) = dir.apply_signed(row, col);

        if r < 0 || c < 0 || r >= height || c + 1 >= width {
            break;
        }

        if grid[(r * width + c) as usize] == b'#' {
            dir = dir.rotr();
        } else {
            row = r;
            col = c;
        }
    }

    visited
}

pub fn part_two_v2(input: &str) -> u32 {
    let grid = input.as_bytes();
    let width = grid.iter().position(|&c| c == b'\n').unwrap() as isize + 1;
    let height = grid.len() as isize / width;

    let start_idx = grid.iter().position(|&c| c == b'^').unwrap() as isize;
    let start_row = start_idx / width;
    let start_col = start_idx % width;

    let mut visited = vec![0u8; grid.len()];

    let mut row = start_row;
    let mut col = start_col;
    let mut dir = Direction::Up;

    let mut answer = 0;

    loop {
        let vis_idx = (row * width + col) as usize;
        visited[vis_idx] |= 1 << dir as usize;

        let (r, c) = dir.apply_signed(row, col);
        if r < 0 || c < 0 || r >= height || c + 1 >= width {
            break;
        }

        if grid[(r * width + c) as usize] == b'#' {
            dir = dir.rotr();
        } else {
            if visited[(r * width + c) as usize] == 0 {
                answer += check_loop(
                    grid,
                    &mut visited.clone(),
                    height,
                    width,
                    row,
                    col,
                    r,
                    c,
                    dir,
                );
            }

            row = r;
            col = c;
        }
    }

    answer
}

fn check_loop(
    grid: &[u8],
    visited: &mut [u8],
    height: isize,
    width: isize,
    row: isize,
    col: isize,
    obs_r: isize,
    obs_c: isize,
    dir: Direction,
) -> u32 {
    let mut row = row;
    let mut col = col;
    let mut dir = dir;

    loop {
        visited[(row * width + col) as usize] |= 1 << dir as usize;

        let (r, c) = dir.apply_signed(row, col);
        if r < 0 || c < 0 || r >= height || c + 1 >= width {
            return 0;
        }

        if grid[(r * width + c) as usize] == b'#' || (r, c) == (obs_r, obs_c) {
            dir = dir.rotr();
        } else {
            row = r;
            col = c;

            if visited[(row * width + col) as usize] & (1 << dir as usize) != 0 {
                return 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(5030, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two_v1(&input);
        assert_eq!(1928, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two_v2(&input);
        assert_eq!(1928, answer);
    }
}
