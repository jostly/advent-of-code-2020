use std::collections::HashSet;
use std::str::FromStr;
use crate::day04::Line::{Empty, Fields};
use crate::day04::Field::{IssueYear, BirthYear, ExpirationYear, Height, HairColor, EyeColor, PassportId, CountryId};
use regex::bytes::Regex;

#[derive(Debug, Eq, PartialEq)]
enum Line {
    Fields(HashSet<Field>),
    Empty,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Empty);
        }

        let mut fields = HashSet::new();

        for substring in s.split(' ') {
            if let Ok(field) = substring.parse() {
                fields.insert(field);
            }
        }

        Ok(Fields(fields))
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Field {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 5 {
            return Err(());
        }
        let identifier = &s[0..3];
        let data = &s[4..];

        match identifier {
            "byr" if valid_byr(data) => Ok(BirthYear),
            "iyr" if valid_iyr(data) => Ok(IssueYear),
            "eyr" if valid_eyr(data) => Ok(ExpirationYear),
            "hgt" if valid_hgt(data) => Ok(Height),
            "hcl" if valid_hcl(data) => Ok(HairColor),
            "ecl" if valid_ecl(data) => Ok(EyeColor),
            "pid" if valid_pid(data) => Ok(PassportId),
            "cid" => Ok(CountryId),
            _ => {
                println!("Invalid: {}", s);
                Err(())
            },
        }
    }
}

fn valid_pid(data: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(data.as_bytes())
}


fn valid_ecl(data: &str) -> bool {
    let valid_ecl_values = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_ecl_values.contains(&data)
}

fn valid_hcl(data: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(data.as_bytes())
}

fn valid_hgt(data: &str) -> bool {
    if data.len() == 4 {
        if &data[2..] != "in" {
            return false;
        }
        if let Ok(inches) = (&data[0..2]).parse::<i32>() {
            return inches >= 59 && inches <= 76;
        }
        return false;
    }
    if data.len() == 5 {
        if &data[3..] != "cm" {
            return false;
        }
        if let Ok(cm) = (&data[0..3]).parse::<i32>() {
            return cm >= 150 && cm <= 193;
        }
        return false;
    }

    return false;
}

fn valid_byr(data: &str) -> bool {
    if let Ok(year) = data.parse::<i32>() {
        return year >= 1920 && year <= 2002
    }
    false
}

fn valid_iyr(data: &str) -> bool {
    if let Ok(year) = data.parse::<i32>() {
        return year >= 2010 && year <= 2020
    }
    false
}

fn valid_eyr(data: &str) -> bool {
    if let Ok(year) = data.parse::<i32>() {
        return year >= 2020 && year <= 2030
    }
    false
}

#[derive(Debug)]
struct Passport {
    fields: HashSet<Field>
}

impl Passport {
    fn is_valid(&self) -> bool {
        let expected_fields = vec![BirthYear, IssueYear, ExpirationYear, Height, HairColor, EyeColor, PassportId];

        for ref field in expected_fields {
            if !self.fields.contains(field) {
                return false;
            }
        }

        true
    }
}

fn parse_passports(lines: Vec<Line>) -> Vec<Passport> {
    let mut out = Vec::new();

    let mut fields = HashSet::new();

    for line in lines {
        match line {
            Fields(f) => {
                for field in f {
                    fields.insert(field);
                }
            },
            Empty => {
                out.push(Passport { fields: fields.clone() });
                fields.clear();
            }
        }
    }
    if !fields.is_empty() {
        out.push( Passport { fields });
    }

    out
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{hash_set, read_file};

    #[test]
    fn parse_field() {
        assert_eq!("byr:1989".parse::<Field>(), Ok(Field::BirthYear));
        assert_eq!("zzz".parse::<Field>(), Err(()));
    }

    #[test]
    fn parse_line() {
        let line = "hcl:#cfa07d byr:1929".parse::<Line>().unwrap();
        let expected_fields = hash_set(vec![HairColor, BirthYear]);

        assert_eq!(line, Fields(expected_fields));

        let empty_line = "".parse::<Line>().unwrap();

        assert_eq!(empty_line, Empty);
    }

    #[test]
    fn reduce_to_passports() {
        let lines: Vec<Line> = read_file("./src/day04/input.txt").unwrap();
        let passports = parse_passports(lines);

        println!("Valid passports: {}", passports.iter().filter(|p| p.is_valid()).count());

        for passport in passports {
            println!("{}: {:?}", passport.is_valid(), passport);
        }
    }
}