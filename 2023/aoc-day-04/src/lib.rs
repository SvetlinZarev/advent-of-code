use std::cmp::min;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Card {
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl FromStr for Card {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((_card_id, nums)) = s.split_once(':') else {
            return Err(format!("invalid card format: {}", s).into());
        };

        let Some((winning, numbers)) = nums.split_once('|') else {
            return Err(format!("invalid card numbers format: {}", s).into());
        };

        let winning = winning
            .split_ascii_whitespace()
            .map(|x| x.parse())
            .collect::<Result<HashSet<u32>, _>>()?;

        let numbers = numbers
            .split_ascii_whitespace()
            .map(|x| x.parse())
            .collect::<Result<HashSet<u32>, _>>()?;

        Ok(Card { winning, numbers })
    }
}

impl Card {
    fn common(&self) -> usize {
        self.winning.intersection(&self.numbers).count()
    }
}

pub fn part_one(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|card| card.common() as u32)
        .filter(|&n| n > 0)
        .map(|n| 2u32.pow(n - 1))
        .sum()
}

pub fn part_two_v1(input: &[Card]) -> usize {
    let mut cards = vec![1; input.len()];

    for idx in 1..input.len() {
        let card = &input[idx - 1];
        let count = card.common();

        for x in idx..min(cards.len(), idx + count) {
            cards[x] += cards[idx - 1];
        }
    }

    cards.iter().sum()
}

pub fn part_two_v2(input: &[Card]) -> usize {
    let mut cards = vec![1; input.len()];
    let mut inc = vec![0; input.len()];
    let mut dec = vec![0; input.len()];
    let mut diff = 0;

    for idx in 1..input.len() {
        let card = &input[idx - 1];
        let count = card.common();

        if count > 0 {
            inc[idx] += cards[idx - 1];
            if idx + count < dec.len() {
                dec[idx + count] += cards[idx - 1];
            }
        }

        diff += inc[idx];
        diff -= dec[idx];
        cards[idx] += diff;
    }

    cards.iter().sum()
}

pub fn part_two_v3(input: &[Card]) -> isize {
    let mut cards = vec![1; input.len()];
    let mut diffs = vec![0isize; input.len()];
    let mut diff = 0;

    for idx in 1..input.len() {
        let card = &input[idx - 1];
        let count = card.common();

        if count > 0 {
            diffs[idx] += cards[idx - 1];
            if idx + count < diffs.len() {
                diffs[idx + count] -= cards[idx - 1];
            }
        }

        diff += diffs[idx];
        cards[idx] += diff;
    }

    cards.iter().sum()
}

pub fn part_two_v4(input: &[Card]) -> isize {
    let mut diffs = vec![0isize; input.len()];
    let mut diff = 0;
    let mut answer = 1;
    let mut prev_cards = 1;

    for idx in 1..input.len() {
        let card = &input[idx - 1];
        let count = card.common();

        if count > 0 {
            diffs[idx] += prev_cards;
            if idx + count < diffs.len() {
                diffs[idx + count] -= prev_cards;
            }
        }

        diff += diffs[idx];
        let cards = diff + 1;

        answer += cards;
        prev_cards = cards;
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
        assert_eq!(23_847, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_two_v1(&input);
        assert_eq!(8_570_000, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_two_v2(&input);
        assert_eq!(8_570_000, answer);
    }

    #[test]
    fn test_part_two_v3() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_two_v3(&input);
        assert_eq!(8_570_000, answer);
    }

    #[test]
    fn test_part_two_v4() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_two_v4(&input);
        assert_eq!(8_570_000, answer);
    }
}
