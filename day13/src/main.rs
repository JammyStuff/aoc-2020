use std::io::{self, Read};

fn calculate_next_arrivals(bus_ids: &[u32], earliest: u32) -> Vec<(u32, u32)> {
    let arrivals = bus_ids.iter().map(|id| {
        let next = ((earliest / id) + 1) * id;
        (*id, next)
    }).collect();
    return arrivals;
}

fn find_earliest_arrival(arrivals: &[(u32, u32)]) -> (u32, u32) {
    let earliest = arrivals.iter().min_by_key(|a| a.1).unwrap().to_owned();
    return earliest;
}

fn find_earliest_timestamp(constraints: &[Option<u64>]) -> i128 {
    let active: Vec<_> = constraints.iter().enumerate().filter_map(|(i, &c)| {
        match c {
            Some(id) => Some((i as i128, id as i128)),
            None => None,
        }
    }).collect();
    let n: i128 = active.iter().map(|&(_, id)| id).product();
    let rem: i128 = active.iter().map(|&(i, id)| {
        let y = n / id;
        i * modinv(y, id).unwrap() * y
    }).sum();
    return n - (rem % n);
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    } else {
        let (g, x, y) = egcd(b % a, a);
        return (g, y - ((b / a) * x), x);
    }
}

fn modinv(a: i128, m: i128) -> Option<i128> {
    let (gcd, x, _y) = egcd(a, m);
    if gcd != 1 {
        return None;
    } else {
        return Some((x % m + m) % m);
    }
}

fn parse_constraints(input: &str) -> Vec<Option<u64>> {
    let constraints_str = input.lines().nth(1).unwrap();
    let constraints = constraints_str.split(',').map(|c| c.parse().ok()).collect();
    return constraints;
}

fn parse_input(input: &str) -> (u32, Vec<u32>) {
    let earliest_str = input.lines().nth(0).unwrap();
    let bus_ids_str = input.lines().nth(1).unwrap();
    let earliest = earliest_str.parse().unwrap();
    let bus_ids = bus_ids_str.split(',').filter_map(|id_str| {
        id_str.parse().ok()
    }).collect();
    return (earliest, bus_ids);
}

fn part1(bus_ids: &[u32], earliest: u32) {
    let arrivals = calculate_next_arrivals(bus_ids, earliest);
    let (bus_id, next_arrival) = find_earliest_arrival(&arrivals);
    let wait = next_arrival - earliest;
    let answer = bus_id * wait;
    println!("Answer for part 1: {}", answer);
}

fn part2(constraints: &[Option<u64>]) {
    let timestamp = find_earliest_timestamp(constraints);
    println!("Earliest timestamp for part 2: {}", timestamp);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (earliest, bus_ids) = parse_input(&input);
    let constraints = parse_constraints(&input);

    part1(&bus_ids, earliest);
    part2(&constraints);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_next_arrivals() {
        let bus_ids = vec![7, 13, 59];
        let expected = vec![(7, 945), (13, 949), (59, 944)];
        let actual = calculate_next_arrivals(&bus_ids, 939);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_earliest_arrival() {
        let arrivals = vec![(7, 945), (13, 949), (59, 944)];
        let earliest = find_earliest_arrival(&arrivals);
        assert_eq!(earliest, (59, 944));
    }

    #[test]
    fn test_parse_input() {
        let input = "939
7,13,x,x,59,x,31,19";
        let expected = (939, vec![7, 13, 59, 31, 19]);
        let actual = parse_input(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_earliest_timestamp_1() {
        let constraints = vec![Some(17), None, Some(13), Some(19)];
        let earliest = find_earliest_timestamp(&constraints);
        assert_eq!(earliest, 3417);
    }

    #[test]
    fn test_find_earliest_timestamp_2() {
        let constraints = vec![Some(1789), Some(37), Some(47), Some(1889)];
        let earliest = find_earliest_timestamp(&constraints);
        assert_eq!(earliest, 1_202_161_486);
    }

    #[test]
    fn test_parse_constraints() {
        let input = "939
7,13,x,x,59,x,31,19";
        let expected = vec![Some(7), Some(13), None, None, Some(59), None, Some(31), Some(19)];
        let actual = parse_constraints(input);
        assert_eq!(actual, expected);
    }
}
