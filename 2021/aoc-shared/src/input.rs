use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;
use crate::parsing::parse_line_delimited;


pub fn load_input_autodetect() -> String {
    match std::env::args().skip(1).next() {
        None => load_input_from_stdin(),
        Some(path) => load_input_from_file(path)
    }
}

pub fn stdin_line_delimited<O: FromStr<Err=impl Debug>>() -> Vec<O> {
    let input = load_input_from_stdin();
    parse_line_delimited(input)
}

pub fn load_input_from_stdin() -> String {
    load_text_input(std::io::stdin().lock())
}

pub fn load_text_input<R: Read>(mut input: R) -> String {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    buffer
}
