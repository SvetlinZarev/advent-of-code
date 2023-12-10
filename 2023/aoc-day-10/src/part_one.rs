use crate::common::{find_start, initial_direction, Direction};

pub fn part_one(grid: &Vec<Vec<u8>>) -> usize {
    let (sr, sc) = find_start(grid);
    let mut dir = initial_direction(grid, sr, sc);

    let (mut r, mut c) = (sr, sc);
    let mut steps = 0;

    while (sr, sc) != (r, c) || steps == 0 {
        steps += 1;

        let Some((nr, nc)) = dir.apply(r, c) else {
            panic!("Cannot go {:?} from {}:{}", dir, r, c);
        };
        (r, c) = (nr, nc);

        dir = match grid[r][c] {
            b'|' => dir,
            b'-' => dir,
            b'L' => match dir {
                Direction::Down => dir.rotl(),
                Direction::Left => dir.rotr(),
                _ => panic!("Cannot enter 'L' from {:?}", dir),
            },
            b'J' => match dir {
                Direction::Down => dir.rotr(),
                Direction::Right => dir.rotl(),
                _ => panic!("Cannot enter 'J' from {:?}", dir),
            },
            b'7' => match dir {
                Direction::Up => dir.rotl(),
                Direction::Right => dir.rotr(),
                _ => panic!("Cannot enter '7' from {:?}", dir),
            },
            b'F' => match dir {
                Direction::Up => dir.rotr(),
                Direction::Left => dir.rotl(),
                _ => panic!("Cannot enter 'F' from {:?}", dir),
            },

            b'S' => {
                break;
            }

            _ => panic!("invalid tile: {:?}", grid[r][c]),
        };
    }

    steps / 2
}
