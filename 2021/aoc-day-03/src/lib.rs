use std::mem::swap;

pub fn part_one(input: &[String]) -> u32 {
    assert!(!input.is_empty());

    let mut ones = vec![0; input[0].len()];

    for element in input.iter().map(|i| i.as_bytes()) {
        assert_eq!(element.len(), ones.len());

        for idx in 0..element.len() {
            // true is cast to 1
            // false is cast to 0
            ones[idx] += (element[idx] == b'1') as usize;
        }
    }

    let mut gamma = 0u32;
    let mut epsilon = 0u32;

    for v in ones.iter().copied().map(|v| if v > input.len() - v { 1 } else { 0 }) {
        gamma <<= 1;
        epsilon <<= 1;

        gamma |= v;
        epsilon |= v ^ 1;
    }

    gamma * epsilon
}

pub fn part_two_v1(input: &mut [String]) -> u32 {
    let (mut oxygen, mut co2) = rearrange_and_split(input, 0);

    // The left part contains the strings with "0" bit, the right one - with "1" bit
    // The oxygen rating uses the slice with more elements. If the slices have
    // equal lengths, then the "oxygen" slice should use the one with the "1" bit
    if oxygen.len() <= co2.len() {
        swap(&mut oxygen, &mut co2);
    }

    oxygen = reduce_input(oxygen, true);
    assert_eq!(1, oxygen.len(), "Too many/few elements for oxygen: {:?}", oxygen);

    co2 = reduce_input(co2, false);
    assert_eq!(1, co2.len(), "Too many/few elements for co2: {:?}", co2);

    let ox_rating = u32::from_str_radix(oxygen[0].as_str(), 2).unwrap();
    let co_rating = u32::from_str_radix(co2[0].as_str(), 2).unwrap();

    ox_rating * co_rating
}

fn reduce_input(input: &mut [String], is_oxygen_rating: bool) -> &mut [String] {
    assert!(!input.is_empty());

    let mut reduced = input;

    for idx in 1..reduced[0].len() {
        if reduced.len() <= 1 {
            break;
        }

        let (zeroes, ones) = rearrange_and_split(reduced, idx);

        let (mut larger, mut smaller) = (zeroes, ones);
        if larger.len() <= smaller.len() {
            swap(&mut larger, &mut smaller);
        }

        if is_oxygen_rating {
            reduced = larger;
        } else {
            reduced = smaller;
        }
    }

    reduced
}

// surprisingly this function is quite fast: ~15us on the benchmark
fn rearrange_and_split(array: &mut [String], filter_idx: usize) -> (&mut [String], &mut [String]) {
    let mut dst = 0;
    let mut src = 0;

    while src < array.len() {
        if array[src].as_bytes()[filter_idx] == b'0' {
            array.swap(dst, src);
            dst += 1;
        }

        src += 1;
    }

    array.split_at_mut(dst)
}

// surprisingly this is slower than v1: 90us in the benchmark
pub fn part_two_v2(input: &mut [String]) -> u32 {
    input.sort_unstable();

    let oxygen = split_on_msb_diff(input, |a, b| if a.len() <= b.len() { b } else { a });
    assert_eq!(1, oxygen.len(), "Too many/few elements for oxygen: {:?}", oxygen);

    let co2 = split_on_msb_diff(input, |a, b| if a.len() <= b.len() { a } else { b });
    assert_eq!(1, co2.len(), "Too many/few elements for co2: {:?}", co2);

    let ox_rating = u32::from_str_radix(oxygen[0].as_str(), 2).unwrap();
    let co_rating = u32::from_str_radix(co2[0].as_str(), 2).unwrap();
    ox_rating * co_rating
}

fn split_on_msb_diff<'a, F>(input: &'a [String], arbiter: F) -> &'a [String]
    where F: Fn(&'a [String], &'a [String]) -> &'a [String]
{
    let mut array = &input[..];

    for idx in 0..input[0].len() {
        if array.len() <= 1 {
            break;
        }

        if let Some(pos) = first_occurrence(array, idx, b'1') {
            let (a, b) = array.split_at(pos);
            array = arbiter(a, b);
        }
    }

    array
}

fn first_occurrence(array: &[String], pos: usize, target: u8) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }

    let mut lo = 0;
    let mut hi = array.len() - 1;
    let mut idx = None;

    while lo <= hi {
        let mid = (hi - lo) / 2 + lo;
        let element = array[mid].as_bytes();

        if target == element[pos] {
            idx = Some(mid);
            if mid == 0 {
                break;
            }

            hi = mid - 1;
        } else if target < element[pos] {
            if mid == 0 {
                break;
            }
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    idx
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use aoc_shared::input::load_text_input;
    use aoc_shared::parsing::parse_line_delimited;
    use super::*;

    #[test]
    fn test_part_one() {
        let file = File::open("inputs/input.txt").unwrap();
        let input = load_text_input(file);

        let parsed = parse_line_delimited(input);
        let answer = part_one(&parsed);

        assert_eq!(741950, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let file = File::open("inputs/input.txt").unwrap();
        let input = load_text_input(file);

        let mut parsed = parse_line_delimited(input);
        let answer = part_two_v1(&mut parsed);

        assert_eq!(903810, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let file = File::open("inputs/input.txt").unwrap();
        let input = load_text_input(file);

        let mut parsed = parse_line_delimited(input);
        let answer = part_two_v2(&mut parsed);

        assert_eq!(903810, answer);
    }
}