use std::collections::HashSet;
use std::fmt::Display;
use std::io::{self, Read};
use std::iter::Sum;
use std::ops::RangeInclusive;

enum ParseState {
    Rules,
    YourTicket,
    NearbyTickets,
}

#[derive(Debug, PartialEq)]
struct Rule<T: Copy + PartialOrd> {
    field: String,
    ranges: Vec<RangeInclusive<T>>,
}

impl<T: Copy + PartialOrd> Rule<T> {
    fn new(field_name: &str, bounds: &[(T, T)]) -> Self {
        let field = field_name.to_string();
        let ranges = bounds
            .iter()
            .map(|&(low, high)| RangeInclusive::new(low, high))
            .collect();
        return Rule { field, ranges };
    }

    fn valid(&self, value: T) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

fn calculate_error_rate<T: Copy + PartialOrd + Sum>(tickets: &[&[T]], rules: &[Rule<T>]) -> T {
    return tickets
        .iter()
        .map(|&ticket| calculate_ticket_error_rate(ticket, rules))
        .sum();
}

fn calculate_ticket_error_rate<T: Copy + PartialOrd + Sum>(ticket: &[T], rules: &[Rule<T>]) -> T {
    check_valid_fields(ticket, rules)
        .iter()
        .filter_map(|&(value, valid)| if valid { None } else { Some(value) })
        .sum()
}

fn check_valid_fields<T: Copy + PartialOrd>(fields: &[T], rules: &[Rule<T>]) -> Vec<(T, bool)> {
    fields
        .iter()
        .map(|&value| {
            let valid = rules.iter().any(|r| r.valid(value));
            (value, valid)
        })
        .collect()
}

fn determine_fields<'a, T: Copy + PartialOrd>(
    tickets: &[&[T]],
    rules: &'a [Rule<T>],
) -> Vec<(usize, &'a Rule<T>)> {
    let mut rule_candidates: Vec<(&Rule<T>, HashSet<usize>)> = rules
        .iter()
        .map(|r| (r, get_candidate_positions(r, tickets)))
        .collect();
    rule_candidates.sort_by_key(|(_, ps)| ps.len());

    let mut positions = Vec::new();
    for rule_candidate in rule_candidates {
        let rule = rule_candidate.0;
        let candidates = rule_candidate.1;
        let position = candidates
            .iter()
            .filter(|c| !positions.iter().any(|(p, _)| p == *c))
            .nth(0)
            .unwrap();
        positions.push((*position, rule));
    }

    return positions;
}

fn get_candidate_positions<T: Copy + PartialOrd>(
    rule: &Rule<T>,
    tickets: &[&[T]],
) -> HashSet<usize> {
    let mut candidates = HashSet::new();
    for i in 0..tickets[0].len() {
        candidates.insert(i);
    }

    for ticket in tickets {
        for (i, &field) in ticket.iter().enumerate() {
            if !rule.valid(field) {
                candidates.remove(&i);
            }
        }
    }

    candidates
}

fn parse_input(input: &str) -> (Vec<Rule<u32>>, Vec<u32>, Vec<Vec<u32>>) {
    let mut state = ParseState::Rules;
    let mut rules = Vec::new();
    let mut your = Vec::new();
    let mut nearby = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line == "your ticket:" {
            state = ParseState::YourTicket;
            continue;
        }

        if line == "nearby tickets:" {
            state = ParseState::NearbyTickets;
            continue;
        }

        match state {
            ParseState::Rules => rules.push(parse_rule(line)),
            ParseState::YourTicket => your = parse_ticket(line),
            ParseState::NearbyTickets => nearby.push(parse_ticket(line)),
        }
    }

    (rules, your, nearby)
}

fn parse_rule(line: &str) -> Rule<u32> {
    let parts: Vec<_> = line.split(':').collect();
    let field = parts[0];
    let ranges: Vec<_> = parts[1]
        .split("or")
        .map(|p| {
            let r = p.trim();
            let ranges: Vec<_> = r.split('-').collect();
            let low = ranges[0].parse().unwrap();
            let high = ranges[1].parse().unwrap();
            (low, high)
        })
        .collect();
    Rule::new(field, &ranges)
}

fn parse_ticket(line: &str) -> Vec<u32> {
    line.split(',').map(|n| n.parse().unwrap()).collect()
}

fn part1<T: Copy + PartialOrd + Sum + Display>(rules: &[Rule<T>], nearby: &[&[T]]) {
    let rate = calculate_error_rate(nearby, rules);
    println!("Ticket scanning error rate: {}", rate);
}

fn part2(rules: &[Rule<u32>], your: &[u32], nearby: &[&[u32]]) {
    let valid_nearby: Vec<_> = nearby
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|&field| rules.iter().any(|r| r.valid(field)))
        })
        .map(|&t| t)
        .collect();
    let fields = determine_fields(&valid_nearby, rules);
    let departure_fields = fields
        .iter()
        .filter(|(_, rule)| rule.field.starts_with("departure"));
    let answer: u64 = departure_fields.map(|(idx, _)| your[*idx] as u64).product();
    println!("Answer for part 2: {}", answer);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (rules, your, nearby) = parse_input(&input);
    let nearby_slice: Vec<_> = nearby.iter().map(|t| &t[..]).collect();

    part1(&rules, &nearby_slice);
    part2(&rules, &your, &nearby_slice);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_rules() -> Vec<Rule<u16>> {
        let rule1 = Rule::new("class", &vec![(1, 3), (5, 7)]);
        let rule2 = Rule::new("row", &vec![(6, 11), (33, 44)]);
        let rule3 = Rule::new("seat", &vec![(13, 40), (45, 50)]);
        vec![rule1, rule2, rule3]
    }

    #[test]
    fn test_rule_new() {
        let bounds = vec![(1, 3), (5, 7)];
        let rule = Rule::new("class", &bounds);
        let expected_ranges = vec![RangeInclusive::new(1, 3), RangeInclusive::new(5, 7)];
        assert_eq!(rule.field, "class");
        assert_eq!(rule.ranges, expected_ranges);
    }

    #[test]
    fn test_rule_valid() {
        let bounds = vec![(1, 3), (5, 7)];
        let rule = Rule::new("class", &bounds);
        assert!(rule.valid(3));
        assert!(!rule.valid(4));
    }

    #[test]
    fn test_check_valid_fields() {
        let rules = get_rules();
        let ticket = vec![40, 4, 50];
        let expected = vec![(40, true), (4, false), (50, true)];
        let actual = check_valid_fields(&ticket, &rules);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_error_rate() {
        let rules = get_rules();
        let ticket1 = vec![7, 3, 47];
        let ticket2 = vec![40, 4, 50];
        let ticket3 = vec![55, 2, 20];
        let ticket4 = vec![38, 6, 12];
        let tickets = vec![&ticket1[..], &ticket2[..], &ticket3[..], &ticket4[..]];
        assert_eq!(calculate_error_rate(&tickets, &rules), 71);
    }

    #[test]
    fn test_parse_rule() {
        let input = "class: 1-3 or 5-7";
        let expected = Rule::new("class", &vec![(1, 3), (5, 7)]);
        let actual = parse_rule(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_ticket() {
        let input = "7,1,14";
        let expected = vec![7, 1, 14];
        let actual = parse_ticket(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let (rules, your, nearby) = parse_input(input);
        assert_eq!(rules.len(), 3);
        assert_eq!(your, vec![7, 1, 14]);
        assert_eq!(nearby.len(), 4);
    }

    #[test]
    fn test_determine_fields() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let (rules, _your, nearby) = parse_input(input);
        let nearby_slice: Vec<_> = nearby.iter().map(|t| &t[..]).collect();
        let expected = vec![(2, &rules[2]), (1, &rules[0]), (0, &rules[1])];
        let actual = determine_fields(&nearby_slice, &rules);
        assert_eq!(actual, expected);
    }
}
