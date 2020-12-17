use std::collections::HashSet;
use std::io::{self, Read};

// TODO: This should probably be generalized...

type Coordinate = (i32, i32, i32);
type Coordinate4D = (i32, i32, i32, i32);

fn active_after_boot(initial_size: Coordinate, initial_active: &HashSet<Coordinate>) -> usize {
    let active_set = perform_boot(initial_size, initial_active);
    return active_set.len();
}

fn active_after_boot_4d(
    initial_size: Coordinate4D,
    initial_active: &HashSet<Coordinate4D>,
) -> usize {
    let active_set = perform_boot_4d(initial_size, initial_active);
    return active_set.len();
}

fn check_active(coordinate: Coordinate, active_set: &HashSet<Coordinate>) -> bool {
    let mut active_neighbours = 0;
    for x in (coordinate.0 - 1)..=(coordinate.0 + 1) {
        for y in (coordinate.1 - 1)..=(coordinate.1 + 1) {
            for z in (coordinate.2 - 1)..=(coordinate.2 + 1) {
                if (x, y, z) == coordinate {
                    continue;
                }

                if active_set.contains(&(x, y, z)) {
                    active_neighbours += 1;
                }
            }
        }
    }

    if active_set.contains(&coordinate) {
        match active_neighbours {
            2 | 3 => true,
            _ => false,
        }
    } else {
        match active_neighbours {
            3 => true,
            _ => false,
        }
    }
}

fn check_active_4d(coordinate: Coordinate4D, active_set: &HashSet<Coordinate4D>) -> bool {
    let mut active_neighbours = 0;
    for x in (coordinate.0 - 1)..=(coordinate.0 + 1) {
        for y in (coordinate.1 - 1)..=(coordinate.1 + 1) {
            for z in (coordinate.2 - 1)..=(coordinate.2 + 1) {
                for w in (coordinate.3 - 1)..=(coordinate.3 + 1) {
                    if (x, y, z, w) == coordinate {
                        continue;
                    }

                    if active_set.contains(&(x, y, z, w)) {
                        active_neighbours += 1;
                    }
                }
            }
        }
    }

    if active_set.contains(&coordinate) {
        match active_neighbours {
            2 | 3 => true,
            _ => false,
        }
    } else {
        match active_neighbours {
            3 => true,
            _ => false,
        }
    }
}

fn parse_input(input: &str) -> (Coordinate, HashSet<Coordinate>) {
    let mut active_set = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        max_y = y as i32;
        for (x, c) in line.chars().enumerate() {
            max_x = x as i32;
            match c {
                '#' => active_set.insert((x as i32, y as i32, 0)),
                '.' => continue,
                _ => panic!("Unexpected input character: {}", c),
            };
        }
    }

    let initial_size = (max_x + 1, max_y + 1, 1);
    return (initial_size, active_set);
}

fn parse_input_4d(input: &str) -> (Coordinate4D, HashSet<Coordinate4D>) {
    let mut active_set = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        max_y = y as i32;
        for (x, c) in line.chars().enumerate() {
            max_x = x as i32;
            match c {
                '#' => active_set.insert((x as i32, y as i32, 0, 0)),
                '.' => continue,
                _ => panic!("Unexpected input character: {}", c),
            };
        }
    }

    let initial_size = (max_x + 1, max_y + 1, 1, 1);
    return (initial_size, active_set);
}

fn perform_boot(
    initial_size: Coordinate,
    initial_active: &HashSet<Coordinate>,
) -> HashSet<Coordinate> {
    let mut active_set = initial_active.clone();
    for cycle in 1..=6 {
        active_set = perform_cycle(cycle, initial_size, active_set);
    }
    return active_set;
}

fn perform_boot_4d(
    initial_size: Coordinate4D,
    initial_active: &HashSet<Coordinate4D>,
) -> HashSet<Coordinate4D> {
    let mut active_set = initial_active.clone();
    for cycle in 1..=6 {
        active_set = perform_cycle_4d(cycle, initial_size, active_set);
    }
    return active_set;
}

fn perform_cycle(
    cycle_num: u8,
    initial_size: Coordinate,
    active_set: HashSet<Coordinate>,
) -> HashSet<Coordinate> {
    let mut new_active = HashSet::new();
    let num = cycle_num as i32;

    for x in (0 - num)..(initial_size.0 + num) {
        for y in (0 - num)..(initial_size.1 + num) {
            for z in (0 - num)..(initial_size.2 + num) {
                if check_active((x, y, z), &active_set) {
                    new_active.insert((x, y, z));
                }
            }
        }
    }

    return new_active;
}

fn perform_cycle_4d(
    cycle_num: u8,
    initial_size: Coordinate4D,
    active_set: HashSet<Coordinate4D>,
) -> HashSet<Coordinate4D> {
    let mut new_active = HashSet::new();
    let num = cycle_num as i32;

    for x in (0 - num)..(initial_size.0 + num) {
        for y in (0 - num)..(initial_size.1 + num) {
            for z in (0 - num)..(initial_size.2 + num) {
                for w in (0 - num)..(initial_size.3 + num) {
                    if check_active_4d((x, y, z, w), &active_set) {
                        new_active.insert((x, y, z, w));
                    }
                }
            }
        }
    }

    return new_active;
}

fn part1(initial_size: Coordinate, initial_active: &HashSet<Coordinate>) {
    let num_active = active_after_boot(initial_size, initial_active);
    println!("Number of active cubes for part 1: {}", num_active);
}

fn part2(initial_size: Coordinate4D, initial_active: &HashSet<Coordinate4D>) {
    let num_active = active_after_boot_4d(initial_size, initial_active);
    println!("Number of active cubes for part 2: {}", num_active);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (initial_size, initial_active) = parse_input(&input);
    let (initial_size_4d, initial_active_4d) = parse_input_4d(&input);

    part1(initial_size, &initial_active);
    part2(initial_size_4d, &initial_active_4d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_after_boot() {
        let initial_size = (3, 3, 1);
        let initial_active: HashSet<Coordinate> =
            [(1, 0, 0), (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0)]
                .iter()
                .cloned()
                .collect();
        let num_active = active_after_boot(initial_size, &initial_active);
        assert_eq!(num_active, 112);
    }

    #[test]
    fn test_parse_input() {
        let input = ".#.
..#
###";
        let expected_size = (3, 3, 1);
        let expected_active: HashSet<Coordinate> =
            [(1, 0, 0), (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0)]
                .iter()
                .cloned()
                .collect();
        let (size, active) = parse_input(input);
        assert_eq!(size, expected_size);
        assert_eq!(active, expected_active);
    }
}
