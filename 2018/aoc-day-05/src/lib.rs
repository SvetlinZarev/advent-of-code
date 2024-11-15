pub fn part_one(input: &str) -> usize {
    let mut input = input.trim();
    let mut stack = vec![];

    for &ch in input.as_bytes() {
        match ch {
            b'a'..=b'z' => {
                if stack.last() == Some(&ch.to_ascii_uppercase()) {
                    stack.pop();
                } else {
                    stack.push(ch);
                }
            }

            b'A'..=b'Z' => {
                if stack.last() == Some(&ch.to_ascii_lowercase()) {
                    stack.pop();
                } else {
                    stack.push(ch);
                }
            }

            _ => unreachable!("Character: {}", ch as char),
        }
    }

    stack.len()
}

pub fn part_two(input: &str) -> usize {
    let input = input.trim();
    let mut stack = vec![];
    let mut best_len = input.len();

    for rm in b'a'..=b'z' {
        stack.clear();

        for &ch in input.as_bytes() {
            if ch == rm || ch == rm.to_ascii_uppercase() {
                continue;
            }

            match ch {
                b'a'..=b'z' => {
                    if stack.last() == Some(&ch.to_ascii_uppercase()) {
                        stack.pop();
                    } else {
                        stack.push(ch);
                    }
                }

                b'A'..=b'Z' => {
                    if stack.last() == Some(&ch.to_ascii_lowercase()) {
                        stack.pop();
                    } else {
                        stack.push(ch);
                    }
                }

                _ => unreachable!("Character: {}", ch as char),
            }
        }

        best_len = best_len.min(stack.len());
    }

    best_len
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(9900, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(4992, answer);
    }
}
