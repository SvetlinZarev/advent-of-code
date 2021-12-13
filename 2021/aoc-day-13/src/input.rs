use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Fold {
    X(u32),
    Y(u32),
}

impl Fold {
    pub fn apply(self, point: Point) -> Point {
        match self {
            Fold::X(x) => point.fold_left(x),
            Fold::Y(y) => point.fold_up(y),
        }
    }
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr = s
            .strip_prefix("fold along ")
            .ok_or_else(|| format!("invalid folding instruction (missing prefix): {}", s))?;

        let (axis, val) = instr
            .split_once('=')
            .ok_or_else(|| format!("invalid folding instruction (cannot split axis=val): {}", s))?;

        let value = val
            .parse()
            .map_err(|e| format!("invalid folding instruction ({:?}): {}", e, s))?;

        match axis {
            "x" => Ok(Fold::X(value)),
            "y" => Ok(Fold::Y(value)),
            _ => Err(format!("invalid folding instruction: {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn fold_left(self, x: u32) -> Self {
        if self.x < x {
            return self;
        }

        let shift = self.x - x;
        let xx = x
            .checked_sub(shift)
            .ok_or_else(|| {
                format!(
                    "The folding will result in negative coordinates: {:?} -> x={}",
                    self, x
                )
            })
            .unwrap();

        Self { x: xx, y: self.y }
    }

    pub fn fold_up(self, y: u32) -> Self {
        if self.y < y {
            return self;
        }

        let shift = self.y - y;
        let yy = y
            .checked_sub(shift)
            .ok_or_else(|| {
                format!(
                    "The folding will result in negative coordinates: {:?} -> y={}",
                    self, y
                )
            })
            .unwrap();

        Self { x: self.x, y: yy }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| format!("invalid point: {}", s))?;

        let x = x.parse().map_err(|e| format!("cannot parse: {:?}", e))?;
        let y = y.parse().map_err(|e| format!("cannot parse: {:?}", e))?;
        Ok(Point::new(x, y))
    }
}
