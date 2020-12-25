use core::ops::RangeInclusive;
use std::path::Path;

use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use aoc_2020_common::parsing::parse_csv_as_u32;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-16.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let mut input = parse_input(&input);

    measure_solution(16, 1, "", || part_one::solve(&mut input));
    measure_solution(16, 2, "v1", || part_two::solve_v1(&input));
    measure_solution(16, 2, "v2", || part_two::solve_v2(&input));
}

#[derive(Debug, Copy, Clone)]
enum ParserStage {
    Fields,
    MyTicketHeader,
    MyTicket,
    NearbyTicketsHeader,
    NearbyTickets,
}

#[derive(Debug, Clone)]
pub struct FieldRange {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

#[derive(Debug, Clone)]
pub struct Input<'i> {
    fields: Vec<(&'i str, FieldRange)>,
    ticket: Vec<u32>,
    nearby: Vec<Vec<u32>>,
}

pub fn parse_input(input: &str) -> Input {
    let mut stage = ParserStage::Fields;
    let mut fields = Vec::<(&str, FieldRange)>::new();
    let mut my_ticket = None;
    let mut nearby_tickets = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            match stage {
                ParserStage::Fields => stage = ParserStage::MyTicketHeader,
                ParserStage::MyTicket => stage = ParserStage::NearbyTicketsHeader,
                stage => panic!("Unexpected parser stage: {:?}", stage),
            }
            continue;
        }

        match stage {
            ParserStage::Fields => {
                let (id, range) = parse_line_field(line);
                fields.push((id, range));
            }
            ParserStage::MyTicketHeader => {
                assert_eq!("your ticket:", line);
                stage = ParserStage::MyTicket;
            }
            ParserStage::MyTicket => {
                if my_ticket.is_some() {
                    panic!("My ticket was already parsed: {:?}", my_ticket);
                }
                my_ticket = Some(parse_csv_as_u32(line));
            }
            ParserStage::NearbyTicketsHeader => {
                assert_eq!("nearby tickets:", line);
                stage = ParserStage::NearbyTickets;
            }

            ParserStage::NearbyTickets => {
                let ticket = parse_ticket(line);
                nearby_tickets.push(ticket);
            }
        }
    }

    let my_ticket = my_ticket.unwrap();
    assert_eq!(my_ticket.len(), fields.len());
    Input {
        fields,
        ticket: my_ticket,
        nearby: nearby_tickets,
    }
}

fn parse_line_field(line: &str) -> (&str, FieldRange) {
    let idx_end_id = line.find(':').unwrap();
    let range_id = &line[..idx_end_id];

    let remaining = &line[idx_end_id + 2..];
    let idx_first_range_end = remaining.find(' ').unwrap();

    let first_range = parse_range(&remaining[..idx_first_range_end]);
    let second_range = parse_range(&remaining[idx_first_range_end + 4..]);

    let field_range = FieldRange {
        first: first_range,
        second: second_range,
    };

    (range_id, field_range)
}

fn parse_range(r: &str) -> RangeInclusive<u32> {
    let idx_range_sep = r.find('-').unwrap();

    let first_value = r[0..idx_range_sep].parse().unwrap();
    let second_value = r[idx_range_sep + 1..].parse().unwrap();

    RangeInclusive::new(first_value, second_value)
}

fn parse_ticket(line: &str) -> Vec<u32> {
    parse_csv_as_u32(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let mut input = parse_input(&input);

        let solution = part_one::solve(&mut input);
        assert_eq!(21071, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let mut input = parse_input(&input);
        part_one::solve(&mut input); // removes the invalid entries

        let solution = part_two::solve_v1(&input);
        assert_eq!(3429967441937, solution);

        let solution = part_two::solve_v2(&input);
        assert_eq!(3429967441937, solution);
    }
}
