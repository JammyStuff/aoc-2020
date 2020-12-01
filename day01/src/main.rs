use std::io::{self, Read};

const DESIRED_SUM: u32 = 2020;

fn find_entries(entries: &[u32], sum: u32) -> Option<(u32, u32)> {
    let num_entries = entries.len();

    for i in 0..(num_entries - 1) {
        let start = i + 1;

        for j in start..num_entries {
            let value = entries[i] + entries[j];
            if value == sum {
                return Some((entries[i], entries[j]));
            }
        }
    }

    return None;
}

fn find_3_entries(entries: &[u32], sum: u32) -> Option<(u32, u32, u32)> {
    let num_entries = entries.len();

    for i in 0..(num_entries - 2) {
        let start_j = i + 1;

        for j in start_j..(num_entries - 1) {
            let start_k = j + 1;

            for k in start_k..num_entries {
                let value = entries[i] + entries[j] + entries[k];
                if value == sum {
                    return Some((entries[i], entries[j], entries[k]));
                }
            }
        }
    }

    return None;
}

fn find_value(entries: &[u32]) -> Option<u32> {
    let entries = find_entries(entries, DESIRED_SUM);
    match entries {
        Some((a, b)) => return Some(a * b),
        None => return None,
    }
}

fn find_3_value(entries: &[u32]) -> Option<u32> {
    let entries = find_3_entries(entries, DESIRED_SUM);
    match entries {
        Some((a, b, c)) => return Some(a * b * c),
        None => return None,
    }
}

fn part1(input: &str) {
    let lines = input.lines();
    let entries: Vec<u32> = lines.map(|l| l.parse().unwrap()).collect();

    match find_value(&entries) {
        Some(x) => println!("Part 1 answer: {}", x),
        None => panic!("Couldn't find answer for part 1"),
    }
}

fn part2(input: &str) {
    let lines = input.lines();
    let entries: Vec<u32> = lines.map(|l| l.parse().unwrap()).collect();

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
    fn test_find_entries_1() {
        let entries = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_entries(&entries, 2020);
        assert_eq!(result, Some((1721, 299)));
    }

    #[test]
    fn test_find_entries_2() {
        let entries = vec![100, 200, 300, 500];
        let result = find_entries(&entries, 500);
        assert_eq!(result, Some((200, 300)));
    }

    #[test]
    fn test_find_entries_3() {
        let entries = vec![1, 2, 3, 4, 5];
        let result = find_entries(&entries, 100);
        assert_eq!(result, None);
    }

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
