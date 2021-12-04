use aoc_day_04::{parse_input, part_one};
use std::io::stdin;

fn main() {
    let std_in = stdin();
    let (numbers, boards) = parse_input(std_in.lock());

    println!("Part 1: {:?}", part_one(&numbers, &boards));
}
