use std::io::{self, Read};

const NUM_COLS: u32 = 8;
const NUM_ROWS: u32 = 128;

struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn from_pass(pass: &str) -> Seat {
        let chars: Vec<_> = pass.chars().collect();
        let row = get_axis(&chars[..7], 0, NUM_ROWS - 1);
        let col = get_axis(&chars[7..], 0, NUM_COLS - 1);
        return Seat {row, col};
    }

    fn id(&self) -> u32 {
        return (self.row * NUM_COLS) + self.col;
    }
}

fn find_missing_seat_id(seats: &[Seat]) -> Option<u32> {
    let mut ids: Vec<_> = seats.iter().map(|s| s.id()).collect();
    ids.sort_unstable();

    let mut first = ids[0];
    for id in ids {
        if (id - first) > 1 {
            return Some(id - 1);
        }
        first = id;
    }
    return None;
}

fn get_axis(seat: &[char], min: u32, max: u32) -> u32 {
    match seat.split_first() {
        None => return min,
        Some((c, rest)) => match c {
            'F' | 'L' => return get_axis(rest, min, min + ((max - min) / 2)),
            'B' | 'R' => return get_axis(rest, min + ((max - min) / 2) + 1, max),
            _ => panic!("Unexpected character: {}", c),
        },
    }
}

fn part1(seats: &[Seat]) {
    let max_id = seats.iter().map(|s| s.id()).max().unwrap();
    println!("Highest seat ID for part 1: {}", max_id);
}

fn part2(seats: &[Seat]) {
    let my_id = find_missing_seat_id(seats).unwrap();
    println!("My seat ID: {}", my_id);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let seats: Vec<_> = input.lines().map(|s| Seat::from_pass(s)).collect();

    part1(&seats);
    part2(&seats);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        let seat = Seat {row: 44, col: 5};
        assert_eq!(seat.id(), 357);
    }

    #[test]
    fn test_get_axis_row() {
        let seat = ['F', 'B', 'F', 'B', 'B', 'F', 'F'];
        let row = get_axis(&seat, 0, 127);
        assert_eq!(row, 44);
    }

    #[test]
    fn test_get_axis_col() {
        let seat = ['R', 'L', 'R'];
        let col = get_axis(&seat, 0, 7);
        assert_eq!(col, 5);
    }

    #[test]
    fn test_seat_from_pass_1() {
        let seat = Seat::from_pass("FBFBBFFRLR");
        assert_eq!(seat.row, 44);
        assert_eq!(seat.col, 5);
    }

    #[test]
    fn test_seat_from_pass_2() {
        let seat = Seat::from_pass("BFFFBBFRRR");
        assert_eq!(seat.row, 70);
        assert_eq!(seat.col, 7);
    }

    #[test]
    fn test_seat_from_pass_3() {
        let seat = Seat::from_pass("FFFBBBFRRR");
        assert_eq!(seat.row, 14);
        assert_eq!(seat.col, 7);
    }

    #[test]
    fn test_seat_from_pass_4() {
        let seat = Seat::from_pass("BBFFBBFRLL");
        assert_eq!(seat.row, 102);
        assert_eq!(seat.col, 4);
    }
}
