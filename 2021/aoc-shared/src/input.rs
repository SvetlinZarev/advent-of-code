use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use crate::parsing::parse_line_delimited;


pub fn load_text_input_from_autodetect() -> String {
    match std::env::args().skip(1).next() {
        None => load_text_input_from_stdin(),
        Some(path) => load_text_input_from_file(path)
    }
}

pub fn load_line_delimited_input_from_autodetect<O: FromStr<Err=impl Debug>>() -> Vec<O> {
    parse_line_delimited(load_text_input_from_autodetect())
}

pub fn load_line_delimited_input_from_stdin<O: FromStr<Err=impl Debug>>() -> Vec<O> {
    let input = load_text_input_from_stdin();
    parse_line_delimited(input)
}

pub fn load_line_delimited_input_from_file<O: FromStr<Err=impl Debug>, P: AsRef<Path>>(path: P) -> Vec<O> {
    let input = load_text_input_from_file(path);
    parse_line_delimited(input)
}

pub fn load_text_input_from_stdin() -> String {
    load_text_input(std::io::stdin().lock())
}

pub fn load_text_input_from_file<P: AsRef<Path>>(path: P) -> String {
    load_text_input(File::open(path).unwrap())
}

pub fn load_text_input<R: Read>(mut input: R) -> String {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    buffer
}
