use crate::core::{Command, Cuboid, Operation};

pub fn part_two(commands: &[Command]) -> u64 {
    let mut active: Vec<Cuboid> = Vec::with_capacity(commands.len() * 2);
    let mut processed = Vec::with_capacity(commands.len() * 2);

    for cmd in commands.iter() {
        let c = cmd.cuboid.clone();

        for kube in active.drain(..) {
            // This, on first sight,  seemingly unnecessary intersection,
            // allows us to avoid a lot allocations, thus resulting in
            // 77% decrease in execution time for part 2
            if let Some(intersection) = kube.intersect(&c) {
                let intersections = kube.subtract(&intersection);
                processed.extend(intersections);
            } else {
                processed.push(kube);
            }
        }

        if Operation::On == cmd.op {
            processed.push(c);
        }
        std::mem::swap(&mut active, &mut processed);
    }

    active.iter().map(|c| c.volume()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(1217140271559773, answer);
    }
}
