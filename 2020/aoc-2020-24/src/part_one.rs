use std::collections::HashMap;

use crate::{Color, Coordinate, Direction};

pub fn solve(input: &[Vec<Direction>]) -> (usize, HashMap<Coordinate, Color>) {
    let mut tiles = HashMap::new();

    for tile_directions in input {
        let mut tile = Coordinate::new(0, 0);
        for direction in tile_directions.iter().copied() {
            tile = tile.on_direction(direction);
        }

        let v = tiles.entry(tile).or_insert(Color::White);
        *v = v.flip();
    }

    let count = tiles
        .values()
        .copied()
        .filter(|&v| v == Color::Black)
        .count();

    (count, tiles)
}
