pub mod v1;

pub fn part_one(input: &str) -> u64 {
    let graph = v1::load_graph(input);
    v1::part_one(&graph)
}

pub fn part_two(input: &str) -> u64 {
    let graph = v1::load_graph(input);
    v1::part_two(&graph)
}
