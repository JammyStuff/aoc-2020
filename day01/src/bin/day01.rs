use day01::find_entries;
use std::io::{self, Read};

const DESIRED_SUM: u64 = 2020;

fn find_3_entries(entries: &[u64], sum: u64) -> Option<(u64, u64, u64)> {
    for i in 0..entries.len() {
        let a = entries[i];
        let rem = sum - a;
        if let Some((b, c)) = find_entries(&entries[i+1..], rem) {
            return Some((a, b, c));
        }
    }

    return None;
}

fn find_value(entries: &[u64]) -> Option<u64> {
    let entries = find_entries(entries, DESIRED_SUM);
    match entries {
        Some((a, b)) => return Some(a * b),
        None => return None,
    }
}

fn find_3_value(entries: &[u64]) -> Option<u64> {
    let entries = find_3_entries(entries, DESIRED_SUM);
    match entries {
        Some((a, b, c)) => return Some(a * b * c),
        None => return None,
    }
}

fn part1(input: &str) {
    let lines = input.lines();
    let entries: Vec<u64> = lines.map(|l| l.parse().unwrap()).collect();

    match find_value(&entries) {
        Some(x) => println!("Part 1 answer: {}", x),
        None => panic!("Couldn't find answer for part 1"),
    }
}

fn part2(input: &str) {
    let lines = input.lines();
    let entries: Vec<u64> = lines.map(|l| l.parse().unwrap()).collect();

    match find_3_value(&entries) {
        Some(x) => println!("Part 2 answer: {}", x),
        None => panic!("Couldn't find answer for part 2"),
    }
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
    fn test_find_3_entries() {
        let entries = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_3_entries(&entries, 2020);
        assert_eq!(result, Some((979, 366, 675)));
    }

    #[test]
    fn test_find_value_1() {
        let entries = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_value(&entries);
        assert_eq!(result, Some(514579));
    }

    #[test]
    fn test_find_value_2() {
        let entries = vec![100, 200, 300, 400];
        let result = find_value(&entries);
        assert_eq!(result, None);
    }
}
