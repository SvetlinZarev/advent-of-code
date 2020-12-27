use std::cmp::Ordering;
use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub type Operand = isize;
type PrecedenceFunction = fn(Function, Function) -> Ordering;

pub const DAY: usize = 18;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (dp, expressions) = measure(DAY, "parsing", || parse_input(&input));
    let (d1, _) = measure(DAY, "part 1", || part_one::solve(&expressions));
    let (d2, _) = measure(DAY, "part 2", || part_two::solve(&expressions));

    dp.add(d1).add(d2)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bracket {
    Opening,
    Closing,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Function {
    Add,
    Mul,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operator {
    Bracket(Bracket),
    Function(Function),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    Operand(Operand),
    Operator(Operator),
}

pub fn parse_input(input: &str) -> Vec<Vec<Token>> {
    let mut expressions = vec![];
    for (ln, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut expression = vec![];

        let mut end_of_last_match = 0;
        for (idx, matched) in line.match_indices(match_split_symbol) {
            if end_of_last_match != idx {
                let token = line[end_of_last_match..idx].trim();
                expression.push(parse_token(token, ln));
            }
            end_of_last_match = idx + matched.len();

            let token = matched.trim();
            if !token.is_empty() {
                expression.push(parse_token(token, ln));
            }
        }

        let remaining = line[end_of_last_match..].trim();
        if !remaining.is_empty() {
            expression.push(parse_token(remaining, ln));
        }

        expressions.push(expression);
    }

    expressions
}

fn match_split_symbol(c: char) -> bool {
    c == ' ' || c == '(' || c == ')' || c == '+' || c == '*'
}

fn parse_token(t: &str, ln: usize) -> Token {
    match t {
        "+" => Token::Operator(Operator::Function(Function::Add)),
        "*" => Token::Operator(Operator::Function(Function::Mul)),
        "(" => Token::Operator(Operator::Bracket(Bracket::Opening)),
        ")" => Token::Operator(Operator::Bracket(Bracket::Closing)),
        token => match token.parse() {
            Ok(value) => Token::Operand(value),
            Err(_e) => panic!("Unexpected token on line {}: \"{}\"", ln, token),
        },
    }
}

fn evaluate_expressions(expressions: &[Vec<Token>], precedence: PrecedenceFunction) -> Operand {
    expressions
        .iter()
        .map(|e| evaluate_expression(e, precedence))
        .sum()
}

pub fn evaluate_expression(expression: &Vec<Token>, precedence: PrecedenceFunction) -> Operand {
    let mut operands = vec![];
    let mut operators = vec![];

    for token in expression.iter().copied() {
        handle_token(token, &mut operands, &mut operators, precedence);
    }

    handle_remaining(&mut operands, &mut operators);

    assert_eq!(1, operands.len());
    operands.pop().unwrap()
}

#[inline]
fn handle_token(
    token: Token,
    operands: &mut Vec<Operand>,
    operators: &mut Vec<Operator>,
    precedence: PrecedenceFunction,
) {
    match token {
        Token::Operand(operand) => handle_number(operands, operand),
        Token::Operator(operator) => handle_operator(operands, operators, operator, precedence),
    }
}

#[inline]
fn handle_number(operands: &mut Vec<Operand>, operand: Operand) {
    operands.push(operand);
}

#[inline]
fn handle_operator(
    operands: &mut Vec<Operand>,
    operators: &mut Vec<Operator>,
    operator: Operator,
    precedence: PrecedenceFunction,
) {
    match operator {
        Operator::Bracket(bracket) => handle_bracket(operands, operators, bracket),
        Operator::Function(function) => handle_function(operands, operators, function, precedence),
    }
}

#[inline]
fn handle_bracket(operands: &mut Vec<Operand>, operators: &mut Vec<Operator>, bracket: Bracket) {
    match bracket {
        Bracket::Opening => operators.push(Operator::Bracket(bracket)),
        Bracket::Closing => {
            let mut has_left_bracket = false;
            while let Some(op) = operators.pop() {
                match op {
                    Operator::Bracket(Bracket::Opening) => {
                        has_left_bracket = true;
                        break;
                    }
                    Operator::Bracket(Bracket::Closing) => {
                        panic!("Closing brackets should never be present in the operator stack!")
                    }
                    Operator::Function(function) => {
                        apply_function(operands, function);
                    }
                }
            }
            assert!(has_left_bracket);
        }
    }
}

fn handle_function(
    operands: &mut Vec<Operand>,
    operators: &mut Vec<Operator>,
    function: Function,
    precedence: PrecedenceFunction,
) {
    if !operators.is_empty() {
        while let Some(operator) = operators.pop() {
            match operator {
                Operator::Bracket(Bracket::Opening) => {
                    // preserve the opening bracket
                    operators.push(operator);
                    break;
                }
                Operator::Bracket(Bracket::Closing) => {
                    panic!("Closing brackets should never be present in the operator stack!")
                }
                Operator::Function(last_fn) => {
                    if precedence(last_fn, function) == Ordering::Less {
                        //preserve the last function
                        operators.push(operator);
                        break;
                    }

                    apply_function(operands, last_fn);
                }
            }
        }
    }
    operators.push(Operator::Function(function))
}

#[inline]
fn handle_remaining(operands: &mut Vec<Operand>, operators: &mut Vec<Operator>) {
    while let Some(op) = operators.pop() {
        match op {
            Operator::Function(function) => apply_function(operands, function),
            _ => panic!("Unexpected operator: {:?}", op),
        }
    }

    assert_eq!(1, operands.len());
}

fn apply_function(operands: &mut Vec<Operand>, function: Function) {
    assert!(operands.len() >= 2);

    let a = operands.pop().unwrap();
    let b = operands.pop().unwrap();

    let r = match function {
        Function::Add => a + b,
        Function::Mul => a * b,
    };
    operands.push(r);
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let expressions = parse_input(&input);

        let solution = part_one::solve(&expressions);
        assert_eq!(202553439706, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let expressions = parse_input(&input);

        let solution = part_two::solve(&expressions);
        assert_eq!(88534268715686, solution);
    }
}
