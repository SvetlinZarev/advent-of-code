pub fn parse_input<I: AsRef<str>>(input: I) -> Vec<Vec<u8>> {
    input.as_ref()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect()
}

pub fn part_one(input: &[Vec<u8>]) -> u64 {
    let mut stack = Vec::with_capacity(128);
    let mut syntax_score = 0;

    for line in input {
        stack.clear();

        for &ch in line {
            match ch {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                b'<' => stack.push(b'>'),
                br => if Some(br) != stack.pop() {
                    syntax_score += invalid_bracket_score(br)
                }
            }
        }
    }

    syntax_score
}

fn invalid_bracket_score(ch: u8) -> u64 {
    match ch {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!()
    }
}


pub fn part_two(input: &[Vec<u8>]) -> u64 {
    let mut stack = Vec::with_capacity(128);
    let mut scores = Vec::with_capacity(input.len());

    'next: for line in input {
        stack.clear();

        for &ch in line {
            match ch {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                b'<' => stack.push(b'>'),
                br => if Some(br) != stack.pop() {
                    continue 'next;
                }
            }
        }

        let mut line_score = 0;
        while let Some(ch) = stack.pop() {
            line_score *= 5;
            line_score += incomplete_bracket_score(ch);
        }
        scores.push(line_score);
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn incomplete_bracket_score(ch: u8) -> u64 {
    match ch {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&input);
        assert_eq!(296535, answer);
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&input);
        assert_eq!(4245130838, answer);
    }
}