use aoc_shared_2019::intcode::Instruction;

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

fn solve(mut mem: Vec<isize>) -> isize {
    let mut ip = 0;

    loop {
        let instruction = Instruction::decode(mem[ip]);

        match instruction {
            Instruction::Add(m1, m2, md) => {
                let v1 = *m1.reference(&mut mem, ip + 1);
                let v2 = *m2.reference(&mut mem, ip + 2);
                let dst = md.reference(&mut mem, ip + 3);

                *dst = v1 + v2;
            }
            Instruction::Mul(m1, m2, md) => {
                let v1 = *m1.reference(&mut mem, ip + 1);
                let v2 = *m2.reference(&mut mem, ip + 2);
                let dst = md.reference(&mut mem, ip + 3);

                *dst = v1 * v2;
            }
            Instruction::Hlt => break,
            _ => panic!("Unexpected instruction: {:?}", instruction),
        }

        ip += instruction.increment();
    }

    mem[0]
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
