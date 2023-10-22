use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    Set(Register, Accessor),
    Sub(Register, Accessor),
    Mul(Register, Accessor),
    Jnz(Accessor, Accessor),
}

impl FromStr for OpCode {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((opcode, params)) = s.split_once(' ') else {
            return Err(format!("invalid instruction format; {}", s).into());
        };

        let Some((x, y)) = params.split_once(' ') else {
            return Err(format!("invalid instruction format; {}", s).into());
        };

        Ok(
            match opcode {
                "set" => OpCode::Set(x.parse()?, y.parse()?),
                "sub" => OpCode::Sub(x.parse()?, y.parse()?),
                "mul" => OpCode::Mul(x.parse()?, y.parse()?),
                "jnz" => OpCode::Jnz(x.parse()?, y.parse()?),
                _ => return Err(format!("unknown instruction: {}", s).into()),
            }
        )
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
    let mut answer = 0;

    let mut ip = 0;
    while ip < input.len() {
        match input[ip] {
            OpCode::Set(x, y) => registers[x.index()] = y.value(&registers),
            OpCode::Sub(x, y) => registers[x.index()] -= y.value(&registers),
            OpCode::Mul(x, y) => {
                registers[x.index()] *= y.value(&registers);
                answer += 1;
            }
            OpCode::Jnz(x, y) => {
                let x = x.value(&registers);
                if x != 0 {
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

    answer
}

pub fn part_two(input: i64) -> i64 {
    const B_INIT_MUL: i64 = 100;
    const B_INIT_ADD: i64 = 100_000;
    const B_INCREMENT: i64 = 17;
    const C_INIT_ADD: i64 = 17_000;

    // Initial value, depends on the input
    // Maybe the rest of the constants depend on the input too
    let mut b = input;

    // H holds the answer to part 2
    let mut h = 0;

    // Program start
    b = b * B_INIT_MUL + B_INIT_ADD;
    let c = b + C_INIT_ADD;

    loop {
        let mut f = 1;
        let mut d = 2;

        // Check if B is a prime, by testing if it's divisible
        // by any number from 2 to sqrt(b)
        while d * d <= b {
            if b % d == 0 {
                f = 0;
                break;
            }

            d += 1;
        }

        // F==0 means that B is not a prime
        if f == 0 {
            h += 1;
        }

        // C_INIT_ADD is `1000 * C_INIT_ADD`, therefore,
        // we loop[ for 1000 cycles
        if b == c {
            break;
        }

        b += B_INCREMENT;
    }


    h
}


#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(3025, answer);
    }

    #[test]
    fn test_part_two() {
        const HARDCODED_INPUT: i64 = 57;
        let answer = part_two(HARDCODED_INPUT);
        assert_eq!(915, answer);
    }
}
