use std::convert::{TryFrom, TryInto};

struct Password<'v> {
    idx_a: usize,
    idx_b: usize,
    letter: u8,
    value: &'v str,
}

impl<'v> Password<'v> {
    pub fn is_valid(&self) -> bool {
        let pwd = self.value.as_bytes();

        let a = pwd[self.idx_a];
        let b = pwd[self.idx_b];

        (a != b) && (a == self.letter || b == self.letter)
    }
}

impl<'v> TryFrom<&'v str> for Password<'v> {
    type Error = std::io::Error;

    fn try_from(value: &'v str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }

        let a_end = value.find('-').ok_or(std::io::ErrorKind::InvalidInput)?;
        let a = &value[0..a_end];
        let idx_a = a
            .parse::<usize>()
            .map_err(|_| std::io::ErrorKind::InvalidInput)?;

        let b_end = value.find(' ').ok_or(std::io::ErrorKind::InvalidInput)?;
        let b = &value[a_end + 1..b_end];
        let idx_b = b
            .parse::<usize>()
            .map_err(|_| std::io::ErrorKind::InvalidInput)?;

        if idx_a == 0 {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }

        if idx_a >= idx_b {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }

        let letter = &value[b_end + 1..b_end + 2];
        let letter = letter.as_bytes()[0];

        let value = &value[b_end + 4..];

        Ok(Password {
            value,
            idx_a: idx_a - 1,
            idx_b: idx_b - 1,
            letter,
        })
    }
}

pub fn solve(input: &str) -> usize {
    let mut valid_passwords = 0;
    for line in input.lines() {
        let pwd: Password = line.try_into().unwrap();
        if pwd.is_valid() {
            valid_passwords += 1;
        }
    }
    valid_passwords
}
