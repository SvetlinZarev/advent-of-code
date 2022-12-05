const IDX_VALUE: usize = 1;
const VALUE_EMPTY: u8 = b' ';

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ParserState {
    Stacks,
    StacksEnd,
    Instructions,
}

pub fn parse_input(input: impl AsRef<str>) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input = input.as_ref();

    let mut stacks = vec![vec![]];
    let mut instructions = vec![];
    let mut state = ParserState::Stacks;

    for line in input.lines() {
        match state {
            ParserState::Stacks => {
                if line.starts_with(" 1") {
                    state = ParserState::StacksEnd;
                    continue;
                }

                for (idx, val) in line.as_bytes().chunks(4).enumerate() {
                    if stacks.len() <= idx {
                        stacks.push(vec![]);
                    }

                    if val[IDX_VALUE] != VALUE_EMPTY {
                        stacks[idx].push(val[IDX_VALUE] as char);
                    }
                }
            }

            ParserState::StacksEnd => {
                // After parsing, the stacks are in reversed order, thus we have to reverse them
                for stack in stacks.iter_mut() {
                    stack.reverse();
                }

                state = ParserState::Instructions;
            }

            ParserState::Instructions => {
                let mut count = 0;
                let mut src = 0;
                let mut dst = 0;

                for (idx, part) in line.split(' ').enumerate() {
                    match idx {
                        0 | 2 | 4 => continue,
                        1 => count = part.parse().unwrap(),
                        3 => src = part.parse().unwrap(),
                        5 => dst = part.parse().unwrap(),
                        _ => panic!("unexpected input: {:?}", line)
                    }
                }

                // -1 because we move from 1 based indexing to 0-based one
                instructions.push((count, src - 1, dst - 1));
            }
        }
    }


    (stacks, instructions)
}

pub fn part_one_v1(stacks: &[Vec<char>], instructions: &[(usize, usize, usize)]) -> String {
    let mut stacks = stacks.to_vec();

    for (count, src, dst) in instructions.iter().copied() {
        assert!(count <= stacks[src].len());

        for _ in 0..count {
            let value = stacks[src].pop().unwrap();
            stacks[dst].push(value);
        }
    }

    let mut answer = String::with_capacity(stacks.len());
    for mut stack in stacks.into_iter() {
        if let Some(ch) = stack.pop() {
            answer.push(ch);
        }
    }

    answer
}


pub fn part_one_v2(stacks: &Vec<Vec<char>>, instructions: &Vec<(usize, usize, usize)>) -> String {
    let mut stacks = stacks.to_vec();

    for (count, src, dst) in instructions.iter().copied() {
        let (a, b) = stacks.split_at_mut(1 + src.min(dst));
        let (s, d) = if src < dst { (a, b) } else { (b, a) };
        let (s, d) = if src < dst { (&mut s[src], &mut d[dst - src - 1]) } else { (&mut s[src - dst - 1], &mut d[dst]) };

        assert!(count <= s.len());
        let take_from = s.len() - count;
        d.extend(s.drain(take_from..).rev());
    }

    let mut answer = String::with_capacity(stacks.len());
    for mut stack in stacks.into_iter() {
        if let Some(ch) = stack.pop() {
            answer.push(ch);
        }
    }

    answer
}

pub fn part_two(stacks: &Vec<Vec<char>>, instructions: &Vec<(usize, usize, usize)>) -> String {
    let mut stacks = stacks.to_vec();

    for (count, src, dst) in instructions.iter().copied() {
        let (a, b) = stacks.split_at_mut(1 + src.min(dst));
        let (s, d) = if src < dst { (a, b) } else { (b, a) };
        let (s, d) = if src < dst { (&mut s[src], &mut d[dst - src - 1]) } else { (&mut s[src - dst - 1], &mut d[dst]) };

        assert!(count <= s.len());
        let take_from = s.len() - count;
        d.extend(s.drain(take_from..));
    }

    let mut answer = String::with_capacity(stacks.len());
    for mut stack in stacks.into_iter() {
        if let Some(ch) = stack.pop() {
            answer.push(ch);
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{parse_input, part_one_v1, part_one_v2, part_two};

    #[test]
    fn test_part_one_v1() {
        let (stacks, instructions) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one_v1(&stacks, &instructions);
        assert_eq!("JCMHLVGMG", answer);
    }

    #[test]
    fn test_part_one_v2() {
        let (stacks, instructions) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one_v2(&stacks, &instructions);
        assert_eq!("JCMHLVGMG", answer);
    }

    #[test]
    fn test_part_two() {
        let (stacks, instructions) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&stacks, &instructions);
        assert_eq!("LVMRWSSPZ", answer);
    }
}
