use std::error::Error;

pub fn parse_input(input: impl AsRef<str>) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    let input = input.as_ref().trim_end();
    let mut parsed = vec![];
    let mut last = 0;

    for line in input.lines() {
        let line = line.trim_end();
        if line.is_empty() || parsed.is_empty() {
            last = parsed.len();
            parsed.push(vec![]);

            // Check again for empty line, because
            //we do not want to skip the very first row
            if line.is_empty() {
                continue;
            }
        }

        parsed[last].push(line.parse()?);
    }

    Ok(parsed)
}

pub fn part_one(input: &[Vec<u64>]) -> u64 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap_or(0)
}

pub fn part_two(input: &[Vec<u64>]) -> u64 {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for elf in input {
        let sum = elf.iter().sum();
        if sum > a {
            c = b;
            b = a;
            a = sum;
        } else if sum > b {
            c = b;
            b = sum;
        } else if sum > c {
            c = sum;
        }
    }


    a + b + c
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{parse_input, part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();
        let answer = part_one(&parsed);
        assert_eq!(68467, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();
        let answer = part_two(&parsed);
        assert_eq!(203420, answer);
    }
}
