use std::cmp::Reverse;
use std::error::Error;
use std::str::FromStr;

const CARD_STRENGTH_1: [u64; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
const CARD_STRENGTH_2: [u64; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 12, 13, 14];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

impl FromStr for Card {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Card::A,
            "K" => Card::K,
            "Q" => Card::Q,
            "J" => Card::J,
            "T" => Card::T,
            "9" => Card::C9,
            "8" => Card::C8,
            "7" => Card::C7,
            "6" => Card::C6,
            "5" => Card::C5,
            "4" => Card::C4,
            "3" => Card::C3,
            "2" => Card::C2,
            _ => return Err(format!("invalid card: {}", s).into()),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Kind {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl FromStr for Hand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((hand, bid)) = s.split_once(' ') else {
            return Err(format!("invalid hand: {}", s).into());
        };

        if hand.len() != 5 {
            return Err(format!("invalid hand: {}", s).into());
        }

        let cards = [
            hand[0..1].parse()?,
            hand[1..2].parse()?,
            hand[2..3].parse()?,
            hand[3..4].parse()?,
            hand[4..5].parse()?,
        ];

        Ok(Hand {
            cards,
            bid: bid.trim().parse()?,
        })
    }
}

pub fn part_one(input: &[Hand]) -> u64 {
    solve(input, hand_kind_part_1, &CARD_STRENGTH_1)
}

fn hand_kind_part_1(hand: &Hand) -> Kind {
    let mut hand = hand.cards.clone();
    hand.sort_unstable();

    let mut freq = [0; 5];
    freq[0] = 1;

    let mut f = 0;
    for h in 1..freq.len() {
        if hand[h] != hand[h - 1] {
            f += 1;
        }

        freq[f] += 1;
    }

    freq.sort_unstable_by_key(|&x| Reverse(x));
    if freq[0] == 5 {
        return Kind::FiveOfAKind;
    }

    if freq[0] == 4 {
        return Kind::FourOfAKind;
    }

    if freq[0] == 3 && freq[1] == 2 {
        return Kind::FullHouse;
    }

    if freq[0] == 3 {
        return Kind::ThreeOfAKind;
    }

    if freq[0] == 2 && freq[1] == 2 {
        return Kind::TwoPairs;
    }

    if freq[0] == 2 {
        return Kind::OnePair;
    }

    Kind::HighCard
}

pub fn part_two(input: &[Hand]) -> u64 {
    solve(input, hand_kind_part_2, &CARD_STRENGTH_2)
}

fn hand_kind_part_2(hand: &Hand) -> Kind {
    let j = hand.cards.iter().filter(|&&c| c == Card::J).count();
    if j >= 4 {
        return Kind::FiveOfAKind;
    }

    let mut hand = hand.cards;
    hand.sort_unstable();

    let mut freq = [0; 5];
    freq[0] = 1;

    let mut f = 0;
    for h in 1..freq.len() {
        if hand[h] == Card::J {
            continue;
        }

        if hand[h] != hand[h - 1] {
            f += 1;
        }

        freq[f] += 1;
    }

    freq.sort_unstable_by_key(|&x| Reverse(x));
    freq[0] += j;

    if freq[0] == 5 {
        return Kind::FiveOfAKind;
    }

    if freq[0] == 4 {
        return Kind::FourOfAKind;
    }

    if freq[0] == 3 && freq[1] == 2 {
        return Kind::FullHouse;
    }

    if freq[0] == 3 {
        return Kind::ThreeOfAKind;
    }

    if freq[0] == 2 && freq[1] == 2 {
        return Kind::TwoPairs;
    }

    if freq[0] == 2 {
        return Kind::OnePair;
    }

    Kind::HighCard
}

fn solve(input: &[Hand], hand_type: fn(&Hand) -> Kind, card_strength: &[u64]) -> u64 {
    let mut hands = input
        .iter()
        .map(|h| (hand_type(&h), sort_score(&h.cards, card_strength), h.bid))
        .collect::<Vec<_>>();

    hands.sort_unstable();

    hands
        .into_iter()
        .map(|h| h.2)
        .enumerate()
        .fold(0, |acc, (rank, bid)| acc + (rank as u64 + 1) * bid)
}

fn sort_score(cards: &[Card; 5], scores: &[u64]) -> u64 {
    cards
        .into_iter()
        .fold(0, |acc, &val| acc * 100 + scores[val as usize])
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(241_344_943, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(243_101_568, answer);
    }
}
