use crate::{Int, Set};

#[derive(Debug, Copy, Clone)]
enum ParserState {
    ReadAlg,
    ReadImg(usize),
}

impl ParserState {
    pub fn next(self) -> Self {
        match self {
            ParserState::ReadAlg => ParserState::ReadImg(0),
            ParserState::ReadImg(r) => ParserState::ReadImg(r + 1),
        }
    }
}
pub fn parse_input<S: AsRef<str>>(input: S) -> (Vec<u8>, Set<(Int, Int)>, (usize, usize)) {
    let input = input.as_ref();

    let mut alg = vec![0u8; 512];
    let mut img = Set::default();
    let (mut r, mut c) = (0, 0);
    let mut state = ParserState::ReadAlg;

    for line in input.lines().map(|l| l.trim()) {
        if line.is_empty() {
            continue;
        }

        match state {
            ParserState::ReadAlg => {
                let chars = line.as_bytes();
                for (idx, &ch) in chars.iter().enumerate() {
                    alg[idx] = (ch == b'#') as u8;
                }

                state = state.next();
            }

            ParserState::ReadImg(row) => {
                for (idx, &ch) in line.as_bytes().iter().enumerate() {
                    if ch == b'#' {
                        c = c.max(idx + 1);
                        img.insert((row, idx));
                    }
                }

                r = row + 1;
                state = state.next();
            }
        }
    }

    (alg, img, (r, c))
}
