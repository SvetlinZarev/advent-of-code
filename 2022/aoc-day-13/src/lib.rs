use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Eq, Clone)]
pub enum Packet {
    Single(u8),
    Many(Vec<Packet>),
}

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Single(x) => write!(f, "[{}]", *x),
            Packet::Many(packets) => {
                write!(f, "[")?;
                for (idx, packet) in packets.iter().enumerate() {
                    if idx != 0 {
                        write!(f, ",")?;
                    }

                    match packet {
                        Packet::Single(single) => write!(f, "{}", single),
                        many @ Packet::Many(_) => write!(f, "{}", many),
                    }?;
                }
                write!(f, "]")
            }
        }
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            return Err(format!("The packet should have started with '[': {}", s).into());
        }
        if !s.ends_with(']') {
            return Err(format!("The packet should have started with '[': {}", s).into());
        }

        let s = &s[1..s.len() - 1];
        let mut stack = vec![];

        let mut many = vec![];
        let mut single = None;

        for ch in s.bytes() {
            match ch {
                b'0'..=b'9' => {
                    single = Some(match single.take() {
                        None => ch - b'0',
                        Some(val) => val * 10 + (ch - b'0'),
                    });
                }

                b',' => {
                    if let Some(value) = single.take() {
                        many.push(Packet::Single(value));
                    }
                }

                b'[' => {
                    stack.push((many, single));
                    many = vec![];
                    single = None;
                }

                b']' => {
                    if !many.is_empty() {
                        if let Some(value) = single.take() {
                            many.push(Packet::Single(value));
                        }
                    }

                    let packet = match single.take() {
                        None => Packet::Many(many),
                        Some(value) => Packet::Single(value),
                    };

                    (many, single) = stack.pop().unwrap();
                    many.push(packet);
                }

                _ => panic!("unexpected character: {}", ch as char),
            }
        }

        assert!(
            stack.is_empty(),
            "the stack should have been empty: {:?}",
            stack
        );

        if !many.is_empty() {
            if let Some(value) = single.take() {
                many.push(Packet::Single(value));
            }
        }

        let packet = match single.take() {
            None => Packet::Many(many),
            Some(value) => Packet::Single(value),
        };

        Ok(packet)
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Single(x), Packet::Single(y)) => x.cmp(y),
            (Packet::Many(x), Packet::Many(y)) => x.cmp(y),
            (x @ Packet::Single(_), Packet::Many(y)) => std::slice::from_ref(x).cmp(y.as_slice()),
            (Packet::Many(x), y @ Packet::Single(_)) => x.as_slice().cmp(std::slice::from_ref(y)),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum ParserState {
        First,
        Second,
        End,
    }

    let mut parsed = vec![];
    let mut first = None;
    let mut second = None;

    let mut state = ParserState::First;
    for line in input.lines() {
        match state {
            ParserState::First => {
                first = Some(line.parse().unwrap());
                state = ParserState::Second;
            }

            ParserState::Second => {
                second = Some(line.parse().unwrap());
                state = ParserState::End;
            }

            ParserState::End => {
                assert!(line.is_empty());

                parsed.push((first.take().unwrap(), second.take().unwrap()));
                state = ParserState::First;
            }
        }
    }

    if first.is_some() || second.is_some() {
        parsed.push((first.take().unwrap(), second.take().unwrap()));
    }

    parsed
}

pub fn part_one(packets: &[(Packet, Packet)]) -> usize {
    packets
        .iter()
        .enumerate()
        .filter_map(|(idx, (a, b))| match a.cmp(b) == Ordering::Less {
            true => Some(idx + 1),
            false => None,
        })
        .sum()
}

pub fn part_two(packets: Vec<(Packet, Packet)>) -> usize {
    let mut packets = packets
        .into_iter()
        .map(|(a, b)| std::iter::once(a).chain(std::iter::once(b)))
        .flatten()
        .collect::<Vec<_>>();
    packets.push(Packet::Single(2));
    packets.push(Packet::Single(6));

    packets.sort_unstable();

    let mut answer = 1;
    let mut idx = 0;
    while idx < packets.len() {
        idx += 1;
        if let Packet::Single(2) = packets[idx - 1] {
            answer *= idx;
            break;
        }
    }
    while idx < packets.len() {
        idx += 1;
        if let Packet::Single(6) = packets[idx - 1] {
            answer *= idx;
            break;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_one, part_two};
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);
        let answer = part_one(&parsed);
        assert_eq!(5555, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);
        let answer = part_two(parsed);
        assert_eq!(22852, answer);
    }
}
