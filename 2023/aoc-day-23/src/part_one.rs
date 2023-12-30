use aoc_shared::grid::DIR4;
use aoc_shared::util::BitSet;

pub fn part_one(input: &str) -> usize {
    let grid = input.as_bytes();
    let mut seen = BitSet::new(grid.len());

    let cols = grid.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = grid.len() / cols;

    dfs(grid, &mut seen, rows, cols, 0, 1).unwrap()
}

fn dfs(
    grid: &[u8],
    seen: &mut BitSet,
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
) -> Option<usize> {
    if (r, c) == (rows - 1, cols - 3) {
        return Some(0);
    }

    let mut len = None;

    for (dr, dc) in DIR4 {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr < 0 || nc < 0 {
            continue;
        }

        let nr = nr as usize;
        let nc = nc as usize;
        if nr >= rows || nc >= cols - 1 {
            continue;
        }

        match grid[nr * cols + nc] {
            b'#' => continue,
            b'<' if dc != -1 => continue,
            b'>' if dc != 1 => continue,
            b'^' if dr != -1 => continue,
            b'v' if dr != 1 => continue,
            _ => {}
        }

        if seen.mark(nr * cols + nc) {
            if let Some(path) = dfs(grid, seen, rows, cols, nr, nc) {
                len = Some(path + 1).max(len);
            }

            seen.unset(nr * cols + nc);
        }
    }

    len
}
