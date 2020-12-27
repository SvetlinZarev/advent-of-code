#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    Nop(i32),
    ImNop(i32),
    Acc(i32),
    Jmp(i32),
    ImJmp(i32),
    Trap,
}

pub fn solve(mut opcodes: Vec<OpCode>) -> Option<i32> {
    let mut solution = None;
    let mut register;
    let mut ip;
    let mut modified_app = opcodes.clone();

    's: for _ in 0..opcodes.len() {
        let mut mutated_opcode_idx = None;

        modified_app.copy_from_slice(&opcodes);
        register = 0;
        ip = 0;

        loop {
            if ip == modified_app.len() {
                solution = Some(register);
                break 's;
            }

            let mut opcode = std::mem::replace(&mut modified_app[ip], OpCode::Trap);

            if mutated_opcode_idx.is_none() {
                if let OpCode::Nop(value) = opcode {
                    mutated_opcode_idx = Some(ip);
                    opcode = OpCode::Jmp(value);
                    opcodes[ip] = OpCode::ImNop(value);
                } else if let OpCode::Jmp(value) = opcode {
                    mutated_opcode_idx = Some(ip);
                    opcode = OpCode::Nop(value);
                    opcodes[ip] = OpCode::ImJmp(value);
                }
            }

            match opcode {
                OpCode::Nop(_) | OpCode::ImNop(_) => ip += 1,
                OpCode::Acc(value) => {
                    ip += 1;
                    register += value;
                }
                OpCode::Jmp(value) | OpCode::ImJmp(value) => {
                    ip = (ip as i32 + value) as usize;
                }
                OpCode::Trap => {
                    break;
                }
            }
        }
    }

    solution
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
