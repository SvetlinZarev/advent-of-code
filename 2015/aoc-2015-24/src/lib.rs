use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::parsing::parse_line_delimited;
use aoc_2015_common::timing::measure;

const DAY: usize = 24;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let weights = parse_line_delimited(&input);

    let (d1, _) = measure(DAY, "part 1", || solve_part_one(&weights));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(&weights));

    d1 + d2
}

fn solve_part_one(input: &[u32]) -> u64 {
    solve::<3>(input)
}

fn solve_part_two(input: &[u32]) -> u64 {
    solve::<4>(input)
}

fn solve<const N: usize>(input: &[u32]) -> u64 {
    let mut weights = input.to_vec();
    weights.sort_unstable_by(|a, b| a.cmp(b).reverse());

    let total = weights.iter().sum::<u32>();
    assert_eq!(0, total % N as u32);
    let target = total / N as u32;

    let mut sums = [0; N];
    let mut vals = vec![vec![]; N];

    let mut len = usize::MAX;
    let mut ent = u64::MAX;

    dfs(
        &mut len, &mut ent, &mut vals, &mut sums, &weights, target, 0,
    );
    ent
}

fn dfs(
    len: &mut usize,
    ent: &mut u64,
    vals: &mut [Vec<u32>],
    sums: &mut [u32],
    weights: &[u32],
    target: u32,
    index: usize,
) {
    if index == weights.len() {
        if sums.iter().all(|&x| x == target) {
            let min_len = vals.iter().fold(usize::MAX, |acc, x| acc.min(x.len()));
            if min_len <= *len {
                if let Some(answer) = vals
                    .iter()
                    .filter(|set| set.len() == min_len)
                    .map(|set| set.iter().map(|&x| x as u64).product::<u64>())
                    .min()
                {
                    if min_len < *len || answer < *ent {
                        *ent = answer;
                    }
                }

                *len = min_len;
            }
        }

        return;
    }

    for i in 0..sums.len() {
        if sums[i] + weights[index] > target {
            continue;
        }

        if sums
            .iter()
            .copied()
            .enumerate()
            .filter(|&(pos, _)| pos < i)
            .any(|(_, val)| val == sums[i])
        {
            continue;
        }

        sums[i] += weights[index];
        vals[i].push(weights[index]);

        dfs(len, ent, vals, sums, weights, target, index + 1);

        vals[i].pop();
        sums[i] -= weights[index];
    }
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let weights = parse_line_delimited(&input);
        let answer = solve_part_one(&weights);
        assert_eq!(11266889531, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let weights = parse_line_delimited(&input);
        let answer = solve_part_two(&weights);
        assert_eq!(77387711, answer);
    }
}
