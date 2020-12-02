use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

struct Policy {
    min: u16,
    max: u16,
    letter: char,
}

fn check_password(password: &str, policy: &Policy) -> bool {
    let count = password.chars().filter(|c| *c == policy.letter).count() as u16;

    return (count >= policy.min) && (count <= policy.max);
}

fn check_toboggan_password(password: &str, policy: &Policy) -> bool {
    let pos1 = (policy.min - 1) as usize;
    let pos2 = (policy.max - 1) as usize;
    let char1 = password.chars().nth(pos1).unwrap();
    let char2 = password.chars().nth(pos2).unwrap();

    return (char1 == policy.letter) ^ (char2 == policy.letter);
}

fn parse_entries(input: &str) -> Vec<(Policy, &str)> {
    let entries: Vec<(Policy, &str)> = input.lines().map(parse_entry).collect();
    return entries;
}

fn parse_entry(entry: &str) -> (Policy, &str) {
    lazy_static! {
        static ref ENTRY_RE: Regex = Regex::new(r"(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)").unwrap();
    }

    let captures = ENTRY_RE.captures(entry).unwrap();
    let min: u16 = captures.get(1).unwrap().as_str().parse().unwrap();
    let max: u16 = captures.get(2).unwrap().as_str().parse().unwrap();
    let letter = captures.get(3).unwrap().as_str().chars().next().unwrap();
    let password = captures.get(4).unwrap().as_str();

    let policy = Policy { min, max, letter };

    return (policy, password);
}

fn part1(entries: &[(Policy, &str)]) {
    let valid = entries.iter().filter(|(policy, password)| check_password(password, policy)).count();
    println!("Valid passwords: {}", valid);
}

fn part2(entries: &[(Policy, &str)]) {
    let valid = entries.iter().filter(|(policy, password)| check_toboggan_password(password, policy)).count();
    println!("Valid Toboggan passwords: {}", valid);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let entries = parse_entries(&input);

    part1(&entries);
    part2(&entries);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entry() {
        let entry = "1-3 a: abcde";
        let (policy, password) = parse_entry(entry);

        assert_eq!(policy.min, 1);
        assert_eq!(policy.max, 3);
        assert_eq!(policy.letter, 'a');
        assert_eq!(password, "abcde");
    }

    #[test]
    fn test_parse_entries() {
        let input = "\
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc";
        let entries = parse_entries(input);
        assert_eq!(entries.len(), 3);

        let (policy, password) = &entries[1];
        assert_eq!(policy.min, 1);
        assert_eq!(policy.max, 3);
        assert_eq!(policy.letter, 'b');
        assert_eq!(*password, "cdefg");
    }

    #[test]
    fn test_check_password_1() {
        let policy = Policy {
            min: 1,
            max: 3,
            letter: 'a',
        };
        let password = "abcde";

        assert!(check_password(password, &policy));
    }

    #[test]
    fn test_check_password_2() {
        let policy = Policy {
            min: 1,
            max: 3,
            letter: 'b',
        };
        let password = "cdefg";

        assert!(!check_password(password, &policy));
    }

    #[test]
    fn test_check_toboggan_password_1() {
        let policy = Policy {
            min: 1,
            max: 3,
            letter: 'a',
        };
        let password = "abcde";

        assert!(check_toboggan_password(password, &policy));
    }

    #[test]
    fn test_check_toboggan_password_2() {
        let policy = Policy {
            min: 1,
            max: 3,
            letter: 'b',
        };
        let password = "cdefg";

        assert!(!check_toboggan_password(password, &policy));
    }

    #[test]
    fn test_check_toboggan_password_3() {
        let policy = Policy {
            min: 2,
            max: 9,
            letter: 'c',
        };
        let password = "ccccccccc";

        assert!(!check_toboggan_password(password, &policy));
    }
}
