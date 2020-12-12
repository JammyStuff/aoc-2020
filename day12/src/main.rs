use std::io::{self, Read};

type ShipPos = (i32, i32);
type WaypointPos = (i32, i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u16),
    Right(u16),
    Forward(u32),
}

fn add_degrees(first: u16, second: u16) -> u16 {
    let mut degrees = first + second;
    if degrees >= 360 {
        degrees -= 360;
    }
    return degrees;
}

fn calculate_position(ew: i32, ns: i32, dir: u16, mv: Move) -> (i32, i32, u16) {
    match mv {
        Move::North(v) => (ew, ns + (v as i32), dir),
        Move::South(v) => (ew, ns - (v as i32), dir),
        Move::East(v) => (ew + (v as i32), ns, dir),
        Move::West(v) => (ew - (v as i32), ns, dir),
        Move::Left(v) => (ew, ns, sub_degrees(dir, v)),
        Move::Right(v) => (ew, ns, add_degrees(dir, v)),
        Move::Forward(v) => match dir {
            0 => (ew, ns + (v as i32), dir),
            90 => (ew + (v as i32), ns, dir),
            180 => (ew, ns - (v as i32), dir),
            270 => (ew - (v as i32), ns, dir),
            _ => panic!("Unexpected direction: {}", dir),
        },
    }
}

fn calculate_waypoint_positions(
    ship: ShipPos,
    waypoint: WaypointPos,
    mv: Move,
) -> (ShipPos, WaypointPos) {
    match mv {
        Move::North(v) => (ship, (waypoint.0, waypoint.1 + v as i32)),
        Move::South(v) => (ship, (waypoint.0, waypoint.1 - v as i32)),
        Move::East(v) => (ship, (waypoint.0 + v as i32, waypoint.1)),
        Move::West(v) => (ship, (waypoint.0 - v as i32, waypoint.1)),
        Move::Left(v) => (ship, rotate_waypoint(waypoint, -(v as i16))),
        Move::Right(v) => (ship, rotate_waypoint(waypoint, v as i16)),
        Move::Forward(v) => {
            let ship_ew = ship.0 + (waypoint.0 * v as i32);
            let ship_ns = ship.1 + (waypoint.1 * v as i32);
            ((ship_ew, ship_ns), waypoint)
        }
    }
}

fn manhattan_distance(ew: i32, ns: i32) -> u32 {
    let distance = (ew.abs() + ns.abs()) as u32;
    return distance;
}

fn parse_move(move_str: &str) -> Move {
    let action = match move_str.chars().nth(0) {
        Some(c) => c,
        None => panic!("Couldn't parse move"),
    };
    let value: u32 = move_str[1..].parse().unwrap();

    match action {
        'N' => Move::North(value),
        'S' => Move::South(value),
        'E' => Move::East(value),
        'W' => Move::West(value),
        'L' => Move::Left(value as u16),
        'R' => Move::Right(value as u16),
        'F' => Move::Forward(value),
        _ => panic!("Unexpected action: {}", action),
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.lines().map(|l| parse_move(l)).collect()
}

fn perform_moves(moves: &[Move], ew: i32, ns: i32, dir: u16) -> (i32, i32, u16) {
    match moves.first() {
        Some(mv) => {
            let (new_ew, new_ns, new_dir) = calculate_position(ew, ns, dir, *mv);
            return perform_moves(&moves[1..], new_ew, new_ns, new_dir);
        }
        None => return (ew, ns, dir),
    }
}

fn perform_waypoint_moves(
    moves: &[Move],
    ship: ShipPos,
    waypoint: WaypointPos,
) -> (ShipPos, WaypointPos) {
    match moves.first() {
        Some(mv) => {
            let (new_ship, new_waypoint) = calculate_waypoint_positions(ship, waypoint, *mv);
            return perform_waypoint_moves(&moves[1..], new_ship, new_waypoint);
        }
        None => return (ship, waypoint),
    }
}

fn rotate_waypoint(pos: WaypointPos, angle: i16) -> WaypointPos {
    let (ew, ns) = pos;
    let rotation = (angle as f32).to_radians();

    let cos = rotation.cos();
    let sin = rotation.sin();
    let new_ew = (cos * ew as f32) + (sin * ns as f32);
    let new_ns = -(sin * ew as f32) + (cos * ns as f32);
    return (new_ew.round() as i32, new_ns.round() as i32);
}

fn run_navigation(moves: &[Move]) -> (i32, i32) {
    let (ew, ns, _dir) = perform_moves(moves, 0, 0, 90);
    return (ew, ns);
}

fn run_waypoint_navigation(moves: &[Move]) -> ShipPos {
    let (ship, _waypoint) = perform_waypoint_moves(moves, (0, 0), (10, 1));
    return ship;
}

fn sub_degrees(first: u16, second: u16) -> u16 {
    match first.checked_sub(second) {
        Some(v) => v,
        None => (first + 360) - second,
    }
}

fn part1(moves: &[Move]) {
    let (ew, ns) = run_navigation(moves);
    let dist = manhattan_distance(ew, ns);
    println!("Distance for part 1: {}", dist);
}

fn part2(moves: &[Move]) {
    let (ew, ns) = run_waypoint_navigation(moves);
    let dist = manhattan_distance(ew, ns);
    println!("Distance for part 2: {}", dist);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let moves = parse_moves(&input);

    part1(&moves);
    part2(&moves);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_degrees() {
        assert_eq!(add_degrees(270, 180), 90);
    }

    #[test]
    fn test_sub_degrees() {
        assert_eq!(sub_degrees(90, 180), 270);
    }

    #[test]
    fn test_calculate_position_1() {
        let result = calculate_position(10, 0, 90, Move::North(3));
        assert_eq!(result, (10, 3, 90));
    }

    #[test]
    fn test_calculate_position_2() {
        let result = calculate_position(10, 3, 90, Move::Forward(7));
        assert_eq!(result, (17, 3, 90));
    }

    #[test]
    fn test_calculate_position_3() {
        let result = calculate_position(17, 3, 90, Move::Right(90));
        assert_eq!(result, (17, 3, 180));
    }

    #[test]
    fn test_run_navigation() {
        let moves = vec![
            Move::Forward(10),
            Move::North(3),
            Move::Forward(7),
            Move::Right(90),
            Move::Forward(11),
        ];
        assert_eq!(run_navigation(&moves), (17, -8));
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(17, -8), 25);
    }

    #[test]
    fn test_parse_move() {
        let mv = parse_move("R90");
        assert_eq!(mv, Move::Right(90));
    }

    #[test]
    fn test_parse_moves() {
        let input = "F10
N3
F7
R90
F11";
        let expected = vec![
            Move::Forward(10),
            Move::North(3),
            Move::Forward(7),
            Move::Right(90),
            Move::Forward(11),
        ];
        assert_eq!(parse_moves(input), expected);
    }

    #[test]
    fn test_rotate_waypoint() {
        let result = rotate_waypoint((10, 4), 90);
        assert_eq!(result, (4, -10));
    }

    #[test]
    fn test_run_waypoint_navigation() {
        let moves = vec![
            Move::Forward(10),
            Move::North(3),
            Move::Forward(7),
            Move::Right(90),
            Move::Forward(11),
        ];
        assert_eq!(run_waypoint_navigation(&moves), (214, -72));
    }
}
