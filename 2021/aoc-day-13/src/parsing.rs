use crate::{Fold, Point};
use std::str::FromStr;

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

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| format!("invalid point: {}", s))?;

        let x = x
            .parse()
            .map_err(|e| format!("cannot parse ({:?}): {}", e, x))?;

        let y = y
            .parse()
            .map_err(|e| format!("cannot parse ({:?}): {}", e, y))?;

        Ok(Point::new(x, y))
    }
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr = s
            .strip_prefix("fold along ")
            .ok_or_else(|| format!("invalid folding instruction (missing prefix): {}", s))?;

        let (axis, val) = instr
            .split_once('=')
            .ok_or_else(|| format!("invalid folding instruction (cannot split axis=val): {}", s))?;

        let value = val
            .parse()
            .map_err(|e| format!("invalid folding instruction ({:?}): {}", e, s))?;

        match axis {
            "x" => Ok(Fold::X(value)),
            "y" => Ok(Fold::Y(value)),
            _ => Err(format!("invalid folding instruction: {}", s)),
        }
    }
}
