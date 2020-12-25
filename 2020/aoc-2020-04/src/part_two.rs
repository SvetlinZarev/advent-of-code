use nom::branch::{alt, permutation};
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::lib::std::str::FromStr;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::{separated_pair, tuple},
    AsChar, IResult,
};

pub fn solve(input: &str) -> usize {
    let mut valid_passports = 0;
    let passports = input.split("\n\n");
    for data in passports {
        match parse_passport(data.trim()) {
            Ok((_, passport)) => {
                if passport.is_valid() {
                    valid_passports += 1;
                } else {
                    if cfg!(debug_assertions) {
                        println!("Invalid passport: {:#?}", passport);
                    }
                }
            }

            Err(_e) => {
                if cfg!(debug_assertions) {
                    println!("Malformed passport data: {:?}:\n{}\n", _e, data);
                }
            }
        }
    }
    valid_passports
}

const TAG_YEAR_OF_BIRTH: &str = "byr";
const TAG_YEAR_OF_ISSUE: &str = "iyr";
const TAG_YEAR_OF_EXPIRATION: &str = "eyr";
const TAG_EYE_COLOR: &str = "ecl";
const TAG_HAIR_COLOR: &str = "hcl";
const TAG_PASSPORT_ID: &str = "pid";
const TAG_COUNTRY_ID: &str = "cid";
const TAG_HEIGHT: &str = "hgt";
const KV_SEPARATOR: char = ':';
const SEP_SPACE: char = ' ';
const SEP_NEW_LINE: char = '\n';

#[derive(Debug, Eq, PartialEq)]
struct Year {
    value: u32,
}

impl Year {
    pub fn new(year: u32) -> Year {
        Year { value: year }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum EyeColor {
    Amb,
    Brown,
    Gray,
    Blue,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = String;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code {
            "amb" => Ok(EyeColor::Amb),
            "blu" => Ok(EyeColor::Blue),
            "grn" => Ok(EyeColor::Green),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Gray),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            code => Err(format!("Unsupported color code: {}", code)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

#[derive(Debug, PartialEq)]
struct Height {
    value: f64,
}

impl Height {
    pub fn from_inches(value: u32) -> Height {
        let cm = value as f64 * 2.54;
        Height { value: cm }
    }

    pub fn from_cm(value: u32) -> Height {
        let value = value as f64;
        Height { value }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PassportId {
    value: String,
}

#[derive(Debug, Eq, PartialEq)]
struct CountryId {
    value: String,
}

#[derive(Debug)]
struct Passport {
    byr: Year,
    iyr: Year,
    eyr: Year,
    ecl: EyeColor,
    hcl: Color,
    hgt: Height,
    pid: PassportId,
    cid: Option<CountryId>,
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        if !(self.byr.value >= 1920 && self.byr.value <= 2002) {
            return false;
        }

        if !(self.iyr.value >= 2010 && self.iyr.value <= 2020) {
            return false;
        }

        if !(self.eyr.value >= 2020 && self.eyr.value <= 2030) {
            return false;
        }

        if !(self.hgt.value > 149.0 && self.hgt.value < 194.0) {
            return false;
        }

        true
    }
}

fn parse_passport(input: &str) -> IResult<&str, Passport> {
    let (remaining, (cid, pid, iyr, eyr, byr, ecl, hcl, hgt)) = permutation((
        parse_field_country_id,
        parse_field_passport_id,
        parse_field_year_of_issue,
        parse_field_year_of_expiration,
        parse_field_year_of_birth,
        parse_field_eye_color,
        parse_field_hair_color,
        parse_field_height,
    ))(input)?;

    Ok((
        remaining,
        Passport {
            iyr,
            eyr,
            byr,
            pid,
            hcl,
            ecl,
            hgt,
            cid,
        },
    ))
}

fn parse_field_country_id(input: &str) -> IResult<&str, Option<CountryId>> {
    if input.is_empty() {
        return Ok((input, None));
    }

    let (remaining, (_, cid)) =
        separated_pair(tag(TAG_COUNTRY_ID), char(KV_SEPARATOR), country_id)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;

    Ok((remaining, Some(cid)))
}

fn country_id(input: &str) -> IResult<&str, CountryId> {
    let (remaining, cid) = take_while(is_digit)(input)?;

    Ok((
        remaining,
        CountryId {
            value: cid.to_owned(),
        },
    ))
}

fn parse_field_passport_id(input: &str) -> IResult<&str, PassportId> {
    let (remaining, (_, pid)) =
        separated_pair(tag(TAG_PASSPORT_ID), char(KV_SEPARATOR), passport_id)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, pid))
}

fn passport_id(input: &str) -> IResult<&str, PassportId> {
    let (remaining, pid) = take_while_m_n(9, 9, is_digit)(input)?;

    Ok((
        remaining,
        PassportId {
            value: pid.to_owned(),
        },
    ))
}

fn parse_field_eye_color(input: &str) -> IResult<&str, EyeColor> {
    let (remaining, (_, clr)) =
        separated_pair(tag(TAG_EYE_COLOR), char(KV_SEPARATOR), eye_color_code)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, clr))
}

fn eye_color_code(input: &str) -> IResult<&str, EyeColor> {
    let (remaining, code) = alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("hzl"),
        tag("grn"),
        tag("oth"),
    ))(input)?;

    let color = EyeColor::from_str(code).map_err(|_e| {
        nom::Err::Failure(nom::error::Error::new(code, nom::error::ErrorKind::Tag))
    })?;

    Ok((remaining, color))
}

fn parse_field_height(input: &str) -> IResult<&str, Height> {
    let (remaining, (_, height)) =
        separated_pair(tag(TAG_HEIGHT), char(KV_SEPARATOR), parse_height)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, height))
}

fn parse_height(input: &str) -> IResult<&str, Height> {
    let (input, num) = map_res(take_while_m_n(1, 3, is_digit), u32_from_str)(input)?;
    let (input, unit) = alt((tag("cm"), tag("in")))(input)?;

    let height = if "cm" == unit {
        Height::from_cm(num)
    } else {
        Height::from_inches(num)
    };

    Ok((input, height))
}

fn parse_field_year_of_birth(input: &str) -> IResult<&str, Year> {
    let (remaining, (_, year)) =
        separated_pair(tag(TAG_YEAR_OF_BIRTH), char(KV_SEPARATOR), parse_year)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, year))
}

fn parse_field_year_of_expiration(input: &str) -> IResult<&str, Year> {
    let (remaining, (_, year)) =
        separated_pair(tag(TAG_YEAR_OF_EXPIRATION), char(KV_SEPARATOR), parse_year)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, year))
}

fn parse_field_year_of_issue(input: &str) -> IResult<&str, Year> {
    let (remaining, (_, year)) =
        separated_pair(tag(TAG_YEAR_OF_ISSUE), char(KV_SEPARATOR), parse_year)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, year))
}

fn parse_year(input: &str) -> IResult<&str, Year> {
    let (input, num) = map_res(take_while_m_n(4, 4, is_digit), u32_from_str)(input)?;
    Ok((input, Year::new(num)))
}

fn parse_field_hair_color(input: &str) -> IResult<&str, Color> {
    let (remaining, (_, color)) =
        separated_pair(tag(TAG_HAIR_COLOR), char(KV_SEPARATOR), parse_hex_color)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, color))
}

fn parse_hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (r, g, b)) = tuple((hex_color_code, hex_color_code, hex_color_code))(input)?;

    Ok((input, Color::new(r, g, b)))
}

fn hex_color_code(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), u8_from_hex)(input)
}

fn u8_from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_hex_digit()
}

fn u32_from_str(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse()
}

fn is_digit(c: char) -> bool {
    c.is_dec_digit()
}

fn remove_trailing_whitespace(input: &str) -> IResult<&str, ()> {
    let (remaining, _) = alt((
        take_while_m_n(1, 1, |x| x == SEP_NEW_LINE),
        take_while(|x| x == SEP_SPACE),
    ))(input)?;

    Ok((remaining, ()))
}

#[cfg(test)]
mod tests {
    use nom::Err;

    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_hex_color__uppercase() {
        let input = "#AA01C5";
        let (remaining, color) = parse_hex_color(input).unwrap();

        assert_eq!("", remaining);
        assert_eq!(Color::new(170, 1, 197), color);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_hex_color__lowercase() {
        let input = "#aa01c5";
        let (remaining, color) = parse_hex_color(input).unwrap();

        assert_eq!("", remaining);
        assert_eq!(Color::new(170, 1, 197), color);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_hex_color__with_remaining_input() {
        let input = "#aa01c5 remaining";
        let (remaining, color) = parse_hex_color(input).unwrap();

        assert_eq!(" remaining", remaining);
        assert_eq!(Color::new(170, 1, 197), color);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_hex_color__with_error() {
        let input = "#not a color";
        let err = parse_hex_color(input).unwrap_err();

        match err {
            Err::Error(e) => {
                assert_eq!("not a color", e.input)
            }

            _ => panic!("Unexpected error type: {:?}", err),
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_year__with_max_digits() {
        let input = "2020";
        let (remaining, year) = parse_year(input).unwrap();

        assert_eq!("", remaining);
        assert_eq!(Year::new(2020), year);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_year__not_a_year() {
        let input = "hey";
        let result = parse_year(input);
        assert!(result.is_err());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_height__cm() {
        let input = "165cm yey";
        let (remaining, height) = parse_height(input).unwrap();

        assert_eq!(" yey", remaining);
        assert_eq!(Height::from_cm(165), height);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_height__in() {
        let input = "70in yey";
        let (remaining, height) = parse_height(input).unwrap();

        assert_eq!(" yey", remaining);
        assert_eq!(Height::from_inches(70), height);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_height__invalid_unit() {
        let input = "70meters";
        let result = parse_height(input);

        assert!(result.is_err());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_eye_color_field__known_colors() {
        let colors = vec![
            ("ecl:amb", EyeColor::Amb),
            ("ecl:blu", EyeColor::Blue),
            ("ecl:grn", EyeColor::Green),
            ("ecl:brn", EyeColor::Brown),
            ("ecl:gry", EyeColor::Gray),
            ("ecl:hzl", EyeColor::Hazel),
            ("ecl:oth", EyeColor::Other),
        ];

        for (input, expectation) in colors {
            let (_, color) = parse_field_eye_color(input).unwrap();
            assert_eq!(expectation, color);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_eye_color_field__invalid() {
        let result = parse_field_eye_color("ecl;blu");
        assert!(result.is_err());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_passport__all_fields() {
        let input = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"#
            .trim();

        let (_, passport) = parse_passport(input).unwrap();

        assert_eq!(
            Some(CountryId {
                value: "147".to_owned()
            }),
            passport.cid
        );

        assert_eq!(
            PassportId {
                value: "860033327".to_owned()
            },
            passport.pid
        );

        assert_eq!(Year::new(2017), passport.iyr);
        assert_eq!(Year::new(2020), passport.eyr);
        assert_eq!(Year::new(1937), passport.byr);
        assert_eq!(EyeColor::Gray, passport.ecl);
        assert_eq!(Color::new(255, 255, 253), passport.hcl);
        assert_eq!(Height::from_cm(183), passport.hgt);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_parse_passport__without_country_id() {
        let input = r#"
ecl:gry pid:860033327  eyr:2020 hcl:#fffffd
byr:1937 iyr:2017  hgt:183cm"#
            .trim();

        let (_, passport) = parse_passport(input).unwrap();

        assert_eq!(
            PassportId {
                value: "860033327".to_owned()
            },
            passport.pid
        );

        assert_eq!(None, passport.cid);
        assert_eq!(Year::new(2017), passport.iyr);
        assert_eq!(Year::new(2020), passport.eyr);
        assert_eq!(Year::new(1937), passport.byr);
        assert_eq!(EyeColor::Gray, passport.ecl);
        assert_eq!(Color::new(255, 255, 253), passport.hcl);
        assert_eq!(Height::from_cm(183), passport.hgt);
    }
}
