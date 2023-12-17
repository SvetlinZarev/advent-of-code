mod common;
pub mod v1;
pub mod v2;

pub fn part_one(input: &[u8]) -> u16 {
    v1::part_one(input)
}

pub fn part_two(input: &[u8]) -> u16 {
    v1::part_two(input)
}
