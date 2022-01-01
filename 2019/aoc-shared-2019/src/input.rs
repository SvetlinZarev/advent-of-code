use crate::parsing::{parse_csv, parse_line_delimited};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::str::FromStr;

pub fn load_text_input_from_autodetect() -> String {
    load_text_input(auto_select_input())
}

pub fn load_line_delimited_input_from_autodetect<O: FromStr<Err = impl Debug>>() -> Vec<O> {
    parse_line_delimited(load_text_input_from_autodetect())
}

pub fn load_line_delimited_input_from_file<O: FromStr<Err = impl Debug>, P>(path: P) -> Vec<O>
where
    P: AsRef<Path>,
{
    parse_line_delimited(load_text_input_from_file(path))
}

pub fn load_csv_input_from_autodetect<O: FromStr<Err = impl Debug>>() -> Vec<O> {
    parse_csv(load_text_input_from_autodetect())
}

pub fn load_csv_input_from_file<O: FromStr<Err = impl Debug>, P>(path: P) -> Vec<O>
where
    P: AsRef<Path>,
{
    parse_csv(load_text_input_from_file(path))
}

pub fn load_text_input_from_stdin() -> String {
    load_text_input(std::io::stdin().lock())
}

pub fn load_text_input_from_file<P: AsRef<Path>>(path: P) -> String {
    load_text_input(File::open(path).unwrap())
}

pub fn auto_select_input() -> Box<dyn BufRead> {
    match std::env::args().skip(1).next() {
        None => Box::new(BufReader::new(std::io::stdin())),
        Some(path) => Box::new(BufReader::new(
            File::open(&path).expect(&format!("file path: {}", &path)),
        )),
    }
}

pub fn load_text_input<R: Read>(mut input: R) -> String {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    buffer
}
