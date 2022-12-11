type Int = u64;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Sum(Int),
    Mul(Int),
    #[default]
    Square,
}

impl Operation {
    fn execute(self, value: Int) -> Int {
        match self {
            Operation::Sum(x) => value + x,
            Operation::Mul(x) => value * x,
            Operation::Square => value * value,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Monkey {
    items: Vec<Int>,
    operation: Operation,
    division: Int,
    on_pass: u32,
    on_fail: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Parser {
    Header,
    StartingItems,
    Operation,
    Test,
    TestTrue,
    TestFalse,
    End,
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let mut last = monkeys.len();

    let mut state = Parser::Header;

    for line in input.lines() {
        match state {
            Parser::Header => {
                last = monkeys.len();
                monkeys.push(Monkey::default());
                state = Parser::StartingItems;
            }

            Parser::StartingItems => {
                let items = &line[18..];
                monkeys[last].items.extend(
                    items
                        .split(',')
                        .map(|x| x.trim())
                        .map(|x| x.parse::<Int>().unwrap()),
                );
                state = Parser::Operation;
            }

            Parser::Operation => {
                let (_, expr) = line.split_once('=').unwrap();
                let (_, right) = expr.trim_start().split_once(' ').unwrap();
                let (op, right) = right.split_once(' ').unwrap();

                let operation = match (op, right) {
                    ("*", "old") => Operation::Square,
                    ("*", n) => Operation::Mul(n.parse().unwrap()),
                    ("+", n) => Operation::Sum(n.parse().unwrap()),
                    _ => panic!("Cannot parse operation: {}", line),
                };

                monkeys[last].operation = operation;
                state = Parser::Test;
            }

            Parser::Test => {
                let (_, value) = line.rsplit_once(' ').unwrap();
                monkeys[last].division = value.parse().unwrap();
                state = Parser::TestTrue;
            }

            Parser::TestTrue => {
                let (_, value) = line.rsplit_once(' ').unwrap();
                monkeys[last].on_pass = value.parse().unwrap();
                state = Parser::TestFalse;
            }

            Parser::TestFalse => {
                let (_, value) = line.rsplit_once(' ').unwrap();
                monkeys[last].on_fail = value.parse().unwrap();
                state = Parser::End;
            }

            Parser::End => {
                state = Parser::Header;
            }
        }
    }

    monkeys
}

pub fn part_one(monkeys: &[Monkey]) -> u64 {
    solve::<20, 3>(monkeys.to_vec())
}

pub fn part_two(monkeys: &[Monkey]) -> u64 {
    solve::<10_000, 1>(monkeys.to_vec())
}

fn solve<const ROUNDS: usize, const RELIEF: Int>(monkeys: Vec<Monkey>) -> u64 {
    let mut monkeys = monkeys.to_vec();
    let mut activity = vec![0; monkeys.len()];

    // Key observations is that all divisors are PRIME numbers. If they were not,
    // then we should have used Least Common Multiple instead
    let modulo = monkeys.iter().map(|m| m.division).product::<Int>();

    // Temporary holder to keep the current monkey in order to
    // avoid multiple mutable borrows over the `monkeys` array.
    //
    // This works under the assumption that the monkey always
    // passes the items to another monkey and never to itself
    let mut monkey = Monkey::default();

    for _ in 0..ROUNDS {
        for idx in 0..monkeys.len() {
            // Remove the current monkey from the vector in order to avoid
            // double mutable borrow over the `monkeys` array. We replace it
            // with a "dummy" monkey
            std::mem::swap(&mut monkey, &mut monkeys[idx]);

            // update the current monkey activity
            activity[idx] += monkey.items.len() as u64;

            for item in monkey.items.drain(..) {
                let mut worry_level = monkey.operation.execute(item);
                worry_level %= modulo;
                worry_level /= RELIEF;

                // Select the next monkey
                let next_monkey = if worry_level % monkey.division == 0 {
                    monkey.on_pass as usize
                } else {
                    monkey.on_fail as usize
                };

                // Pass the item to the next monkey
                monkeys[next_monkey].items.push(worry_level);
            }

            // Return the monkey back to the array
            std::mem::swap(&mut monkey, &mut monkeys[idx]);
        }
    }

    let (a, b) = top_two(&activity);
    a * b
}

fn top_two(array: &[u64]) -> (u64, u64) {
    let mut a = 0;
    let mut b = 0;

    for &x in array {
        if x > a {
            b = a;
            a = x;
        } else if x > b {
            b = x;
        }
    }

    (a, b)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{parse_input, part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let monkeys = parse_input(&input);
        let answer = part_one(&monkeys);

        assert_eq!(56350, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let monkeys = parse_input(&input);
        let answer = part_two(&monkeys);

        assert_eq!(13954061248, answer);
    }
}
