use std::collections::{HashMap, HashSet};

use crate::{Color, Coordinate};

const DAYS: usize = 100;

pub fn solve(tiles: &HashMap<Coordinate, Color>) -> usize {
    let mut current = HashSet::new();
    let mut next = HashMap::new();

    for (&tile, &color) in tiles.iter() {
        if color == Color::Black {
            current.insert(tile);
        }
    }

    for _ in 0..DAYS {
        current.iter().copied().for_each(|t| {
            next.insert(t, State::black());
        });

        for tile in current.drain() {
            // The current state contains only black tiles

            for tile in tile.iter() {
                next.entry(tile).or_insert(State::white()).stats.black += 1;
            }
        }

        for (tile, mut state) in next.drain() {
            match state.color {
                Color::Black => {
                    if state.stats.black == 0 || state.stats.black > 2 {
                        state.color = Color::White;
                    }
                }

                Color::White => {
                    if state.stats.black == 2 {
                        state.color = Color::Black;
                    }
                }
            }

            if state.color == Color::Black {
                current.insert(tile);
            }
        }
    }

    current.len()
}

#[derive(Debug, Copy, Clone)]
struct State {
    stats: Stats,
    color: Color,
}

impl State {
    fn white() -> State {
        State {
            color: Color::White,
            stats: Stats::default(),
        }
    }

    fn black() -> State {
        State {
            color: Color::Black,
            stats: Stats::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Stats {
    black: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats { black: 0 }
    }
}
