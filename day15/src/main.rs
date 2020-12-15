use std::collections::HashMap;
use std::io::{self, Read};

fn parse_start(input: &str) -> Vec<u64> {
    return input.lines().next().unwrap().split(',').map(|n| n.parse().unwrap()).collect();
}

fn play_game(start: &[u64], end: u64) -> u64 {
    let mut say = 0;
    let mut last = None;
    let mut mem: HashMap<u64, Vec<u64>> = HashMap::new();

    for i in 1..=end {
        say = match start.get(i as usize - 1) {
            Some(&num) => num,
            None => {
                match mem.get_mut(&last.unwrap()) {
                    Some(v) => {
                        let p = v.iter().last().unwrap();
                        i - 1 - p
                    },
                    None => 0
                }
            },
        };

        if let Some(num) = last {
            match mem.get_mut(&num) {
                Some(v) => v.push(i - 1),
                None => {
                    let v = vec![i - 1];
                    mem.insert(num, v);
                }
            }
        }

        last = Some(say);
    }

    return say;
}

fn part1(start: &[u64]) {
    let number = play_game(start, 2020);
    println!("2020th number for part 1: {}", number);
}

fn part2(start: &[u64]) {
    let number = play_game(start, 30_000_000);
    println!("30,000,000th number for part 2: {}", number);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let start = parse_start(&input);

    part1(&start);
    part2(&start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_game() {
        let start = vec![1, 3, 2];
        let result = play_game(&start, 2020);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_parse_start() {
        let input = "0,3,6";
        let expected = vec![0, 3, 6];
        let actual = parse_start(input);
        assert_eq!(actual, expected);
    }
}
