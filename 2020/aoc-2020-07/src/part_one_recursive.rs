use std::collections::{HashMap, HashSet};

use regex::Regex;

const SHINY_GOLD_BAG: &str = "shiny gold";
const CONTAINS_NO_OTHER: &str = "contain no other";
const REGEX_BAG_TYPE: &str = "([a-z]+ [a-z]+ [a-z]+)";

pub fn solve_v1(input: &HashMap<&str, HashSet<&str>>) -> usize {
    let mut result = 0;
    for (_, v) in input.iter() {
        let mut set = HashSet::new();
        add_to_set(&input, &mut set, v);

        if set.contains(SHINY_GOLD_BAG) {
            result += 1;
        }
    }
    result
}

fn add_to_set<'v>(
    graph: &HashMap<&'v str, HashSet<&'v str>>,
    set: &mut HashSet<&'v str>,
    to_add: &HashSet<&'v str>,
) {
    for &value in to_add {
        if !set.insert(value) {
            continue;
        }

        match graph.get(value) {
            None => panic!("Unexpected bag: {}", value),
            Some(additional) => add_to_set(graph, set, additional),
        }
    }
}

pub fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let pattern_bag_type = Regex::new(REGEX_BAG_TYPE).unwrap();
    let mut graph = HashMap::<&str, HashSet<&str>>::new();

    for line in input.lines() {
        let mut matches = pattern_bag_type.find_iter(line);

        let bag_match = matches.next().unwrap();
        let bag = remove_suffix(&line[bag_match.start()..bag_match.end()]);

        let mut content = HashSet::new();
        for bag_match in matches {
            let value = remove_suffix(&line[bag_match.start()..bag_match.end()]);
            if value != CONTAINS_NO_OTHER {
                content.insert(value);
            }
        }

        let old_entry = graph.insert(bag, content);
        assert!(old_entry.is_none());
    }

    graph
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
