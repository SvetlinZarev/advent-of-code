use std::collections::VecDeque;
use std::hash::Hash;

use aoc_shared::hashing::FxHashSet;

type Int = i32;
type HashSet<T> = FxHashSet<T>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Brick {
    pub a: Point,
    pub b: Point,
}

impl Brick {
    pub fn new(start: Point, end: Point) -> Self {
        Self { a: start, b: end }
    }

    pub fn bottom(&self) -> Int {
        self.a.z
    }

    pub fn top(&self) -> Int {
        self.b.z
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: Int,
    pub y: Int,
    pub z: Int,
}

impl Point {
    pub fn new(x: Int, y: Int, z: Int) -> Self {
        Self { x, y, z }
    }
}

pub fn part_one(input: impl Into<Vec<Brick>>) -> usize {
    let mut answer = 0;
    solve(input.into(), |count| answer += (count == 0) as usize);
    answer
}

pub fn part_two(input: impl Into<Vec<Brick>>) -> usize {
    let mut answer = 0;
    solve(input.into(), |count| answer += count);
    answer
}

pub fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|l| l.split_once('~').unwrap())
        .map(|(a, b)| (parse_point(a), parse_point(b)))
        .map(|(a, b)| Brick::new(a, b))
        .collect()
}

fn parse_point(input: &str) -> Point {
    let (a, rest) = input.split_once(',').unwrap();
    let (b, c) = rest.split_once(',').unwrap();

    Point {
        x: a.parse().unwrap(),
        y: b.parse().unwrap(),
        z: c.parse().unwrap(),
    }
}

fn solve(bricks: impl Into<Vec<Brick>>, mut on_falling: impl FnMut(usize)) {
    let bricks = settle(bricks.into());
    let (above, below) = brick_supports(&bricks);

    let mut queue = VecDeque::with_capacity(bricks.len());
    let mut falling = HashSet::default();

    // how many block will fall if we disintegrate this one ?
    for brick in 0..bricks.len() {
        queue.push_back(brick);
        falling.clear();

        while let Some(brick) = queue.pop_front() {
            falling.insert(brick);

            for starting_to_fall in above[brick].iter().copied() {
                // if there is a brick, below that one, that has not fallen,
                // then this one won't fall too
                if is_subset(&below[starting_to_fall], &falling) {
                    queue.push_back(starting_to_fall);
                }
            }
        }

        // "-1" because of the brick we are disintegrating
        // does not count towards the bricks that are to fall
        on_falling(falling.len() - 1)
    }
}

// settle the bricks on the ground given the initial snapshot from the input
// and return them sorted in ascending Z order
fn settle(mut bricks: Vec<Brick>) -> Vec<Brick> {
    // Sort by the Z axis, so we can process them from bottom to top
    bricks.sort_unstable_by_key(|t| t.a.z);

    for idx in 0..bricks.len() {
        // As the ground is 0, the min bottom level
        // is 1 (i.e. sitting on the ground)
        let mut bottom = 1;

        // Check if the brick at IDX will settle on top of
        // another previously processed brick, or it will
        // fall on the ground
        for pos in 0..idx {
            if intersects_xy(&bricks[idx], &bricks[pos]) {
                bottom = bottom.max(bricks[pos].top() + 1);
            }
        }

        // Calculate by how much this brick will fall down and
        // apply the correction to its Z axis
        let fall = bricks[idx].bottom() - bottom;
        bricks[idx].a.z -= fall;
        bricks[idx].b.z -= fall;
    }

    // re-sort the bricks because changing the Z values might
    // have gotten them out of order
    bricks.sort_unstable_by_key(|t| t.a.z);
    bricks
}

fn brick_supports(bricks: &[Brick]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    // Note: assume that the bricks are sorted in Z order

    let mut above = vec![vec![]; bricks.len()];
    let mut below = vec![vec![]; bricks.len()];

    for i in 0..bricks.len() {
        for j in i + 1..bricks.len() {
            // because the bricks are sorted in Z order,
            // all following bricks will have higher Z value,
            // thus we can skip checking them
            if bricks[j].bottom() > bricks[i].top() + 1 {
                break;
            }

            if bricks[j].bottom() == bricks[i].top() + 1 {
                // Brick J sits right on top of brick I.
                //
                // If their XY planes intersect, then brick I will
                // act as a support for brick J.
                if intersects_xy(&bricks[i], &bricks[j]) {
                    // J sits on I
                    above[i].push(j);

                    // I acts as a support for J
                    below[j].push(i);
                }
            }
        }
    }

    (above, below)
}

fn intersects_xy(a: &Brick, b: &Brick) -> bool {
    intersects(a, b, |v| (v.a.x, v.b.x)) && intersects(a, b, |v| (v.a.y, v.b.y))
}

fn intersects(a: &Brick, b: &Brick, key: impl Fn(&Brick) -> (Int, Int)) -> bool {
    // https://stackoverflow.com/a/3269471
    let (a0, a1) = key(a);
    let (b0, b1) = key(b);

    a0 <= b1 && b0 <= a1
}

fn is_subset<T: Hash + Eq>(subset: &[T], set: &HashSet<T>) -> bool {
    // Because of the length check, all elements must be unique
    if subset.len() > set.len() {
        return false;
    }

    subset.iter().all(|x| set.contains(x))
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input);

        let answer = part_one(input);
        assert_eq!(517, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input);

        let answer = part_two(input);
        assert_eq!(61276, answer);
    }
}
