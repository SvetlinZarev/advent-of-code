use aoc_day_20::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;

fn main() {
    let (alg, img, lim) = parse_input(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one(&alg, &img, lim));
    println!("Part 2: {}", part_two(&alg, &img, lim));
}
