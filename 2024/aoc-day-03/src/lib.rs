pub mod with_fsm;
pub mod with_regex;

pub fn part_one(input: &str) -> u64 {
    with_fsm::part_one_v2(input)
}

pub fn part_two(input: &str) -> u64 {
    with_fsm::part_two_v2(input)
}
