use std::cmp::Ordering;

use crate::{evaluate_expressions, Function, Operand, Token};

pub fn solve(input: &[Vec<Token>]) -> Operand {
    evaluate_expressions(input, precedence_part_one)
}

#[inline(always)]
fn precedence_part_one(_first: Function, _second: Function) -> Ordering {
    Ordering::Equal
}
