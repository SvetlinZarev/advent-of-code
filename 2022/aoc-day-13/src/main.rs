use aoc_shared::input::load_line_delimited_input_from_autodetect;

use aoc_day_13::{part_one, part_two};

fn main() {
    let packets = load_line_delimited_input_from_autodetect();

    println!("Part 1: {}", part_one(&packets));
    println!("Part 2: {}", part_two(packets));
}
