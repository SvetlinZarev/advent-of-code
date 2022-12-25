use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_22::parse_input;
use aoc_day_22::part_one::part_one;

fn main() {
    let input = load_text_input_from_autodetect();
    let (map, instr) = parse_input(input);

    println!("Part 1: {}", part_one(&map, &instr));
    //println!("Part 2: {}", part_two(&input));
}
