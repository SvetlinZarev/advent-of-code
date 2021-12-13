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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
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
