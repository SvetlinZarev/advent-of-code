use std::error::Error;

// The array based solution works only because there are 5 different letters
// in the input and will not work for any general input.
// The hash based solution can work with everything
mod array_node_based;
mod flat_array_based;
mod hash_node_based;

#[cfg(test)]
pub mod tests {
    pub const PART_1_ANSWER: usize = 255;
    pub const PART_2_ANSWER: u64 = 621_820_080_273_474;
}

pub fn parse_input(input: &str) -> Result<(Vec<&str>, Vec<&str>), Box<dyn Error>> {
    let mut input = input.lines();

    let Some(pattern_line) = input.next() else {
        return Err("Invalid input".into());
    };

    let patterns = pattern_line
        .split(',')
        .map(|x| x.trim())
        .collect::<Vec<_>>();

    // skip new-line separator
    input.next();

    Ok((patterns, input.collect()))
}

pub fn part_one_v1(patterns: &[&str], towels: &[&str]) -> usize {
    hash_node_based::part_one(patterns, towels)
}

pub fn part_two_v1(patterns: &[&str], towels: &[&str]) -> u64 {
    hash_node_based::part_two(patterns, towels)
}

pub fn part_one_v2(patterns: &[&str], towels: &[&str]) -> usize {
    array_node_based::part_one(patterns, towels)
}

pub fn part_two_v2(patterns: &[&str], towels: &[&str]) -> u64 {
    array_node_based::part_two(patterns, towels)
}

pub fn part_one_v3(patterns: &[&str], towels: &[&str]) -> usize {
    flat_array_based::part_one(patterns, towels)
}

pub fn part_two_v3(patterns: &[&str], towels: &[&str]) -> u64 {
    flat_array_based::part_two(patterns, towels)
}
