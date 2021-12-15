use crate::Position;

pub fn a_star_pf(grid: &[Vec<u8>]) -> u32 {
    pathfinding::directed::astar::astar(
        &(0, 0),
        |&(r, c)| {
            let mut s = Vec::with_capacity(4);
            if r > 0 {
                s.push(((r - 1, c), grid[r - 1][c] as usize));
            }
            if c > 0 {
                s.push(((r, c - 1), grid[r][c - 1] as usize));
            }
            if c < grid[r].len() - 1 {
                s.push(((r, c + 1), grid[r][c + 1] as usize));
            }
            if r < grid.len() - 1 {
                s.push(((r + 1, c), grid[r + 1][c] as usize));
            }

            s
        },
        |&(r, c)| {
            Position::new(r, c).manhattan(Position::new(grid.len() - 1, grid[0].len() - 1)) as usize
        },
        |&(r, c)| r == grid.len() - 1 && c == grid[0].len() - 1,
    )
    .unwrap()
    .1 as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_numeric_grid;

    #[test]
    fn test_a_start_pf() {
        let input = parse_numeric_grid(load_text_input_from_file("inputs/input.txt"));
        let answer = a_star_pf(&input);

        assert_eq!(656, answer);
    }
}
