use aoc_shared::grid::Point;

pub fn part_one(input: &str) -> i64 {
    let mut p = Point::ZERO;
    let mut area = 0;

    // Shoelace formula + Pick's theorem
    for line in input.lines() {
        let amount = &line[2..line.len() - 10];
        let amount = amount.parse::<i64>().unwrap();
        let direction = Point::from_direction(&line[..1]).unwrap();

        let q = p + direction * amount;
        area += (p.x * q.y - p.y * q.x) + amount;
        p = q;
    }

    area.abs() / 2 + 1
}

pub fn part_two(input: &str) -> i64 {
    let mut p = Point::ZERO;
    let mut area = 0;

    // Shoelace formula + Pick's theorem
    for line in input.lines() {
        let config = &line[line.len() - 7..line.len() - 1];
        let config = config.as_bytes();

        let amount = hex(config[0]) << 16
            | hex(config[1]) << 12
            | hex(config[2]) << 8
            | hex(config[3]) << 4
            | hex(config[4]) << 0;

        let direction = match config[5] - b'0' {
            0 => Point::RIGHT,
            1 => Point::DOWN,
            2 => Point::LEFT,
            3 => Point::UP,
            _ => panic!("invalid direction: {:?}", config[5]),
        };

        let q = p + direction * amount;
        area += (p.x * q.y - p.y * q.x) + amount;
        p = q;
    }

    area.abs() / 2 + 1
}

fn hex(x: u8) -> i64 {
    if (b'0'..=b'9').contains(&x) {
        return (x - b'0') as i64;
    }

    if (b'a'..=b'f').contains(&x) {
        return (x - b'a' + 10) as i64;
    }

    if (b'A'..=b'F').contains(&x) {
        return (x - b'a' + 10) as i64;
    }

    panic!("invalid HEX: {:?}", x)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(50_465, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(82_712_746_433_310, answer);
    }
}
