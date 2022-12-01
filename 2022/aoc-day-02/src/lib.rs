use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Kind {
    Rock,
    Paper,
    Scissors,
}

impl Kind {
    pub fn score(self) -> u32 {
        match self {
            Kind::Rock => 1,
            Kind::Paper => 2,
            Kind::Scissors => 3,
        }
    }

    /// Return the type that the current variant wins over
    pub fn wins(self) -> Kind {
        match self {
            Kind::Rock => Kind::Scissors,
            Kind::Paper => Kind::Rock,
            Kind::Scissors => Kind::Paper,
        }
    }

    /// Return the type that the current variant loses to
    pub fn loses(self) -> Kind {
        match self {
            Kind::Rock => Kind::Paper,
            Kind::Paper => Kind::Scissors,
            Kind::Scissors => Kind::Rock,
        }
    }

    /// Return the type that the current variant neither wins, nor loses to
    pub fn draw(self) -> Kind {
        self
    }
}

impl FromStr for Kind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Kind::Rock),
            "B" => Ok(Kind::Paper),
            "C" => Ok(Kind::Scissors),
            _ => Err(format!("invalid input: {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Guide {
    X,
    Y,
    Z,
}

impl Guide {
    pub fn guess(self) -> Kind {
        match self {
            Guide::X => Kind::Rock,
            Guide::Y => Kind::Paper,
            Guide::Z => Kind::Scissors,
        }
    }

    pub fn follow(self, opponent: Kind) -> Kind {
        match self {
            // We must lose, thus return the variant that will help our opponent to win
            Guide::X => opponent.wins(),

            // We must end in a draw, thus return the variant that will help us do so
            Guide::Y => opponent.draw(),

            // We must win, thus return the variant that will help our opponent to lose
            Guide::Z => opponent.loses(),
        }
    }
}

impl FromStr for Guide {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Guide::X),
            "Y" => Ok(Guide::Y),
            "Z" => Ok(Guide::Z),
            _ => Err(format!("invalid input: {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Game {
    opponent: Kind,
    guide: Guide,
}

impl Game {
    pub fn play<F: Fn(Guide, Kind) -> Kind>(self, guide: F) -> u32 {
        let decoded = guide(self.guide, self.opponent);

        let game = if decoded.draw() == self.opponent {
            3
        } else if decoded.wins() == self.opponent {
            6
        } else {
            0
        };

        game + decoded.score()
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(' ')
            .ok_or_else(|| format!("cannot split: {:?}", s))?;
        Ok(Game {
            opponent: a.parse()?,
            guide: b.parse()?,
        })
    }
}

pub fn part_one(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| game.play(|guide, _opponent| guide.guess()))
        .sum()
}

pub fn part_two(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| game.play(|guide, opponent| guide.follow(opponent)))
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use crate::{part_one, part_two, Game, Guide, Kind};

    #[test]
    fn test_scores() {
        assert_eq!(1, Kind::Rock.score());
        assert_eq!(2, Kind::Paper.score());
        assert_eq!(3, Kind::Scissors.score());
    }

    #[test]
    fn test_parse_kind() {
        assert_eq!(Kind::Rock, "A".parse().unwrap());
        assert_eq!(Kind::Paper, "B".parse().unwrap());
        assert_eq!(Kind::Scissors, "C".parse().unwrap());
    }

    #[test]
    fn test_parse_guide() {
        assert_eq!(Guide::X, "X".parse().unwrap());
        assert_eq!(Guide::Y, "Y".parse().unwrap());
        assert_eq!(Guide::Z, "Z".parse().unwrap());
    }

    #[test]
    fn test_guide_guess() {
        assert_eq!(Kind::Rock, Guide::X.guess());
        assert_eq!(Kind::Paper, Guide::Y.guess());
        assert_eq!(Kind::Scissors, Guide::Z.guess());
    }

    #[test]
    fn test_guide_follow() {
        assert_eq!(Kind::Scissors, Guide::X.follow(Kind::Rock));
        assert_eq!(Kind::Rock, Guide::Y.follow(Kind::Rock));
        assert_eq!(Kind::Paper, Guide::Z.follow(Kind::Rock));

        assert_eq!(Kind::Rock, Guide::X.follow(Kind::Paper));
        assert_eq!(Kind::Paper, Guide::Y.follow(Kind::Paper));
        assert_eq!(Kind::Scissors, Guide::Z.follow(Kind::Paper));

        assert_eq!(Kind::Paper, Guide::X.follow(Kind::Scissors));
        assert_eq!(Kind::Scissors, Guide::Y.follow(Kind::Scissors));
        assert_eq!(Kind::Rock, Guide::Z.follow(Kind::Scissors));
    }

    #[test]
    fn test_play_game_lose() {
        assert_eq!(
            3,
            Game {
                opponent: Kind::Rock,
                guide: Guide::X,
            }
            .play(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            1,
            Game {
                opponent: Kind::Paper,
                guide: Guide::X,
            }
            .play(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            2,
            Game {
                opponent: Kind::Scissors,
                guide: Guide::X,
            }
            .play(|guide, opponent| guide.follow(opponent))
        );
    }

    #[test]
    fn test_play_game_win() {
        assert_eq!(
            8,
            Game {
                opponent: Kind::Rock,
                guide: Guide::Z,
            }
            .play(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            9,
            Game {
                opponent: Kind::Paper,
                guide: Guide::Z,
            }
            .play(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            7,
            Game {
                opponent: Kind::Scissors,
                guide: Guide::Z,
            }
            .play(|guide, opponent| guide.follow(opponent))
        );
    }

    #[test]
    fn test_play_game_draw() {
        assert_eq!(
            4,
            Game {
                opponent: Kind::Rock,
                guide: Guide::Y,
            }
            .play(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            5,
            Game {
                opponent: Kind::Paper,
                guide: Guide::Y,
            }
            .play(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            6,
            Game {
                opponent: Kind::Scissors,
                guide: Guide::Y,
            }
            .play(|guide, opponent| guide.follow(opponent))
        );
    }

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        assert_eq!(13924, part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        assert_eq!(13448, part_two(&input));
    }
}
