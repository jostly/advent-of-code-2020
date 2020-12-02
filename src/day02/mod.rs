use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
struct PasswordAndPolicy {
    policy: Policy,
    password: String,
}

impl PasswordAndPolicy {
    fn new(input: &str) -> Self {
        input.parse().unwrap()
    }

    fn old_is_valid(&self) -> bool {
        let actual = self.password.chars()
            .filter(|c| c == &self.policy.character)
            .count();

        actual >= self.policy.min && actual <= self.policy.max
    }

    fn is_valid(&self) -> bool {
        let chars: Vec<_> = self.password.chars().collect();
        let num_chars = self.password.len();

        // Assuming indices must be inside password to be valid
        if self.policy.min > num_chars || self.policy.max > num_chars {
            return false;
        }

        let match_first = chars[self.policy.min - 1] == self.policy.character;
        let match_second = chars[self.policy.max - 1] == self.policy.character;

        match_first ^ match_second
    }
}

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    character: char,
}

impl FromStr for Policy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+) ([a-z])$").unwrap();
        let captures = re.captures(s).unwrap();
        let min = captures[1].parse().unwrap();
        let max = captures[2].parse().unwrap();
        let character = captures[3].parse().unwrap();

        Ok(Policy { min, max, character })
    }
}

impl FromStr for PasswordAndPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let strings: Vec<_> = s.split(":").collect();
        Ok(PasswordAndPolicy { policy: strings[0].parse().unwrap(), password: strings[1].trim().to_string() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn parse_policy() {
        let policy = "1-2 a".parse::<Policy>().unwrap();

        assert_eq!(policy.min, 1);
        assert_eq!(policy.max, 2);
        assert_eq!(policy.character, 'a');
    }

    #[test]
    fn parse_password_and_policy() {
        let pap = PasswordAndPolicy::new("2-7 b: abba");

        assert_eq!(pap.policy.min, 2);
        assert_eq!(pap.policy.max, 7);
        assert_eq!(pap.policy.character, 'b');
        assert_eq!(pap.password, "abba");
    }

    #[test]
    fn matching_password_and_policy_is_valid() {
        let pap = PasswordAndPolicy::new("1-2 b: abba");

        assert_eq!(pap.is_valid(), true);
    }

    #[test]
    fn too_few_chars_password_and_policy_is_not_valid() {
        let pap = PasswordAndPolicy::new("3-4 b: abca");

        assert_eq!(pap.is_valid(), false);
    }

    #[test]
    fn too_many_chars_password_and_policy_is_not_valid() {
        let pap = PasswordAndPolicy::new("2-3 b: abbabb");

        assert_eq!(pap.is_valid(), false);
    }

    #[test]
    fn read_passwords() {
        let pws: Vec<PasswordAndPolicy> = read_file("./src/day02/input.txt").unwrap();
        let valid_count = pws.iter()
            .filter(|p| p.is_valid())
            .count();

        println!("File contains {} valid passwords", valid_count);
    }
}