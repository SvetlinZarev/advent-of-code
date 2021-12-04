use aoc_day_04::{parse_input, part_one, part_two};
use aoc_shared::input::load_input_autodetect;
use std::io::BufReader;

fn main() {
    let (numbers, boards) = parse_input(BufReader::new(load_input_autodetect().as_bytes()));

    println!("Part 1: {:?}", part_one(&numbers, &boards));
    println!("Part 2: {:?}", part_two(&numbers, &boards));
}
