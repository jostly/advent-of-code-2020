
pub fn find_pair(target: i64, head: i64, tail: &[i64]) -> Option<(i64, i64)> {
    if tail.len() == 0 {
        return None;
    }

    return tail.iter()
        .find(|n| head + **n == target)
        .map(|n| (head, *n))
        .or_else(|| find_pair(target, tail[0], &tail[1..]));
}

pub fn find_2020_in_2(numbers: &[i64]) -> Option<(i64, i64)> {
    return find_pair(2020, numbers[0], &numbers[1..]);
}

pub fn find_2020_in_3(numbers: &[i64]) -> Option<(i64, i64, i64)> {
    if numbers.len() < 3 {
        return None;
    }

    let a = numbers[0];

    if let Some((b, c)) = find_pair(2020 - a, numbers[1], &numbers[2..]) {
        Some((a, b, c))
    } else {
        find_2020_in_3(&numbers[1..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn part_1() {
        let numbers = read_file("./src/day01/input.txt").unwrap();

        let result = find_2020_in_2(&numbers).unwrap();
        println!("Day 1 part 1: {:?}, product: {}", result, result.0 * result.1);
    }

    #[test]
    fn part_2() {
        let numbers = read_file("./src/day01/input.txt").unwrap();

        let result = find_2020_in_3(&numbers).unwrap();
        println!("Day 1 part 2: {:?}, product: {}", result, result.0 * result.1 * result.2);
    }
}