use crate::Board;
use aoc_shared::parsing::parse_csv;
use std::io::BufRead;

pub fn parse_input<R: BufRead>(mut reader: R) -> (Vec<u32>, Vec<Board>) {
    let mut line = String::with_capacity(256);

    reader.read_line(&mut line).unwrap();
    let numbers = parse_csv(&line);
    let mut boards = vec![];

    'read_boards: loop {
        // skip all empty lines before a board's data
        loop {
            line.clear();
            if 0 == reader.read_line(&mut line).unwrap() {
                // we have reached EOF
                break 'read_boards;
            }

            if !line.trim_end().is_empty() {
                // we have reached a board
                break;
            }
        }

        let mut board = Board::new();
        for r in 0..5 {
            line.split(' ')
                .map(|x| x.trim())
                .filter(|&x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .enumerate()
                .for_each(|(c, val)| board.modify(r, c, val));

            line.clear();
            reader.read_line(&mut line).unwrap();
        }

        boards.push(board);
    }

    (numbers, boards)
}
