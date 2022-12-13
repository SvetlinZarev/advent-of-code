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
            Packet::Single(x) => write!(f, "{}", *x),
            Packet::Many(packets) => {
                write!(f, "[")?;
                for (idx, packet) in packets.iter().enumerate() {
                    if idx != 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", packet)?;
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
                    if let Some(value) = single.take() {
                        many.push(Packet::Single(value));
                    }

                    let packet = Packet::Many(many);
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

        if let Some(value) = single.take() {
            many.push(Packet::Single(value));
        }

        Ok(Packet::Many(many))
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

pub fn part_one(packets: &[Packet]) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .filter_map(
            |(idx, chunk)| match chunk[0].cmp(&chunk[1]) == Ordering::Less {
                true => Some(idx + 1),
                false => None,
            },
        )
        .sum()
}

pub fn part_two(mut packets: Vec<Packet>) -> usize {
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
    use aoc_shared::input::load_line_delimited_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let parsed = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&parsed);
        assert_eq!(5555, answer);
    }

    #[test]
    fn test_part_two() {
        let parsed = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(parsed);
        assert_eq!(22852, answer);
    }
}
