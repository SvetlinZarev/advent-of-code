const SPACE: char = ' ';

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.trim_start().split_once(SPACE).and_then(|(x, r)| {
                r.trim_start()
                    .split_once(SPACE)
                    .and_then(|(y, z)| Some((x, y, z)))
            })
        })
        .flatten()
        .map(|(x, y, z)| (x.trim(), y.trim(), z.trim()))
        .map(|(x, y, z)| {
            x.parse::<u32>().and_then(|x| {
                y.parse::<u32>()
                    .and_then(|y| z.parse::<u32>().and_then(|z| Ok((x, y, z))))
            })
        })
        .flatten()
        .filter(|&(x, y, z)| x < y + z)
        .filter(|&(x, y, z)| y < x + z)
        .filter(|&(x, y, z)| z < x + y)
        .count()
}

pub fn part_two(input: &str) -> usize {
    let mut buffer = Vec::with_capacity(3);

    input
        .lines()
        .map(|line| {
            line.trim_start().split_once(SPACE).and_then(|(x, r)| {
                r.trim_start()
                    .split_once(SPACE)
                    .and_then(|(y, z)| Some((x, y, z)))
            })
        })
        .flatten()
        .map(|(x, y, z)| (x.trim(), y.trim(), z.trim()))
        .map(|(x, y, z)| {
            x.parse::<u32>().and_then(|x| {
                y.parse::<u32>()
                    .and_then(|y| z.parse::<u32>().and_then(|z| Ok((x, y, z))))
            })
        })
        .flatten()
        .map(|(x, y, z)| {
            buffer.push((x, y, z));

            let mut count = 0;
            if buffer.len() == 3 {
                // the first triangle is the 0th column
                count += (buffer[0].0 < buffer[1].0 + buffer[2].0
                    && buffer[1].0 < buffer[0].0 + buffer[2].0
                    && buffer[2].0 < buffer[0].0 + buffer[1].0) as usize;

                // the second triangle is the 1st column
                count += (buffer[0].1 < buffer[1].1 + buffer[2].1
                    && buffer[1].1 < buffer[0].1 + buffer[2].1
                    && buffer[2].1 < buffer[0].1 + buffer[1].1) as usize;

                // the third triangle is the 2nd column
                count += (buffer[0].2 < buffer[1].2 + buffer[2].2
                    && buffer[1].2 < buffer[0].2 + buffer[2].2
                    && buffer[2].2 < buffer[0].2 + buffer[1].2) as usize;

                // clear the buffer for the next 3 triangles
                buffer.clear();
            }

            count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(869, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(1544, answer);
    }
}
