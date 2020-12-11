use std::collections::HashMap;
use std::hash::Hash;

pub fn differences(input: &[i32]) -> Vec<i32> {
    let mut diffs = Vec::with_capacity(input.len());
    input.iter().fold(0, |a, b| {
        diffs.push(*b - a);
        *b
    });
    diffs
}

pub fn count_entries<T>(input: &[T]) -> HashMap<T, usize>
    where T: Hash + Eq + Copy {
    let mut counts = HashMap::new();

    for i in input {
        counts.entry(*i).and_modify(|n| *n += 1).or_insert(1);
    }

    counts
}

fn jolts(mut input: Vec<i32>) -> (usize, usize) {
    input.sort();
    input.push(input.last().unwrap() + 3);
    let diffs = differences(&input);
    let counts = count_entries(&diffs);
    (counts[&1], counts[&3])
}

fn count_consecutive_ones(diffs: &[i32]) -> Vec<usize> {
    let mut prev = 0;
    let mut acc = 0;
    let mut consec = Vec::new();
    for diff in diffs {
        if *diff == prev && *diff == 1 {
            acc += 1;
        } else {
            if acc > 0 {
                consec.push(acc);
            }
            acc = 0;
        }
        prev = *diff;
    }
    consec
}

fn combinations(consec: usize) -> usize {
    //n(i) = n(i-1) + n(i-2) + 2^(i-3)
    //n(1) = 2
    //n(2) = 4
    match consec {
        1 => 2,
        2 => 4,
        i => combinations(i - 1) + combinations(i - 2) + (1 << (i - 3)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn jolting_difference() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let (ones, threes) = jolts(adapters);
        println!("Small example: Ones: {}, Threes: {}", ones, threes);
    }

    #[test]
    fn larger_example() {
        let adapters = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        let (ones, threes) = jolts(adapters);
        println!("Larger example: Ones: {}, Threes: {}", ones, threes);
    }

    #[test]
    fn part_1() {
        let adapters = read_file("./src/day10/input.txt").unwrap();
        let (ones, threes) = jolts(adapters);
        println!("Part 1: Ones: {}, Threes: {}", ones, threes);
    }

    #[test]
    fn permutations() {
        //let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        // [1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3]
        //        2           1
        let mut adapters = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        // [1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 1, 1, 1, 3, 1, 1, 3, 3, 1, 1, 1, 1, 3, 1, 3, 3, 1, 1, 1, 1, 3]
        //  3              3                 2           1           3                       3
        //n(i) = n(i-1) + n(i-2) + 2^(i-3)
        //n(1) = 2
        //n(2) = 4
        //n(3) = 4 + 2 + 2^0 = 7
        //n(4) = 7 + 4 + 2^1 = 13
        //n(5) = 13 + 7 + 2^2 = 24


        //  1  1  1
        //  1  1  0
        //  1  0  1
        //  1  0  0

        //  0  1  1
        //  0  1  0

        //  0  0  1
        //7 = 4 + 2 + (2/2)

        // 1 1 1 1
        // 1 1 1 0
        // 1 1 0 1
        // 1 1 0 0
        // 1 0 1 1
        // 1 0 1 0
        // 1 0 0 1

        // 0 1 1 1
        // 0 1 1 0
        // 0 1 0 1
        // 0 1 0 0

        // 0 0 1 1
        // 0 0 1 0
        //13 = 7 + 4 + (4/2)

        // 1 1 1 1 1
        // 1 1 1 1 0
        // 1 1 1 0 1
        // 1 1 1 0 0
        // 1 1 0 1 1
        // 1 1 0 1 0
        // 1 1 0 0 1

        // 1 0 1 1 1
        // 1 0 1 1 0
        // 1 0 1 0 1
        // 1 0 1 0 0
        // 1 0 0 1 1
        // 1 0 0 1 0

        // 0 1 1 1 1
        // 0 1 1 1 0
        // 0 1 1 0 1
        // 0 1 1 0 0
        // 0 1 0 1 1
        // 0 1 0 1 0
        // 0 1 0 0 1

        // 0 0 1 1 1
        // 0 0 1 1 0
        // 0 0 1 0 1
        // 0 0 1 0 0
        //24 = 13 + 7 + (8/2)



        adapters.sort();
        adapters.push(adapters.last().unwrap() + 3);

        let diffs = differences(&adapters);
        println!("{:?}", diffs);
        let consec = count_consecutive_ones(&diffs);

        println!("{:?}", consec);

        let combs = consec.iter().map(|n| combinations(*n)).fold(1, |a, b| a * b);
        println!("Total combinations: {}", combs);

    }

    #[test]
    fn test_combinations() {
        assert_eq!(combinations(1), 2);
        assert_eq!(combinations(2), 4);
        assert_eq!(combinations(3), 7);
        assert_eq!(combinations(4), 13);
        assert_eq!(combinations(5), 24);
    }

    #[test]
    fn part_2() {

        let mut adapters = read_file("./src/day10/input.txt").unwrap();
        adapters.sort();
        adapters.push(adapters.last().unwrap() + 3);

        let diffs = differences(&adapters);
        println!("{:?}", diffs);
        let consec = count_consecutive_ones(&diffs);

        println!("{:?}", consec);
        let combs = consec.iter().map(|n| combinations(*n)).fold(1, |a, b| a * b);
        println!("Total combinations: {}", combs);
    }
}