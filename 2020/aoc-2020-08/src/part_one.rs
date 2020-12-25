#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
    Trap,
}

pub fn parse_input(input: &str) -> Vec<OpCode> {
    let mut instructions = vec![];
    for line in input.lines() {
        let opcode = &line[..3];
        let value = line[4..].parse().unwrap();

        instructions.push(match opcode {
            "nop" => OpCode::Nop(value),
            "acc" => OpCode::Acc(value),
            "jmp" => OpCode::Jmp(value),
            _ => panic!("Unrecognized instruction: {:?}", line),
        });
    }

    instructions
}

pub fn solve(mut opcodes: Vec<OpCode>) -> i32 {
    let mut ip = 0;
    let mut acc = 0;

    loop {
        let opcode = std::mem::replace(&mut opcodes[ip], OpCode::Trap);

        match opcode {
            OpCode::Nop(_) => ip += 1,
            OpCode::Acc(value) => {
                ip += 1;
                acc += value;
            }
            OpCode::Jmp(value) => ip = (ip as i32 + value) as usize,
            OpCode::Trap => {
                break;
            }
        }
    }

    acc
}
