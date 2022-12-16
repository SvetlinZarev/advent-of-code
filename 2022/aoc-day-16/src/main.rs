use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_16::{p1v1, p2v1, parse_input};

fn main() {
    let input = load_text_input_from_autodetect();
    let graph = parse_input(input);

    let greedy = match std::env::var("GREEDY") {
        Ok(val) => val != "0",
        Err(_) => false,
    };
    println!("GREEDY: {}", greedy);

    if greedy {
        println!("Part 1: {}", p1v1::part_one::<true>(&graph));
        println!("Part 2: {}", p2v1::part_two::<true>(&graph));
    } else {
        println!("Part 1: {}", p1v1::part_one::<false>(&graph));
        println!("Part 2: {}", p2v1::part_two::<false>(&graph));
    }
}
