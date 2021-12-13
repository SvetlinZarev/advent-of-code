use crate::input::FastRule;
use crate::Rule;
use std::str::FromStr;

enum ParserState {
    ReadPolymer,
    ReadRules,
}

pub fn parse_input<S: AsRef<str>>(input: S) -> (String, Vec<Rule>) {
    let mut state = ParserState::ReadPolymer;

    let mut polymer = String::new();
    let mut rules = vec![];

    for line in input.as_ref().lines().map(|l| l.trim()) {
        if line.is_empty() {
            continue;
        }

        match state {
            ParserState::ReadPolymer => {
                polymer = line.to_owned();
                state = ParserState::ReadRules;
            }

            ParserState::ReadRules => {
                let rule = line.parse().unwrap();
                rules.push(rule);
            }
        }
    }

    rules.sort_unstable();

    (polymer, rules)
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, insert) = s
            .split_once(" -> ")
            .ok_or_else(|| format!("Cannot split string: {:?}", s))?;

        if from.len() != 2 {
            return Err(format!("Invalid 'from' length: {:?}", from));
        }
        if insert.len() != 1 {
            return Err(format!("Invalid 'insert' length: {:?}", insert));
        }

        Ok(Rule::new(from.as_bytes(), insert.as_bytes()[0]))
    }
}

pub fn optimize_rules(rules: &[Rule]) -> Vec<FastRule> {
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
