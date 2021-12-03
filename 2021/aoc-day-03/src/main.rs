use std::mem::swap;
use aoc_shared::input::stdin_line_delimited;

fn main() {
    let mut input: Vec<String> = stdin_line_delimited();
    part_one(&input);
    part_two(&mut input);
}

fn part_one(input: &[String]) {
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

    println!("Part 1: {}", gamma * epsilon);
}

fn part_two(input: &mut [String]) {
    let (mut oxygen, mut co2) = split(input, 0);

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

    println!("Part 2: {:?}", ox_rating * co_rating);
}

fn reduce_input(input: &mut [String], is_oxygen_rating: bool) -> &mut [String] {
    assert!(!input.is_empty());

    let mut reduced = input;

    for idx in 1..reduced[0].len() {
        let (zeroes, ones) = split(reduced, idx);

        let (mut larger, mut smaller) = (zeroes, ones);
        if larger.len() <= smaller.len() {
            swap(&mut larger, &mut smaller);
        }

        if is_oxygen_rating {
            reduced = larger;
        } else {
            reduced = smaller;
        }

        if reduced.len() <= 1 {
            break;
        }
    }

    reduced
}

fn split(array: &mut [String], idx: usize) -> (&mut [String], &mut [String]) {
    let mut dst = 0;
    let mut src = 0;

    while src < array.len() {
        if array[src].as_bytes()[idx] == b'0' {
            array.swap(dst, src);
            dst += 1;
        }

        src += 1;
    }

    array.split_at_mut(dst)
}
