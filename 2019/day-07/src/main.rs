use aoc_shared_2019::input::load_csv_input_from_autodetect;
use day_07::{part_one, part_two};

fn main() {
    let program = load_csv_input_from_autodetect();
    println!("Part 1: {}", part_one(&program));
    println!("Part 2: {}", part_two(&program));
}
