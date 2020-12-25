use std::path::Path;

use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;

pub const INPUT_PATH_P1: &'static str = "../puzzle-inputs/day-19-1.txt";
pub const INPUT_PATH_P2: &'static str = "../puzzle-inputs/day-19-2.txt";

pub fn demo_part_one<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let (rules, messages) = parse_input(&input);
    measure_solution(19, 1, "", || solve(&rules, &messages));
}

pub fn demo_part_two<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let (rules, messages) = parse_input(&input);

    measure_solution(19, 2, "", || solve(&rules, &messages));
}

#[derive(Debug, Clone)]
pub enum Rule {
    Panic,
    MatchByte(u8),
    MatchAll(Vec<u8>),
    MatchEither(Vec<u8>, Vec<u8>),
}

pub fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<u8>>) {
    let mut messages = vec![];
    let mut rules = Vec::with_capacity(256);
    rules.resize_with(256, || Rule::Panic);

    let mut parse_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            parse_rules = false;
            continue;
        }

        if parse_rules {
            let (idx, rule) = parse_rule(line);
            rules[idx] = rule;
        } else {
            let message = line.bytes().collect();
            messages.push(message);
        }
    }

    (rules, messages)
}

pub fn parse_rule(line: &str) -> (usize, Rule) {
    let mut idx_from = 0;
    let mut idx_to = line.find(':').unwrap();
    let rule_number = line[idx_from..idx_to].parse().unwrap();

    idx_from = idx_to + 2;
    let line = &line[idx_from..];

    if line.starts_with('"') {
        return (rule_number, Rule::MatchByte(line.as_bytes()[1]));
    }

    let mut fg = vec![];
    let mut line = line;
    while !line.is_empty() && line.as_bytes()[0] != b'|' {
        idx_to = match line.find(' ') {
            Some(idx) => idx,
            None => line.len(),
        };

        let num = line[..idx_to].parse().unwrap();
        fg.push(num);

        idx_from = (idx_to + 1).min(line.len());
        line = &line[idx_from..];
    }

    if !line.is_empty() {
        line = &line[2..];
    }

    //reverse in order to be able to use vec.pop() in MatchAll
    fg.reverse();

    if line.is_empty() {
        return (rule_number, Rule::MatchAll(fg));
    }

    let mut sg = vec![];
    while !line.is_empty() {
        idx_to = match line.find(' ') {
            Some(idx) => idx,
            None => line.len(),
        };

        let num = line[..idx_to].parse().unwrap();
        sg.push(num);

        idx_from = (idx_to + 1).min(line.len());
        line = &line[idx_from..];
    }

    //reverse in order to be able to use vec.pop() in MatchAll
    // The first group - fg - was already reversed in the code above
    sg.reverse();

    (rule_number, Rule::MatchEither(fg, sg))
}

pub fn solve(rules: &[Rule], msgs: &[Vec<u8>]) -> usize {
    msgs.iter()
        .filter(|&m| is_valid(rules, m))
        .map(|m| m)
        .count()
}

fn is_valid(rules: &[Rule], msg: &[u8]) -> bool {
    match_rule_idx(rules, msg, 0, 0).map_or(false, |m| m.contains(&msg.len()))
}

fn match_rule_idx(rules: &[Rule], msg: &[u8], rn: u8, midx: usize) -> Option<Vec<usize>> {
    let rule = rules[rn as usize].clone();
    match_rule(rules, msg, rule, midx)
}

fn match_rule(rules: &[Rule], msg: &[u8], rule: Rule, midx: usize) -> Option<Vec<usize>> {
    match rule {
        Rule::Panic => {
            panic!("Unexpected rule");
        }

        Rule::MatchByte(b) => {
            if midx >= msg.len() {
                return None;
            }

            return if msg[midx] == b { Some(vec![1]) } else { None };
        }

        Rule::MatchAll(mut group) => {
            let r = group.pop().unwrap();

            return match match_rule_idx(rules, msg, r, midx) {
                None => None,
                Some(matched) => {
                    if group.is_empty() {
                        return Some(matched);
                    }

                    let mut result = vec![];
                    for m in matched {
                        if let Some(matches) =
                            match_rule(rules, msg, Rule::MatchAll(group.clone()), midx + m)
                        {
                            matches.iter().map(|&x| x + m).for_each(|x| result.push(x));
                        }
                    }

                    if result.is_empty() {
                        return None;
                    }

                    // duplicate offsets will will result in the same three
                    // being visited many times
                    result.sort_unstable();
                    result.dedup();
                    Some(result)
                }
            };
        }
        Rule::MatchEither(a, b) => {
            let ga = match_rule(rules, msg, Rule::MatchAll(a), midx);
            let gb = match_rule(rules, msg, Rule::MatchAll(b), midx);

            if ga.is_none() {
                return gb;
            }

            if gb.is_none() {
                return ga;
            }

            let mut matches = ga.unwrap();
            matches.extend_from_slice(&gb.unwrap());

            // duplicate offsets will will result in the same three
            // being visited many times
            matches.sort_unstable();
            matches.dedup();

            Some(matches)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(INPUT_PATH_P1);
        let (rules, messages) = parse_input(&input);

        let solution = solve(&rules, &messages);
        assert_eq!(272, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(INPUT_PATH_P2);
        let (rules, messages) = parse_input(&input);

        let solution = solve(&rules, &messages);
        assert_eq!(374, solution);
    }
}
