use aoc_shared_2019::intcode::{Computer, Outcome};

pub fn part_one(input: &[isize]) -> isize {
    let mut mem = input.to_vec();

    // replace values as instructed
    mem[1] = 12;
    mem[2] = 2;

    solve(mem)
}

pub fn part_two(input: &[isize]) -> isize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = input.to_vec();
            mem[1] = noun;
            mem[2] = verb;

            if solve(mem) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("There is no solution");
}

fn solve(mem: Vec<isize>) -> isize {
    let mut vm = Computer::new(mem);
    if let Outcome::Fault(f) = vm.run(&mut std::iter::empty(), &mut vec![]) {
        panic!("{:?}", f);
    }
    *vm.mem(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared_2019::input::load_csv_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_csv_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(10566835, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_csv_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(2347, answer);
    }
}
