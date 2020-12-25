use std::cmp::Ordering;

use crate::{evaluate_expressions, Function, Operand, Token};

pub fn solve(input: &[Vec<Token>]) -> Operand {
    evaluate_expressions(input, precedence_part_two)
}

#[inline(always)]
fn precedence_part_two(first: Function, second: Function) -> Ordering {
    match first {
        Function::Add => match second {
            Function::Add => Ordering::Equal,
            Function::Mul => Ordering::Greater,
        },
        Function::Mul => match second {
            Function::Add => Ordering::Less,
            Function::Mul => Ordering::Equal,
        },
    }
}
