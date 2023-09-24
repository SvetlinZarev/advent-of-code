use std::cmp::Reverse;
use std::collections::BinaryHeap;

const NUM_ELEMENTS: usize = 5;

pub fn part_one(input: &str) -> u64 {
    let mut pq = BinaryHeap::with_capacity(NUM_ELEMENTS);
    let mut answer = 0;

    'next: for line in input.lines() {
        let (line, checksum) = line[..line.len() - 1].rsplit_once('[').unwrap();
        let (line, sector_id) = line.rsplit_once('-').unwrap();

        let mut freq = [0u32; (b'z' - b'a' + 1) as usize];
        for ch in line.bytes() {
            if ch.is_ascii_lowercase() {
                freq[(ch - b'a') as usize] += 1;
            }
        }

        pq.clear();
        for (ch, cnt) in freq.iter().copied().enumerate().filter(|&(_, cnt)| cnt > 0) {
            let ch = ch as u8 + b'a';

            if pq.len() < NUM_ELEMENTS {
                pq.push((Reverse(cnt), ch))
            } else {
                let (Reverse(c), x) = pq.peek().copied().unwrap();
                if c < cnt || (c == cnt && x > ch) {
                    pq.pop();
                    pq.push((Reverse(cnt), ch))
                }
            }
        }

        if pq.len() != checksum.len() {
            continue;
        }

        let checksum = checksum.as_bytes();
        let mut idx = checksum.len();

        while let Some((_, ch)) = pq.pop() {
            idx -= 1;

            if checksum[idx] != ch {
                continue 'next;
            }
        }

        let sector_id = sector_id.parse::<u64>().unwrap();
        answer += sector_id;
    }

    answer
}

pub fn part_two(input: &str) -> u64 {
    let mut pq = BinaryHeap::with_capacity(NUM_ELEMENTS);
    let mut decoded = vec![];

    'next: for line in input.lines() {
        let (line, checksum) = line[..line.len() - 1].rsplit_once('[').unwrap();
        let (line, sector_id) = line.rsplit_once('-').unwrap();

        let mut freq = [0u32; (b'z' - b'a' + 1) as usize];
        for ch in line.bytes() {
            if ch.is_ascii_lowercase() {
                freq[(ch - b'a') as usize] += 1;
            }
        }

        pq.clear();
        for (ch, cnt) in freq.iter().copied().enumerate().filter(|&(_, cnt)| cnt > 0) {
            let ch = ch as u8 + b'a';

            if pq.len() < NUM_ELEMENTS {
                pq.push((Reverse(cnt), ch))
            } else {
                let (Reverse(c), x) = pq.peek().copied().unwrap();
                if c < cnt || (c == cnt && x > ch) {
                    pq.pop();
                    pq.push((Reverse(cnt), ch))
                }
            }
        }

        if pq.len() != checksum.len() {
            continue;
        }

        let checksum = checksum.as_bytes();
        let mut idx = checksum.len();

        while let Some((_, ch)) = pq.pop() {
            idx -= 1;

            if checksum[idx] != ch {
                continue 'next;
            }
        }

        let sector_id = sector_id.parse::<u64>().unwrap();
        decoded.clear();

        for ch in line.bytes() {
            if ch.is_ascii_lowercase() {
                let ch = (((ch - b'a') as u64 + sector_id) % (b'z' - b'a' + 1) as u64) as u8 + b'a';
                decoded.push(ch);
            }
        }

        if decoded.starts_with("northpole".as_bytes()) {
            return sector_id;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(137_896, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(501, answer);
    }
}
