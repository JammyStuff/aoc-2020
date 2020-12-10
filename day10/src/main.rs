use std::collections::HashMap;
use std::io::{self, Read};

fn calculate_differences(chain: &[u16]) -> Option<(u16, u16, u16)> {
    let mut counts = (0, 0, 0);

    let mut prev = 0;
    for adapter in chain {
        let diff = adapter - prev;
        match diff {
            1 => counts.0 += 1,
            2 => counts.1 += 1,
            3 => counts.2 += 1,
            _ => return None,
        }
        prev = *adapter;
    }
    counts.2 += 1;

    return Some(counts);
}

fn count_chains(chain: &[u16]) -> u64 {
    let mut cache = HashMap::new();
    return count_chains_rec(chain, 0, &mut cache);
}

fn count_chains_rec(chain: &[u16], prev: u16, cache: &mut HashMap<u16, u64>) -> u64 {
    match cache.get(&prev) {
        Some(&count) => count,
        None => {
            if chain.len() < 2 {
                return 1;
            }
            let mut count = 0;
            for (i, adapter) in chain.iter().take_while(|a| *a - prev <= 3).enumerate() {
                count += count_chains_rec(&chain[i+1..], *adapter, cache);
            }
            cache.insert(prev, count);
            return count;
        },
    }
}

fn parse_adapters(input: &str) -> Vec<u16> {
    let adapters = input.lines().map(|l| l.parse().unwrap()).collect();
    return adapters;
}

fn prepare_chain(adapters: &[u16]) -> Vec<u16> {
    let mut chain = Vec::new();
    chain.extend_from_slice(adapters);
    chain.sort_unstable();
    return chain;
}

fn part1(chain: &[u16]) {
    let diffs = calculate_differences(chain).unwrap();
    let answer = diffs.0 * diffs.2;
    println!("Answer for part 1: {}", answer);
}

fn part2(chain: &[u16]) {
    let chains = count_chains(&chain);
    println!("Answer for part 2: {}", chains);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let adapters = parse_adapters(&input);
    let chain = prepare_chain(&adapters);

    part1(&chain);
    part2(&chain);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_chain() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let expected = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
        let actual = prepare_chain(&adapters);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_differences() {
        let chain = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
        let expected = (7, 0, 5);
        let actual = calculate_differences(&chain).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adapters() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        let expected = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let actual = parse_adapters(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_larger_example_1() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let adapters = parse_adapters(&input);
        let chain = prepare_chain(&adapters);
        let diffs = calculate_differences(&chain).unwrap();
        assert_eq!((22, 0, 10), diffs);
    }

    #[test]
    fn test_count_chains() {
        let chain = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
        let actual = count_chains(&chain);
        assert_eq!(actual, 8);
    }

    #[test]
    fn test_larger_example_2() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let adapters = parse_adapters(&input);
        let chain = prepare_chain(&adapters);
        let actual = count_chains(&chain);
        assert_eq!(actual, 19208);
    }
}
