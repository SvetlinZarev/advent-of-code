use std::collections::{HashMap, HashSet};

use regex::Regex;

const SHINY_GOLD_BAG: &str = "shiny gold";
const REGEX_BAG_TYPE: &str = "([a-z]+ [a-z]+ [a-z]+)";
const REGEX_BAG_CONTENT: &str = "(\\d+ [a-z]+ [a-z]+ [a-z]+)";

pub fn parse_input(input: &str) -> HashMap<&str, HashSet<(&str, usize, usize)>> {
    let pattern_bag_type = Regex::new(REGEX_BAG_TYPE).unwrap();
    let pattern_bag_content = Regex::new(REGEX_BAG_CONTENT).unwrap();
    let mut graph = HashMap::<&str, HashSet<(&str, usize, usize)>>::new();

    for line in input.lines() {
        let bag_type_match = pattern_bag_type.find(line).unwrap();
        let bag = remove_suffix(&line[bag_type_match.start()..bag_type_match.end()]);

        let mut content = HashSet::new();
        let bag_content_matcher = pattern_bag_content.find_iter(line);

        for content_match in bag_content_matcher {
            let raw = &line[content_match.start()..content_match.end()];
            let count_separator_idx = raw.find(" ").unwrap();

            let count = raw[..count_separator_idx].parse::<usize>().unwrap();
            let kind = remove_suffix(&raw[count_separator_idx + 1..]);

            content.insert((kind, count, 1));
        }

        let old_entry = graph.insert(bag, content);
        assert!(old_entry.is_none());
    }

    graph
}

pub fn solve(input: &HashMap<&str, HashSet<(&str, usize, usize)>>) -> usize {
    let mut queue = vec![];
    input
        .get(SHINY_GOLD_BAG)
        .unwrap()
        .iter()
        .for_each(|v| queue.push(v.clone()));

    let mut result = 0;
    while let Some((kind, count, mul)) = queue.pop() {
        result += count * mul;

        let additional = input.get(kind).unwrap();
        for a in additional {
            let mut to_add = a.clone();
            to_add.2 = to_add.2 * count * mul;
            queue.push(to_add);
        }
    }

    result
}

fn remove_suffix(v: &str) -> &str {
    if v.ends_with("bags") {
        &v[..v.len() - 5]
    } else if v.ends_with("bag") {
        &v[..v.len() - 4]
    } else {
        v
    }
}
