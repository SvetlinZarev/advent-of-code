use std::collections::VecDeque;

pub trait Input {
    fn read(&mut self) -> Option<isize>;
}

pub trait Output {
    fn write(&mut self, value: isize);
}

impl Input for std::iter::Once<isize> {
    fn read(&mut self) -> Option<isize> {
        self.next()
    }
}

impl<A: Iterator<Item = isize>, B: Iterator<Item = isize>> Input for std::iter::Chain<A, B> {
    fn read(&mut self) -> Option<isize> {
        self.next()
    }
}

impl Input for VecDeque<isize> {
    fn read(&mut self) -> Option<isize> {
        self.pop_front()
    }
}

impl Output for VecDeque<isize> {
    fn write(&mut self, value: isize) {
        self.push_back(value);
    }
}

impl Output for Vec<isize> {
    fn write(&mut self, value: isize) {
        self.push(value);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Evaluation {
    Continue(usize),
    InsufficientInput(usize),
    Halt,
    Fault(Option<String>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Position = 0,
    Immediate = 1,
}

impl Mode {
    pub fn reference(self, mem: &mut [isize], idx: usize) -> &mut isize {
        match self {
            Mode::Position => {
                let pos = mem[idx] as usize;
                &mut mem[pos]
            }
            Mode::Immediate => &mut mem[idx],
        }
    }
}

impl TryFrom<isize> for Mode {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => return Err(format!("Invalid mode: {:?}", value)),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Inp(Mode),
    Out(Mode),
    JiT(Mode, Mode),
    JiF(Mode, Mode),
    LT(Mode, Mode, Mode),
    EQ(Mode, Mode, Mode),
    Hlt,
}

impl Instruction {
    pub fn decode(n: isize) -> Self {
        assert!(n >= 0);

        let instr = n % 100;
        let modes = n / 100;

        match instr {
            1 => {
                let (a, b, c) = decode_3(modes);
                Instruction::Add(a, b, c)
            }

            2 => {
                let (a, b, c) = decode_3(modes);
                Instruction::Mul(a, b, c)
            }

            3 => {
                let mode = decode_1(modes);
                Instruction::Inp(mode)
            }

            4 => {
                let mode = decode_1(modes);
                Instruction::Out(mode)
            }

            5 => {
                let (a, b) = decode_2(modes);
                Instruction::JiT(a, b)
            }

            6 => {
                let (a, b) = decode_2(modes);
                Instruction::JiF(a, b)
            }

            7 => {
                let (a, b, c) = decode_3(modes);
                Instruction::LT(a, b, c)
            }

            8 => {
                let (a, b, c) = decode_3(modes);
                Instruction::EQ(a, b, c)
            }

            99 => Instruction::Hlt,
            _ => panic!("Cannot decode int-code opcode: {:?}", n),
        }
    }

    pub fn eval(
        self,
        mem: &mut [isize],
        input: &mut impl Input,
        output: &mut impl Output,
        ip: usize,
    ) -> Evaluation {
        let instruction = Instruction::decode(mem[ip]);

        match instruction {
            Instruction::Add(m1, m2, md) => {
                let v1 = *m1.reference(mem, ip + 1);
                let v2 = *m2.reference(mem, ip + 2);
                let dst = md.reference(mem, ip + 3);

                *dst = v1 + v2;
            }

            Instruction::Mul(m1, m2, md) => {
                let v1 = *m1.reference(mem, ip + 1);
                let v2 = *m2.reference(mem, ip + 2);
                let dst = md.reference(mem, ip + 3);

                *dst = v1 * v2;
            }

            Instruction::Inp(md) => match input.read() {
                None => return Evaluation::InsufficientInput(ip),
                Some(value) => *md.reference(mem, ip + 1) = value,
            },

            Instruction::Out(md) => {
                let value = *md.reference(mem, ip + 1);
                output.write(value);
            }

            Instruction::JiT(m1, m2) => {
                let v = *m1.reference(mem, ip + 1);
                if v != 0 {
                    let addr = *m2.reference(mem, ip + 2);
                    return match addr.try_into() {
                        Ok(addr) => Evaluation::Continue(addr),
                        Err(e) => Evaluation::Fault(Some(format!(
                            "Cannot jump to negative address: IP={}; Addr={}; Err={:?}",
                            ip, addr, e
                        ))),
                    };
                }
            }

            Instruction::JiF(m1, m2) => {
                let v = *m1.reference(mem, ip + 1);
                if v == 0 {
                    let addr = *m2.reference(mem, ip + 2);
                    return match addr.try_into() {
                        Ok(addr) => Evaluation::Continue(addr),
                        Err(e) => Evaluation::Fault(Some(format!(
                            "Cannot jump to negative address: IP={}; Addr={}; Err={:?}",
                            ip, addr, e
                        ))),
                    };
                }
            }

            Instruction::LT(m1, m2, md) => {
                let v1 = *m1.reference(mem, ip + 1);
                let v2 = *m2.reference(mem, ip + 2);
                let dst = md.reference(mem, ip + 3);

                *dst = (v1 < v2) as isize;
            }

            Instruction::EQ(m1, m2, md) => {
                let v1 = *m1.reference(mem, ip + 1);
                let v2 = *m2.reference(mem, ip + 2);
                let dst = md.reference(mem, ip + 3);

                *dst = (v1 == v2) as isize;
            }

            Instruction::Hlt => return Evaluation::Halt,
        }

        Evaluation::Continue(ip + self.increment())
    }

    pub fn increment(self) -> usize {
        match self {
            Instruction::Add(_, _, _) => 4,
            Instruction::Mul(_, _, _) => 4,
            Instruction::Inp(_) => 2,
            Instruction::Out(_) => 2,
            Instruction::JiT(_, _) => 3,
            Instruction::JiF(_, _) => 3,
            Instruction::LT(_, _, _) => 4,
            Instruction::EQ(_, _, _) => 4,
            Instruction::Hlt => 0,
        }
    }
}

fn decode_3(modes: isize) -> (Mode, Mode, Mode) {
    assert!(modes >= 0);

    let mut code = modes;

    let mode_1 = Mode::try_from(code % 10).unwrap();
    code /= 10;

    let mode_2 = Mode::try_from(code % 10).unwrap();
    code /= 10;

    let mode_3 = Mode::try_from(code % 10).unwrap();

    (mode_1, mode_2, mode_3)
}

fn decode_2(modes: isize) -> (Mode, Mode) {
    assert!(modes >= 0);

    let mut code = modes;

    let mode_1 = Mode::try_from(code % 10).unwrap();
    code /= 10;

    let mode_2 = Mode::try_from(code % 10).unwrap();

    (mode_1, mode_2)
}

fn decode_1(modes: isize) -> Mode {
    assert!(modes >= 0);

    Mode::try_from(modes % 10).unwrap()
}
