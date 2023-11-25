use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Op {
    Spin(usize),
    Exchange(usize, usize),
    Partner(usize, usize),
}

impl FromStr for Op {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(format!("the empty string is not a valid operation").into());
        }

        Ok(match &s[..1] {
            "s" => Op::Spin(s[1..].parse()?),
            "x" => {
                let Some((a, b)) = s[1..].split_once('/') else {
                    return Err(format!("invalid operation: {}", s).into());
                };

                Op::Exchange(a.parse()?, b.parse()?)
            }
            "p" => {
                let Some((a, b)) = s[1..].split_once('/') else {
                    return Err(format!("invalid operation: {}", s).into());
                };

                if a.len() != 1
                    || b.len() != 1
                    || !(b'a'..=b'p').contains(&a.as_bytes()[0])
                    || !(b'a'..=b'p').contains(&b.as_bytes()[0])
                {
                    return Err(format!("invalid operation: {}", s).into());
                }

                Op::Partner(
                    (a.as_bytes()[0] - b'a') as usize,
                    (b.as_bytes()[0] - b'a') as usize,
                )
            }
            _ => return Err(format!("invalid operation: {}", s).into()),
        })
    }
}
pub fn part_one(input: &[Op]) -> String {
    const LEN: usize = 16;
    let mut data: [usize; LEN] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut index: [usize; LEN] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    dance(input, &mut data, &mut index);

    data.into_iter()
        .map(|x| x as u8 + b'a')
        .map(|x| x as char)
        .collect()
}

pub fn part_two(input: &[Op]) -> String {
    const LEN: usize = 16;
    let mut data: [usize; LEN] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut index: [usize; LEN] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // There is a cycle in the output of the "dance" function, thus when we find out
    // its length, we can immediately return the result
    let mut seen = HashMap::new();
    seen.insert(data, 0);

    let mut cycle_len = 0;
    for cycle in 0..1_000_000_000 {
        dance(input, &mut data, &mut index);

        if let Some(prev_cycle) = seen.insert(data, cycle + 1) {
            cycle_len = cycle + 1 - prev_cycle;
            break;
        }
    }

    let target = 1_000_000_000 % cycle_len;
    let answer = seen
        .iter()
        .find(|(_, &cyc)| cyc == target)
        .map(|(array, _)| array.clone())
        .unwrap();

    answer
        .into_iter()
        .map(|x| x as u8 + b'a')
        .map(|x| x as char)
        .collect()
}

fn dance<const LEN: usize>(input: &[Op], data: &mut [usize; LEN], index: &mut [usize; LEN]) {
    for op in input.iter().copied() {
        match op {
            Op::Spin(n) => {
                data.rotate_right(n);
                index.iter_mut().for_each(|x| *x = (*x + n) % LEN);
            }
            Op::Exchange(a, b) => {
                index.swap(data[a], data[b]);
                data.swap(a, b);
            }
            Op::Partner(a, b) => {
                data.swap(index[a], index[b]);
                index.swap(a, b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_csv;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_csv(input);

        let answer = part_one(&parsed);
        assert_eq!("padheomkgjfnblic", answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_csv(input);

        let answer = part_two(&parsed);
        assert_eq!("bfcdeakhijmlgopn", answer);
    }
}
