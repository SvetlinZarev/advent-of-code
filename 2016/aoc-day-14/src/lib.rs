use std::collections::{HashMap, VecDeque};
use std::fmt::Write;

use aoc_shared::hashing::{FnvHasher, HashBuilder};
use md5::{Digest, Md5};

const HEX: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];
const NTH_KEY: usize = 64;

pub fn part_one(input: &str) -> u32 {
    let mut hasher = Md5::new();
    let mut raw_hash = Default::default();
    let mut hex_hash = vec![0; Md5::output_size() * 2];

    let mut sequence_string = String::with_capacity(8);
    let mut triples = HashMap::with_hasher(HashBuilder::<FnvHasher>::default());
    let mut remaining = NTH_KEY;

    for sequence in 0u32.. {
        sequence_string.clear();
        write!(sequence_string, "{}", sequence).unwrap();

        hasher.update(input);
        hasher.update(&sequence_string);

        hasher.finalize_into_reset(&mut raw_hash);
        for idx in 0..raw_hash.len() {
            hex_hash[idx * 2] = HEX[(raw_hash[idx] >> 4) as usize];
            hex_hash[idx * 2 + 1] = HEX[(raw_hash[idx] & 0x0F) as usize];
        }

        if let Some(pos) = hex_hash
            .windows(3)
            .position(|w| w[0] == w[1] && w[1] == w[2])
        {
            triples
                .entry(hex_hash[pos])
                .or_insert(VecDeque::new())
                .push_back(sequence);
        }

        if let Some(pos) = hex_hash
            .windows(5)
            .position(|w| w[0] == w[1] && w[0] == w[2] && w[0] == w[3] && w[0] == w[4])
        {
            if let Some(positions) = triples.get_mut(&hex_hash[pos]) {
                while let Some(pos) = positions.front().copied() {
                    if sequence == pos {
                        break;
                    }

                    if sequence - pos <= 1000 {
                        remaining -= 1;
                        if remaining == 0 {
                            return pos;
                        }
                    }

                    positions.pop_front();
                }
            }
        }
    }

    unreachable!()
}

pub fn part_two(input: &str) -> u32 {
    let mut hasher = Md5::new();
    let mut raw_hash = Default::default();
    let mut hex_hash = vec![0; Md5::output_size() * 2];

    let mut sequence_string = String::with_capacity(8);
    let mut triples = HashMap::with_hasher(HashBuilder::<FnvHasher>::default());
    let mut remaining = NTH_KEY;

    for sequence in 0u32.. {
        sequence_string.clear();
        write!(sequence_string, "{}", sequence).unwrap();

        hasher.update(input);
        hasher.update(&sequence_string);

        hasher.finalize_into_reset(&mut raw_hash);
        for idx in 0..raw_hash.len() {
            hex_hash[idx * 2] = HEX[(raw_hash[idx] >> 4) as usize];
            hex_hash[idx * 2 + 1] = HEX[(raw_hash[idx] & 0x0F) as usize];
        }

        for _ in 0..2016 {
            hasher.update(&hex_hash);
            hasher.finalize_into_reset(&mut raw_hash);

            for idx in 0..raw_hash.len() {
                hex_hash[idx * 2] = HEX[(raw_hash[idx] >> 4) as usize];
                hex_hash[idx * 2 + 1] = HEX[(raw_hash[idx] & 0x0F) as usize];
            }
        }

        if let Some(pos) = hex_hash
            .windows(3)
            .position(|w| w[0] == w[1] && w[1] == w[2])
        {
            triples
                .entry(hex_hash[pos])
                .or_insert(VecDeque::new())
                .push_back(sequence);
        }

        if let Some(pos) = hex_hash
            .windows(5)
            .position(|w| w[0] == w[1] && w[0] == w[2] && w[0] == w[3] && w[0] == w[4])
        {
            if let Some(positions) = triples.get_mut(&hex_hash[pos]) {
                while let Some(pos) = positions.front().copied() {
                    if sequence == pos {
                        break;
                    }

                    if sequence - pos <= 1000 {
                        remaining -= 1;
                        if remaining == 0 {
                            return pos;
                        }
                    }

                    positions.pop_front();
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(input.trim());
        assert_eq!(23769, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(input.trim());
        assert_eq!(20606, answer);
    }
}
