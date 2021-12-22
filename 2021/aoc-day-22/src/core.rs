pub type Int = i32;
pub type Point = (Int, Int, Int);

#[derive(Debug, Clone)]
pub struct Command {
    pub op: Operation,
    pub cuboid: Cuboid,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    On,
    Off,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cuboid {
    pub(crate) a: Point,
    pub(crate) b: Point,
}

impl Cuboid {
    pub fn new(lo: Point, hi: Point) -> Option<Cuboid> {
        if lo.0 >= hi.0 || lo.1 >= hi.1 || lo.2 >= hi.2 {
            return None;
        }

        Some(Cuboid { a: lo, b: hi })
    }

    pub fn volume(&self) -> u64 {
        (self.b.0 - self.a.0).abs() as u64
            * (self.b.1 - self.a.1).abs() as u64
            * (self.b.2 - self.a.2).abs() as u64
    }

    pub fn intersect(&self, other: &Self) -> Option<Cuboid> {
        let xlo = self.a.0.max(other.a.0);
        let ylo = self.a.1.max(other.a.1);
        let zlo = self.a.2.max(other.a.2);

        let xhi = self.b.0.min(other.b.0);
        let yhi = self.b.1.min(other.b.1);
        let zhi = self.b.2.min(other.b.2);

        Cuboid::new((xlo, ylo, zlo), (xhi, yhi, zhi))
    }

    pub fn subtract(&self, other: &Self) -> Vec<Self> {
        let isn = match self.intersect(other) {
            Some(intersection) => intersection,
            None => return vec![self.clone()],
        };

        let mut result = Vec::with_capacity(26);

        // The cuboid has 3 "floors" each looking like that:
        //
        //   |-----|-----|-----|
        //   |  6  |  7  |  8  |
        //   |-----|-----|-----|
        //   |  3  |  4  |  5  |
        //   |-----|-----|-----|
        //   |  0  |  1  |  2  |
        //   |-----|-----|-----|
        //
        // For a total of 27 different sections.

        // floor 0, section 0
        if let Some(x) = Cuboid::new((self.a.0, self.a.1, self.a.2), (isn.a.0, isn.a.1, isn.a.2)) {
            result.push(x);
        }

        // floor 0, section 1
        if let Some(x) = Cuboid::new((isn.a.0, self.a.1, self.a.2), (isn.b.0, isn.a.1, isn.a.2)) {
            result.push(x);
        }

        // floor 0, section 2
        if let Some(x) = Cuboid::new((isn.b.0, self.a.1, self.a.2), (self.b.0, isn.a.1, isn.a.2)) {
            result.push(x);
        }

        // floor 0, section 3
        if let Some(x) = Cuboid::new((self.a.0, isn.a.1, self.a.2), (isn.a.0, isn.b.1, isn.a.2)) {
            result.push(x);
        }

        // floor 0, section 4
        if let Some(x) = Cuboid::new((isn.a.0, isn.a.1, self.a.2), (isn.b.0, isn.b.1, isn.a.2)) {
            result.push(x);
        }

        // floor 0, section 5
        if let Some(x) = Cuboid::new((isn.b.0, isn.a.1, self.a.2), (self.b.0, isn.b.1, isn.a.2)) {
            result.push(x);
        }

        // floor 0, section 6
        if let Some(x) = Cuboid::new((self.a.0, isn.b.1, self.a.2), (isn.a.0, self.b.1, isn.a.2)) {
            result.push(x);
        }

        // floor 0, section 7
        if let Some(x) = Cuboid::new((isn.a.0, isn.b.1, self.a.2), (isn.b.0, self.b.1, isn.a.2)) {
            result.push(x);
        }

        // floor 0, section 8
        if let Some(x) = Cuboid::new((isn.b.0, isn.b.1, self.a.2), (self.b.0, self.b.1, isn.a.2)) {
            result.push(x);
        }

        // floor 1, section 0
        if let Some(x) = Cuboid::new((self.a.0, self.a.1, isn.a.2), (isn.a.0, isn.a.1, isn.b.2)) {
            result.push(x);
        }

        // floor 1, section 1
        if let Some(x) = Cuboid::new((isn.a.0, self.a.1, isn.a.2), (isn.b.0, isn.a.1, isn.b.2)) {
            result.push(x);
        }

        // floor 1, section 2
        if let Some(x) = Cuboid::new((isn.b.0, self.a.1, isn.a.2), (self.b.0, isn.a.1, isn.b.2)) {
            result.push(x);
        }

        // floor 1, section 3
        if let Some(x) = Cuboid::new((self.a.0, isn.a.1, isn.a.2), (isn.a.0, isn.b.1, isn.b.2)) {
            result.push(x);
        }

        // floor 1, section 4
        // if let Some(x) = Cuboid::new((isn.a.0, isn.a.1, isn.a.2), (isn.b.0, isn.b.1, isn.b.2)) {
        //     result.push(x);
        // }

        // floor 1, section 5
        if let Some(x) = Cuboid::new((isn.b.0, isn.a.1, isn.a.2), (self.b.0, isn.b.1, isn.b.2)) {
            result.push(x);
        }

        // floor 1, section 6
        if let Some(x) = Cuboid::new((self.a.0, isn.b.1, isn.a.2), (isn.a.0, self.b.1, isn.b.2)) {
            result.push(x);
        }

        // floor 1, section 7
        if let Some(x) = Cuboid::new((isn.a.0, isn.b.1, isn.a.2), (isn.b.0, self.b.1, isn.b.2)) {
            result.push(x);
        }

        // floor 1, section 8
        if let Some(x) = Cuboid::new((isn.b.0, isn.b.1, isn.a.2), (self.b.0, self.b.1, isn.b.2)) {
            result.push(x);
        }

        // floor 2, section 0
        if let Some(x) = Cuboid::new((self.a.0, self.a.1, isn.b.2), (isn.a.0, isn.a.1, self.b.2)) {
            result.push(x);
        }

        // floor 2, section 1
        if let Some(x) = Cuboid::new((isn.a.0, self.a.1, isn.b.2), (isn.b.0, isn.a.1, self.b.2)) {
            result.push(x);
        }

        // floor 2, section 2
        if let Some(x) = Cuboid::new((isn.b.0, self.a.1, isn.b.2), (self.b.0, isn.a.1, self.b.2)) {
            result.push(x);
        }

        // floor 2, section 3
        if let Some(x) = Cuboid::new((self.a.0, isn.a.1, isn.b.2), (isn.a.0, isn.b.1, self.b.2)) {
            result.push(x);
        }

        // floor 2, section 4
        if let Some(x) = Cuboid::new((isn.a.0, isn.a.1, isn.b.2), (isn.b.0, isn.b.1, self.b.2)) {
            result.push(x);
        }

        // floor 2, section 5
        if let Some(x) = Cuboid::new((isn.b.0, isn.a.1, isn.b.2), (self.b.0, isn.b.1, self.b.2)) {
            result.push(x);
        }

        // floor 2, section 6
        if let Some(x) = Cuboid::new((self.a.0, isn.b.1, isn.b.2), (isn.a.0, self.b.1, self.b.2)) {
            result.push(x);
        }

        // floor 2, section 7
        if let Some(x) = Cuboid::new((isn.a.0, isn.b.1, isn.b.2), (isn.b.0, self.b.1, self.b.2)) {
            result.push(x);
        }

        // floor 2, section 8
        if let Some(x) = Cuboid::new((isn.b.0, isn.b.1, isn.b.2), (self.b.0, self.b.1, self.b.2)) {
            result.push(x);
        }

        return result;
    }
}
