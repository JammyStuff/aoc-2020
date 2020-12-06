use std::io::{self, Read};

type GroupParser = fn(&str) -> Vec<char>;

fn parse_group(group: &str) -> Vec<char> {
    let mut answers = group.lines().map(|p| parse_person(p))
        .collect::<Vec<_>>()
        .concat();
    answers.sort();
    answers.dedup();
    return answers;
}

fn parse_group_all(group: &str) -> Vec<char> {
    let people: Vec<_> = group.lines().map(|p| parse_person(p)).collect();

    return people.first().unwrap().iter().filter(|c| {
        people.iter().all(|p| p.contains(c))
    }).map(|c| c.to_owned()).collect()
}

fn parse_input(input: &str, parser: GroupParser) -> Vec<Vec<char>> {
    return input.split("\n\n").map(|g| parser(g)).collect();
}

fn parse_person(person: &str) -> Vec<char> {
    return person.chars().collect();
}

fn part1(input: &str) {
    let groups = parse_input(&input, parse_group);
    let answer: usize = groups.iter().map(|g| g.len()).sum();
    println!("Answer for part 1: {}", answer);
}

fn part2(input: &str) {
    let groups = parse_input(&input, parse_group_all);
    let answer: usize = groups.iter().map(|g| g.len()).sum();
    println!("Answer for part 2: {}", answer);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let expected = vec!['a', 'b', 'c', 'x'];
        let actual = parse_person("abcx");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_group() {
        let group = "abcx
abcy
abcz";
        let expected = vec!['a', 'b', 'c', 'x', 'y', 'z'];
        let actual = parse_group(group);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let groups = parse_input(input, parse_group);

        assert_eq!(groups.len(), 5);
        assert_eq!(groups[2], vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_parse_group_all() {
        let group = "ab
ac";
        let expected = vec!['a'];
        let actual = parse_group_all(group);
        assert_eq!(actual, expected);
    }
}
