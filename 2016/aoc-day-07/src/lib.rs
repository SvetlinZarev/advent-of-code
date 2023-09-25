use std::collections::HashSet;

pub fn part_one(input: &[String]) -> usize {
    let mut answer = 0;

    'l: for line in input.iter().map(|line| line.as_bytes()) {
        let mut in_brackets = false;
        let mut has_abba = false;

        for w in line.windows(4) {
            if w[3] == b'[' {
                in_brackets = true;
            } else if w[3] == b']' {
                in_brackets = false;
            } else if w[0] != w[1] && (w[0], w[1]) == (w[3], w[2]) {
                if in_brackets {
                    continue 'l;
                }

                has_abba = true;
            }
        }

        if has_abba {
            answer += 1;
        }
    }

    answer
}

pub fn part_two(input: &[String]) -> usize {
    let mut answer = 0;
    let mut aba = HashSet::new();
    let mut bab = HashSet::new();

    'l: for line in input.iter().map(|line| line.as_bytes()) {
        let mut in_brackets = false;
        aba.clear();
        bab.clear();

        for w in line.windows(3) {
            if w[2] == b'[' {
                in_brackets = true;
            } else if w[2] == b']' {
                in_brackets = false;
            } else if w[0] != w[1] && w[0] == w[2] {
                if in_brackets {
                    // Normally we would add the whole triplet: `(w[0], w[1], w[2])`,
                    // but we know that `w[0] == w[2]`, so we can just omit the third
                    // value in order to save a few CPU cycles
                    bab.insert((w[0], w[1]));
                    if aba.contains(&(w[1], w[0])) {
                        answer += 1;
                        continue 'l;
                    }
                } else {
                    aba.insert((w[0], w[1]));
                    if bab.contains(&(w[1], w[0])) {
                        answer += 1;
                        continue 'l;
                    }
                }
            }
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
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(110, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(242, answer);
    }
}
