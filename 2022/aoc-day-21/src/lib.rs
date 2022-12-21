use aoc_shared::hashing::HashMap;
use std::cmp::Ordering;

const NODE_HUMAN: &'static str = "humn";
const NODE_ROOT: &'static str = "root";

const INITIAL_LEFT: i64 = 0i64;

// At least for my input I need at least 42 bits to get the answer
// and the solver crashes with "int overflow" with 58 bits, so
// the maximum right side should be between 2^42 and 2^57
const INITIAL_RIGHT: i64 = 2i64.pow(56);

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Sub,
    Add,
    Mul,
    Div,
}

impl Op {
    pub fn apply(self, x: i64, y: i64) -> i64 {
        match self {
            Op::Sub => x - y,
            Op::Add => x + y,
            Op::Mul => x * y,
            Op::Div => x / y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Node<'l> {
    Value(i64),
    Expr(&'l str, &'l str, Op),
}

pub fn parse_input(input: &str) -> HashMap<&str, Node> {
    let mut parsed = HashMap::default();

    for line in input.lines() {
        let (key, mut rest) = line.split_once(':').unwrap();
        rest = &rest[1..];

        let node = match rest.split_once(' ') {
            None => Node::Value(
                rest.parse()
                    .map_err(|_| format!("cannot parse: {}", rest))
                    .unwrap(),
            ),

            Some((a, rest)) => {
                let b = &rest[2..];
                let op = match &rest[0..1] {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => unreachable!(),
                };
                Node::Expr(a, b, op)
            }
        };

        parsed.insert(key, node);
    }

    parsed
}

pub fn part_one(input: &HashMap<&str, Node>) -> i64 {
    solve(input, NODE_ROOT)
}

pub fn part_two(mut input: HashMap<&str, Node>) -> i64 {
    let Some(Node::Expr(l, r, _)) = input.get(NODE_ROOT).copied() else{
        panic!("The ROOT node is invalid or missing!");
    };

    // Note: Another idea is to "invert" the tree in order to make `humn` the root node,
    // then we can just run the part-1 solution on the inverted tree

    // The equation seems to be "monotonic", thus we can binary search for the answer.
    // Because we don't know if it's increasing/decreasing, we try one of the
    // directions, and if it does nto produce an answer, then we try the other
    binary_search(&mut input, l, r, |ord| ord.reverse())
        .or_else(|| binary_search(&mut input, l, r, |ord| ord))
        .expect("no solution found")
}

fn binary_search<O: Fn(Ordering) -> Ordering>(
    input: &mut HashMap<&str, Node>,
    l: &str,
    r: &str,
    ord: O,
) -> Option<i64> {
    // Due to the integer division, there might be several numbers  that
    // seemingly give the correct answer. So we must find that range [begin; end)
    // and take the only number that gives the correct answer. Example:
    // 3/3=>1; 4/3=>1; 5/3=>1
    // But only `3/3` truly gives 1, thus we are interested only
    // of the left side of the range (i.e. `begin`)

    let mut lo = INITIAL_LEFT;
    let mut hi = INITIAL_RIGHT;
    let mut answer = None;

    // dynamically decide whether to compute both the L & R sides
    let (mut lval, mut lcnt) = (0, 0);
    let (mut rval, mut rcnt) = (0, 0);

    // find the left side of the range
    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        input.insert(NODE_HUMAN, Node::Value(mid));
        if lcnt < 2 || rcnt >= lcnt {
            let value = solve(&input, l);
            lcnt = if lval == value { lcnt + 1 } else { 0 };
            lval = value;
        }

        if rcnt < 2 || lcnt >= rcnt {
            let value = solve(&input, r);
            rcnt = if rval == value { rcnt + 1 } else { 0 };
            rval = value;
        }

        // This might not be correct for all problem inputs
        // If it does not work for yours => remove the `.reverse()`
        match ord(lval.cmp(&rval)) {
            Ordering::Equal => {
                hi = mid;
                answer = Some(hi);
            }

            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid - 1,
        }
    }

    answer
}

fn solve<'l>(input: &'l HashMap<&'l str, Node<'l>>, key: &str) -> i64 {
    match input.get(key).copied().unwrap() {
        Node::Value(x) => x,
        Node::Expr(a, b, op) => {
            let x = solve(input, a);
            let y = solve(input, b);
            op.apply(x, y)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_one, part_two};
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);
        let answer = part_one(&parsed);

        assert_eq!(159591692827554, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);
        let answer = part_two(parsed);

        assert_eq!(3509819803065, answer);
    }
}
