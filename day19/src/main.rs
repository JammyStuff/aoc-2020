use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Rule {
    Literal(String),
    Concat(Vec<u16>),
    Or((Vec<u16>, Vec<u16>)),
}

fn convert_concat(nums: &[u16], rules: &HashMap<u16, Rule>) -> String {
    nums.iter()
        .map(|&r| convert_rule(r, rules))
        .fold(String::new(), |a, b| a + &b)
}

fn convert_rule(num: u16, rules: &HashMap<u16, Rule>) -> String {
    let rule = rules.get(&num).unwrap();
    match rule {
        Rule::Literal(s) => s.to_owned(),
        Rule::Concat(v) => convert_concat(v, rules),
        Rule::Or((l, r)) => {
            let l_str = convert_concat(l, rules);
            let r_str = convert_concat(r, rules);
            format!("(?:{}|{})", l_str, r_str)
        }
    }
}

fn get_matches<'a>(messages: &[&'a str], rule_str: &str) -> Vec<&'a str> {
    let re_str = format!("^{}$", rule_str);
    let re = Regex::new(&re_str).unwrap();
    messages
        .iter()
        .filter(|m| re.is_match(m))
        .map(|&m| m)
        .collect()
}

fn get_part2_matches<'a>(messages: &[&'a str], rules: &HashMap<u16, Rule>) -> Vec<&'a str> {
    let rule_42_str = convert_rule(42, rules);
    let rule_31_str = convert_rule(31, rules);

    let mut matching = Vec::new();
    for i in 1..100 {
        let count_42 = String::from("{") + &(i + 1).to_string() + ",}";
        let count_31 = String::from("{") + &i.to_string() + "}";
        let re_str = format!("^{}{}{}{}$", rule_42_str, count_42, rule_31_str, count_31);
        let re = Regex::new(&re_str).unwrap();
        for &msg in messages {
            if re.is_match(msg) {
                matching.push(msg);
            }
        }
    }
    return matching;
}

fn parse_input(input: &str) -> (HashMap<u16, Rule>, Vec<&str>) {
    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains(':') {
            let (num, rule) = parse_rule(line);
            rules.insert(num, rule);
        } else {
            messages.push(line);
        }
    }

    return (rules, messages);
}

fn parse_rule(rule_str: &str) -> (u16, Rule) {
    lazy_static! {
        static ref LITERAL_RE: Regex = Regex::new(r#"^(\d+): "([[:alpha:]]+)"$"#).unwrap();
        static ref CONCAT_RE: Regex = Regex::new(r"^(\d+):((?: \d+)+)$").unwrap();
    }

    if let Some(caps) = LITERAL_RE.captures(rule_str) {
        let num = caps.get(1).unwrap().as_str().parse().unwrap();
        let literal = caps.get(2).unwrap().as_str().to_owned();
        return (num, Rule::Literal(literal));
    }

    if let Some(caps) = CONCAT_RE.captures(rule_str) {
        let num = caps.get(1).unwrap().as_str().parse().unwrap();
        let concat = caps.get(2).unwrap().as_str();
        let concats = concat
            .trim()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();
        return (num, Rule::Concat(concats));
    }

    let parts: Vec<_> = rule_str.split(':').collect();
    let num = parts[0].parse().unwrap();
    let or_parts: Vec<_> = parts[1].split('|').collect();
    let ors1 = or_parts[0]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let ors2 = or_parts[1]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    return (num, Rule::Or((ors1, ors2)));
}

fn part1(rules: &HashMap<u16, Rule>, messages: &[&str]) {
    let rule_str = convert_rule(0, rules);
    let matches = get_matches(messages, &rule_str);
    let num = matches.len();
    println!("Number of messages matching rule 0: {}", num);
}

fn part2(rules: &HashMap<u16, Rule>, messages: &[&str]) {
    let matches = get_part2_matches(messages, rules);
    let num = matches.len();
    println!("Number of matches for part 2: {}", num);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (rules, messages) = parse_input(&input);

    part1(&rules, &messages);
    part2(&rules, &messages);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_rules() -> HashMap<u16, Rule> {
        [
            (0, Rule::Concat(vec![4, 1, 5])),
            (1, Rule::Or((vec![2, 3], vec![3, 2]))),
            (2, Rule::Or((vec![4, 4], vec![5, 5]))),
            (3, Rule::Or((vec![4, 5], vec![5, 4]))),
            (4, Rule::Literal("a".to_string())),
            (5, Rule::Literal("b".to_string())),
            (6, Rule::Concat(vec![4, 5])),
        ]
        .iter()
        .cloned()
        .collect()
    }

    #[test]
    fn test_convert_rule_1() {
        let rules = get_rules();
        let expected = "a";
        let actual = convert_rule(4, &rules);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_convert_rule_2() {
        let rules = get_rules();
        let expected = "ab";
        let actual = convert_rule(6, &rules);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_convert_rule_3() {
        let rules = get_rules();
        let expected = "(?:ab|ba)";
        let actual = convert_rule(3, &rules);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_convert_rule_4() {
        let rules = get_rules();
        let expected = "a(?:(?:aa|bb)(?:ab|ba)|(?:ab|ba)(?:aa|bb))b";
        let actual = convert_rule(0, &rules);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_matches() {
        let messages = vec!["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"];
        let rule_str = "a(?:(?:aa|bb)(?:ab|ba)|(?:ab|ba)(?:aa|bb))b";
        let expected = vec!["ababbb", "abbbab"];
        let actual = get_matches(&messages, rule_str);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_rule_1() {
        let rule_str = "39: \"a\"";
        let (num, rule) = parse_rule(rule_str);
        let expected = Rule::Literal("a".to_owned());
        assert_eq!(num, 39);
        assert_eq!(rule, expected);
    }

    #[test]
    fn test_parse_rule_2() {
        let rule_str = "46: 20 20";
        let (num, rule) = parse_rule(rule_str);
        let expected = Rule::Concat(vec![20, 20]);
        assert_eq!(num, 46);
        assert_eq!(rule, expected);
    }

    #[test]
    fn test_parse_rule_3() {
        let rule_str = "108: 39 107 | 20 128";
        let (num, rule) = parse_rule(rule_str);
        let expected = Rule::Or((vec![39, 107], vec![20, 128]));
        assert_eq!(num, 108);
        assert_eq!(rule, expected);
    }
}
