use std::collections::HashSet;
use std::str::FromStr;
use crate::util::read_file;

struct Map {
    lines: Vec<Line>,
}

impl Map {
    fn new(filename: &str) -> Self {
        let lines = read_file(filename).unwrap();
        Map { lines }
    }

    fn goal_at(&self, row: usize) -> bool {
        row >= self.lines.len()
    }

    fn tree_at(&self, row: usize, column: usize) -> bool {
        self.lines[row].tree_at(column)
    }

    fn count_trees(&self, delta_row: usize, delta_column: usize) -> usize {
        let mut row = 0;
        let mut column = 0;
        let mut trees = 0;

        while !self.goal_at(row) {
            trees += if self.tree_at(row, column) { 1 } else { 0 };
            column += delta_column;
            row += delta_row;
        }

        trees
    }
}

struct Line {
    trees: HashSet<usize>,
    width: usize,
}

impl Line {
    fn tree_at(&self, i: usize) -> bool {
        self.trees.contains(&(i % self.width))
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees = HashSet::new();
        let width = s.len();
        for (i, c) in s.chars().enumerate() {
            if c == '#' {
                trees.insert(i);
            }
        }

        Ok(Line { trees, width })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::hash_set;


    #[test]
    fn read_map_line() {
        let line: Line = "...#.#.".parse().unwrap();

        assert_eq!(line.trees, hash_set(vec![3, 5]));
        assert_eq!(line.width, 7);

        assert_eq!(line.tree_at(0), false);
        assert_eq!(line.tree_at(1), false);
        assert_eq!(line.tree_at(3), true);
        assert_eq!(line.tree_at(5), true);
        assert_eq!(line.tree_at(6), false);
        assert_eq!(line.tree_at(7), false);
        assert_eq!(line.tree_at(10), true);
    }

    #[test]
    fn count_trees() {
        let map = Map::new("./src/day03/input.txt");

        assert_eq!(map.count_trees(1, 3), 169);

        let mut accum = 1;
        for (dc, dr) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            let trees = map.count_trees(dr, dc);
            accum *= trees;
            println!("Right {}, down {}: {}", dc, dr, trees);
        }
        println!("Product: {}", accum);
    }
}