use aoc_shared_2019::intcode::{Evaluation, Input, Instruction};

pub fn part_one(mem: Vec<isize>) -> isize {
    let mut output = solve(mem, std::iter::once(1));

    let last = output.pop().unwrap();
    output.iter().copied().for_each(|v| assert_eq!(0, v));
    last
}

pub fn part_two(mem: Vec<isize>) -> isize {
    let mut output = solve(mem, std::iter::once(5));

    let last = output.pop().unwrap();
    output.iter().copied().for_each(|v| assert_eq!(0, v));
    last
}

fn solve(mut mem: Vec<isize>, mut input: impl Input) -> Vec<isize> {
    let mut state = Evaluation::Continue(0);
    let mut output = vec![];

    loop {
        match state {
            Evaluation::Continue(ip) => {
                let instruction = Instruction::decode(mem[ip]);
                state = instruction.eval(&mut mem, &mut input, &mut output, ip);
            }
            Evaluation::Halt => break,
            _ => panic!("Unexpected state: {:?}", state),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared_2019::input::load_csv_input_from_file;

    #[test]
    fn test_part_one() {
        let program = load_csv_input_from_file("inputs/input.txt");
        let answer = part_one(program);
        assert_eq!(15314507, answer);
    }

    #[test]
    fn test_part_two() {
        let program = load_csv_input_from_file("inputs/input.txt");
        let answer = part_two(program);
        assert_eq!(652726, answer);
    }
}
