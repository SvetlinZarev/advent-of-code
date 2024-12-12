use regex::Regex;
use std::error::Error;
use std::sync::LazyLock;

static GAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)"#,
    )
    .unwrap()
});

#[derive(Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub struct Game {
    pub a: Point,
    pub b: Point,
    pub p: Point,
}

pub fn parse_input(input: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let mut games = vec![];

    for c in GAME_REGEX.captures_iter(input) {
        let Some(ax) = c.get(1) else {
            return Err("Invalid input".into());
        };

        let Some(ay) = c.get(2) else {
            return Err("Invalid input".into());
        };

        let Some(bx) = c.get(3) else {
            return Err("Invalid input".into());
        };

        let Some(by) = c.get(4) else {
            return Err("Invalid input".into());
        };

        let Some(px) = c.get(5) else {
            return Err("Invalid input".into());
        };

        let Some(py) = c.get(6) else {
            return Err("Invalid input".into());
        };

        games.push(Game {
            a: Point {
                x: ax.as_str().parse()?,
                y: ay.as_str().parse()?,
            },
            b: Point {
                x: bx.as_str().parse()?,
                y: by.as_str().parse()?,
            },
            p: Point {
                x: px.as_str().parse()?,
                y: py.as_str().parse()?,
            },
        })
    }

    Ok(games)
}

pub fn part_one(input: &[Game]) -> u64 {
    let mut tokens = 0;

    for g in input {
        let a = (g.b.y * g.p.x - g.b.x * g.p.y) / (g.b.y * g.a.x - g.b.x * g.a.y);
        let ok_a = (g.b.y * g.p.x - g.b.x * g.p.y) % (g.b.y * g.a.x - g.b.x * g.a.y);

        let b = (g.a.y * g.p.x - g.a.x * g.p.y) / (g.a.y * g.b.x - g.a.x * g.b.y);
        let ok_b = (g.a.y * g.p.x - g.a.x * g.p.y) % (g.a.y * g.b.x - g.a.x * g.b.y);

        if ok_a == 0 && ok_b == 0 {
            debug_assert!((0..=100).contains(&a));
            debug_assert!((0..=100).contains(&b));

            tokens += (3 * a + b) as u64;
        }
    }

    tokens
}

pub fn part_two(input: &[Game]) -> u64 {
    const DIFF: i64 = 10_000_000_000_000;

    let mut tokens = 0;

    for g in input {
        let a = (g.b.y * (g.p.x + DIFF) - g.b.x * (g.p.y + DIFF)) / (g.b.y * g.a.x - g.b.x * g.a.y);
        let ok_a =
            (g.b.y * (g.p.x + DIFF) - g.b.x * (g.p.y + DIFF)) % (g.b.y * g.a.x - g.b.x * g.a.y);

        let b = (g.a.y * (g.p.x + DIFF) - g.a.x * (g.p.y + DIFF)) / (g.a.y * g.b.x - g.a.x * g.b.y);
        let ok_b =
            (g.a.y * (g.p.x + DIFF) - g.a.x * (g.p.y + DIFF)) % (g.a.y * g.b.x - g.a.x * g.b.y);

        if ok_a == 0 && ok_b == 0 {
            tokens += (3 * a + b) as u64;
        }
    }

    tokens
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
        assert_eq!(36_954, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two(&parsed);
        assert_eq!(79_352_015_273_424, answer);
    }
}
