use aoc_shared_2019::input::load_csv_input_from_autodetect;
use day_05::{part_one, part_two};

fn main() {
    let input = load_csv_input_from_autodetect();

    println!("Part 1: {:?}", part_one(input.clone()));
    println!("Part 2: {:?}", part_two(input));
}
