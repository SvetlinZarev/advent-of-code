use std::mem::swap;
use aoc_shared::input::stdin_line_delimited;

fn main() {
    let mut input: Vec<String> = stdin_line_delimited();
    part_one(&input);
    part_two(&mut input);
}

fn part_one(input: &[String]) {
    assert!(input.len() > 0);

    let mut ones = vec![0; input[0].len()];

    for element in input.iter().map(|i| i.as_bytes()) {
        assert_eq!(element.len(), ones.len());

        for idx in 0..element.len() {
            if element[idx] == b'1' {
                ones[idx] += 1;
            }
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
    input.sort_unstable_by(|a, b| {
        let a = a.as_bytes();
        let b = b.as_bytes();

        a[0].cmp(&b[0])
    });

    let (mut oxigen, mut co2) = match first_occurence(input, 0, b'1') {
        None => input.split_at_mut(input.len()),
        Some(idx) => input.split_at_mut(idx)
    };

    if oxigen.len() < co2.len() {
        swap(&mut oxigen, &mut co2);
    }

    oxigen = reduce_input(oxigen, |a, b| a.len() > b.len());
    assert_eq!(1, oxigen.len(), "Too many elements for oxygen: {:?}", oxigen);

    co2 = reduce_input(co2, |a, b| a.len() <= b.len());
    assert_eq!(1, co2.len(), "Too many elements for co2: {:?}", co2);

    let ox_rating = str_to_num(oxigen[0].as_str());
    let co_rating = str_to_num(co2[0].as_str());

    println!("part 2: {:?}", ox_rating * co_rating);
}

fn reduce_input(input: &mut [String], cmp: fn(&[String], &[String]) -> bool) -> &mut [String] {
    let mut reduced = input;

    for idx in 1..reduced[0].len() {
        reduced.sort_unstable_by(|a, b| {
            let a = a.as_bytes();
            let b = b.as_bytes();

            a[idx].cmp(&b[idx])
        });

        let (a, b) = match first_occurence(reduced, idx, b'1') {
            None => reduced.split_at_mut(reduced.len()),
            Some(idx) => reduced.split_at_mut(idx)
        };

        if cmp(a, b) {
            reduced = a;
        } else {
            reduced = b;
        }

        if reduced.len() <= 1 {
            break;
        }
    }

    reduced
}

fn first_occurence(array: &[String], pos: usize, target: u8) -> Option<usize> {
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

fn str_to_num(s: &str) -> u32 {
    let mut n = 0;
    for ch in s.as_bytes().iter().copied().map(|c| (c - b'0') as u32) {
        n = (n << 1) | ch;
    }
    n
}