use aoc_day_04::{parse_input, part_one, part_two};
use std::io::stdin;

fn main() {
    let std_in = stdin();
    let (numbers, boards) = parse_input(std_in.lock());

    println!("Part 1: {:?}", part_one(&numbers, &boards));
    println!("Part 2: {:?}", part_two(&numbers, &boards));
}
