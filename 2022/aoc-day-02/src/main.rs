use aoc_day_02::v1::{part_one, part_two};
use aoc_shared::input::load_line_delimited_input_from_autodetect;

fn main() {
    let input = load_line_delimited_input_from_autodetect();
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}
