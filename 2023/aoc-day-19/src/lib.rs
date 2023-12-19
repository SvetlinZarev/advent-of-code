use std::error::Error;
use std::fmt::Debug;
use std::num::NonZeroUsize;

use aoc_shared::hashing::FxHashMap;

type HashMap<K, V> = FxHashMap<K, V>;

#[derive(Debug, Clone)]
pub struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    pub fn apply(&self, xmas: Xmas, on_accept: &mut dyn FnMut(Xmas)) {
        self.rules[0].apply(&self.rules, xmas, on_accept);
    }

    pub fn accepted_combinations(
        &self,
        min: Xmas,
        max: Xmas,
        on_accept: &mut dyn FnMut(Xmas, Xmas),
    ) {
        self.rules[0].accepted_ranges(&self.rules, min, max, on_accept)
    }
}

#[derive(Debug, Clone)]
struct Rule {
    checks: [(Check, Action); 4],
    size: usize,
    default: Action,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            checks: [
                (Check::X(0, u32::MAX), Action::Reject),
                (Check::M(0, u32::MAX), Action::Reject),
                (Check::A(0, u32::MAX), Action::Reject),
                (Check::S(0, u32::MAX), Action::Reject),
            ],
            size: 0,
            default: Action::Reject,
        }
    }
}

impl Rule {
    pub fn apply(&self, rules: &[Rule], xmas: Xmas, on_accept: &mut dyn FnMut(Xmas)) {
        let mut action = self.default;

        for idx in 0..self.size {
            if self.checks[idx].0.check(xmas) {
                action = self.checks[idx].1;
                break;
            }
        }

        match action {
            Action::Next(next) => {
                rules[next.get()].apply(rules, xmas, on_accept);
            }

            Action::Accept => {
                on_accept(xmas);
            }

            Action::Reject => {
                // do nothing
            }
        }
    }

    pub fn accepted_ranges(
        &self,
        rules: &[Rule],
        min: Xmas,
        max: Xmas,
        on_accept: &mut dyn FnMut(Xmas, Xmas),
    ) {
        let mut range = Some((min, max));

        for idx in 0..self.size {
            if let Some((min, max)) = range.take() {
                let (mut p, mut q) = (min, max); // passes check
                let (m, mut n) = (min, max); // does not pass check: left
                let (mut l, r) = (min, max); // does not pass check: right

                match self.checks[idx].0 {
                    Check::X(start, end) => {
                        // left of intersection
                        n.x = n.x.min(start);

                        // right of intersection
                        l.x = l.x.max(end);

                        // intersection
                        p.x = p.x.max(start);
                        q.x = q.x.min(end);
                    }
                    Check::M(start, end) => {
                        // left of intersection
                        n.m = n.m.min(start);

                        // right of intersection
                        l.m = l.m.max(end);

                        // intersection
                        p.m = p.m.max(start);
                        q.m = q.m.min(end);
                    }
                    Check::A(start, end) => {
                        // left of intersection
                        n.a = n.a.min(start);

                        // right of intersection
                        l.a = l.a.max(end);

                        // intersection
                        p.a = p.a.max(start);
                        q.a = q.a.min(end);
                    }
                    Check::S(start, end) => {
                        // left of intersection
                        n.s = n.s.min(start);

                        // right of intersection
                        l.s = l.s.max(end);

                        // intersection
                        p.s = p.s.max(start);
                        q.s = q.s.min(end);
                    }
                }

                match self.checks[idx].1 {
                    Action::Next(rule) => {
                        if is_valid_range(p, q) {
                            rules[rule.get()].accepted_ranges(rules, p, q, on_accept);
                        }
                    }

                    Action::Accept => {
                        if is_valid_range(p, q) {
                            on_accept(p, q);
                        }
                    }

                    Action::Reject => {
                        // do nothing
                    }
                }

                // Because we are always splitting the range in TWO parts,
                // one of the two non-overlapping parts is actually invalid.
                // Thus we can produce at most 1 range for the next iteration
                if is_valid_range(m, n) {
                    range = Some((m, n));
                } else if is_valid_range(l, r) {
                    range = Some((l, r));
                }
            }
        }

        if let Some((min, max)) = range.take() {
            match self.default {
                Action::Next(next) => rules[next.get()].accepted_ranges(rules, min, max, on_accept),
                Action::Accept => on_accept(min, max),
                Action::Reject => { /* do nothing */ }
            }
        }
    }
}

fn is_valid_range(min: Xmas, max: Xmas) -> bool {
    (min.x < max.x) & (min.m < max.m) & (min.a < max.a) && (min.s < max.s)
}

#[derive(Debug, Clone, Copy)]
enum Check {
    X(u32, u32),
    M(u32, u32),
    A(u32, u32),
    S(u32, u32),
}

impl Check {
    pub fn check(self, xmas: Xmas) -> bool {
        match self {
            Check::X(a, b) => (a..b).contains(&xmas.x),
            Check::M(a, b) => (a..b).contains(&xmas.m),
            Check::A(a, b) => (a..b).contains(&xmas.a),
            Check::S(a, b) => (a..b).contains(&xmas.s),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Action {
    Next(NonZeroUsize),
    Accept,
    Reject,
}

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Xmas {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Xmas {
    fn sum(self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

pub fn parse_input(input: &str, parse_data: bool) -> Result<(RuleSet, Vec<Xmas>), Box<dyn Error>> {
    let mut rules = RuleSet { rules: vec![] };

    let mut lines = input.lines();
    let mut rule_names = HashMap::default();

    //always put the "in" rule at position 0; Push dummy rule to reserve the sport
    rule_names.insert("in", 0);
    rules.rules.push(Rule::default());

    // parse the rules
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let Some((name, text)) = line.split_once('{') else {
            return Err(format!("Invalid rule: {:?}", line).into());
        };

        let pos = rule_position(&mut rule_names, name);
        let mut rule = Rule::default();

        for r in text[..text.len() - 1].split(',') {
            // handle default rule
            if r.len() <= 3 {
                rule.default = to_action(&mut rule_names, r);
                break;
            }

            let key = &r[..1].as_bytes()[0];
            let op = &r[1..2].as_bytes()[0];

            let Some((val, target)) = r[2..].rsplit_once(':') else {
                return Err(format!("Invalid sub-rule: {:?}", r).into());
            };

            let action = to_action(&mut rule_names, target);

            let val = val.parse()?;
            let (lo, hi) = match op {
                b'>' => (val + 1, u32::MAX),
                b'<' => (u32::MIN, val),
                _ => return Err(format!("Invalid sub-rule: {:?}", r).into()),
            };

            let check = match key {
                b'x' => Check::X(lo, hi),
                b'm' => Check::M(lo, hi),
                b'a' => Check::A(lo, hi),
                b's' => Check::S(lo, hi),
                _ => return Err(format!("Invalid sub-rule: {:?}", r).into()),
            };

            rule.checks[rule.size] = (check, action);
            rule.size += 1;
        }

        if pos >= rules.rules.len() {
            rules.rules.resize(pos + 1, Rule::default());
        }
        rules.rules[pos] = rule;
    }

    // Parse the data
    let mut data = vec![];

    if parse_data {
        while let Some(line) = lines.next() {
            let mut xmas = Xmas::default();

            for part in line[1..line.len() - 1].split(',') {
                let key = &part[..1].as_bytes()[0];
                let value = &part[2..];
                let value = value.parse()?;

                match key {
                    b'x' => xmas.x = value,
                    b'm' => xmas.m = value,
                    b'a' => xmas.a = value,
                    b's' => xmas.s = value,
                    _ => return Err(format!("Invalid xmas definition: {:?}", line).into()),
                }
            }

            data.push(xmas);
        }
    }

    Ok((rules, data))
}

fn to_action<'l>(rule_names: &mut HashMap<&'l str, usize>, target: &'l str) -> Action {
    match target {
        "A" => Action::Accept,
        "R" => Action::Reject,
        _ => {
            let pos = rule_position(rule_names, target);
            Action::Next(NonZeroUsize::new(pos).unwrap())
        }
    }
}

fn rule_position<'l>(rule_names: &mut HashMap<&'l str, usize>, name: &'l str) -> usize {
    let pos = rule_names.len();
    *rule_names.entry(name).or_insert(pos)
}

pub fn part_one(rules: &RuleSet, data: &[Xmas]) -> u32 {
    let mut answer = 0;

    for xmas in data.iter().copied() {
        rules.apply(xmas, &mut |xmas| answer += xmas.sum());
    }

    answer
}

pub fn part_two(rules: &RuleSet) -> u64 {
    let mut answer = 0;

    let min = Xmas {
        x: 1,
        m: 1,
        a: 1,
        s: 1,
    };

    let max = Xmas {
        x: 4001,
        m: 4001,
        a: 4001,
        s: 4001,
    };

    rules.accepted_combinations(min, max, &mut |min, max| {
        answer += (max.x - min.x) as u64
            * (max.m - min.m) as u64
            * (max.a - min.a) as u64
            * (max.s - min.s) as u64;
    });

    answer
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (rules, data) = parse_input(&input, true).unwrap();

        let answer = part_one(&rules, &data);
        assert_eq!(495_298, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (rules, _) = parse_input(&input, false).unwrap();

        let answer = part_two(&rules);
        assert_eq!(132_186_256_794_011, answer);
    }
}
