use crate::{find_start_column, Instruction};

type MoveFunction = fn(&[Vec<u8>], (usize, usize), (usize, usize)) -> State;

const WALL: u8 = b'#';
const SIDE_LEN: usize = 50;

// HARDCODED top-left corners for my input as they appear in the
const SQUARE_START: [(usize, usize); 6] = [
    // TOP SIDE
    (0, 50),
    // RIGHT SIDE
    (0, 100),
    // LEFT SIDE
    (100, 0),
    // FRONT SIDE
    (50, 50),
    // BACK SIDE
    (150, 0),
    // BOTTOM
    (100, 50),
];

pub fn part_two(map: &[Vec<u8>], instructions: &[Instruction]) -> usize {
    let mut c = find_start_column(map);
    let mut r = 0;

    // Initial state
    let mut side = Side::Top;
    let mut facing = Facing::Right;

    'next: for instr in instructions.iter().copied() {
        match instr {
            Instruction::RotR => facing = facing.rotate_right(),
            Instruction::RotL => facing = facing.rotate_left(),
            Instruction::Move(steps) => {
                let mut top_left = SQUARE_START[side.index()];
                let mut move_fn = select_move_function::<SIDE_LEN>(facing);

                let mut steps = steps.get();
                while steps > 0 {
                    match move_fn(map, top_left, (r, c)) {
                        State::Wall => {
                            // we hit a wall, cannot move in that direction anymore
                            continue 'next;
                        }

                        State::Moved(rx, cx) => {
                            r = rx;
                            c = cx;
                            steps -= 1;
                        }

                        State::OutOfBounds => {
                            // we need to select a new "side" and new "facing"
                            // then check if the new position is a wall
                            let (new_side, new_facing, rx, cx) = wrap(side, facing, (r, c));

                            // The new position is a wall, so we cannot move there
                            if map[rx][cx] == WALL {
                                continue 'next;
                            }

                            // the new position is ok, so we should update our
                            // state: side, facing, position, top-left corner
                            side = new_side;
                            facing = new_facing;
                            move_fn = select_move_function::<SIDE_LEN>(facing);
                            top_left = SQUARE_START[side.index()];
                            r = rx;
                            c = cx;
                            // we've moved to another side of the cube
                            steps -= 1;
                        }
                    }
                }
            }
        }
    }

    //println!("R: {}; C: {}; F: {}", r + 1, c + 1, facing.index());
    1000 * (r + 1) + 4 * (c + 1) + facing.index()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn rotate_right(self) -> Self {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }

    fn rotate_left(self) -> Self {
        match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }

    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Side {
    Top = 0,
    Right = 1,
    Left = 2,
    Front = 3,
    Back = 4,
    Down = 5,
}

impl Side {
    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Moved(usize, usize),
    Wall,
    OutOfBounds,
}

fn up<const N: usize>(
    grid: &[Vec<u8>],
    top_left: (usize, usize),
    position: (usize, usize),
) -> State {
    let (r, c) = position;
    let (r0, c0) = top_left;
    let (r1, c1) = (r0 + N - 1, c0 + N - 1);

    // The r1/c1 bounds are inclusive! Verify that we are within the square
    assert!(
        r0 <= r && r <= r1,
        "r0 ({}) <= r ({}) <= r1 ({})",
        r0,
        r,
        r1
    );
    assert!(
        c0 <= c && c <= c1,
        "c0 ({}) <= c ({}) <= c1 ({})",
        c0,
        c,
        c1
    );

    // Verify that there is such square
    assert!(grid.len() >= r1);
    assert!(grid[r0].len() >= c1);
    assert!(grid[r1].len() >= c1);

    if r == 0 || r - 1 < r0 {
        return State::OutOfBounds;
    }

    if grid[r - 1][c] == WALL {
        return State::Wall;
    }

    State::Moved(r - 1, c)
}

fn down<const N: usize>(
    grid: &[Vec<u8>],
    top_left: (usize, usize),
    position: (usize, usize),
) -> State {
    let (r, c) = position;
    let (r0, c0) = top_left;
    let (r1, c1) = (r0 + N - 1, c0 + N - 1);

    // The r1/c1 bounds are inclusive! Verify that we are within the square
    assert!(
        r0 <= r && r <= r1,
        "r0 ({}) <= r ({}) <= r1 ({})",
        r0,
        r,
        r1
    );
    assert!(
        c0 <= c && c <= c1,
        "c0 ({}) <= c ({}) <= c1 ({})",
        c0,
        c,
        c1
    );

    // Verify that there is such square
    assert!(grid.len() >= r1);
    assert!(grid[r0].len() >= c1);
    assert!(grid[r1].len() >= c1);

    if r + 1 > r1 {
        return State::OutOfBounds;
    }

    if grid[r + 1][c] == WALL {
        return State::Wall;
    }

    State::Moved(r + 1, c)
}

fn left<const N: usize>(
    grid: &[Vec<u8>],
    top_left: (usize, usize),
    position: (usize, usize),
) -> State {
    let (r, c) = position;
    let (r0, c0) = top_left;
    let (r1, c1) = (r0 + N - 1, c0 + N - 1);

    // The r1/c1 bounds are inclusive! Verify that we are within the square
    assert!(
        r0 <= r && r <= r1,
        "r0 ({}) <= r ({}) <= r1 ({})",
        r0,
        r,
        r1
    );
    assert!(
        c0 <= c && c <= c1,
        "c0 ({}) <= c ({}) <= c1 ({})",
        c0,
        c,
        c1
    );

    // Verify that there is such square
    assert!(grid.len() >= r1);
    assert!(grid[r0].len() >= c1);
    assert!(grid[r1].len() >= c1);

    if c == 0 || c - 1 < c0 {
        return State::OutOfBounds;
    }

    if grid[r][c - 1] == WALL {
        return State::Wall;
    }

    State::Moved(r, c - 1)
}

fn right<const N: usize>(
    grid: &[Vec<u8>],
    top_left: (usize, usize),
    position: (usize, usize),
) -> State {
    let (r, c) = position;
    let (r0, c0) = top_left;
    let (r1, c1) = (r0 + N - 1, c0 + N - 1);

    // The r1/c1 bounds are inclusive! Verify that we are within the square
    assert!(
        r0 <= r && r <= r1,
        "r0 ({}) <= r ({}) <= r1 ({})",
        r0,
        r,
        r1
    );
    assert!(
        c0 <= c && c <= c1,
        "c0 ({}) <= c ({}) <= c1 ({})",
        c0,
        c,
        c1
    );

    // Verify that there is such square
    assert!(grid.len() >= r1);
    assert!(grid[r0].len() >= c1);
    assert!(grid[r1].len() >= c1);

    if c + 1 > c1 {
        return State::OutOfBounds;
    }

    if grid[r][c + 1] == WALL {
        return State::Wall;
    }

    State::Moved(r, c + 1)
}

fn wrap(side: Side, facing: Facing, position: (usize, usize)) -> (Side, Facing, usize, usize) {
    match side {
        Side::Top => wrap_top(facing, position),
        Side::Right => wrap_right(facing, position),
        Side::Left => wrap_left(facing, position),
        Side::Front => wrap_front(facing, position),
        Side::Back => wrap_back(facing, position),
        Side::Down => wrap_down(facing, position),
    }
}

fn wrap_top(facing: Facing, (r, c): (usize, usize)) -> (Side, Facing, usize, usize) {
    match facing {
        Facing::Right => (Side::Right, Facing::Right, r, c + 1),
        Facing::Down => (Side::Front, Facing::Down, r + 1, c),
        Facing::Left => (Side::Left, Facing::Right, 3 * SIDE_LEN - 1 - r, 0),
        Facing::Up => (Side::Back, Facing::Right, 3 * SIDE_LEN + c - SIDE_LEN, 0),
    }
}

fn wrap_right(facing: Facing, (r, c): (usize, usize)) -> (Side, Facing, usize, usize) {
    match facing {
        Facing::Right => (
            Side::Down,
            Facing::Left,
            2 * SIDE_LEN + (SIDE_LEN - 1 - r),
            99,
        ),
        Facing::Down => (
            Side::Front,
            Facing::Left,
            SIDE_LEN + c - 2 * SIDE_LEN,
            2 * SIDE_LEN - 1,
        ),
        Facing::Left => (Side::Top, Facing::Left, r, c - 1),
        Facing::Up => (Side::Back, Facing::Up, 4 * SIDE_LEN - 1, c - 2 * SIDE_LEN),
    }
}

fn wrap_left(facing: Facing, (r, c): (usize, usize)) -> (Side, Facing, usize, usize) {
    match facing {
        Facing::Right => (Side::Down, Facing::Right, r, c + 1),
        Facing::Down => (Side::Back, Facing::Down, r + 1, c),
        Facing::Left => (Side::Top, Facing::Right, 3 * SIDE_LEN - r - 1, SIDE_LEN),
        Facing::Up => (Side::Front, Facing::Right, SIDE_LEN + c, SIDE_LEN),
    }
}

fn wrap_front(facing: Facing, (r, c): (usize, usize)) -> (Side, Facing, usize, usize) {
    match facing {
        Facing::Right => (
            Side::Right,
            Facing::Up,
            SIDE_LEN - 1,
            2 * SIDE_LEN + r - SIDE_LEN,
        ),
        Facing::Down => (Side::Down, Facing::Down, r + 1, c),
        Facing::Left => (Side::Left, Facing::Down, 2 * SIDE_LEN, r - SIDE_LEN),
        Facing::Up => (Side::Top, Facing::Up, r - 1, c),
    }
}
fn wrap_back(facing: Facing, (r, c): (usize, usize)) -> (Side, Facing, usize, usize) {
    match facing {
        Facing::Right => (
            Side::Down,
            Facing::Up,
            3 * SIDE_LEN - 1,
            SIDE_LEN + r - 3 * SIDE_LEN,
        ),
        Facing::Down => (Side::Right, Facing::Down, 0, 2 * SIDE_LEN + c),
        Facing::Left => (Side::Top, Facing::Down, 0, r - 3 * SIDE_LEN + SIDE_LEN),
        Facing::Up => (Side::Left, Facing::Up, r - 1, c),
    }
}
fn wrap_down(facing: Facing, (r, c): (usize, usize)) -> (Side, Facing, usize, usize) {
    match facing {
        Facing::Right => (
            Side::Right,
            Facing::Left,
            3 * SIDE_LEN - r - 1,
            3 * SIDE_LEN - 1,
        ),
        Facing::Down => (
            Side::Back,
            Facing::Left,
            3 * SIDE_LEN + c - SIDE_LEN,
            SIDE_LEN - 1,
        ),
        Facing::Left => (Side::Left, Facing::Left, r, c - 1),
        Facing::Up => (Side::Front, Facing::Up, r - 1, c),
    }
}

fn select_move_function<const N: usize>(facing: Facing) -> MoveFunction {
    match facing {
        Facing::Right => right::<N>,
        Facing::Down => down::<N>,
        Facing::Left => left::<N>,
        Facing::Up => up::<N>,
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::parse_input;
    use crate::part_two::part_two;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (map, instr) = parse_input(input);

        let answer = part_two(&map, &instr);
        assert_eq!(104385, answer);
    }
}
