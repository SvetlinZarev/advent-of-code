use std::cell::RefCell;
use std::collections::HashMap;

const SHINY_GOLD: &str = "shiny gold";

#[derive(Debug, Clone)]
pub struct Rule<'l> {
    pub kind: &'l str,
    pub count: usize,
}

impl<'l> Rule<'l> {
    pub fn new(kind: &str, count: usize) -> Rule {
        Rule { kind, count }
    }
}

pub fn parse_input(input: &str) -> HashMap<&str, Vec<Rule>> {
    let mut graph = HashMap::with_capacity(800);

    for line in input.lines() {
        let mut idx_kind = line.find(' ').unwrap() + 1; // +1 because of the space
        idx_kind += &line[idx_kind..].find(' ').unwrap();
        let kind = &line[..idx_kind];

        let mut rules = vec![];
        let mut remaining = &line[idx_kind + 14..]; // skip ' bags contains '

        if !remaining.starts_with("no ") {
            // chick if there ar any rules for this bag
            //each iteration processes one rule
            while !remaining.is_empty() {
                let idx_cnt = remaining.find(' ').unwrap();

                let mut idx_kind = remaining[idx_cnt + 1..].find(' ').unwrap() + idx_cnt + 1;
                idx_kind += remaining[idx_kind + 1..].find(' ').unwrap() + 1;

                let count = remaining[..idx_cnt].parse().unwrap();
                let kind = &remaining[idx_cnt + 1..idx_kind];

                remaining = if count == 1 {
                    &remaining[idx_kind + 4..] // skip ' bag'
                } else {
                    &remaining[idx_kind + 5..] // skip ' bags'
                };

                // The dot appears only at the end, so if there is only
                // one byte left, then it's the dot, otherwise it's a separator
                remaining = if remaining.len() > 1 {
                    &remaining[2..] // skip ', '
                } else {
                    &remaining[1..] // skip '.'
                };

                let rule = Rule::new(kind, count);
                rules.push(rule);
            }
        }

        let old = graph.insert(kind, rules);
        assert!(old.is_none(), "{:#?}", old);
    }

    graph
}

pub fn solve_dfs(graph: &HashMap<&str, Vec<Rule>>) -> usize {
    let processed = RefCell::new(HashMap::<&str, bool>::with_capacity(graph.capacity()));

    let mut solution = 0;
    for &kind in graph.keys() {
        if part_one_dfs(graph, &processed, kind) {
            solution += 1;
        }
    }

    solution
}

fn part_one_dfs<'l>(
    graph: &'l HashMap<&'l str, Vec<Rule<'l>>>,
    processed: &RefCell<HashMap<&'l str, bool>>,
    name: &'l str,
) -> bool {
    if let Some(&can_contain_gold) = processed.borrow().get(name) {
        return can_contain_gold;
    }

    let rules = graph.get(name).unwrap();
    for rule in rules {
        if rule.kind == SHINY_GOLD {
            processed.borrow_mut().insert(name, true);
            return true;
        }

        if part_one_dfs(graph, processed, rule.kind) {
            processed.borrow_mut().insert(name, true);
            return true;
        }
    }

    processed.borrow_mut().insert(name, false);
    false
}
