pub fn part_one(input: &[i64]) -> i64 {
    let mut mem = input.to_vec();

    // replace values as instructed
    mem[1] = 12;
    mem[2] = 2;

    solve(mem)
}

pub fn part_two(input: &[i64]) -> i64 {
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

fn solve(mut mem: Vec<i64>) -> i64 {
    let mut ip = 0;

    while mem[ip] != 99 {
        let addr1 = mem[ip + 1] as usize;
        let addr2 = mem[ip + 2] as usize;
        let addr3 = mem[ip + 3] as usize;

        match mem[ip] {
            1 => mem[addr3] = mem[addr1] + mem[addr2],
            2 => mem[addr3] = mem[addr1] * mem[addr2],
            _ => panic!("Unsupported opcode '{}' at '{}'", mem[ip], ip),
        }

        ip += 4;
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
