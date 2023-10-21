use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Scanner {
    depth: u32,
    range: u32,
}

impl FromStr for Scanner {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((d, r)) = s.split_once(": ") else {
            return Err(format!("invalid scanner description: {:?}", s).into());
        };

        Ok(Scanner {
            depth: d.parse()?,
            range: r.parse()?,
        })
    }
}

pub fn part_one(input: &[Scanner]) -> u32 {
    let mut answer = 0;
    let mut time = 0;

    for idx in 0..input.len() {
        if idx > 0 {
            time += input[idx].depth - input[idx - 1].depth;
        }

        // It's `(s.range -1)`, because we have `N-1` connections
        // in a list of `N` nodes
        //
        // It's `x * 2`, because we have to go to the last node,
        // and then return back - i.e. we walk the distance twice
        let s = &input[idx];
        if time % ((s.range - 1) * 2) == 0 {
            answer += s.depth * s.range;
        }
    }

    answer
}

pub fn part_two(input: &[Scanner]) -> u32 {
    let mut delay = 0;

    'next: loop {
        let mut time = 0;

        for idx in 0..input.len() {
            if idx > 0 {
                time += input[idx].depth - input[idx - 1].depth;
            }

            // It's `(s.range -1)`, because we have `N-1` connections
            // in a list of `N` nodes
            //
            // It's `x * 2`, because we have to go to the last node,
            // and then return back - i.e. we walk the distance twice
            let s = &input[idx];
            if (time + delay) % ((s.range - 1) * 2) == 0 {
                delay += 1;
                continue 'next;
            }
        }

        return delay;
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(1300, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(3870382, answer);
    }
}
