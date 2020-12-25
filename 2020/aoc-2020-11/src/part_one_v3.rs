#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Empty,
    Occupied,
    Floor,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    state: State,
    neighbours: u8,
}

#[derive(Debug, Clone)]
pub struct Seats {
    grid: Vec<Tile>,
    num_row: usize,
    num_col: usize,
}

impl Seats {
    pub fn state(&mut self, r: usize, c: usize) -> State {
        if cfg!(debug_assertions) {
            if r >= self.num_row || c >= self.num_col {
                panic!("Coordinates out of range: R={}; C={};", r, c);
            }
        }

        let idx = r * self.num_col + c;
        self.grid[idx].state
    }

    pub fn neighbours(&mut self, r: usize, c: usize, count: u8) -> u8 {
        if cfg!(debug_assertions) {
            if r >= self.num_row || c >= self.num_col {
                panic!("Coordinates out of range: R={}; C={};", r, c);
            }
        }

        let idx = r * self.num_col + c;
        let neighbours = self.grid[idx].neighbours;
        self.grid[idx].neighbours = count;
        neighbours
    }
}

//(changed, occupied)
fn part_one_transition(tile: &mut Tile) -> (bool, bool) {
    let (changed, occupied) = match tile.state {
        State::Empty => {
            if tile.neighbours == 0 {
                tile.state = State::Occupied;
                (true, true)
            } else {
                (false, false)
            }
        }

        State::Occupied => {
            if tile.neighbours >= 4 {
                tile.state = State::Empty;
                (true, false)
            } else {
                (false, true)
            }
        }

        State::Floor => (false, false),
    };

    tile.neighbours = 0;
    (changed, occupied)
}

pub fn solve(grid: &mut Seats) -> usize {
    let mut has_changed = true;
    let mut occupied_seats = 0;

    while has_changed {
        for r in 0..grid.num_row {
            for c in 0..grid.num_col {
                if State::Floor == grid.state(r, c) {
                    continue;
                }

                let mut neighbours = 0;
                if r > 0 {
                    if c > 0 {
                        if State::Occupied == grid.state(r - 1, c - 1) {
                            neighbours += 1;
                        }
                    }

                    if State::Occupied == grid.state(r - 1, c) {
                        neighbours += 1;
                    }

                    if c < grid.num_col - 1 {
                        if State::Occupied == grid.state(r - 1, c + 1) {
                            neighbours += 1;
                        }
                    }
                }

                if c > 0 {
                    if State::Occupied == grid.state(r, c - 1) {
                        neighbours += 1;
                    }
                }

                if c < grid.num_col - 1 {
                    if State::Occupied == grid.state(r, c + 1) {
                        neighbours += 1;
                    }
                }

                if r < grid.num_row - 1 {
                    if c > 0 {
                        if State::Occupied == grid.state(r + 1, c - 1) {
                            neighbours += 1;
                        }
                    }

                    if State::Occupied == grid.state(r + 1, c) {
                        neighbours += 1;
                    }

                    if c < grid.num_col - 1 {
                        if State::Occupied == grid.state(r + 1, c + 1) {
                            neighbours += 1;
                        }
                    }
                }

                grid.neighbours(r, c, neighbours);
            }
        }

        occupied_seats = 0;
        has_changed = false;

        for tile in grid.grid.iter_mut() {
            let (changed, occupied) = part_one_transition(tile);
            has_changed |= changed;

            if occupied {
                occupied_seats += 1;
            }
        }
    }

    occupied_seats
}

pub fn parse_input(input: &str) -> Seats {
    let mut seats = Seats {
        grid: vec![],
        num_col: 0,
        num_row: 0,
    };

    for line in input.lines() {
        seats.num_row += 1;
        seats.num_col = line.bytes().len();

        for b in line.as_bytes().iter().copied() {
            let state = match b {
                b'.' => State::Floor,
                b'L' => State::Empty,
                b'#' => State::Occupied,
                _ => unreachable!("Unexpected input: {}", b as char),
            };

            seats.grid.push(Tile {
                state,
                neighbours: 0,
            });
        }
    }

    seats
}
