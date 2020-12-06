fn count_answers_in_records<F>(records: Vec<Vec<String>>, initial: u32, join_func: F) -> u32
    where F: Fn(u32, u32) -> u32 {
    let mut sum = 0;

    for ref record in records {
        sum += count_answers(record, initial, &join_func);
    }

    sum
}

fn count_answers<T, F>(answers: &[T], initial: u32, join_func: &F) -> u32
    where T: AsRef<str>,
          F: Fn(u32, u32) -> u32 {

    let bitfield = answers.iter().fold(initial,
                                       |a, b| join_func(a, answer_to_bitfield(b.as_ref())));
    bitfield.count_ones()
}

fn answer_to_bitfield(s: &str) -> u32 {
    let mut bf = 0;
    for c in s.bytes() {
        let i = c - 'a' as u8;
        bf = bf | (1 << i);
    }
    bf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_records;

    #[test]
    fn count_answers_in_test_file_1() {
        let records = read_records("./src/day06/input_test.txt").unwrap();
        let answers = count_answers_in_records(records, 0, |a, b| a | b);
        assert_eq!(answers, 11);
    }

    #[test]
    fn count_answers_in_test_file_2() {
        let records = read_records("./src/day06/input_test.txt").unwrap();
        let answers = count_answers_in_records(records, 0xffffffff, |a, b| a & b);
        assert_eq!(answers, 6);
    }

    #[test]
    fn part_1() {
        let records = read_records("./src/day06/input.txt").unwrap();
        let answers = count_answers_in_records(records, 0, |a, b| a | b);
        println!("Part 1 answer: {}", answers);
    }

    #[test]
    fn part_2() {
        let records = read_records("./src/day06/input.txt").unwrap();
        let answers = count_answers_in_records(records, 0xffffffff, |a, b| a & b);
        println!("Part 2 answer: {}", answers);
    }
}