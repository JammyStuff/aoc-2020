use std::io::{self, Read};

fn parse_map(input: &str) -> Vec<Vec<bool>> {
    let map: Vec<Vec<bool>> = input.lines().map(parse_row).collect();
    return map;
}

fn parse_row(line: &str) -> Vec<bool> {
    let row: Vec<bool> = line.chars().map(|c| {
        match c {
            '#' => true,
            '.' => false,
            _ => panic!("Invalid square found: {}", c),
        }
    }).collect();
    return row;
}

fn traverse_map(map: &Vec<Vec<bool>>, right: u8, down: u8) -> u32 {
    let mut encountered = 0;
    let mut x = 0;
    let mut y = 0;
    let length = map[0].len();

    while y < map.len() {
        if map[y][x] {
            encountered += 1;
        }

        x += right as usize;
        if x >= length {
            x -= length;
        }
        y += down as usize;
    }

    return encountered;
}

fn part1(map: &Vec<Vec<bool>>) {
    let trees_encountered = traverse_map(map, 3, 1);
    println!("Trees encountered in part 1: {}", trees_encountered);
}

fn part2(map: &Vec<Vec<bool>>) {
    let inputs = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let result: u32 = inputs.iter()
        .map(|(right, down)| traverse_map(map, *right, *down)).product();
    println!("Answer for part 2: {}", result);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = parse_map(&input);

    part1(&map);
    part2(&map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        let line = "..##.......";
        let row = parse_row(&line);
        let expected = vec![
            false,
            false,
            true,
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
        ];

        assert_eq!(row.len(), 11);
        assert_eq!(row, expected);
    }

    #[test]
    fn test_parse_map() {
        let input ="\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let map = parse_map(&input);

        assert_eq!(map.len(), 11);
        for row in &map {
            assert_eq!(row.len(), 11);
        }

        let expected = vec![
            true,
            false,
            false,
            false,
            true,
            false,
            false,
            false,
            true,
            false,
            false
        ];
        assert_eq!(map[1], expected);
    }

    #[test]
    fn test_traverse_map() {
        let input ="\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let map = parse_map(&input);
        let trees_encountered = traverse_map(&map, 3, 1);
        assert_eq!(trees_encountered, 7);
    }
}
