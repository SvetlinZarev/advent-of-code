use crate::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ParserState {
    OnNextScanner,
    ReadScannerData,
}

impl ParserState {
    pub fn next(self) -> Self {
        match self {
            ParserState::OnNextScanner => ParserState::ReadScannerData,
            ParserState::ReadScannerData => ParserState::OnNextScanner,
        }
    }
}

pub fn parse_input<S: AsRef<str>>(input: S) -> Vec<Vec<Point>> {
    let input = input.as_ref();
    let mut scanners = vec![];
    let mut scanner = vec![];

    let mut state = ParserState::OnNextScanner;
    for line in input.lines().map(|l| l.trim()) {
        if line.is_empty() {
            state = state.next();
            continue;
        }

        match state {
            ParserState::OnNextScanner => {
                if !scanner.is_empty() {
                    scanners.push(scanner);
                    scanner = vec![];
                }

                state = state.next();
            }

            ParserState::ReadScannerData => {
                let (x, yz) = line.split_once(',').unwrap();
                let (y, z) = yz.split_once(',').unwrap();

                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                let z = z.parse().unwrap();

                scanner.push((x, y, z));
            }
        }
    }

    if !scanner.is_empty() {
        scanners.push(scanner);
    }

    scanners
}
