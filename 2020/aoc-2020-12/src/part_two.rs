#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rotation {
    None,
    Right,
    Left,
    Back,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    N(i32),
    E(i32),
    S(i32),
    W(i32),
    F(i32),
    R(Rotation),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Waypoint {
    north: i32,
    east: i32,
}

impl Waypoint {
    pub fn new(north: i32, east: i32) -> Waypoint {
        Waypoint { north, east }
    }

    pub fn rotate(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::None => self,
            Rotation::Right => Waypoint::new(-self.east, self.north),
            Rotation::Left => Waypoint::new(self.east, -self.north),
            Rotation::Back => Waypoint::new(-self.north, -self.east),
        }
    }
    pub fn change(self, dn: i32, de: i32) -> Waypoint {
        Waypoint::new(self.north + dn, self.east + de)
    }
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

    pub fn advance(self, direction: Waypoint, amount: i32) -> Position {
        Position::new(
            self.north + direction.north * amount,
            self.east + direction.east * amount,
        )
    }

    pub fn distance(self) -> i32 {
        self.north.abs() + self.east.abs()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ship {
    position: Position,
    waypoint: Waypoint,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            position: Position::new(0, 0),
            waypoint: Waypoint::new(1, 10),
        }
    }

    pub fn advance(&mut self, dir: Direction) {
        match dir {
            Direction::N(x) => self.waypoint = self.waypoint.change(x, 0),
            Direction::E(x) => self.waypoint = self.waypoint.change(0, x),
            Direction::S(x) => self.waypoint = self.waypoint.change(-x, 0),
            Direction::W(x) => self.waypoint = self.waypoint.change(0, -x),
            Direction::F(x) => self.position = self.position.advance(self.waypoint, x),
            Direction::R(r) => self.waypoint = self.waypoint.rotate(r),
        }
    }

    pub fn distance(&self) -> i32 {
        self.position.distance()
    }
}

pub fn parse_input_data(input: &str) -> Vec<Direction> {
    let mut data = Vec::with_capacity(800);
    for line in input.lines() {
        let value = line[1..].parse().unwrap();
        match &line[..1] {
            "N" => data.push(Direction::N(value)),
            "S" => data.push(Direction::S(value)),
            "E" => data.push(Direction::E(value)),
            "W" => data.push(Direction::W(value)),
            "F" => data.push(Direction::F(value)),
            "L" => {
                let rotation = match value {
                    90 => Rotation::Left,
                    180 => Rotation::Back,
                    270 => Rotation::Right,
                    360 => Rotation::None,
                    _ => panic!("Unexpected rotation: {}", line),
                };
                data.push(Direction::R(rotation));
            }
            "R" => {
                let rotation = match value {
                    90 => Rotation::Right,
                    180 => Rotation::Back,
                    270 => Rotation::Left,
                    360 => Rotation::None,
                    _ => panic!("Unexpected rotation: {}", line),
                };
                data.push(Direction::R(rotation));
            }
            _ => panic!("Invalid input: {}", line),
        }
    }
    data
}

pub fn solve(input: &[Direction]) -> i32 {
    let mut ship = Ship::new();

    for direction in input.iter().copied() {
        ship.advance(direction);
    }

    ship.distance()
}
