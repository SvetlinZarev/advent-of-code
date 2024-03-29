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

    pub fn collect_non_overlapping(&self, other: &Self, dst: &mut Vec<Self>) {
        let isn = match self.intersect(other) {
            Some(intersection) => intersection,
            None => {
                dst.push(self.clone());
                return;
            }
        };

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

        // Floor-0: Bellow the intersection: all, sections
        let lo = (self.a.0, self.a.1, self.a.2);
        let hi = (self.b.0, self.b.1, isn.a.2);
        if let Some(cuboid) = Cuboid::new(lo, hi) {
            dst.push(cuboid);
        }

        // Floor-2: Above the intersection: all, sections
        let lo = (self.a.0, self.a.1, isn.b.2);
        let hi = (self.b.0, self.b.1, self.b.2);
        if let Some(cuboid) = Cuboid::new(lo, hi) {
            dst.push(cuboid);
        }

        // At the left of the intersection (section, 0,3,6)
        let lo = (self.a.0, self.a.1, isn.a.2);
        let hi = (isn.a.0, self.b.1, isn.b.2);
        if let Some(cuboid) = Cuboid::new(lo, hi) {
            dst.push(cuboid);
        }

        // At the right of the intersection (section, 2,5,8)
        let lo = (isn.b.0, self.a.1, isn.a.2);
        let hi = (self.b.0, self.b.1, isn.b.2);
        if let Some(cuboid) = Cuboid::new(lo, hi) {
            dst.push(cuboid);
        }

        // floor 1, section 1 (in front of the intersection)
        let lo = (isn.a.0, self.a.1, isn.a.2);
        let hi = (isn.b.0, isn.a.1, isn.b.2);
        if let Some(x) = Cuboid::new(lo, hi) {
            dst.push(x);
        }

        // floor 1, section 7 (behind the intersection)
        let lo = (isn.a.0, isn.b.1, isn.a.2);
        let hi = (isn.b.0, self.b.1, isn.b.2);
        if let Some(x) = Cuboid::new(lo, hi) {
            dst.push(x);
        }
    }
}
