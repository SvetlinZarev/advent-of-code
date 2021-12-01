use std::fmt::Debug;
use std::io::{Read, stdin};
use std::str::FromStr;

fn main() {
    let mut input = String::with_capacity(4096);
    stdin().read_to_string(&mut input).unwrap();
    if input.is_empty() {
        println!("Please provide input via STDIN");
        return;
    }

    let input = parse_line_delimited::<_, u32, _>(&input);

    // part 01
    let count = input.windows(2)
        .filter(|w| w[1] > w[0])
        .count();
    println!("Part 1: {}", count);


    // part 02
    let count = input.windows(4)
        .filter(|w| w[3] > w[0])
        .count();
    println!("Part 2: {}", count);
}


pub fn parse_line_delimited<I, R, E>(input: I) -> Vec<R>
    where
        I: AsRef<str>,
        E: Debug,
        R: FromStr<Err=E>,
{
    input
        .as_ref()
        .lines()
        .map(|l| l.trim())
        .filter(|&l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}
