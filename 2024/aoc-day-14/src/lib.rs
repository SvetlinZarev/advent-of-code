use aoc_shared::grid::DIR4;
use regex::Regex;
use std::error::Error;
use std::sync::LazyLock;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

const Q_EAST: i32 = WIDTH / 2;
const Q_WEST: i32 = WIDTH / 2 + 1;

const Q_SOUTH: i32 = HEIGHT / 2;
const Q_NORTH: i32 = HEIGHT / 2 + 1;

static REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap());

#[derive(Debug, Copy, Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}
pub fn parse_input(input: &str) -> Result<Vec<Robot>, Box<dyn Error>> {
    let mut robots = vec![];

    for line in input.lines() {
        let Some(c) = REGEX.captures(line) else {
            return Err("Invalid input".into());
        };

        let Some(x) = c.get(1).and_then(|m| Some(m.as_str())) else {
            return Err("Invalid input".into());
        };

        let Some(y) = c.get(2).and_then(|m| Some(m.as_str())) else {
            return Err("Invalid input".into());
        };

        let Some(vx) = c.get(3).and_then(|m| Some(m.as_str())) else {
            return Err("Invalid input".into());
        };

        let Some(vy) = c.get(4).and_then(|m| Some(m.as_str())) else {
            return Err("Invalid input".into());
        };

        robots.push(Robot {
            x: x.parse()?,
            y: y.parse()?,
            vx: vx.parse()?,
            vy: vy.parse()?,
        })
    }

    Ok(robots)
}

pub fn part_one(robots: &Vec<Robot>) -> u32 {
    const SECONDS: i32 = 100;

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for robot in robots.iter() {
        let x = (robot.x + robot.vx * SECONDS).wrapping_rem_euclid(WIDTH);
        let y = (robot.y + robot.vy * SECONDS).wrapping_rem_euclid(HEIGHT);

        #[allow(non_contiguous_range_endpoints)]
        match (x, y) {
            (0..Q_EAST, 0..Q_SOUTH) => q1 += 1,
            (0..Q_EAST, Q_NORTH..HEIGHT) => q3 += 1,
            (Q_WEST..WIDTH, 0..Q_SOUTH) => q2 += 1,
            (Q_WEST..WIDTH, Q_NORTH..HEIGHT) => q4 += 1,
            _ => {}
        }
    }

    q1 * q2 * q3 * q4
}

pub fn part_two_v1(robots: &Vec<Robot>) -> u32 {
    // Randomly assume that the Christmas tree is somewhat in the center
    const CENTER_Y: usize = HEIGHT as usize / 2;
    const CENTER_X: usize = WIDTH as usize / 2;

    let mut grid = vec![0; WIDTH as usize * HEIGHT as usize];
    let mut stack = vec![];

    for seconds in 1..1_000_000 {
        for robot in robots.iter() {
            let x = (robot.x + robot.vx * seconds).wrapping_rem_euclid(WIDTH);
            let y = (robot.y + robot.vy * seconds).wrapping_rem_euclid(HEIGHT);

            grid[y as usize * WIDTH as usize + x as usize] = seconds;
        }

        if grid[CENTER_Y * WIDTH as usize + CENTER_X] == seconds {
            stack.push((CENTER_Y as isize, CENTER_X as isize));

            let mut count = 0;
            while let Some((r, c)) = stack.pop() {
                count += 1;

                for (dr, dc) in DIR4 {
                    let y = r + dr;
                    let x = c + dc;

                    if !(0..HEIGHT as isize).contains(&y) && !(0..WIDTH as isize).contains(&x) {
                        continue;
                    }

                    if grid[y as usize * WIDTH as usize + x as usize] != seconds {
                        continue;
                    }

                    grid[y as usize * WIDTH as usize + x as usize] = -seconds;
                    stack.push((y, x));
                }
            }

            if count >= robots.len() / 3 {
                return seconds as u32;
            }
        }
    }

    0
}

pub fn part_two_v2(robots: &Vec<Robot>) -> u64 {
    // Randomly assume that the Christmas tree is somewhat in the center
    const CY: u32 = HEIGHT as u32 / 2;
    const CX: u32 = WIDTH as u32 / 2;

    const CX_L: u32 = CX - 1;
    const CX_R: u32 = CX + 1;
    const CY_U: u32 = CY - 1;
    const CY_D: u32 = CY + 1;

    for seconds in 1.. {
        let mut bitmask = 0u32;

        for robot in robots.iter() {
            let x = (robot.x + robot.vx * seconds).wrapping_rem_euclid(WIDTH) as u32;
            let y = (robot.y + robot.vy * seconds).wrapping_rem_euclid(HEIGHT) as u32;

            let bit = match (y, x) {
                (CY_U, CX_L) => 0,
                (CY_U, CX) => 1,
                (CY_U, CX_R) => 2,
                (CY, CX_L) => 3,
                (CY, CX) => 4,
                (CY, CX_R) => 5,
                (CY_D, CX_L) => 6,
                (CY_D, CX) => 7,
                (CY_D, CX_R) => 8,
                _ => continue,
            };

            bitmask |= 1 << bit;

            // We try to find a square block of 9 robots around the center like that:
            //```
            // XXX
            // XXX
            // XXX
            //```
            // It's pure luck tht it works
            if bitmask == (1 << 9) - 1 {
                return seconds as u64;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_one(&parsed);
        assert_eq!(229_839_456, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two_v1(&parsed);
        assert_eq!(7138, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two_v2(&parsed);
        assert_eq!(7138, answer);
    }
}
