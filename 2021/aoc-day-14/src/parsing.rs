use std::str::FromStr;

enum ParserState {
    ReadPolymer,
    ReadRules,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rule {
    pub(crate) from: [u8; 2],
    pub(crate) to: [u8; 2],
}

impl Rule {
    pub fn new<S: AsRef<[u8]>>(from: S, insert: u8) -> Self {
        let from = from.as_ref();
        assert_eq!(2, from.len());

        Self {
            from: [from[0], from[1]],
            to: [from[0], insert],
        }
    }
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
