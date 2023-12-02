use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub gs: Vec<GameSet>,
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((game, sets)) = s.split_once(':') else {
            return Err(format!("cannot split game info: {}", s).into());
        };

        let Some((_, game_id)) = game.split_once(' ') else {
            return Err(format!("cannot split game info: {}", game).into());
        };

        let mut gs = vec![];
        for game_set in sets.split(';') {
            gs.push(game_set.trim().parse()?);
        }

        Ok(Game {
            id: game_id
                .parse()
                .map_err(|_| format!("cannot parse the game id: {}", game_id))?,
            gs,
        })
    }
}

#[derive(Debug)]
pub struct GameSet {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl FromStr for GameSet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut r, mut g, mut b) = (0, 0, 0);

        for segment in s.split(',').map(|s| s.trim()) {
            let Some((n, txt)) = segment.split_once(' ') else {
                return Err(format!("cannot split segment: {}", segment).into());
            };

            let n = n
                .parse::<u32>()
                .map_err(|_| format!("cannot parse the color value: {}", n))?;
            match txt {
                "red" => r += n,
                "green" => g += n,
                "blue" => b += n,
                _ => return Err(format!("unrecognized color: {}", txt).into()),
            }
        }

        Ok(GameSet { r, g, b })
    }
}

pub fn part_one(input: &[Game]) -> u32 {
    const RGB_LIMIT: (u32, u32, u32) = (12, 13, 14);
    let mut answer = 0;

    'next: for game in input.iter() {
        for gs in game.gs.iter() {
            if gs.r > RGB_LIMIT.0 || gs.g > RGB_LIMIT.1 || gs.b > RGB_LIMIT.2 {
                continue 'next;
            }
        }

        answer += game.id;
    }

    answer
}

pub fn part_two(input: &[Game]) -> u32 {
    let mut answer = 0;

    for game in input.iter() {
        let (mut r, mut g, mut b) = (0, 0, 0);

        for gs in game.gs.iter() {
            r = r.max(gs.r);
            g = g.max(gs.g);
            b = b.max(gs.b);
        }

        answer += r * g * b;
    }

    answer
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(2879, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(65122, answer);
    }
}
