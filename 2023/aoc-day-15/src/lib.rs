use aoc_shared::hashing::FxHashMap;
use indexmap::IndexMap;

type HashMap<K, V> = FxHashMap<K, V>;

pub fn part_one(input: &str) -> u64 {
    input.trim_end().split(',').map(|x| hash(x) as u64).sum()
}

pub fn part_two_v1(input: &str) -> u64 {
    let mut boxes = vec![IndexMap::new(); 256];

    for step in input.trim_end().split(',').map(|x| x.as_bytes()) {
        let offset = if step[step.len() - 1].is_ascii_digit() {
            2
        } else {
            1
        };

        let label = &step[..step.len() - offset];
        let hash = hash(label);

        if offset == 1 {
            boxes[hash].shift_remove(label);
        } else {
            let value = step[step.len() - 1] - b'0';
            *boxes[hash].entry(label).or_default() = value;
        }
    }

    let mut answer = 0;
    for (idx, lenses) in boxes.iter().enumerate() {
        for (pos, &value) in lenses.values().enumerate() {
            answer += (idx as u64 + 1) * (pos as u64 + 1) * (value as u64);
        }
    }

    answer
}

pub fn part_two_v2(input: &str) -> u64 {
    let mut boxes = vec![HashMap::default(); 256];

    for (ord, step) in input
        .trim_end()
        .split(',')
        .map(|x| x.as_bytes())
        .enumerate()
    {
        let offset = if step[step.len() - 1].is_ascii_digit() {
            2
        } else {
            1
        };

        let label = &step[..step.len() - offset];
        let hash = hash(label);

        if offset == 1 {
            boxes[hash].remove(label);
        } else {
            let value = step[step.len() - 1] - b'0';
            boxes[hash].entry(label).or_insert((ord, value)).1 = value;
        }
    }

    let mut answer = 0;
    let mut buffer = vec![];

    for (idx, lenses) in boxes.iter().enumerate() {
        buffer.extend(lenses.values());
        if buffer.len() > 1 {
            buffer.sort_unstable_by_key(|&(ord, _val)| ord);
        }

        for (pos, (_, value)) in buffer.drain(..).enumerate() {
            answer += (idx as u64 + 1) * (pos as u64 + 1) * (value as u64);
        }
    }

    answer
}

fn hash(segment: impl AsRef<[u8]>) -> usize {
    let mut hash = 0;

    for &ch in segment.as_ref() {
        hash += ch as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(517_965, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v1(&input);
        assert_eq!(267_372, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v2(&input);
        assert_eq!(267_372, answer);
    }
}
