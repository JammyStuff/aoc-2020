use std::cmp;
use std::io::{self, Read};

type CountFn = fn(&Layout, usize, usize) -> u8;
type Layout = Vec<Vec<Option<Seat>>>;
type Range = (usize, usize);
type Seat = bool;

#[derive(Clone, Copy)]
enum Direction {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

const DIRECTIONS: [Direction; 8] = [
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
];

struct PosIter {
    row: usize,
    col: usize,
    width: usize,
    height: usize,
    dir: Direction,
}

impl Iterator for PosIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let up = self.row.checked_sub(1);
        let left = self.col.checked_sub(1);
        let down_can = self.row + 1;
        let down = if down_can < self.height {
            Some(down_can)
        } else {
            None
        };
        let right_can = self.col + 1;
        let right = if right_can < self.width {
            Some(right_can)
        } else {
            None
        };

        match &self.dir {
            Direction::UpLeft => {
                if up.is_some() && left.is_some() {
                    self.row = up.unwrap();
                    self.col = left.unwrap();
                } else {
                    return None;
                }
            }
            Direction::Up => {
                if up.is_some() {
                    self.row = up.unwrap();
                } else {
                    return None;
                }
            }
            Direction::UpRight => {
                if up.is_some() && right.is_some() {
                    self.row = up.unwrap();
                    self.col = right.unwrap();
                } else {
                    return None;
                }
            }
            Direction::Right => {
                if right.is_some() {
                    self.col = right.unwrap();
                } else {
                    return None;
                }
            }
            Direction::DownRight => {
                if down.is_some() && right.is_some() {
                    self.row = down.unwrap();
                    self.col = right.unwrap();
                } else {
                    return None;
                }
            }
            Direction::Down => {
                if down.is_some() {
                    self.row = down.unwrap();
                } else {
                    return None;
                }
            }
            Direction::DownLeft => {
                if down.is_some() && left.is_some() {
                    self.row = down.unwrap();
                    self.col = left.unwrap();
                } else {
                    return None;
                }
            }
            Direction::Left => {
                if left.is_some() {
                    self.col = left.unwrap();
                } else {
                    return None;
                }
            }
        }

        Some((self.row, self.col))
    }
}

fn calculate_seating(layout: &Layout, f: CountFn, allowance: u8) -> Layout {
    let mut seating = layout.to_owned();

    loop {
        let changes = changes_for_round(&seating, f, allowance);
        if changes.is_empty() {
            break;
        }

        for (row, col) in changes {
            let current = seating[row][col].unwrap();
            seating[row][col] = Some(!current);
        }
    }

    return seating;
}

fn changes_for_round(layout: &Layout, f: CountFn, allowance: u8) -> Vec<(usize, usize)> {
    let mut changes = Vec::new();

    let rows = layout.len();
    let cols = layout[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if seat_should_change(layout, row, col, f, allowance) {
                changes.push((row, col));
            }
        }
    }

    return changes;
}

fn count_adjacent(layout: &Layout, row: usize, col: usize) -> u8 {
    let ((min_row, max_row), (min_col, max_col)) = get_ranges(layout, row, col);
    let mut count = 0;
    for row in &layout[min_row..max_row] {
        for seat in &row[min_col..max_col] {
            if let Some(true) = seat {
                count += 1;
            }
        }
    }
    if let Some(true) = layout[row][col] {
        count -= 1;
    }
    return count;
}

fn count_nearest(layout: &Layout, row: usize, col: usize) -> u8 {
    let mut count = 0;
    let height = layout.len();
    let width = layout[0].len();

    for dir in &DIRECTIONS {
        let iter = PosIter {
            row,
            col,
            width,
            height,
            dir: *dir,
        };
        for (r, c) in iter {
            match layout[r][c] {
                Some(true) => {
                    count += 1;
                    break;
                }
                Some(false) => break,
                None => (),
            }
        }
    }

    return count;
}

fn get_ranges(layout: &Layout, row: usize, col: usize) -> (Range, Range) {
    let min_row = match row.checked_sub(1) {
        Some(v) => v,
        None => 0,
    };
    let max_row = cmp::min(row + 2, layout.len());
    let min_col = match col.checked_sub(1) {
        Some(v) => v,
        None => 0,
    };
    let max_col = cmp::min(col + 2, layout[row].len());
    return ((min_row, max_row), (min_col, max_col));
}

fn parse_layout(input: &str) -> Layout {
    let layout = input.lines().map(|l| parse_row(l)).collect();
    return layout;
}

fn parse_row(line: &str) -> Vec<Option<Seat>> {
    let row = line
        .chars()
        .map(|c| match c {
            '.' => None,
            'L' => Some(false),
            '#' => Some(true),
            _ => panic!("Unexpected character: {}", c),
        })
        .collect();
    return row;
}

fn seat_should_change(layout: &Layout, row: usize, col: usize, f: CountFn, allowance: u8) -> bool {
    let adjacent = f(layout, row, col);
    match layout[row][col] {
        Some(false) => adjacent == 0,
        Some(true) => adjacent >= allowance,
        None => false,
    }
}

fn part1(layout: &Layout) {
    let seating = calculate_seating(layout, count_adjacent, 4);
    let occupied: usize = seating
        .iter()
        .map(|r| r.iter().filter(|s| s.unwrap_or_default()).count())
        .sum();
    println!("Seats occupied in part 1: {}", occupied);
}

fn part2(layout: &Layout) {
    let seating = calculate_seating(layout, count_nearest, 5);
    let occupied: usize = seating
        .iter()
        .map(|r| r.iter().filter(|s| s.unwrap_or_default()).count())
        .sum();
    println!("Seats occupied in part 2: {}", occupied);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let layout = parse_layout(&input);

    part1(&layout);
    part2(&layout);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_layout() -> Layout {
        let input = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";
        let layout = parse_layout(input);
        return layout;
    }

    #[test]
    fn test_parse_row() {
        let line = "#.LL.L#.##";
        let expected = vec![
            Some(true),
            None,
            Some(false),
            Some(false),
            None,
            Some(false),
            Some(true),
            None,
            Some(true),
            Some(true),
        ];
        let actual = parse_row(line);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_layout() {
        let input = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";
        let expected_row = vec![
            Some(true),
            None,
            Some(false),
            Some(false),
            None,
            Some(false),
            Some(true),
            None,
            Some(true),
            Some(true),
        ];
        let layout = parse_layout(input);
        assert_eq!(layout.len(), 10);
        assert_eq!(layout[0], expected_row);
    }

    #[test]
    fn test_get_ranges() {
        let layout = get_layout();
        assert_eq!(get_ranges(&layout, 3, 6), ((2, 5), (5, 8)));
        assert_eq!(get_ranges(&layout, 0, 0), ((0, 2), (0, 2)));
        assert_eq!(get_ranges(&layout, 9, 9), ((8, 10), (8, 10)));
    }

    #[test]
    fn test_count_adjacent() {
        let layout = get_layout();
        assert_eq!(count_adjacent(&layout, 5, 5), 1);
    }

    #[test]
    fn test_seat_should_change_1() {
        let input = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
        let layout = parse_layout(input);
        assert!(seat_should_change(&layout, 1, 1, count_adjacent, 4));
        assert!(!seat_should_change(&layout, 1, 0, count_adjacent, 4));
    }

    #[test]
    fn test_seat_should_change_2() {
        let input = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";
        let layout = parse_layout(input);
        assert!(seat_should_change(&layout, 1, 2, count_adjacent, 4));
    }

    #[test]
    fn test_changes_for_round() {
        let input = "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##";
        let layout = parse_layout(input);
        let expected = vec![(2, 2), (3, 2), (3, 3), (4, 2), (5, 2), (7, 4), (7, 5)];
        let actual = changes_for_round(&layout, count_adjacent, 4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_seating() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let layout = parse_layout(input);
        let seating = calculate_seating(&layout, count_adjacent, 4);
        let expected = vec![
            Some(false),
            None,
            Some(true),
            None,
            Some(false),
            None,
            None,
            Some(true),
            None,
            None,
        ];
        assert_eq!(seating[2], expected);
    }
}
