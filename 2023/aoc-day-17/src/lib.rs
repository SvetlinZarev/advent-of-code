mod common;
pub mod v1;
pub mod v2;
pub mod v3;

pub fn part_one(input: &[u8]) -> u16 {
    v2::part_one(input)
}

pub fn part_two(input: &[u8]) -> u16 {
    v2::part_two(input)
}
