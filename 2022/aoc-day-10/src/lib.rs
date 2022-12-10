#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    Add(i32),
    Nop,
}

impl Instruction {
    fn cycles(self) -> i32 {
        match self {
            Instruction::Add(_) => 2,
            Instruction::Nop => 1,
        }
    }

    fn register(self) -> i32 {
        match self {
            Instruction::Add(x) => x,
            Instruction::Nop => 0,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in input.lines() {
        if line.starts_with("n") {
            instructions.push(Instruction::Nop);
        } else {
            instructions.push(Instruction::Add(line[5..].parse().unwrap()));
        }
    }

    instructions
}

pub fn part_one(instructions: &[Instruction]) -> i32 {
    let measure_at = [20, 60, 100, 140, 180, 220];
    let mut measurement = 0;

    let mut register = 1;
    let mut cycles = 0;

    let mut signal_strength = 0;

    for instr in instructions.iter().copied() {
        let c = instr.cycles();

        if cycles < measure_at[measurement] && cycles + c >= measure_at[measurement] {
            signal_strength += measure_at[measurement] * register;

            measurement += 1;
            if measurement >= measure_at.len() {
                break;
            }
        }

        cycles += c;
        register += instr.register();
    }

    signal_strength
}

pub fn part_two(instructions: &[Instruction]) -> [[char; 40]; 6] {
    let mut crt = [[' '; 40]; 6];
    let mut register = 1;
    let mut cycles = 0;

    for instr in instructions.iter().copied() {
        let c = instr.cycles();
        let r = instr.register();

        for idx in cycles..cycles + c {
            let row = idx / 40;
            let col = idx % 40;
            let sprite = register - 1..register + 2;

            if sprite.contains(&col) {
                crt[row as usize][col as usize] = '█';
            }
        }

        cycles += c;
        register += r;
    }

    crt
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{parse_input, part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let instructions = parse_input(&input);
        assert_eq!(16020, part_one(&instructions))
    }

    #[test]
    #[rustfmt::skip]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let instructions = parse_input(&input);
        let answer = part_two(&instructions);
        
        assert_eq!("████  ██  ████ █  █ ████  ██  █    ███  ", answer[0].iter().collect::<String>());
        assert_eq!("█    █  █    █ █  █    █ █  █ █    █  █ ", answer[1].iter().collect::<String>());
        assert_eq!("███  █      █  █  █   █  █  █ █    █  █ ", answer[2].iter().collect::<String>());
        assert_eq!("█    █     █   █  █  █   ████ █    ███  ", answer[3].iter().collect::<String>());
        assert_eq!("█    █  █ █    █  █ █    █  █ █    █ █  ", answer[4].iter().collect::<String>());
        assert_eq!("████  ██  ████  ██  ████ █  █ ████ █  █ ", answer[5].iter().collect::<String>());
    }
}
