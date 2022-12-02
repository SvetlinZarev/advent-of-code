pub fn part_one(input: &[String]) -> u32 {
    let mut answer = 0;

    for s in input {
        let a = &s[0..s.len() / 2];
        let b = &s[s.len() / 2..];

        let x = char_set(a);
        let y = char_set(b);
        let mut xy = x & y;

        while xy > 0 {
            answer += 1 + xy.trailing_zeros();
            xy &= xy - 1;
        }
    }

    answer
}

pub fn part_two(input: &[String]) -> u64 {
    let mut answer = 0;

    for w in input.windows(3).step_by(3) {
        let mut badge = u64::MAX;

        for elf in w {
            badge &= char_set(elf);
        }

        answer += badge.trailing_zeros() as u64 + 1;
    }

    answer
}

fn char_set(s: &str) -> u64 {
    let mut set = 0u64;

    for ch in s.bytes() {
        let shift = if ch.is_ascii_lowercase() {
            ch - b'a'
        } else {
            26 + (ch - b'A')
        };

        set |= 1 << shift;
    }

    set
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(7889, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(2825, answer);
    }
}
