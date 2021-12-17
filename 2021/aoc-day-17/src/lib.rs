use aoc_shared::hashing::{FnvHasher, HashBuilder};
use num::integer::Roots;
use std::collections::HashSet;
use std::ops::Sub;

pub type Unit = i32;

pub fn parse_input<S: AsRef<str>>(input: S) -> (Unit, Unit, Unit, Unit) {
    let (from, to) = input.as_ref()[13..].split_once(',').unwrap();

    let (x0, x1) = from.split_once("..").unwrap();
    let (y0, y1) = to.split_once("..").unwrap();

    (
        x0.trim()[2..].parse().unwrap(),
        x1.trim().parse().unwrap(),
        y0.trim()[2..].parse().unwrap(),
        y1.trim().parse().unwrap(),
    )
}

pub fn part_one(_x0: Unit, _x1: Unit, y0: Unit, _y1: Unit) -> Unit {
    // The distances travelled are actually arithmetic progressions
    // Thus, we can easily find the height if we know the largest
    // "step". Given that velocity decreases by 1 upwards and increases
    // by 1 downwards, we can conclude that we will always have a
    // "step" at `y==0` at the Nth step. And because the speed increases
    // on the downward direction, then after N+1 steps we will be on the
    // negative 'y' axis. Thus the largest `N+1` step would be to have or
    // 'y' be equal to the lowest point in the target area. Then then the
    // Nth step would be that lowest point - 1 (because speed increases by 1),
    // thus the max height would be the arithmetic progression of N, with a
    // step of 1)

    distance(y0.abs().sub(1))
}

fn distance(velocity: Unit) -> Unit {
    // The rules for the X/Y-velocity turn out to be
    // just a arithmetic progression from 1-to-Vx
    velocity * (velocity + 1) / 2
}

pub fn part_two(x0: Unit, x1: Unit, y0: Unit, y1: Unit) -> usize {
    assert!(x0 <= x1);
    assert!(y0 <= y1);

    let horizontal_to_zero = horizontal_velocity_zero_in_target(x0, x1);
    let horizontal = horizontal_velocity_non_zero_in_target(x0, x1);
    let vertical = vertical_velocities(y0, y1);

    let mut velocities = HashSet::with_capacity_and_hasher(
        ((x1 - x0) * (y0.abs() - y1.abs()) * 3) as usize,
        HashBuilder::<FnvHasher>::default(),
    );

    for idx in 0..horizontal_to_zero.len().min(vertical.len()) {
        for &vh in horizontal_to_zero[idx].iter() {
            for pos in idx..vertical.len() {
                for &vv in vertical[pos].iter() {
                    velocities.insert((vh, vv));
                }
            }
        }
    }

    for idx in 0..horizontal.len().min(vertical.len()) {
        for &vh in horizontal[idx].iter() {
            for &vv in vertical[idx].iter() {
                velocities.insert((vh, vv));
            }
        }
    }

    velocities.len()
}

fn horizontal_velocity_zero_in_target(x0: Unit, x1: Unit) -> Vec<Vec<Unit>> {
    assert!(x0 <= x1);

    let v_min = reverse_arithmetic_progression(x0);
    let v_max = reverse_arithmetic_progression(x1);

    horizontal_inner(x0, x1, v_min, v_max)
}

fn horizontal_velocity_non_zero_in_target(x0: Unit, x1: Unit) -> Vec<Vec<Unit>> {
    assert!(x0 <= x1);

    let v_min = reverse_arithmetic_progression(x1);
    horizontal_inner(x0, x1, v_min, x1)
}

fn horizontal_inner(x0: Unit, x1: Unit, v_min: Unit, v_max: Unit) -> Vec<Vec<Unit>> {
    let max_steps = (1 + reverse_arithmetic_progression(x1)) as usize;
    let mut velocities = vec![vec![]; max_steps];

    for initial_v in v_min..=v_max {
        let mut v = initial_v;
        let mut dist = 0;
        let mut steps = 0;

        while v > 0 && dist <= x1 {
            dist += v;
            v -= 1;
            steps += 1;

            if dist >= x0 && dist <= x1 {
                velocities[steps].push(initial_v);
            }
        }
    }
    velocities
}

fn vertical_velocities(y0: Unit, y1: Unit) -> Vec<Vec<Unit>> {
    assert!(y0 <= y1);

    // let max_negative_steps = (1 + reverse_arithmetic_progression(y0.abs())) as usize;
    //let max_steps = (y0.abs() as usize).max(max_negative_steps);
    let max_steps = 2 * y0.abs() as usize + 1;
    let mut velocities = vec![vec![]; max_steps];

    // negative velocities -> points the probe downwards
    for y in 0..=y0.abs() {
        let mut v = y;
        let mut dist = 0;
        let mut steps = 0;

        // negative velocity (i.e. downward)
        while dist <= y0.abs() {
            if dist >= y1.abs() && dist <= y0.abs() {
                velocities[steps].push(-y);
            }

            dist += v;
            v += 1;
            steps += 1;
        }
    }

    // positive velocity (upwards)
    for y in 1..y0.abs() {
        // There is one step, at the peak where the velocity is 0
        let mut v = y;
        let mut dist = 0;
        let mut steps = 2 * y as usize + 1;

        while dist <= y0.abs() {
            v += 1;
            dist += v;
            steps += 1;

            if dist >= y1.abs() && dist <= y0.abs() {
                velocities[steps].push(y);
            }
        }
    }

    velocities
}

// Get the max(x) which sums to the target sum. This is the
// well known quadratic equation, which tuns out to be the
// reverse of the arithmetic progression, which gives the sum
// for all numbers from 1 to X
fn reverse_arithmetic_progression(sum: i32) -> i32 {
    // The equation is `a*n^2 + b*n + c = 0`. In  our
    // particular case, following the arithmetic
    // progression `n * (n + 1) / 2 = sum, we can
    // calculate that:
    // => a = 1
    // => b = 1
    // => c = -2*sum
    //
    //Then we have:
    // D = sqrt(b^2 - 4*a*c)
    // roots = (-1 +- D) / 2*a
    //
    // The equation has two roots:
    // => (D - 1) / 2*a
    // => (-D - 1) / 2*a
    // => but we are only interested in the positive root

    let d = (1 + 4 * 2 * sum).sqrt();
    (d - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let (x0, x1, y0, y1) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(x0, x1, y0, y1);
        assert_eq!(25200, answer);
    }

    #[test]
    fn test_part_two() {
        let (x0, x1, y0, y1) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(x0, x1, y0, y1);
        assert_eq!(3012, answer);
    }
}
