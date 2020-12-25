#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rotation {
    None,
    Right,
    Left,
    Back,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Orientation {
    N,
    E,
    S,
    W,
}

impl Orientation {
    pub fn rotate(self, rot: Rotation) -> Orientation {
        match self {
            Orientation::N => match rot {
                Rotation::None => Orientation::N,
                Rotation::Right => Orientation::E,
                Rotation::Left => Orientation::W,
                Rotation::Back => Orientation::S,
            },
            Orientation::E => match rot {
                Rotation::None => Orientation::E,
                Rotation::Right => Orientation::S,
                Rotation::Left => Orientation::N,
                Rotation::Back => Orientation::W,
            },
            Orientation::S => match rot {
                Rotation::None => Orientation::S,
                Rotation::Right => Orientation::W,
                Rotation::Left => Orientation::E,
                Rotation::Back => Orientation::N,
            },
            Orientation::W => match rot {
                Rotation::None => Orientation::W,
                Rotation::Right => Orientation::N,
                Rotation::Left => Orientation::S,
                Rotation::Back => Orientation::E,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Command {
    P(Orientation, i32),
    F(i32),
    R(Rotation),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    north: i32,
    east: i32,
}

impl Position {
    pub fn new(north: i32, east: i32) -> Position {
        Position { north, east }
    }

    pub fn distance(self) -> i32 {
        self.north.abs() + self.east.abs()
    }

    pub fn change(self, dn: i32, de: i32) -> Position {
        Position::new(self.north + dn, self.east + de)
    }

    pub fn forward(self, orientation: Orientation, amount: i32) -> Position {
        match orientation {
            Orientation::N => self.change(amount, 0),
            Orientation::E => self.change(0, amount),
            Orientation::S => self.change(-amount, 0),
            Orientation::W => self.change(0, -amount),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ship {
    orientation: Orientation,
    position: Position,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            orientation: Orientation::E,
            position: Position::new(0, 0),
        }
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::P(o, x) => self.position = self.position.forward(o, x),
            Command::F(x) => self.position = self.position.forward(self.orientation, x),
            Command::R(r) => self.orientation = self.orientation.rotate(r),
        }
    }

    pub fn distance(&self) -> i32 {
        self.position.distance()
    }
}

pub fn parse_input_data(input: &str) -> Vec<Command> {
    let mut data = Vec::with_capacity(800);
    for line in input.lines() {
        let value = line[1..].parse().unwrap();
        match &line[..1] {
            "N" => data.push(Command::P(Orientation::N, value)),
            "S" => data.push(Command::P(Orientation::S, value)),
            "E" => data.push(Command::P(Orientation::E, value)),
            "W" => data.push(Command::P(Orientation::W, value)),
            "F" => data.push(Command::F(value)),
            "L" => {
                let rotation = match value {
                    90 => Rotation::Left,
                    180 => Rotation::Back,
                    270 => Rotation::Right,
                    360 => Rotation::None,
                    _ => panic!("Unexpected rotation: {}", line),
                };
                data.push(Command::R(rotation));
            }
            "R" => {
                let rotation = match value {
                    90 => Rotation::Right,
                    180 => Rotation::Back,
                    270 => Rotation::Left,
                    360 => Rotation::None,
                    _ => panic!("Unexpected rotation: {}", line),
                };
                data.push(Command::R(rotation));
            }
            _ => panic!("Invalid input: {}", line),
        }
    }
    data
}

pub fn solve(input: &[Command]) -> i32 {
    let mut ship = Ship::new();

    for command in input.iter().copied() {
        ship.execute(command);
    }

    ship.distance()
}
