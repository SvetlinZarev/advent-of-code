use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const ZERO: Point = Point::new(0, 0);
    pub const UP: Point = Point::new(0, -1);
    pub const DOWN: Point = Point::new(0, 1);
    pub const LEFT: Point = Point::new(-1, 0);
    pub const RIGHT: Point = Point::new(1, 0);

    #[inline(always)]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn from_direction(dir: &str) -> Result<Point, String> {
        Ok(match dir {
            "U" | "N" => Point::UP,
            "D" | "S" => Point::DOWN,
            "R" | "E" => Point::RIGHT,
            "L" | "W" => Point::LEFT,
            _ => return Err(format!("Invalid direction: {:?}", dir)),
        })
    }

    #[inline(always)]
    pub const fn manhattan(self, other: Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i64> for Point {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl AddAssign for Point {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Point {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<i64> for Point {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
