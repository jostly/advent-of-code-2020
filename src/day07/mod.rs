use regex::Regex;
use std::str::FromStr;
use lazy_static::lazy_static;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Bag(String);

impl Bag {
    fn new(s: &str) -> Self {
        let captures = BAG_REGEX.captures(s).unwrap();
        Bag(captures[1].to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BagManifest(Vec<(u32, Bag)>);

impl BagManifest {
    fn new(s: &str) -> Self {
        if s == "no other bags" {
            return BagManifest(Vec::new());
        }
        let mut v = Vec::new();
        let parts = s.split(", ");
        for p in parts {
            let captures = MANIFEST_REGEX.captures(p).unwrap();
            let n = captures[1].parse().unwrap();
            v.push((n, Bag::new(&captures[2])));
        }

        BagManifest(v)
    }
}

#[derive(Debug)]
struct Line(Bag, BagManifest);


lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"^(.+) contain (.+)\.$").unwrap();
    static ref BAG_REGEX: Regex = Regex::new(r"^(.+) bags?$").unwrap();
    static ref MANIFEST_REGEX: Regex = Regex::new(r"^(\d+) (.+)$").unwrap();

}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = LINE_REGEX.captures(s).unwrap();
        let bag = Bag::new(&captures[1]);
        let manifest = BagManifest::new(&captures[2]);
        Ok(Line(bag, manifest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn parse_bag() {
        assert_eq!(Bag::new("light red bags"), Bag("light red".to_string()));
        assert_eq!(Bag::new("shiny gold bag"), Bag("shiny gold".to_string()));
    }

    #[test]
    fn parse_bag_manifest() {
        assert_eq!(BagManifest::new("no other bags"), BagManifest(Vec::new()));
        assert_eq!(BagManifest::new("1 shiny gold bag"), BagManifest(vec![(1, Bag::new("shiny gold bag"))]));
        assert_eq!(BagManifest::new("2 shiny gold bags, 9 faded blue bags"), BagManifest(vec![
            (2, Bag::new("shiny gold bags")),
            (9, Bag::new("faded blue bags")),
        ]));
    }

    #[test]
    fn part_1() {
        let lines: Vec<Line> = read_file("./src/day07/input.txt").unwrap();

        let nodes: HashMap<Bag, BagManifest> = lines.iter()
            .map(|line| (line.0.clone(), line.1.clone()))
            .collect();

        let mut contained_in: HashMap<Bag, Vec<Bag>> = HashMap::with_capacity(nodes.len());

        for (ref in_bag, ref manifest) in nodes {
            for (_, ref bag) in &manifest.0 {
                contained_in.entry(bag.clone()).or_insert_with(|| Vec::new()).push(in_bag.clone());
            }
        }

        let mut containers = HashSet::new();
        let mut candidates = vec![Bag::new("shiny gold bag")];

        while !candidates.is_empty() {
            let next_bag = candidates.pop().unwrap();
            if let Some(c) = contained_in.get(&next_bag) {
                for b in c {
                    if !containers.contains(b) {
                        containers.insert(b.clone());
                        candidates.push(b.clone());
                    }
                }
            }
        }

        assert_eq!(containers.len(), 101);
    }

    fn count_contained_bags(nodes: &HashMap<Bag, BagManifest>, bag: &Bag) -> u32 {
        if let Some(contained) = nodes.get(bag) {
            let sum = contained.0.iter()
                .map(|(n, b)| n * (1+count_contained_bags(nodes, b)))
                .sum();
            sum
        } else {
            0
        }
    }

    #[test]
    fn part_2() {
        let lines: Vec<Line> = read_file("./src/day07/input.txt").unwrap();

        let nodes: HashMap<Bag, BagManifest> = lines.iter()
            .map(|line| (line.0.clone(), line.1.clone()))
            .collect();

        let contained = count_contained_bags(&nodes, &Bag::new("shiny gold bag"));

        println!("Contained: {}", contained);

    }
}