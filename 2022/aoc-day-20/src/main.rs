use aoc_shared::input::load_line_delimited_input_from_autodetect;

use aoc_day_20::v2;

fn main() {
    let input = load_line_delimited_input_from_autodetect();
    println!("Part 1: {}", v2::part_one(&input));
    println!("Part 2: {}", v2::part_two(&input));
}
