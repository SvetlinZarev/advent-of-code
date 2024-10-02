use aoc_shared::hashing::FnvHasher;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::hash::BuildHasherDefault;
use std::sync::LazyLock;

static REGEX_INITIAL_STATE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^Begin in state (?<state>[A-Z])\.$"#).unwrap());

static REGEX_NUM_STEPS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^Perform a diagnostic checksum after (?<steps>\d+) steps\.$"#).unwrap()
});

static REGEX_STATE_KEY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^In state (?<state>[A-Z]):$"#).unwrap());

static REGEX_WRITE_VALUE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^ *- Write the value (?<value>[01]).$"#).unwrap());

static REGEX_MOVE_DIRECTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^ *- Move one slot to the (?<direction>right|left).$"#).unwrap()
});

static REGEX_MOVE_TO_STATE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^ *- Continue with state (?<state>[A-Z]).$"#).unwrap());

#[derive(Debug)]
pub struct Input {
    pub number_of_steps: usize,
    pub initial_state: State,
    pub transitions: HashMap<State, (Action, Action)>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct State(char);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Value {
    Zero,
    One,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub write_value: Value,
    pub move_to: Direction,
    pub next_state: State,
}

enum ParserState {
    ParseInitialState,
    ParseSteps,
    ParseStateKey,
    ExpectOnZero,
    ExpectOnOne,
    ParseWriteValue,
    ParseMoveDirection,
    ParseMoveToState,
    InvalidInput,
}

pub fn parse_input(input: &str) -> Result<Input, Box<dyn Error>> {
    let mut parser_state = ParserState::ParseInitialState;

    let mut result = Input {
        number_of_steps: 0,
        initial_state: State(' '),
        transitions: HashMap::new(),
    };

    let mut state_key = State(' ');

    let mut is_on_zero = true;
    let mut on_zero = Action {
        write_value: Value::Zero,
        move_to: Direction::Left,
        next_state: State(' '),
    };
    let mut on_one = Action {
        write_value: Value::Zero,
        move_to: Direction::Left,
        next_state: State(' '),
    };

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        match parser_state {
            ParserState::ParseInitialState => {
                let Some(capture) = REGEX_INITIAL_STATE.captures(line) else {
                    return Err("missing initial state".into());
                };

                let Some(m) = &capture.name("state") else {
                    return Err("missing initial state".into());
                };

                let s = m.as_str().chars().next().unwrap();
                result.initial_state = State(s);

                parser_state = ParserState::ParseSteps;
            }

            ParserState::ParseSteps => {
                let Some(capture) = REGEX_NUM_STEPS.captures(line) else {
                    return Err("missing number of steps".into());
                };

                let Some(m) = &capture.name("steps") else {
                    return Err("missing number of steps".into());
                };

                result.number_of_steps = m.as_str().parse::<usize>()?;
                parser_state = ParserState::ParseStateKey;
            }

            ParserState::ParseStateKey => {
                let Some(capture) = REGEX_STATE_KEY.captures(line) else {
                    return Err("missing state key".into());
                };

                let Some(m) = &capture.name("state") else {
                    return Err("missing state key".into());
                };

                let s = m.as_str().chars().next().unwrap();
                state_key = State(s);

                parser_state = ParserState::ExpectOnZero;
            }

            ParserState::ExpectOnZero => {
                parser_state = if line == "  If the current value is 0:" {
                    is_on_zero = true;
                    ParserState::ParseWriteValue
                } else {
                    ParserState::InvalidInput
                };
            }

            ParserState::ExpectOnOne => {
                parser_state = if line == "  If the current value is 1:" {
                    is_on_zero = false;
                    ParserState::ParseWriteValue
                } else {
                    ParserState::InvalidInput
                };
            }

            ParserState::ParseWriteValue => {
                let Some(capture) = REGEX_WRITE_VALUE.captures(line) else {
                    return Err("missing calue to write".into());
                };

                let Some(m) = &capture.name("value") else {
                    return Err("missing calue to write".into());
                };

                let s = m.as_str().chars().next().unwrap();
                let value = match s {
                    '0' => Value::Zero,
                    '1' => Value::One,
                    _ => unreachable!(),
                };

                if is_on_zero {
                    on_zero.write_value = value;
                } else {
                    on_one.write_value = value;
                }

                parser_state = ParserState::ParseMoveDirection;
            }

            ParserState::ParseMoveDirection => {
                let Some(capture) = REGEX_MOVE_DIRECTION.captures(line) else {
                    return Err("missing move direction".into());
                };

                let Some(m) = &capture.name("direction") else {
                    return Err("missing move direction".into());
                };

                let dir = match m.as_str() {
                    "left" => Direction::Left,
                    "right" => Direction::Right,
                    _ => unreachable!(),
                };

                if is_on_zero {
                    on_zero.move_to = dir;
                } else {
                    on_one.move_to = dir;
                }

                parser_state = ParserState::ParseMoveToState;
            }

            ParserState::ParseMoveToState => {
                let Some(capture) = REGEX_MOVE_TO_STATE.captures(line) else {
                    return Err("missing target state".into());
                };

                let Some(m) = &capture.name("state") else {
                    return Err("missing target state".into());
                };

                let s = m.as_str().chars().next().unwrap();
                if is_on_zero {
                    on_zero.next_state = State(s);
                } else {
                    on_one.next_state = State(s);
                }

                if is_on_zero {
                    parser_state = ParserState::ExpectOnOne;
                } else {
                    parser_state = ParserState::ParseStateKey;
                    result
                        .transitions
                        .insert(state_key, (on_zero.clone(), on_one.clone()));
                }
            }

            ParserState::InvalidInput => {
                return Err(format!("invalid input: {}", line).into());
            }
        }
    }

    Ok(result)
}

pub fn part_one_v1(input: &Input) -> usize {
    let mut tape = HashMap::with_hasher(BuildHasherDefault::<FnvHasher>::default());
    let mut position = 0i32;

    let mut state = input.initial_state;
    for _ in 0..input.number_of_steps {
        let (on_zero, on_one) = &input.transitions[&state];
        let value = tape.entry(position).or_insert(Value::Zero);

        match *value {
            Value::Zero => {
                state = on_zero.next_state;
                *value = on_zero.write_value;
                position += match on_zero.move_to {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
            }

            Value::One => {
                state = on_one.next_state;
                *value = on_one.write_value;
                position += match on_one.move_to {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
            }
        }
    }

    tape.into_values().filter(|&v| v == Value::One).count()
}

pub fn part_one_v2(input: &Input) -> usize {
    let mut tape = VecDeque::new();
    let mut position = 0i32;
    let mut offset = 0i32;

    let mut state = input.initial_state;
    for _ in 0..input.number_of_steps {
        let (on_zero, on_one) = &input.transitions[&state];

        while position + offset < 0 {
            tape.push_front(Value::Zero);
            offset += 1;
        }

        while tape.len() <= (position + offset) as usize {
            tape.push_back(Value::Zero);
        }

        let value = tape.get_mut((position + offset) as usize).unwrap();

        match *value {
            Value::Zero => {
                state = on_zero.next_state;
                *value = on_zero.write_value;
                position += match on_zero.move_to {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
            }

            Value::One => {
                state = on_one.next_state;
                *value = on_one.write_value;
                position += match on_one.move_to {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
            }
        }
    }

    tape.into_iter().filter(|&v| v == Value::One).count()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_one_v1(&input);
        assert_eq!(4769, answer);
    }


    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_one_v2(&input);
        assert_eq!(4769, answer);
    }
}
