use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    pub fn is_straight(self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    pub fn is_horizontal(self) -> bool {
        self.a.y == self.b.y
    }

    pub fn is_vertical(self) -> bool {
        self.a.x == self.b.x
    }

    pub fn steps(self) -> usize {
        let x = self.a.x.max(self.b.x) - self.a.x.min(self.b.x);
        let y = self.a.y.max(self.b.y) - self.a.y.min(self.b.y);
        x.max(y) as usize
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(" -> ")
            .unwrap_or_else(|| panic!("invalid input (missing points separator): {}", s));

        let first = a
            .split_once(',')
            .and_then(|(x, y)| {
                let x = x.parse().ok()?;
                let y = y.parse().ok()?;
                Some((x, y))
            })
            .and_then(|(x, y)| Some(Point::new(x, y)));

        let first = match first {
            None => return Err(format!("failed to parse first point: {:?}", a)),
            Some(point) => point,
        };

        let second = b
            .split_once(',')
            .and_then(|(x, y)| {
                let x = x.parse().ok()?;
                let y = y.parse().ok()?;
                Some((x, y))
            })
            .and_then(|(x, y)| Some(Point::new(x, y)));

        let second = match second {
            None => return Err(format!("failed to parse first point: {:?}", b)),
            Some(point) => point,
        };

        Ok(Line::new(first, second))
    }
}
