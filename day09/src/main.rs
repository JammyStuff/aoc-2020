use std::io::{self, Read};
use day01::find_entries;

const PREV_ENTRIES: u8 = 25;

fn find_contiguous_sum(entries: &[u64], num: u64) -> Option<&[u64]> {
    for n in 2..entries.len() {
        for i in n..entries.len() {
            let set = &entries[i-n..i];
            let sum: u64 = set.iter().sum();
            if sum == num {
                return Some(set);
            }
        }
    }
    return None;
}

fn find_number(entries: &[u64], prev_n: u8) -> Option<u64> {
    let n = prev_n as usize;
    for i in n..entries.len() {
        let number = entries[i];
        match find_entries(&entries[i-n..i], number) {
            Some(_) => (),
            None => return Some(number),
        }
    }
    return None;
}

fn parse_entries(input: &str) -> Vec<u64> {
    let entries: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    return entries;
}

fn part1(entries: &[u64]) -> u64 {
    let number = find_number(entries, PREV_ENTRIES).unwrap();
    println!("Number for part 1: {}", number);
    return number;
}

fn part2(entries: &[u64], number: u64) {
    let set = find_contiguous_sum(entries, number).unwrap();
    let smallest = set.iter().min().unwrap();
    let largest = set.iter().max().unwrap();
    let result = smallest + largest;
    println!("Number for part 2: {}", result);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let entries = parse_entries(&input);

    let number = part1(&entries);
    part2(&entries, number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number() {
        let entries = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117,
            150, 182, 127, 219, 299, 277, 309, 576];
        let number = find_number(&entries, 5);
        assert_eq!(number, Some(127));
    }

    #[test]
    fn test_parse_entries() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let actual = parse_entries(input);
        let expected = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117,
            150, 182, 127, 219, 299, 277, 309, 576];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_contiguous_sum() {
        let entries = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117,
            150, 182, 127, 219, 299, 277, 309, 576];
        let actual = find_contiguous_sum(&entries, 127).unwrap();
        let expected = vec![15, 25, 47, 40];
        assert_eq!(actual, expected);
    }
}
