pub mod part_one {
    pub fn solve(input: &[u64]) -> u64 {
        input.iter().map(|&m| (m / 3).saturating_sub(2)).sum()
    }
}

pub mod part_two {
    pub fn solve(input: &[u64]) -> u64 {
        let mut fuel = 0;
        for mass in input.iter().copied() {
            let mut additional_mass = mass;
            while additional_mass > 0 {
                additional_mass = (additional_mass / 3).saturating_sub(2);
                fuel += additional_mass;
            }
        }
        fuel
    }
}


#[cfg(test)]
mod tests {
    use aoc_shared_2019::input::load_line_delimited_input_from_file;
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        assert_eq!(3427947, part_one::solve(&input));
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        assert_eq!(5139037, part_two::solve(&input));
    }
}