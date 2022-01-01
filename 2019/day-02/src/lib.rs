use aoc_shared_2019::intcode::{addr, instruction, OPERANDS, OP_ADD, OP_HLT, OP_MUL};

pub fn part_one(input: &[usize]) -> usize {
    let mut mem = input.to_vec();

    // replace values as instructed
    mem[1] = 12;
    mem[2] = 2;

    solve(mem)
}

pub fn part_two(input: &[usize]) -> usize {
    let mut noun = 0;
    let mut verb = 0;

    loop {
        let mut mem = input.to_vec();
        mem[1] = noun;
        mem[2] = verb;

        if solve(mem) == 19690720 {
            break 100 * noun + verb;
        }

        verb = (verb + 1) % 100;
        if verb == 0 {
            noun += 1;
        }
    }
}

fn solve(mut mem: Vec<usize>) -> usize {
    let mut ip = 0;

    loop {
        let opcode = instruction(&mem, ip);

        match opcode {
            OP_ADD => {
                let addr1 = addr(&mem, ip + 1);
                let addr2 = addr(&mem, ip + 2);
                let addr3 = addr(&mem, ip + 3);

                mem[addr3] = mem[addr1] + mem[addr2];
            }
            OP_MUL => {
                let addr1 = addr(&mem, ip + 1);
                let addr2 = addr(&mem, ip + 2);
                let addr3 = addr(&mem, ip + 3);

                mem[addr3] = mem[addr1] * mem[addr2];
            }
            OP_HLT => break,
            _ => panic!("Unsupported opcode '{}' at '{}'", mem[ip], ip),
        }

        ip += OPERANDS[opcode] + 1;
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
