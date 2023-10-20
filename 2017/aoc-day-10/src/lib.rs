use std::fmt::Write;

pub fn part_one(input: &[u32]) -> u32 {
    let mut data = (0..=255).collect::<Vec<_>>();

    let mut skip = 0;
    let mut from = 0;

    for length in input.iter().copied() {
        reverse(&mut data, from, length as usize);

        from = (from + skip + length as usize) % data.len();
        skip += 1;
    }

    data[0] * data[1]
}

pub fn part_two(input: &str) -> String {
    let input = input
        .bytes()
        .map(|x| x as usize)
        .chain([17, 31, 73, 47, 23].into_iter())
        .collect::<Vec<_>>();
    let mut data = (0..=255).collect::<Vec<_>>();

    let mut skip = 0;
    let mut from = 0;

    for _ in 0..64 {
        for length in input.iter().copied() {
            reverse(&mut data, from, length as usize);

            from = (from + skip + length) % data.len();
            skip += 1;
        }
    }

    let mut answer = String::with_capacity(16 * 2);
    data.chunks(16)
        .map(|x| x.into_iter().fold(0, |acc, &val| acc ^ val))
        .for_each(|x| write!(&mut answer, "{:02x}", x as u8).unwrap());

    answer
}

fn reverse(arr: &mut [u32], from: usize, length: usize) {
    let len = arr.len();

    for idx in 0..(length / 2) {
        arr.swap((from + idx) % len, (from + length - idx - 1) % len);
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_csv;

    use super::*;

    #[test]
    fn test_reverse_1() {
        let arr = &mut [1, 2, 3, 4];
        reverse(arr, 0, 4);
        assert_eq!(&[4, 3, 2, 1], arr);
    }

    #[test]
    fn test_reverse_2() {
        let arr = &mut [1, 2, 3, 4];
        reverse(arr, 1, 2);
        assert_eq!(&[1, 3, 2, 4], arr);
    }

    #[test]
    fn test_reverse_3() {
        let arr = &mut [1, 2, 3, 4];
        reverse(arr, 1, 1);
        assert_eq!(&[1, 2, 3, 4], arr);
    }

    #[test]
    fn test_reverse_4() {
        let arr = &mut [1, 2, 3, 4];
        reverse(arr, 2, 3);
        assert_eq!(&[3, 2, 1, 4], arr);
    }

    #[test]
    fn test_reverse_5() {
        let arr = &mut [1, 2, 3, 4, 5];
        reverse(arr, 3, 4);
        assert_eq!(&[5, 4, 3, 2, 1], arr);
    }

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_csv(&input);

        let answer = part_one(&input);
        assert_eq!(2928, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(input.trim_end());
        assert_eq!("0c2f794b2eb555f7830766bf8fb65a16", answer);
    }
}
