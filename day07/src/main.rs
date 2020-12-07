use std::collections::HashMap;
use std::io::{self, Read};
use lazy_static::lazy_static;
use regex::Regex;

struct Rule<'a> {
    colour: &'a str,
    contains: Vec<(&'a str, u8)>,
}

impl<'a> Rule<'a> {
    fn from_string(rule_str: &'a str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-zA-Z ]+) bags contain (.+).").unwrap();
        }

        let captures = RE.captures(rule_str).unwrap();
        let colour = captures.get(1).unwrap().as_str();
        let sub_bags_str = captures.get(2).unwrap().as_str();
        let sub_bags = Self::parse_sub_bags(sub_bags_str);

        return Rule {
            colour,
            contains: sub_bags,
        };
    }

    fn parse_sub_bags(sub_bags_str: &str) -> Vec<(&str, u8)> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s*(\d+) ([a-zA-Z ]+) bag").unwrap();
        }

        let mut bags = Vec::new();
        if sub_bags_str == "no other bags" {
            return bags;
        }

        for bag_str in sub_bags_str.split(',') {
            let captures = RE.captures(bag_str).unwrap();
            let num: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
            let colour = captures.get(2).unwrap().as_str();
            bags.push((colour, num));
        }

        return bags;
    }

    fn can_contain(&self, colour: &str) -> bool {
        return self.contains.iter().any(|(c, _n)| *c == colour);
    }
}

fn count_bags(colour: &str, rules: &HashMap<&str, Rule>) -> usize {
    let mut total = 0;

    let rule = &rules[colour];
    for (sub_colour, sub_count) in &rule.contains {
        let count = *sub_count as usize;
        total += count;
        total += count * count_bags(sub_colour, rules);
    }

    return total;
}

fn find_paths<'a>(colour: &'a str, rules: &'a HashMap<&str, Rule>) -> Vec<Vec<&'a str>> {
    let mut new_paths = vec![vec![colour]];
    let mut all_paths = vec![];

    while !new_paths.is_empty() {
        new_paths = new_paths.iter().flat_map(|p| {
            let c = p[0];
            let paths: Vec<Vec<&str>> = rules.values().filter(|r| {
                r.can_contain(c)
            }).map(|r| {
                let mut path = vec![r.colour];
                path.append(&mut p.to_owned());
                path
            }).collect();
            paths
        }).collect();
        all_paths.append(&mut new_paths.to_owned());
    }

    return all_paths;
}

fn parse_rules(input: &str) -> HashMap<&str, Rule> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        let rule = Rule::from_string(line);
        rules.insert(rule.colour, rule);
    }

    return rules;
}

fn part1(rules: &HashMap<&str, Rule>) {
    let mut colours: Vec<&str> = find_paths("shiny gold", rules).iter()
        .map(|p| *p.first().unwrap()).collect();
    colours.sort_unstable();
    colours.dedup();
    println!("Colours that can eventually contain shiny gold: {}",
        colours.len());
}

fn part2(rules: &HashMap<&str, Rule>) {
    let bags = count_bags("shiny gold", rules);
    println!("Bags required: {}", bags);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rules = parse_rules(&input);

    part1(&rules);
    part2(&rules);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_rule() -> Rule<'static> {
        return Rule {
            colour: "light red",
            contains: vec![
                ("bright white", 1),
                ("muted yellow", 2),
            ],
        };
    }

    #[test]
    fn test_rule_parse_sub_bags_1() {
        let sub_bags_str = "1 bright white bag, 2 muted yellow bags";
        let sub_bags = Rule::parse_sub_bags(sub_bags_str);
        assert_eq!(sub_bags.len(), 2);
        assert_eq!(sub_bags[0], ("bright white", 1));
        assert_eq!(sub_bags[1], ("muted yellow", 2));
    }

    #[test]
    fn test_rule_parse_sub_bags_2() {
        let sub_bags_str = "no other bags";
        let sub_bags = Rule::parse_sub_bags(sub_bags_str);
        assert!(sub_bags.is_empty());
    }

    #[test]
    fn test_rule_from_string_1() {
        let rule_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let rule = Rule::from_string(rule_str);
        assert_eq!(rule.colour, "light red");
        assert_eq!(rule.contains.len(), 2);
        assert_eq!(rule.contains[0], ("bright white", 1));
        assert_eq!(rule.contains[1], ("muted yellow", 2));
    }

    #[test]
    fn test_rule_from_string_2() {
        let rule_str = "faded blue bags contain no other bags.";
        let rule = Rule::from_string(rule_str);
        assert_eq!(rule.colour, "faded blue");
        assert!(rule.contains.is_empty());
    }

    #[test]
    fn test_parse_rules() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let rules = parse_rules(input);
        assert_eq!(rules.len(), 9);
        assert_eq!(rules["light red"].colour, "light red");
        assert_eq!(rules["light red"].contains.len(), 2);
    }

    #[test]
    fn test_can_contain_1() {
        let rule = create_rule();
        assert!(rule.can_contain("bright white"));
    }

    #[test]
    fn test_can_contain_2() {
        let rule = create_rule();
        assert!(!rule.can_contain("lime green"));
    }

    #[test]
    fn test_find_paths() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let rules = parse_rules(input);
        let colour = "shiny gold";
        let paths = find_paths(colour, &rules);
        let expected: Vec<Vec<&str>> = vec![
            vec!["bright white", "shiny gold"],
            vec!["muted yellow", "shiny gold"],
            vec!["light red", "bright white", "shiny gold"],
            vec!["dark orange", "bright white", "shiny gold"],
            vec!["light red", "muted yellow", "shiny gold"],
            vec!["dark orange", "muted yellow", "shiny gold"],
        ];
        for path in expected {
            assert!(paths.contains(&path));
        }
    }

    #[test]
    fn test_count_bags() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let rules = parse_rules(input);
        let colour = "shiny gold";
        let count = count_bags(colour, &rules);
        assert_eq!(count, 32);
    }
}
