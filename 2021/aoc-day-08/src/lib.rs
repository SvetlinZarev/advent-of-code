use std::str::FromStr;

const DISPAY_LEN: usize = 4;

#[derive(Debug, Copy, Clone)]
pub struct Entry {
    notes: [u8; 10],
    displ: [u8; 4],
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once('|')
            .ok_or_else(|| format!("invalid input: {}", s))?;

        let mut notes = [0u8; 10];
        let mut displ = [0u8; DISPAY_LEN];
        parse_connections(left, &mut notes)?;
        parse_connections(right, &mut displ)?;

        Ok(Entry { notes, displ })
    }
}

#[must_use]
fn parse_connections(input: &str, dst: &mut [u8]) -> Result<(), String> {
    let mut parts = 0;

    input
        .split(' ')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.as_bytes()
                .iter()
                .map(|&b| b - b'a')
                .fold(0u8, |acc, v| acc | 1 << v)
        })
        .enumerate()
        .for_each(|(idx, v)| {
            parts += 1;
            if idx < dst.len() {
                dst[idx] = v;
            }
        });

    if parts != dst.len() {
        return Err(format!("Expected {}, but got {} parts", dst.len(), parts));
    }
    Ok(())
}

pub fn part_one(input: &[Entry]) -> usize {
    input
        .iter()
        .flat_map(|e| e.displ.iter().copied())
        .filter(|&v| match v.count_ones() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

pub fn part_two_v1(input: &[Entry]) -> usize {
    let mut result = 0;

    for entry in input.iter().copied() {
        let mut numbers = [0u8; 10];

        // Find the numbers with unique number of bits set to 1
        let mut to_find = 4;
        for idx in 0..entry.notes.len() {
            if to_find == 0 {
                break;
            }

            match entry.notes[idx].count_ones() {
                2 => numbers[1] = entry.notes[idx],
                3 => numbers[7] = entry.notes[idx],
                4 => numbers[4] = entry.notes[idx],
                7 => numbers[8] = entry.notes[idx],
                _ => continue,
            }

            to_find -= 1;
        }

        // find the remaining numbers
        let mut to_find = 6;
        for idx in 0..entry.notes.len() {
            if to_find == 0 {
                break;
            }

            let x = entry.notes[idx];
            let bits = x.count_ones() as usize;
            match bits {
                2 | 3 | 4 | 7 => continue,

                5 => {
                    if (x ^ numbers[8]) & numbers[1] == 0 {
                        numbers[3] = x;
                    } else if (x & (numbers[1] ^ numbers[4])) == numbers[1] ^ numbers[4] {
                        numbers[5] = x;
                    } else {
                        numbers[2] = x;
                    }
                }

                6 => {
                    if ((x ^ numbers[8]) & numbers[1]) != 0 {
                        numbers[6] = x;
                    } else if x & (numbers[7] | numbers[4]) == numbers[7] | numbers[4] {
                        numbers[9] = x;
                    } else {
                        numbers[0] = x;
                    }
                }

                _ => panic!("Unexpected number of bits ({}) for {}", bits, x),
            }

            to_find -= 1;
        }

        let mut value = 0;
        for displayed in entry.displ.iter().copied() {
            for (digit, disp) in numbers.iter().copied().enumerate() {
                if disp == displayed {
                    value = value * 10 + digit;
                    break;
                }
            }
        }

        result += value;
    }

    result
}

pub fn part_two_v2(input: &[Entry]) -> usize {
    let mut result = 0;

    for entry in input.iter().copied() {
        let (mut one, mut four, mut seven, mut eight) = (0, 0, 0, 0);

        // Find the numbers with unique number of bits set to 1
        let mut to_find = 4;
        for idx in 0..entry.notes.len() {
            if to_find == 0 {
                break;
            }

            match entry.notes[idx].count_ones() {
                2 => one = entry.notes[idx],
                3 => seven = entry.notes[idx],
                4 => four = entry.notes[idx],
                7 => eight = entry.notes[idx],
                _ => continue,
            }

            to_find -= 1;
        }

        // decode the displayed numbers
        let mut display = [0u8; DISPAY_LEN];
        for idx in 0..DISPAY_LEN {
            let x = entry.displ[idx];
            let bits = x.count_ones() as usize;

            let val = match bits {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,

                5 => {
                    if (x ^ eight) & one == 0 {
                        3
                    } else if (x & (one ^ four)) == one ^ four {
                        5
                    } else {
                        2
                    }
                }

                6 => {
                    if (x ^ eight) & one != 0 {
                        6
                    } else if x & (seven | four) == seven | four {
                        9
                    } else {
                        0
                    }
                }

                _ => panic!("Unexpected number of bits ({}) for {}", bits, x),
            };
            display[idx] = val;
        }

        result += display[0] as usize * 1000
            + display[1] as usize * 100
            + display[2] as usize * 10
            + display[3] as usize;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(362, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two_v1(&input);
        assert_eq!(1020159, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two_v2(&input);
        assert_eq!(1020159, answer);
    }
}
