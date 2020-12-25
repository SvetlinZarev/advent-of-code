use crate::{COLUMNS, MARK_TREE, WIDTH};

pub fn solve(input: &[u8]) -> usize {
    let mut trees = 0;
    let mut column = 0usize;
    let mut row = 0usize;

    let number_of_rows = input.len() / WIDTH;
    while row < number_of_rows {
        if input[column + row * (WIDTH)] == MARK_TREE {
            trees += 1;
        }

        column = (column + 3) % COLUMNS;
        row += 1;
    }
    trees
}
