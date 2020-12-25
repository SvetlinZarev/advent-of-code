use std::convert::{TryFrom, TryInto};

struct Password<'v> {
    min: usize,
    max: usize,
    letter: u8,
    value: &'v str,
}

impl<'v> Password<'v> {
    pub fn is_valid(&self) -> bool {
        let count = self
            .value
            .as_bytes()
            .iter()
            .filter(|&&x| x == self.letter)
            .count();

        count >= self.min && count <= self.max
    }
}

impl<'v> TryFrom<&'v str> for Password<'v> {
    type Error = std::io::Error;

    fn try_from(value: &'v str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }

        let min_end = value.find('-').ok_or(std::io::ErrorKind::InvalidInput)?;
        let min = &value[0..min_end];
        let min = min
            .parse::<usize>()
            .map_err(|_| std::io::ErrorKind::InvalidInput)?;

        let max_end = value.find(' ').ok_or(std::io::ErrorKind::InvalidInput)?;
        let max = &value[min_end + 1..max_end];
        let max = max
            .parse::<usize>()
            .map_err(|_| std::io::ErrorKind::InvalidInput)?;

        if min >= max {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }

        let letter = &value[max_end + 1..max_end + 2];
        let letter = letter.as_bytes()[0];

        let value = &value[max_end + 4..];

        Ok(Password {
            value,
            min,
            max,
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
