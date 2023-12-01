pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|l| {
            let first = l
                .iter()
                .find(|x| x.is_ascii_digit())
                .and_then(|&x| Some(x - b'0'))
                .map(|x| x as u32)
                .unwrap();
            let last = l
                .iter()
                .rfind(|x| x.is_ascii_digit())
                .and_then(|&x| Some(x - b'0'))
                .map(|x| x as u32)
                .unwrap();
            first * 10 + last
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    const NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|l| {
            let first = l
                .as_bytes()
                .iter()
                .position(|x| x.is_ascii_digit())
                .unwrap();
            let last = l
                .as_bytes()
                .iter()
                .rposition(|x| x.is_ascii_digit())
                .unwrap();

            let mut a = (l.as_bytes()[first] - b'0') as u32;
            let mut b = (l.as_bytes()[last] - b'0') as u32;
            let mut a_pos = first;
            let mut b_pos = last;

            for (idx, num) in NUMS.iter().enumerate() {
                if let Some(pos) = l[..first].find(num) {
                    if pos < a_pos {
                        a_pos = pos;
                        a = (idx + 1) as u32;
                    }
                }
            }

            for (idx, num) in NUMS.iter().enumerate() {
                if let Some(pos) = l[last..].rfind(num) {
                    if pos + last > b_pos {
                        b_pos = pos + last;
                        b = (idx + 1) as u32;
                    }
                }
            }

            a * 10 + b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(54634, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(&input);

        assert_eq!(53855, answer);
    }
}
