use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Kind {
    Rock,
    Paper,
    Scissors,
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

pub mod v1 {
    use super::*;

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

    impl Game {
        pub fn play_v1<F: Fn(Guide, Kind) -> Kind>(self, guide: F) -> u32 {
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

    pub fn part_one(input: &[Game]) -> u32 {
        input
            .iter()
            .map(|game| game.play_v1(|guide, _opponent| guide.guess()))
            .sum()
    }

    pub fn part_two(input: &[Game]) -> u32 {
        input
            .iter()
            .map(|game| game.play_v1(|guide, opponent| guide.follow(opponent)))
            .sum()
    }
}

pub mod v2 {
    use super::*;

    // Precompute all valid combinations:
    // * The row represents the other player's move
    // * The column - my move
    // Thus if the other player chooses "paper" and I choose "scissors",
    // we have to select the `state[1][2]` value which is `9`, i.e. `6 + 3`:
    // * 6 because I win
    // * +3 because that's the value for "scissors"
    const STATE_PART_1: [[u32; 3]; 3] = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];

    // Precompute all valid combinations:
    // * The row represents the other player's move
    // * The column - my move
    const STATE_PART_2: [[u32; 3]; 3] = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];

    impl Game {
        pub fn play_v2(self, state: &[[u32; 3]; 3]) -> u32 {
            let a = self.opponent as usize;
            let b = self.guide as usize;

            state[a][b]
        }
    }

    pub fn part_one(input: &[Game]) -> u32 {
        input.iter().map(|game| game.play_v2(&STATE_PART_1)).sum()
    }

    pub fn part_two(input: &[Game]) -> u32 {
        input.iter().map(|game| game.play_v2(&STATE_PART_2)).sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use crate::{v1, v2, Game, Guide, Kind};

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
            .play_v1(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            1,
            Game {
                opponent: Kind::Paper,
                guide: Guide::X,
            }
            .play_v1(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            2,
            Game {
                opponent: Kind::Scissors,
                guide: Guide::X,
            }
            .play_v1(|guide, opponent| guide.follow(opponent))
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
            .play_v1(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            9,
            Game {
                opponent: Kind::Paper,
                guide: Guide::Z,
            }
            .play_v1(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            7,
            Game {
                opponent: Kind::Scissors,
                guide: Guide::Z,
            }
            .play_v1(|guide, opponent| guide.follow(opponent))
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
            .play_v1(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            5,
            Game {
                opponent: Kind::Paper,
                guide: Guide::Y,
            }
            .play_v1(|guide, opponent| guide.follow(opponent))
        );

        assert_eq!(
            6,
            Game {
                opponent: Kind::Scissors,
                guide: Guide::Y,
            }
            .play_v1(|guide, opponent| guide.follow(opponent))
        );
    }

    #[test]
    fn test_v1_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        assert_eq!(13924, v1::part_one(&input));
    }

    #[test]
    fn test_v1_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        assert_eq!(13448, v1::part_two(&input));
    }

    #[test]
    fn test_v2_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        assert_eq!(13924, v2::part_one(&input));
    }

    #[test]
    fn test_v2_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        assert_eq!(13448, v2::part_two(&input));
    }
}
