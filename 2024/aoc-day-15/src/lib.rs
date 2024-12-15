use aoc_shared::grid::Direction;
use std::error::Error;

const ROBOT: u8 = b'@';
const WALL: u8 = b'#';
const DOT: u8 = b'.';
const BOX: u8 = b'O';
const LB: u8 = b'[';
const RB: u8 = b']';

#[derive(Debug, Copy, Clone)]
enum ParseState {
    ParseGrid,
    ParseNewLine,
    ParseRobot,
}

pub struct Input {
    pub grid: Vec<u8>,
    pub robot: Vec<Direction>,
    pub rows: usize,
    pub cols: usize,
    pub start_row: usize,
    pub start_col: usize,
}

pub fn parse_input(input: &str) -> Result<Input, Box<dyn Error>> {
    let input = input.as_bytes();
    let mut state = ParseState::ParseGrid;

    let mut rows = 0;
    let mut cols = 0;
    let mut start_row = 0;
    let mut start_col = 0;
    let mut grid = vec![];
    let mut robot = vec![];

    for (idx, &ch) in input.iter().enumerate() {
        match state {
            ParseState::ParseGrid => match ch {
                b'\n' => {
                    state = ParseState::ParseNewLine;
                    rows += 1;
                    if cols == 0 {
                        cols = idx;
                    }
                }

                ROBOT => {
                    start_row = rows;
                    start_col = idx;
                    if cols != 0 {
                        start_col %= cols + 1;
                    }

                    grid.push(ch);
                }

                _ => grid.push(ch),
            },

            ParseState::ParseNewLine => match ch {
                b'\n' => state = ParseState::ParseRobot,
                ch => {
                    if ch == ROBOT {
                        start_row = rows;
                        start_col = 0;
                    }

                    grid.push(ch);
                    state = ParseState::ParseGrid;
                }
            },

            ParseState::ParseRobot => match ch {
                b'<' => robot.push(Direction::Left),
                b'>' => robot.push(Direction::Right),
                b'^' => robot.push(Direction::Up),
                b'v' => robot.push(Direction::Down),
                b'\n' => {}
                _ => return Err(format!("Invalid character '{}'", ch).into()),
            },
        }
    }

    assert_eq!(grid.len(), rows * cols);
    Ok(Input {
        grid,
        robot,
        rows,
        cols,
        start_row,
        start_col,
    })
}

pub fn part_one(input: &Input) -> u32 {
    let rows = input.rows;
    let cols = input.cols;

    let mut row = input.start_row;
    let mut col = input.start_col;
    let mut grid = input.grid.clone();

    for &direction in &input.robot {
        // we don;t check for overflow because the grid is surrounded by walls '#'
        let Some((r, c)) = direction.apply(row, col) else {
            continue;
        };

        match grid[r * cols + c] {
            DOT => {
                grid[r * cols + c] = ROBOT;
                grid[row * cols + col] = DOT;
            }

            WALL => {
                continue;
            }

            BOX => {
                if !move_box(&mut grid, cols, direction, row, col) {
                    continue;
                }

                grid[row * cols + col] = DOT;
            }

            _ => unreachable!("{}", grid[r * cols + c] as char),
        }

        row = r;
        col = c;
    }

    score::<BOX>(rows, cols, &grid)
}

fn move_box(grid: &mut [u8], cols: usize, dir: Direction, r: usize, c: usize) -> bool {
    let Some((y, x)) = dir.apply(r, c) else {
        return false;
    };

    if grid[y * cols + x] == WALL {
        return false;
    }

    if grid[y * cols + x] == DOT || move_box(grid, cols, dir, y, x) {
        grid[y * cols + x] = grid[r * cols + c];
        return true;
    };

    false
}

pub fn part_two(input: &Input) -> u32 {
    let rows = input.rows;
    let cols = input.cols * 2;

    let mut row = input.start_row;
    let mut col = input.start_col * 2;
    let mut grid = Vec::with_capacity(rows * cols);

    for &ch in &input.grid {
        match ch {
            DOT => {
                grid.push(DOT);
                grid.push(DOT);
            }

            WALL => {
                grid.push(WALL);
                grid.push(WALL);
            }

            BOX => {
                grid.push(LB);
                grid.push(RB);
            }

            ROBOT => {
                grid.push(ROBOT);
                grid.push(DOT);
            }

            _ => unreachable!("{}", ch as char),
        }
    }

    for &direction in &input.robot {
        // we don;t check for overflow because the grid is surrounded by walls '#'
        let Some((r, c)) = direction.apply(row, col) else {
            continue;
        };

        match grid[r * cols + c] {
            DOT => {
                grid[r * cols + c] = ROBOT;
                grid[row * cols + col] = DOT;
            }

            WALL => {
                continue;
            }

            LB | RB => {
                if !can_push_box(&mut grid, cols, direction, row, col) {
                    continue;
                }

                push_box(&mut grid, cols, direction, row, col);
            }

            _ => unreachable!("{}", grid[r * cols + c] as char),
        }

        row = r;
        col = c;
    }

    score::<LB>(rows, cols, &grid)
}

fn can_push_box(grid: &[u8], cols: usize, dir: Direction, r: usize, c: usize) -> bool {
    let Some((y, x)) = dir.apply(r, c) else {
        return false;
    };

    if grid[y * cols + x] == WALL {
        return false;
    }

    if grid[y * cols + x] == DOT {
        return true;
    }

    let direct = can_push_box(grid, cols, dir, y, x);

    let mut pair = true;
    if direct && (dir == Direction::Up || dir == Direction::Down) {
        pair = if grid[y * cols + x] == LB {
            can_push_box(grid, cols, dir, y, x + 1)
        } else {
            can_push_box(grid, cols, dir, y, x - 1)
        }
    }

    direct & pair
}

fn push_box(grid: &mut [u8], cols: usize, dir: Direction, r: usize, c: usize) {
    let mut buffer = [(0usize, 0usize, 0u8); 2];
    let mut queue = buffer.as_mut_slice();

    match grid[r * cols + c] {
        DOT => return,

        ROBOT => {
            queue[0] = (r, c, grid[r * cols + c]);
            queue = &mut queue[..1];

            grid[r * cols + c] = DOT;
        }

        LB => {
            queue[0] = (r, c, grid[r * cols + c]);
            queue[1] = (r, c + 1, grid[r * cols + c + 1]);

            grid[r * cols + c] = DOT;
            grid[r * cols + c + 1] = DOT;
        }

        RB => {
            queue[0] = (r, c, grid[r * cols + c]);
            queue[1] = (r, c - 1, grid[r * cols + c - 1]);

            grid[r * cols + c] = DOT;
            grid[r * cols + c - 1] = DOT;
        }

        _ => unreachable!("{}", grid[r * cols + c] as char),
    }

    handle_boxes(grid, cols, dir, &queue);
}

fn handle_boxes(grid: &mut [u8], cols: usize, dir: Direction, next: &[(usize, usize, u8)]) {
    for &(r, c, _) in next {
        let Some((y, x)) = dir.apply(r, c) else {
            unreachable!();
        };

        push_box(grid, cols, dir, y, x);
    }

    for &(r, c, v) in next {
        let Some((y, x)) = dir.apply(r, c) else {
            unreachable!();
        };

        grid[y * cols + x] = v;
    }
}

fn score<const CH: u8>(rows: usize, cols: usize, grid: &[u8]) -> u32 {
    let mut answer = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row * cols + col] == CH {
                answer += 100 * row as u32 + col as u32;
            }
        }
    }

    answer
}
#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_parsing() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        assert_eq!(50, parsed.rows);
        assert_eq!(50, parsed.cols);
        assert_eq!(parsed.rows * parsed.cols, parsed.grid.len(),);

        assert_eq!(24, parsed.start_row);
        assert_eq!(24, parsed.start_col);
    }

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_one(&input);
        assert_eq!(1517819, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_two(&input);
        assert_eq!(1538862, answer);
    }
}
