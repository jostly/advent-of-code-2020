use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashSet;
use std::hash::Hash;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_records<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Vec<String>>> {
    let lines = read_lines(filename)?;
    let mut out = Vec::new();
    let mut record = Vec::new();

    for try_line in lines {
        let line = try_line?;
        if line.is_empty() {
            out.push(record);
            record = Vec::new();
        } else {
            record.push(line);
        }
    }

    if !record.is_empty() {
        out.push(record);
    }

    Ok(out)
}

pub fn read_file<T, P>(filename: P) -> Result<Vec<T>, T::Err>
    where P: AsRef<Path>,
          T: FromStr {
    let lines = read_lines(filename).unwrap();
    lines
        .map(|line| line.unwrap().parse::<T>())
        .collect()
}

pub fn hash_set<T: Eq + Hash>(v: Vec<T>) -> HashSet<T> {
    v.into_iter().collect()
}
