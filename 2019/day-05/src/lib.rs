use aoc_shared_2019::intcode::Instruction;

pub fn part_one(mem: Vec<isize>) -> isize {
    let inputs = vec![1];
    let mut output = solve(mem, inputs.iter().copied());

    let last = output.pop().unwrap();
    output.iter().copied().for_each(|v| assert_eq!(0, v));
    last
}

pub fn part_two(mem: Vec<isize>) -> isize {
    let inputs = vec![5];
    let mut output = solve(mem, inputs.iter().copied());

    let last = output.pop().unwrap();
    output.iter().copied().for_each(|v| assert_eq!(0, v));
    last
}

fn solve<Input: Iterator<Item = isize>>(mut mem: Vec<isize>, mut input: Input) -> Vec<isize> {
    let mut ip = Some(0);
    let mut output = vec![];

    while let Some(addr) = ip.take() {
        let instruction = Instruction::decode(mem[addr]);
        ip = instruction.eval(&mut mem, &mut input, &mut output, addr);
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
    fn test_part_one() {
        let program = load_csv_input_from_file("inputs/input.txt");
        let answer = part_two(program);
        assert_eq!(652726, answer);
    }
}
