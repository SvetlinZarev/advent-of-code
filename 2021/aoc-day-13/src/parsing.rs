use crate::{Fold, Point};

enum ParserState {
    ParsePoints,
    ParseFoldingInstructions,
}

pub fn parse_input<S: AsRef<str>>(s: S) -> (Vec<Point>, Vec<Fold>) {
    let s = s.as_ref();

    let mut points = vec![];
    let mut fold_instr = vec![];

    let mut state = ParserState::ParsePoints;
    for line in s.lines() {
        match state {
            ParserState::ParsePoints => {
                if line.is_empty() {
                    state = ParserState::ParseFoldingInstructions;
                    continue;
                }

                let point = line.parse().unwrap();
                points.push(point);
            }

            ParserState::ParseFoldingInstructions => {
                if line.is_empty() {
                    break;
                }

                let fold = line.parse().unwrap();
                fold_instr.push(fold);
            }
        }
    }

    (points, fold_instr)
}
