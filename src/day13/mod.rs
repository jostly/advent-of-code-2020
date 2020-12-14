fn find_next_bus(timestamp: u128, buses: &[u128]) -> (u128, u128) {
    let mut min_id = 0;
    let mut min_wait = u128::max_value();

    for bus_id in buses {
        let time_since_departure = timestamp % *bus_id;
        let time_until_next = if time_since_departure == 0 { 0 } else { *bus_id - time_since_departure };
        if time_until_next < min_wait {
            min_wait = time_until_next;
            min_id = *bus_id;
        }
    }

    (min_wait, min_id)
}

pub fn parse_it(s: &str) -> Vec<(u128, u128)> {
    s.split(",")
        .enumerate()
        .filter(|(_, id)| *id != "x")
        .map(|(t, id)| (id.parse().unwrap(), t as u128))
        .collect()
}

pub fn solve_it(parameters: &[(u128, u128)]) -> u128 {
    let mut period = 1;
    let mut time = 0;

    for (a, b) in parameters {
        while ((time + *b) % *a) != 0 {
            time += period;
        }
        period *= *a;
    }

    time
}

fn solve(s: &str) -> u128 {
    solve_it(&parse_it(s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn buses() {
        let timestamp = 939;
        let buses = vec![7, 13, 59, 31, 19];
        let (time, bus_id) = find_next_bus(timestamp, &buses);

        assert_eq!(time, 5);
        assert_eq!(bus_id, 59);
    }

    #[test]
    fn solve_mod() {
        assert_eq!(solve("7,13,x,x,59,x,31,19"), 1068781);
        assert_eq!(solve("17,x,13,19"), 3417);
        assert_eq!(solve("67,7,59,61"), 754018);
    }

    #[test]
    fn part_1() {
        let lines: Vec<String> = read_file("./src/day13/input.txt").unwrap();
        let timestamp = lines[0].parse().unwrap();
        let buses: Vec<_> = lines[1]
            .split(",")
            .filter(|b| *b != "x")
            .map(|b| b.parse().unwrap())
            .collect();

        let (time, bus_id) = find_next_bus(timestamp, &buses);
        println!("{} {} {}", time, bus_id, time * bus_id);
    }

    #[test]
    fn part_2() {
        let lines: Vec<String> = read_file("./src/day13/input.txt").unwrap();
        let p = parse_it(&lines[1]);

        let t = solve_it(&p);
        println!("{}", t);
    }
}