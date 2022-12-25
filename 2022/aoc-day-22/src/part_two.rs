use crate::{find_start_column, Instruction};

type MoveFunction = fn(&[Vec<u8>], (usize, usize), (usize, usize)) -> State;

const WALL: u8 = b'#';

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
    let (mut r, mut c) = find_start_column(map);

    // Initial state
    let mut side = Side::Top;
    let mut facing = Facing::Right;

    'next: for instr in instructions.iter().copied() {
        match instr {
            Instruction::RotR => facing = facing.rotate_right(),
            Instruction::RotL => facing = facing.rotate_left(),
            Instruction::Move(steps) => {
                let mut top_left = SQUARE_START[side.index()];
                let mut move_fn = select_move_function(side, facing);

                let mut steps = steps;
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
                            move_fn = select_move_function(side, facing);
                            top_left = SQUARE_START[side.index()];

                            // we've moved to another side of the cube
                            steps -= 1;
                        }
                    }
                }
            }
        }
    }

    1000 * r + 4 * c + facing.index()
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    let (r1, c1) = (r0 + N, c0 + N);

    // The r1/c1 bounds are exclusive! verify that we are withing the square
    assert!(r0 <= r && r < r1);
    assert!(c0 <= c && c < c1);

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
    let (r1, c1) = (r0 + N, c0 + N);

    // The r1/c1 bounds are exclusive! verify that we are withing the square
    assert!(r0 <= r && r < r1);
    assert!(c0 <= c && c < c1);

    // Verify that there is such square
    assert!(grid.len() >= r1);
    assert!(grid[r0].len() >= c1);
    assert!(grid[r1].len() >= c1);

    if r + 1 >= r1 {
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
    let (r1, c1) = (r0 + N, c0 + N);

    // The r1/c1 bounds are exclusive! verify that we are withing the square
    assert!(r0 <= r && r < r1);
    assert!(c0 <= c && c < c1);

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
    let (r1, c1) = (r0 + N, c0 + N);

    // The r1/c1 bounds are exclusive! verify that we are withing the square
    assert!(r0 <= r && r < r1);
    assert!(c0 <= c && c < c1);

    // Verify that there is such square
    assert!(grid.len() >= r1);
    assert!(grid[r0].len() >= c1);
    assert!(grid[r1].len() >= c1);

    if c + 1 >= c1 {
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

fn wrap_top(facing: Facing, position: (usize, usize)) -> (Side, Facing, usize, usize) {
    match facing {
        Facing::Right => (Side::Right, Facing::Down, position.0, position.1 + 1),
        Facing::Down => (Side::Front, Facing::Down, position.0 + 1, position.1),
        Facing::Left => {}
        Facing::Up => {}
    }
}

fn wrap_right(facing: Facing, position: (usize, usize)) -> (Side, Facing, usize, usize) {
    todo!()
}
fn wrap_left(facing: Facing, position: (usize, usize)) -> (Side, Facing, usize, usize) {
    todo!()
}
fn wrap_front(facing: Facing, position: (usize, usize)) -> (Side, Facing, usize, usize) {
    todo!()
}
fn wrap_back(facing: Facing, position: (usize, usize)) -> (Side, Facing, usize, usize) {
    todo!()
}
fn wrap_down(facing: Facing, position: (usize, usize)) -> (Side, Facing, usize, usize) {
    todo!()
}

fn select_move_function(side: Side, facing: Facing) -> MoveFunction {
    match side {
        Side::Top => select_move_function_top(facing),
        Side::Right => select_move_function_right(facing),
        Side::Left => select_move_function_left(facing),
        Side::Front => select_move_function_front(facing),
        Side::Back => select_move_function_back(facing),
        Side::Down => select_move_function_down(facing),
    }
}

fn select_move_function_top(facing: Facing) -> MoveFunction {
    match facing {
        Facing::Right => right,
        Facing::Down => down,
        Facing::Left => left,
        Facing::Up => up,
    }
}

fn select_move_function_right(facing: Facing) -> MoveFunction {
    match facing {
        Facing::Right => up,
        Facing::Down => right,
        Facing::Left => down,
        Facing::Up => left,
    }
}

fn select_move_function_left(facing: Facing) -> MoveFunction {
    match facing {
        Facing::Right => up,
        Facing::Down => right,
        Facing::Left => down,
        Facing::Up => left,
    }
}

fn select_move_function_front(facing: Facing) -> MoveFunction {
    match facing {
        Facing::Right => right,
        Facing::Down => down,
        Facing::Left => left,
        Facing::Up => up,
    }
}

fn select_move_function_back(facing: Facing) -> MoveFunction {
    match facing {
        Facing::Right => down,
        Facing::Down => left,
        Facing::Left => up,
        Facing::Up => right,
    }
}

fn select_move_function_down(facing: Facing) -> MoveFunction {
    match facing {
        Facing::Right => right,
        Facing::Down => up,
        Facing::Left => left,
        Facing::Up => down,
    }
}
