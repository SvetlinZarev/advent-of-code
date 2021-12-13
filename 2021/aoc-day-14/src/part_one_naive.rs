use crate::Rule;

pub fn part_one_naive(polymer: &str, rules: &[Rule]) -> u32 {
    let mut a = polymer.as_bytes().to_vec();
    let mut b = vec![];

    for _ in 0..10 {
        for w in a.windows(2) {
            let rule_idx = rules
                .binary_search_by(|r| r.from.as_slice().cmp(w))
                .unwrap();
            let rule = rules[rule_idx];

            b.extend_from_slice(rule.to.as_slice());
        }

        b.push(a[a.len() - 1]);
        std::mem::swap(&mut a, &mut b);
        b.clear();
    }

    let mut freq = [0u32; (b'Z' - b'A' + 1) as usize];
    a.iter()
        .map(|&c| (c - b'A') as usize)
        .for_each(|c| freq[c] += 1);

    let (most, least) = freq
        .iter()
        .copied()
        .filter(|&c| c != 0)
        .fold((0, u32::MAX), |(m, l), c| (m.max(c), l.min(c)));

    most - least
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one_v1() {
        let (polymer, rules) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one_naive(&polymer, &rules);
        assert_eq!(3411, answer);
    }
}
