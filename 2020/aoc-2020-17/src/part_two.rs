use std::collections::{HashMap, HashSet};

const ACTIVE_SIZE: usize = 1024 * 8;
const INACTIVE_SIZE: usize = 1024 * 28;

pub fn solve(input: &mut HashSet<Coordinate>) -> usize {
    let mut cache_inactive = HashMap::with_capacity(INACTIVE_SIZE);
    let mut next_state_container = HashSet::with_capacity(ACTIVE_SIZE);

    let mut current_state = input;
    let mut next_state = &mut next_state_container;

    for _ in 0..6 {
        for root in current_state.iter().copied() {
            let mut root_active = 0;

            for neighbour in root.neighbours() {
                if current_state.contains(&neighbour) {
                    root_active += 1;
                } else {
                    *cache_inactive.entry(neighbour).or_insert(0) += 1;
                }
            }

            if root_active == 2 || root_active == 3 {
                next_state.insert(root);
            }
        }

        for (cube, count) in cache_inactive.drain() {
            if count == 3 {
                next_state.insert(cube);
            }
        }

        current_state.clear();
        std::mem::swap(&mut current_state, &mut next_state);
    }

    current_state.len()
}

pub fn parse_input(input: &str) -> HashSet<Coordinate> {
    let mut active = HashSet::with_capacity(ACTIVE_SIZE);
    let (mut x, mut y, z, w) = (0, 0, 0, 0);

    for line in input.lines() {
        for ch in line.chars() {
            match ch {
                '.' => { /*no-op*/ }
                '#' => {
                    active.insert(Coordinate::new(x, y, z, w));
                }
                _ => {
                    panic!("Unexpected input: '{}' at {}/{}/{}/{}", ch, x, y, z, w);
                }
            }

            x += 1;
        }
        x = 0;
        y += 1;
    }

    active
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Coordinate { x, y, z, w }
    }

    pub fn neighbours(self) -> NeighbourIter {
        NeighbourIter::new(self)
    }
}

pub struct NeighbourIter {
    base: Coordinate,
    next: Option<Coordinate>,
    remaining: usize,
}

impl NeighbourIter {
    pub fn new(base: Coordinate) -> Self {
        let mut next = base;
        next.x -= 1;
        next.y -= 1;
        next.z -= 1;
        next.w -= 1;

        NeighbourIter {
            base,
            next: Some(next),
            remaining: 80,
        }
    }
}

impl Iterator for NeighbourIter {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = self.next.take();

        if let Some(ref c) = to_return {
            self.remaining -= 1;
            let mut next = Coordinate::new(c.x, c.y, c.z, c.w);

            loop {
                next.x += 1;

                if next.x > self.base.x + 1 {
                    next.x = self.base.x - 1;
                    next.y += 1;
                }

                if next.y > self.base.y + 1 {
                    next.y = self.base.y - 1;
                    next.z += 1;
                }

                if next.z > self.base.z + 1 {
                    next.z = self.base.z - 1;
                    next.w += 1;
                }

                if next.w <= self.base.w + 1 {
                    if self.base == next {
                        continue;
                    }

                    self.next = Some(next);
                }

                break;
            }
        }

        to_return
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.remaining
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "\
          .#.\n\
          ..#\n\
          ###\n\
        ";

        let parsed = parse_input(input);
        assert_eq!(
            5,
            parsed.len(),
            "Input contains unexpected number of elements"
        );

        assert!(parsed.contains(&Coordinate::new(1, 0, 0, 0)));
        assert!(parsed.contains(&Coordinate::new(2, 1, 0, 0)));
        assert!(parsed.contains(&Coordinate::new(0, 2, 0, 0)));
        assert!(parsed.contains(&Coordinate::new(1, 2, 0, 0)));
        assert!(parsed.contains(&Coordinate::new(2, 2, 0, 0)));
    }

    #[test]
    fn test_neighbour_iterator_count_all() {
        let c = Coordinate::new(0, 0, 0, 0);

        assert_eq!(80, c.neighbours().count())
    }

    #[test]
    fn test_neighbour_iterator_count() {
        let c = Coordinate::new(0, 0, 0, 0);
        let mut i = c.neighbours();
        let _v = i.next();

        assert_eq!(79, i.count())
    }

    #[test]
    fn test_neighbour_iterator_size_hint() {
        let c = Coordinate::new(0, 0, 0, 0);
        let mut i = c.neighbours();

        let (min, max) = i.size_hint();
        assert_eq!(80, min);
        assert_eq!(Some(80), max);

        let _v = i.next();
        let (min, max) = i.size_hint();
        assert_eq!(79, min);
        assert_eq!(Some(79), max);
    }
}
