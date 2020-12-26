use aoc_2015_common::input::default_input;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 && args.len() != 3 {
        println!("usage: aoc day-xx <puzzle_input>");
        return;
    }

    let demo = match args[1].as_str() {
        "day-01" => aoc_2015_01::demo,
        "day-02" => aoc_2015_02::demo,
        "day-03" => aoc_2015_03::demo,
        "day-04" => aoc_2015_04::demo,
        "day-05" => aoc_2015_05::demo,
        "day-06" => aoc_2015_06::demo,
        "day-07" => aoc_2015_07::demo,
        _ => {
            println!("Invalid selection: {}", args[1]);
            std::process::exit(1);
        }
    };

    let mut puzzle_input = default_input(args[1].as_str());
    if args.len() == 3 {
        puzzle_input = args[2].to_owned();
    }

    demo(puzzle_input);
}
