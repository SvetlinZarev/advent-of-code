use crate::common::{find_start, initial_direction, Direction, DIRECTIONS, START};

const LOOP_MARKER: u8 = b' ';
const NORMAL: u8 = b'N';
const MARKED: u8 = b'M';
const UNMARKED: u8 = b'X';

pub fn part_two(input: &Vec<Vec<u8>>) -> usize {
    let rows = input.len();
    let cols = input[0].len();

    let mut grid = vec![vec![UNMARKED; cols]; rows];
    let (sr, sc) = find_start(&input);
    let mut direction = initial_direction(&input, sr, sc);

    let (mut r, mut c) = (sr, sc);
    while (sr, sc) != (r, c) || grid[sr][sc] == UNMARKED {
        grid[r][c] = LOOP_MARKER;

        // Mark the INSIDE/OUTSIDE cells according to the vector
        // perpendicular to the direction of movement
        let normal = direction.normal();
        match input[r][c] {
            /*
                IMPORTANT: in my input the S can be replaced with |,
                thus I skip the logic for checking the correct replacement
                symbol. If 'S' needs to be replaced with 'F', '7', etc,
                then we should use the appropriate cell markings corresponding
                to the replacement symbol
            */
            START => mark_normal(&mut grid, r, c, normal),
            b'|' | b'-' => mark_normal(&mut grid, r, c, normal),
            b'L' => {
                if normal == Direction::Down || normal == Direction::Left {
                    mark_normal(&mut grid, r, c, Direction::Left);
                    mark_normal(&mut grid, r, c, Direction::Down);
                }
            }

            b'F' => {
                if normal == Direction::Up || normal == Direction::Left {
                    mark_normal(&mut grid, r, c, Direction::Left);
                    mark_normal(&mut grid, r, c, Direction::Up);
                }
            }

            b'7' => {
                if normal == Direction::Up || normal == Direction::Right {
                    mark_normal(&mut grid, r, c, Direction::Up);
                    mark_normal(&mut grid, r, c, Direction::Right);
                }
            }

            b'J' => {
                if normal == Direction::Down || normal == Direction::Right {
                    mark_normal(&mut grid, r, c, Direction::Down);
                    mark_normal(&mut grid, r, c, Direction::Right);
                }
            }

            _ => panic!("unexpected pipe: {:?}", input[r][c]),
        }

        // Move to the next cell
        (r, c) = match direction.apply(r, c) {
            None => panic!("Cannot go {:?} from {}:{}", direction, r, c),
            Some((r, c)) => (r, c),
        };

        direction = match input[r][c] {
            b'|' | b'-' => direction,

            b'L' => match direction {
                Direction::Down => direction.rotl(),
                Direction::Left => direction.rotr(),
                _ => panic!("Cannot enter 'L' from {:?}", direction),
            },

            b'J' => match direction {
                Direction::Down => direction.rotr(),
                Direction::Right => direction.rotl(),
                _ => panic!("Cannot enter 'J' from {:?}", direction),
            },

            b'7' => match direction {
                Direction::Up => direction.rotl(),
                Direction::Right => direction.rotr(),
                _ => panic!("Cannot enter '7' from {:?}", direction),
            },

            b'F' => match direction {
                Direction::Up => direction.rotr(),
                Direction::Left => direction.rotl(),
                _ => panic!("Cannot enter 'F' from {:?}", direction),
            },

            START => break,

            _ => panic!("invalid tile: {:?}", grid[r][c]),
        };
    }

    let mut ff_buf = vec![];
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == NORMAL {
                flood_fill(&mut ff_buf, &mut grid, r, c);
            }
        }
    }

    // Print the grid for visual inspection
    // for row in grid.iter() {
    //     println!("{}", std::str::from_utf8(row).unwrap());
    // }

    // IMPORTANT: For simplicity, assume that GRID[0][0]
    // is not enclosed inside the loop
    let target = if grid[0][0] == UNMARKED {
        MARKED
    } else {
        UNMARKED
    };

    grid.into_iter()
        .flat_map(|x| x.into_iter())
        .filter(|&x| x == target)
        .count()
}

fn mark_normal(grid: &mut Vec<Vec<u8>>, r: usize, c: usize, n: Direction) {
    if let Some((r, c)) = n.apply(r, c) {
        if r < grid.len() && c < grid[r].len() && grid[r][c] != LOOP_MARKER {
            grid[r][c] = NORMAL;
        }
    }
}

fn flood_fill(stack: &mut Vec<(usize, usize)>, grid: &mut Vec<Vec<u8>>, r: usize, c: usize) {
    stack.push((r, c));
    grid[r][c] = MARKED;

    while let Some((r, c)) = stack.pop() {
        for &dir in DIRECTIONS {
            let Some((r, c)) = dir.apply(r, c) else {
                continue;
            };

            if r >= grid.len() || c >= grid[r].len() {
                continue;
            }

            if grid[r][c] == UNMARKED {
                grid[r][c] = MARKED;
                stack.push((r, c));
            }
        }
    }
}
