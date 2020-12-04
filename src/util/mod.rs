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
