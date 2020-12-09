use crate::day01::find_pair;

struct Validator {
    preamble: usize
}

impl Validator {
    fn new(preamble: usize) -> Self {
        Self { preamble }
    }

    fn check(&self, numbers: &[i64]) -> Option<i64> {
        let n = numbers.len();
        for i in self.preamble..n {
            let number = numbers[i];
            match find_pair(number,
                            numbers[i - self.preamble],
                            &numbers[i - self.preamble + 1..i]) {
                Some(_) => {
                    //println!("{} is made up of {} and {}", number, a, b);
                }
                None => {
                    //println!("No pair found for {}", number);
                    return Some(number);
                }
            }
        }
        return None;
    }
}

pub fn find_contiguous_set_in_list(list: &[i64], target: i64) -> Option<&[i64]> {
    let n = list.len();
    for i in 0..n {
        for j in i + 2..n {
            let sum: i64 = list[i..j].iter().sum();
            if sum == target {
                return Some(&list[i..j]);
            } else if sum > target {
                break;
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn validate_small_example() {
        let validator = Validator::new(5);
        let numbers = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102,
                        117, 150, 182, 127, 219, 299, 277, 309, 576];
        let r = validator.check(
            &numbers
        );
        assert_eq!(r, Some(127));
    }

    #[test]
    fn part_1() {
        let numbers = read_file("./src/day09/input.txt").unwrap();
        let validator = Validator::new(25);
        let r = validator.check(&numbers).unwrap();
        assert_eq!(r, 542529149);
    }

    #[test]
    fn find_set() {
        let numbers = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102,
                           117, 150, 182, 127, 219, 299, 277, 309, 576];
        let r = find_contiguous_set_in_list(&numbers, 127).unwrap();
        println!("{:?}", &r);
    }

    #[test]
    fn part_2() {
        let numbers = read_file("./src/day09/input.txt").unwrap();
        let r = find_contiguous_set_in_list(&numbers, 542529149).unwrap();
        let min = r.iter().min().unwrap();
        let max = r.iter().max().unwrap();
        println!("Magic number: {} (set of {})", min + max, r.len());
    }
}