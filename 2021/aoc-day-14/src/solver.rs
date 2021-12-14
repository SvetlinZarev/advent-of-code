use crate::Rule;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
struct FastRule {
    pub(crate) key: [u8; 2],
    pub(crate) first: usize,
    pub(crate) second: usize,
}

impl FastRule {
    pub fn new(key: [u8; 2], first: usize, second: usize) -> Self {
        Self { key, first, second }
    }
}

pub fn solve(polymer: &str, rules: &[Rule], rounds: u32) -> u64 {
    let rules = optimize_rules(rules);

    let mut freq = vec![0; rules.len()];
    polymer.as_bytes().windows(2).for_each(|w| {
        let idx = rules.binary_search_by(|r| r.key.as_slice().cmp(w)).unwrap();
        freq[idx] += 1;
    });

    let mut next = vec![0; rules.len()];
    for _ in 0..rounds {
        for idx in 0..freq.len() {
            if freq[idx] != 0 {
                let rule = rules[idx];
                next[rule.first] += freq[idx];
                next[rule.second] += freq[idx];
                freq[idx] = 0;
            }
        }
        std::mem::swap(&mut freq, &mut next);
    }

    let mut chars = [0u64; (b'Z' - b'A' + 1) as usize];
    for idx in 0..freq.len() {
        // Add only the first character of the key, because the second character
        // will appear as "first" in another key, except for the very last rule
        let pos = (rules[idx].key[0] - b'A') as usize;
        chars[pos] += freq[idx];
    }

    // Update the frequency of the very last character in the polymer. It
    // never changes, so we can take it directly from the polymer string
    let last = (polymer.as_bytes()[polymer.len() - 1] - b'A') as usize;
    chars[last] += 1;

    let (most, least) = chars
        .iter()
        .copied()
        .filter(|&c| c != 0)
        .fold((0u64, u64::MAX), |(m, l), c| (m.max(c), l.min(c)));

    most - least
}

fn optimize_rules(rules: &[Rule]) -> Vec<FastRule> {
    let mut fast_rules = vec![FastRule::default(); rules.len()];

    for (idx, &rule) in rules.iter().enumerate() {
        debug_assert!(rules.iter().any(|r| r.from == rule.to));

        let first = rules.binary_search_by(|r| r.from.cmp(&rule.to)).unwrap();
        let second = rules
            .binary_search_by(|r| {
                let key = [rule.to[1], rule.from[1]];
                r.from.cmp(&key)
            })
            .unwrap();

        fast_rules[idx] = FastRule::new(rule.from, first, second);
    }

    fast_rules
}
