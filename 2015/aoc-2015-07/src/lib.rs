use std::collections::HashMap;
use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

pub const DAY: usize = 7;

const WIRE_B: &'static str = "b";

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d_p, mut wires) = measure(DAY, "parsing", || parse_input(&input));
    let (d_1, signal) = measure(DAY, "part 1", || solve(&wires));

    wires.insert(WIRE_B, Operation::Set(signal));
    let (d_2, _) = measure(DAY, "part 2", || solve(&wires));

    d_p.add(d_1).add(d_2)
}

#[derive(Debug, Copy, Clone)]
pub enum Operation<'l> {
    Set(u16),
    SetReg(&'l str),
    Not(&'l str),
    AndReg(&'l str, &'l str),
    AndVal(u16, &'l str),
    OR(&'l str, &'l str),
    Shl(&'l str, u32),
    Shr(&'l str, u32),
}

pub fn parse_input(input: &str) -> HashMap<&str, Operation> {
    let mut ops = HashMap::new();

    for line in input.lines() {
        let (dst, op) = if line.contains("AND") {
            let (a, b, c) = parse_triple(line, 3);
            let op = match a.parse() {
                Ok(value) => Operation::AndVal(value, b),
                Err(_) => Operation::AndReg(a, b),
            };
            (c, op)
        } else if line.contains("OR") {
            let (a, b, c) = parse_triple(line, 2);
            (c, Operation::OR(a, b))
        } else if line.contains("LSHIFT") {
            let (a, b, c) = parse_triple(line, 6);
            (c, Operation::Shl(a, b.parse().unwrap()))
        } else if line.contains("RSHIFT") {
            let (a, b, c) = parse_triple(line, 6);
            (c, Operation::Shr(a, b.parse().unwrap()))
        } else if line.starts_with("NOT") {
            let (a, b) = parse_tuple(line, 3);
            (b, Operation::Not(a))
        } else {
            let end = line.rfind(' ').unwrap();
            let dst = &line[end + 1..];

            let end = line.find(' ').unwrap();
            let op = match line[..end].parse() {
                Ok(value) => Operation::Set(value),
                Err(_) => Operation::SetReg(&line[..end]),
            };
            (dst, op)
        };

        ops.insert(dst, op);
    }

    ops
}

fn parse_triple(line: &str, op_len: usize) -> (&str, &str, &str) {
    let mut line = line;

    let mut end = line.find(' ').unwrap();
    let a = &line[..end];

    line = &line[end + 2 + op_len..];
    end = line.find(' ').unwrap();
    let b = &line[..end];

    let c = &line[end + 4..];

    (a, b, c)
}

fn parse_tuple(line: &str, op_len: usize) -> (&str, &str) {
    let mut line = &line[op_len + 1..];

    let end = line.find(' ').unwrap();
    let a = &line[..end];

    line = &line[end + 4..];

    (a, line)
}

pub fn solve(ops: &HashMap<&str, Operation>) -> u16 {
    let mut cache: HashMap<&str, u16> = HashMap::new();
    wire(ops, &mut cache, "a")
}

fn wire<'c, 'l: 'c>(
    ops: &'c HashMap<&'l str, Operation<'l>>,
    cache: &'c mut HashMap<&'l str, u16>,
    key: &'l str,
) -> u16 {
    let op = ops.get(key).unwrap();

    let value = match *op {
        Operation::Set(x) => {
            x //
        }
        Operation::SetReg(x) => {
            resolve(ops, cache, x) //
        }
        Operation::Not(x) => {
            !resolve(ops, cache, x) //
        }
        Operation::AndReg(a, b) => {
            let av = resolve(ops, cache, a);
            let bv = resolve(ops, cache, b);
            av & bv
        }
        Operation::AndVal(av, b) => {
            av & resolve(ops, cache, b) //
        }
        Operation::OR(a, b) => {
            let av = resolve(ops, cache, a);
            let bv = resolve(ops, cache, b);
            av | bv
        }
        Operation::Shl(a, bv) => {
            wire(ops, cache, a) << bv //
        }
        Operation::Shr(a, bv) => {
            wire(ops, cache, a) >> bv //
        }
    };

    cache.insert(key, value);
    value
}

fn resolve<'c, 'l: 'c>(
    ops: &'c HashMap<&'l str, Operation<'l>>,
    cache: &'c mut HashMap<&'l str, u16>,
    key: &'l str,
) -> u16 {
    if let Some(&value) = cache.get(key) {
        return value;
    }

    return wire(ops, cache, key);
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_parse_and_reg() {
        let mut x = parse_input("jx AND jz -> ka");
        match x.remove("ka").unwrap() {
            Operation::AndReg("jx", "jz") => {}
            x => panic!("unexpected operation: {:?}", x),
        }
    }

    #[test]
    fn test_parse_and_val() {
        let mut x = parse_input("2 AND jz -> ka");
        match x.remove("ka").unwrap() {
            Operation::AndVal(2, "jz") => {}
            x => panic!("unexpected operation: {:?}", x),
        }
    }

    #[test]
    fn test_parse_or() {
        let mut x = parse_input("jx OR jz -> ka");
        match x.remove("ka").unwrap() {
            Operation::OR("jx", "jz") => {}
            op => panic!("Unexpected operation: {:?}", op),
        }
    }

    #[test]
    fn test_parse_not() {
        let mut x = parse_input("NOT ax -> ka");
        match x.remove("ka").unwrap() {
            Operation::Not("ax") => {}
            op => panic!("Unexpected operation: {:?}", op),
        }
    }

    #[test]
    fn test_parse_set() {
        let mut x = parse_input("1024 -> ka");
        match x.remove("ka").unwrap() {
            Operation::Set(1024) => {}
            op => panic!("Unexpected operation: {:?}", op),
        }
    }

    #[test]
    fn test_parse_set_reg() {
        let mut x = parse_input("boo -> ka");
        match x.remove("ka").unwrap() {
            Operation::SetReg("boo") => {}
            op => panic!("Unexpected operation: {:?}", op),
        }
    }

    #[test]
    fn test_parse_shl() {
        let mut x = parse_input("jx LSHIFT 3 -> ka");
        match x.remove("ka").unwrap() {
            Operation::Shl("jx", 3) => {}
            op => panic!("Unexpected operation: {:?}", op),
        }
    }

    #[test]
    fn test_parse_shr() {
        let mut x = parse_input("jx RSHIFT 3 -> ka");
        match x.remove("ka").unwrap() {
            Operation::Shr("jx", 3) => {}
            op => panic!("Unexpected operation: {:?}", op),
        }
    }

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let wires = parse_input(&input);

        let answer = solve(&wires);
        assert_eq!(3176, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let mut wires = parse_input(&input);

        let wire_override = solve(&wires);
        wires.insert("b", Operation::Set(wire_override));

        let answer = solve(&wires);
        assert_eq!(14710, answer);
    }
}
