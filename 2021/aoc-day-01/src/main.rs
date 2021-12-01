use aoc_shared::input::stdin_line_delimited;

fn main() {
    let input: Vec<u32> = stdin_line_delimited();

    let part_one = input.windows(2)
        .filter(|w| w[1] > w[0])
        .count();

    let part_two = input.windows(4)
        .filter(|w| w[3] > w[0])
        .count();

    println!("Part 1: {}\nPart 2: {}", part_one, part_two);
}
