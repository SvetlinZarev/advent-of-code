use std::ops::Add;

use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

#[derive(Debug, Clone)]
pub struct X3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Add for X3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HailStone {
    position: X3D,
    velocity: X3D,
}

pub fn parse_input(input: &str) -> Vec<HailStone> {
    input
        .lines()
        .map(|l| l.split_once('@').unwrap())
        .map(|(a, b)| (parse_x3d(a.trim()), parse_x3d(b.trim())))
        .map(|(a, b)| HailStone {
            position: a,
            velocity: b,
        })
        .collect()
}

fn parse_x3d(line: &str) -> X3D {
    let (x, rest) = line.split_once(',').unwrap();
    let (y, z) = rest.split_once(',').unwrap();

    X3D {
        x: x.trim().parse().unwrap(),
        y: y.trim().parse().unwrap(),
        z: z.trim().parse().unwrap(),
    }
}

pub fn part_one(input: impl AsRef<[HailStone]>) -> usize {
    const MIN: f64 = 200_000_000_000_000.0;
    const MAX: f64 = 400_000_000_000_000.0;

    let input = input.as_ref();
    let mut intersections = 0;

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if let Some(((x, y), (t0, t1))) = intersects_xy(&input[i], &input[j]) {
                if (MIN..=MAX).contains(&x) && (MIN..=MAX).contains(&y) && t0 >= 0.0 && t1 >= 0.0 {
                    intersections += 1;
                }
            }
        }
    }

    intersections
}

fn intersects_xy(
    a: &HailStone,
    b: &HailStone,
) -> Option<(/* XY */ (f64, f64), /* Time */ (f64, f64))> {
    let (x0, y0, vx0, vy0) = (a.position.x, a.position.y, a.velocity.x, a.velocity.y);
    let (x1, y1, vx1, vy1) = (b.position.x, b.position.y, b.velocity.x, b.velocity.y);

    let tn = (y0 - y1) * vx1 - (x0 - x1) * vy1;
    let un = (y0 - y1) * vx0 - (x0 - x1) * vy0;

    let d = vx0 * vy1 - vy0 * vx1;
    if d == 0 {
        return None;
    }

    let x = (x0 as f64) + (vx0 as f64 / d as f64) * tn as f64;
    let y = (y0 as f64) + (vy0 as f64 / d as f64) * tn as f64;

    Some(((x, y), (tn as f64 / d as f64, un as f64 / d as f64)))
}

// solution from: https://github.com/arthomnix/aoc23/blob/master/src/days/day24.rs
pub fn part_two(input: impl AsRef<[HailStone]>) -> i64 {
    let hailstones = input.as_ref();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // positions
    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");

    // velocities
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for hailstone in hailstones.iter() {
        let pxn = Int::from_i64(&ctx, hailstone.position.x);
        let pyn = Int::from_i64(&ctx, hailstone.position.y);
        let pzn = Int::from_i64(&ctx, hailstone.position.z);

        let vxn = Int::from_i64(&ctx, hailstone.velocity.x);
        let vyn = Int::from_i64(&ctx, hailstone.velocity.y);
        let vzn = Int::from_i64(&ctx, hailstone.velocity.z);

        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();

    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    x + y + z
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);

        let answer = part_one(&parsed);
        assert_eq!(23_760, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);

        let answer = part_two(&parsed);
        assert_eq!(888_708_704_663_413, answer);
    }
}
