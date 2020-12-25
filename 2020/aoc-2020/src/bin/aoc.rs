use aoc_2020_common::input::default_input;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 && args.len() != 3 {
        println!("usage: aoc day-xx <puzzle_input>");
        return;
    }

    let demo = match args[1].as_str() {
        "day-01" => aoc_2020_01::demo,
        "day-02" => aoc_2020_02::demo,
        "day-03" => aoc_2020_03::demo,
        "day-04" => aoc_2020_04::demo,
        "day-05" => aoc_2020_05::demo,
        "day-06" => aoc_2020_06::demo,
        "day-07" => aoc_2020_07::demo,
        "day-08" => aoc_2020_08::demo,
        "day-09" => aoc_2020_09::demo,
        "day-10" => aoc_2020_10::demo,
        "day-11" => aoc_2020_11::demo,
        "day-12" => aoc_2020_12::demo,
        "day-13" => aoc_2020_13::demo,
        "day-14" => aoc_2020_14::demo,
        "day-15" => aoc_2020_15::demo,
        "day-16" => aoc_2020_16::demo,
        "day-17" => aoc_2020_17::demo,
        "day-18" => aoc_2020_18::demo,
        "day-19-1" => aoc_2020_19::demo_part_one,
        "day-19-2" => aoc_2020_19::demo_part_two,
        "day-20" => aoc_2020_20::demo,
        "day-21" => aoc_2020_21::demo,
        "day-22" => aoc_2020_22::demo,
        "day-23" => aoc_2020_23::demo,
        "day-24" => aoc_2020_24::demo,
        "day-25" => aoc_2020_25::demo,
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
