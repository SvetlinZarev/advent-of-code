use regex::Regex;
use std::error::Error;
use std::sync::LazyLock;

const INPUT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^#(?<id>\d+) @ (?<left>\d+),(?<top>\d+): (?<width>\d+)x(?<height>\d+)$"#).unwrap());

#[derive(Debug)]
pub struct InputLine {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

pub fn parse_input(input: &str) -> Result<Vec<InputLine>, Box<dyn Error>> {
    let mut answer = vec![];

    for line in input.lines() {
        let captures = INPUT_REGEX.captures(line).unwrap();

        let id = captures.name("id").ok_or_else(|| "'id' not found")?.as_str().parse()?;
        let left = captures.name("left").ok_or_else(|| "'left' not found")?.as_str().parse()?;
        let top = captures.name("top").ok_or_else(|| "'top' not found")?.as_str().parse()?;
        let width = captures.name("width").ok_or_else(|| "'width' not found")?.as_str().parse()?;
        let height = captures.name("height").ok_or_else(|| "'height' not found")?.as_str().parse()?;

        answer.push(InputLine { id, left, top, width, height });
    }

    Ok(answer)
}

pub fn part_one(input: &Vec<InputLine>) -> usize {
    let mut sheet = vec![0u32; 1000 * 1000];

    for sh in input {
        for r in sh.top..sh.top + sh.height {
            for c in sh.left..sh.left + sh.width {
                sheet[r * 1000 + c] += 1;
            }
        }
    }

    sheet.into_iter().filter(|&x| x > 1).count()
}

pub fn part_two(input: &Vec<InputLine>) -> usize {
    let mut sheet = vec![0usize; 1000 * 1000];
    let mut claims = vec![true; input.len() + 1];
    claims[0] = false;

    for sh in input {
        let mut prev_id = 0;

        for r in sh.top..sh.top + sh.height {
            for c in sh.left..sh.left + sh.width {
                let id = sheet[r * 1000 + c];
                sheet[r * 1000 + c] = sh.id;

                if prev_id != id && id != 0 {
                    prev_id = id;
                    claims[id] = false;
                }
            }
        }

        if prev_id != 0 {
            claims[sh.id] = false;
        }
    }

    claims.into_iter().position(|x| x).unwrap()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_one(&parsed);

        assert_eq!(112378, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two(&parsed);

        assert_eq!(603, answer);
    }
}
