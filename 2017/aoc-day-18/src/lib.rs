use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    Snd(Accessor),
    Set(Register, Accessor),
    Add(Register, Accessor),
    Mul(Register, Accessor),
    Mod(Register, Accessor),
    Rcv(Accessor),
    Jgz(Accessor, Accessor),
}

impl FromStr for OpCode {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((opcode, params)) = s.split_once(' ') else {
            return Err(format!("invalid instruction format; {}", s).into());
        };

        Ok(match opcode {
            "snd" => OpCode::Snd(params.parse()?),
            "rcv" => OpCode::Rcv(params.parse()?),
            opcode => {
                let Some((x, y)) = params.split_once(' ') else {
                    return Err(format!("invalid instruction format; {}", s).into());
                };

                match opcode {
                    "set" => OpCode::Set(x.parse()?, y.parse()?),
                    "add" => OpCode::Add(x.parse()?, y.parse()?),
                    "mul" => OpCode::Mul(x.parse()?, y.parse()?),
                    "mod" => OpCode::Mod(x.parse()?, y.parse()?),
                    "jgz" => OpCode::Jgz(x.parse()?, y.parse()?),
                    _ => return Err(format!("unknown instruction: {}", s).into()),
                }
            }
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Accessor {
    Value(i64),
    Register(Register),
}

impl FromStr for Accessor {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 && (b'a'..=b'z').contains(&s.as_bytes()[0]) {
            return Ok(Accessor::Register(Register(s.as_bytes()[0])));
        }

        Ok(Accessor::Value(s.parse()?))
    }
}

impl Accessor {
    fn value(self, registers: &[i64]) -> i64 {
        match self {
            Accessor::Value(value) => value,
            Accessor::Register(register) => registers[register.index()],
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Register(u8);

impl FromStr for Register {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(format!("invalid register: {}", s).into());
        }

        Ok(Register(s.as_bytes()[0]))
    }
}

impl Register {
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

pub fn part_one(input: &[OpCode]) -> i64 {
    let mut registers = [0; 256];
    let mut sounds = vec![];

    let mut ip = 0;
    while ip < input.len() {
        match input[ip] {
            OpCode::Snd(x) => sounds.push(x.value(&registers)),
            OpCode::Set(x, y) => registers[x.index()] = y.value(&registers),
            OpCode::Add(x, y) => registers[x.index()] += y.value(&registers),
            OpCode::Mul(x, y) => registers[x.index()] *= y.value(&registers),
            OpCode::Mod(x, y) => registers[x.index()] %= y.value(&registers),
            OpCode::Rcv(x) => {
                let value = x.value(&registers);
                if value != 0 {
                    return sounds.pop().unwrap();
                }
            }
            OpCode::Jgz(x, y) => {
                let x = x.value(&registers);
                if x > 0 {
                    let offset = y.value(&registers) as isize;
                    ip = match ip.checked_add_signed(offset) {
                        None => panic!(
                            "JGZ caused an jump outside the instructions range: {}/{}",
                            ip, offset
                        ),
                        Some(value) => value,
                    };

                    continue;
                }
            }
        }

        ip += 1;
    }

    panic!("failed to find a solution")
}

pub fn part_two(input: &[OpCode]) -> usize {
    let mut reg_0 = [0; 256];
    let mut reg_1 = [0; 256];
    reg_1[b'p' as usize] = 1;

    let mut ip_0 = 0;
    let mut ip_1 = 0;

    let mut queue_0 = VecDeque::new();
    let mut queue_1 = VecDeque::new();

    let mut answer = 0;

    // Assume that we can never go back tp IP=0
    // Then the only way to HALT is to complet ethe program (IP >= input.len())
    // Or to block waiting on an empty queue.
    //
    // Therefore, allow the first loop (ip=0), but any subsequent iteration
    // should require a non-empty queue
    while ip_0 == 0 || (ip_0 >= input.len() || !queue_1.is_empty()) {
        ip_0 = run_program(input, ip_0, &mut reg_0, &mut queue_0, &mut queue_1);
        if queue_0.is_empty() {
            break;
        }

        answer -= queue_1.len();
        ip_1 = run_program(input, ip_1, &mut reg_1, &mut queue_1, &mut queue_0);
        answer += queue_1.len();
    }

    answer
}

fn run_program(
    instructions: &[OpCode],
    ip: usize,
    registers: &mut [i64],
    tx: &mut VecDeque<i64>,
    rx: &mut VecDeque<i64>,
) -> usize {
    let mut ip = ip;

    while ip < instructions.len() {
        match instructions[ip] {
            OpCode::Snd(x) => tx.push_back(x.value(registers)),
            OpCode::Set(x, y) => registers[x.index()] = y.value(registers),
            OpCode::Add(x, y) => registers[x.index()] += y.value(registers),
            OpCode::Mul(x, y) => registers[x.index()] *= y.value(registers),
            OpCode::Mod(x, y) => registers[x.index()] %= y.value(registers),
            OpCode::Rcv(x) => match rx.pop_front() {
                None => return ip,
                Some(value) => match x {
                    Accessor::Value(_) => {
                        panic!("RCV called with a literal value. Expected a register")
                    }
                    Accessor::Register(r) => registers[r.index()] = value,
                },
            },
            OpCode::Jgz(x, y) => {
                let x = x.value(&registers);
                if x > 0 {
                    let offset = y.value(&registers) as isize;
                    ip = match ip.checked_add_signed(offset) {
                        None => panic!(
                            "JGZ caused an jump outside the instructions range: {}/{}",
                            ip, offset
                        ),
                        Some(value) => value,
                    };

                    continue;
                }
            }
        }

        ip += 1;
    }

    ip
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(7071, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(8001, answer);
    }
}
