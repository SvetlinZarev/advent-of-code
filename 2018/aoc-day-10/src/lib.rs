use regex::Regex;
use std::error::Error;
use std::sync::LazyLock;

static REGEX_PARSE_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^position=<\s*(?<X>-?\d+),\s*(?<Y>-?\d+)> velocity=<\s*(?<VX>-?\d+),\s*(?<VY>-?\d+)>$"#,
    )
    .unwrap()
});

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Self { x, y }
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

impl Velocity {
    pub fn new(x: i32, y: i32) -> Velocity {
        Self { x, y }
    }
}

pub fn parse_input(input: &str) -> Result<(Vec<Point>, Vec<Velocity>), Box<dyn Error>> {
    let mut points = vec![];
    let mut velocities = vec![];

    for line in input.lines() {
        let Some(cap) = REGEX_PARSE_LINE.captures(line) else {
            return Err(format!("Could not parse line: {}", line).into());
        };

        let Some(x) = cap.name("X").map(|x| x.as_str()) else {
            return Err(format!("Could not parse X: {}", line).into());
        };

        let Some(y) = cap.name("Y").map(|y| y.as_str()) else {
            return Err(format!("Could not parse Y: {}", line).into());
        };

        let Some(vx) = cap.name("VX").map(|vx| vx.as_str()) else {
            return Err(format!("Could not parse VX: {}", line).into());
        };

        let Some(vy) = cap.name("VY").map(|vy| vy.as_str()) else {
            return Err(format!("Could not parse VY: {}", line).into());
        };

        let p = Point::new(x.parse()?, y.parse()?);
        let v = Velocity::new(vx.parse()?, vy.parse()?);

        points.push(p);
        velocities.push(v);
    }

    Ok((points, velocities))
}

pub fn part_one(points: &[Point], velocities: &[Velocity]) -> String {
    assert_eq!(velocities.len(), points.len());

    let mut points = points.to_vec();
    let mut next_points = vec![Point::default(); points.len()];

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;

    let mut width = i32::MAX;
    let mut height = i32::MAX;

    loop {
        let mut n_min_x = i32::MAX;
        let mut n_max_x = i32::MIN;

        let mut n_min_y = i32::MAX;
        let mut n_max_y = i32::MIN;

        for idx in 0..points.len() {
            let mut p = points[idx];
            p.x += velocities[idx].x;
            p.y += velocities[idx].y;

            n_min_x = p.x.min(n_min_x);
            n_min_y = p.y.min(n_min_y);

            n_max_x = p.x.max(n_max_x);
            n_max_y = p.y.max(n_max_y);

            next_points[idx] = p;
        }

        let n_width = n_max_x - n_min_x;
        let n_height = n_max_y - n_min_y;

        if width < n_width || height < n_height {
            break;
        }

        std::mem::swap(&mut points, &mut next_points);

        width = n_width;
        height = n_height;

        min_x = n_min_x;
        min_y = n_min_y;
    }

    let width = width + 2; // +1 additional character for the new-line separator
    let height = height + 1;

    let mut answer = vec![' '; (width * height) as usize];
    for row in 0..height {
        answer[(row * width + width - 1) as usize] = '\n';
    }

    for point in points {
        let row = point.y - min_y;
        let col = point.x - min_x;
        let idx = (row * width + col) as usize;

        answer[idx] = '█';
    }

    String::from_iter(answer)
}

pub fn part_two(points: &[Point], velocities: &[Velocity]) -> u32 {
    assert_eq!(velocities.len(), points.len());

    let mut points = points.to_vec();
    let mut next_points = vec![Point::default(); points.len()];

    let mut width = i32::MAX;
    let mut height = i32::MAX;

    let mut seconds = 0;

    loop {
        let mut n_min_x = i32::MAX;
        let mut n_max_x = i32::MIN;

        let mut n_min_y = i32::MAX;
        let mut n_max_y = i32::MIN;

        for idx in 0..points.len() {
            let mut p = points[idx];
            p.x += velocities[idx].x;
            p.y += velocities[idx].y;

            n_min_x = p.x.min(n_min_x);
            n_min_y = p.y.min(n_min_y);

            n_max_x = p.x.max(n_max_x);
            n_max_y = p.y.max(n_max_y);

            next_points[idx] = p;
        }

        let n_width = n_max_x - n_min_x;
        let n_height = n_max_y - n_min_y;

        if width < n_width || height < n_height {
            break;
        }

        std::mem::swap(&mut points, &mut next_points);

        width = n_width;
        height = n_height;

        seconds += 1;
    }

    seconds
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        const EXPECTED: &str = "\
█    █  ██████  █    █   ████   █████      ███   ████   ██████\n\
█   █        █  █    █  █    █  █    █      █   █    █       █\n\
█  █         █  █    █  █       █    █      █   █            █\n\
█ █         █   █    █  █       █    █      █   █           █ \n\
██         █    ██████  █       █████       █   █          █  \n\
██        █     █    █  █  ███  █  █        █   █  ███    █   \n\
█ █      █      █    █  █    █  █   █       █   █    █   █    \n\
█  █    █       █    █  █    █  █   █   █   █   █    █  █     \n\
█   █   █       █    █  █   ██  █    █  █   █   █   ██  █     \n\
█    █  ██████  █    █   ███ █  █    █   ███     ███ █  ██████\n\
";

        let input = load_text_input_from_file("inputs/input.txt");
        let (points, velocities) = parse_input(&input).unwrap();

        let answer = part_one(&points, &velocities);
        assert_eq!(EXPECTED, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (points, velocities) = parse_input(&input).unwrap();

        let answer = part_two(&points, &velocities);
        assert_eq!(10932, answer);
    }
}
