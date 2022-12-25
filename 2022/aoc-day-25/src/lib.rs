pub fn part_one(input: &str) -> String {
    let mut sum = 0;
    for snafu in input.lines() {
        sum += snafu_to_decimal(snafu);
    }

    decimal_to_snafu(sum)
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut num = 0;

    for (idx, ch) in snafu.bytes().rev().enumerate() {
        let mut to_add = 5i64.pow(idx as u32);

        to_add *= match ch {
            b'=' => -2,
            b'-' => -1,
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            _ => panic!("unexpected character: {}", ch as char),
        };

        num += to_add;
    }

    num
}

fn decimal_to_snafu(num: i64) -> String {
    const MAPPING: [u8; 5] = [b'=', b'-', b'0', b'1', b'2'];
    assert!(num >= 0);

    let mut decimal = num;
    let mut buf = vec![];

    while decimal > 0 {
        let mut x = decimal % 5;
        if x > 2 {
            x -= 5;
        }

        decimal -= x;
        decimal /= 5;

        buf.push(MAPPING[(x + 2) as usize]);
    }

    buf.reverse();
    String::from_utf8(buf).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{decimal_to_snafu, part_one, snafu_to_decimal};
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(1, snafu_to_decimal("1"));
        assert_eq!(2, snafu_to_decimal("2"));
        assert_eq!(3, snafu_to_decimal("1="));
        assert_eq!(4, snafu_to_decimal("1-"));
        assert_eq!(5, snafu_to_decimal("10"));
        assert_eq!(6, snafu_to_decimal("11"));
        assert_eq!(7, snafu_to_decimal("12"));
        assert_eq!(8, snafu_to_decimal("2="));
        assert_eq!(9, snafu_to_decimal("2-"));
        assert_eq!(10, snafu_to_decimal("20"));
        assert_eq!(15, snafu_to_decimal("1=0"));
        assert_eq!(20, snafu_to_decimal("1-0"));
        assert_eq!(906, snafu_to_decimal("12111"));
        assert_eq!(1257, snafu_to_decimal("20012"));
        assert_eq!(1747, snafu_to_decimal("1=-0-2"));
        assert_eq!(2022, snafu_to_decimal("1=11-2"));
        assert_eq!(12345, snafu_to_decimal("1-0---0"));
        assert_eq!(314159265, snafu_to_decimal("1121-1110-1=0"));
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!("1", decimal_to_snafu(1));
        assert_eq!("2", decimal_to_snafu(2));
        assert_eq!("1=", decimal_to_snafu(3));
        assert_eq!("1-", decimal_to_snafu(4));
        assert_eq!("10", decimal_to_snafu(5));
        assert_eq!("11", decimal_to_snafu(6));
        assert_eq!("12", decimal_to_snafu(7));
        assert_eq!("2=", decimal_to_snafu(8));
        assert_eq!("2-", decimal_to_snafu(9));
        assert_eq!("20", decimal_to_snafu(10));
        assert_eq!("1=0", decimal_to_snafu(15));
        assert_eq!("1-0", decimal_to_snafu(20));
        assert_eq!("1=-0-2", decimal_to_snafu(1747));
        assert_eq!("1=11-2", decimal_to_snafu(2022));
        assert_eq!("1-0---0", decimal_to_snafu(12345));
        assert_eq!("1121-1110-1=0", decimal_to_snafu(314159265));
    }

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!("2=-0=1-0012-=-2=0=01", answer);
    }
}
