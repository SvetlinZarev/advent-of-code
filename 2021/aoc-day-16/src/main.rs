use aoc_shared::input::load_text_input_from_autodetect;
use aoc_day_16::{decode_packets, parse_to_binary, part_one, part_two};

fn main() {
    let (binary_input, bits) = parse_to_binary(load_text_input_from_autodetect());
    let decoded_input = decode_packets(&binary_input, bits);

    println!("Part 1: {}", part_one(&decoded_input));
    println!("Part 1: {}", part_two(&decoded_input));
}
