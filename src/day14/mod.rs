use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Mask {
    set: u64,
    clear: u64,
    floating: Vec<u64>,
}

impl Mask {
    fn new(mask: &str) -> Self {
        let mut set = 0;
        let mut clear = 0x0000000fffffffff;
        let mut floating = Vec::new();

        let mut bit = 1u64 << 35;

        for c in mask.chars() {
            match c {
                '1' => {
                    set ^= bit;
                }
                '0' => {
                    clear ^= bit;
                }
                'X' => {
                    floating.push(bit);
                }
                _ => {}
            }
            bit >>= 1;
        }

        Self {
            set,
            clear,
            floating,
        }
    }

    fn apply(&self, value: u64) -> u64 {
        (value | self.set) & self.clear
    }

    fn apply_mem(&self, mut address: u64) -> Vec<u64> {
        let mut out: HashSet<u64> = HashSet::new();
        for b in 0..36 {
            let bit = 1 << b;
            if self.set & bit != 0 {
                address |= bit;
            }
        }

        //println!("address: {}, floatings: {:?}", address, &self.floating);
        Mask::add_mutations(&mut out, address, &self.floating);

        out.into_iter().collect()
    }

    fn add_mutations(out: &mut HashSet<u64>, base: u64, floatings: &[u64]) {
        out.insert(base & !floatings[0]);
        out.insert(base | floatings[0]);
        for t in 1..floatings.len() {
            Mask::add_mutations(out, base & !floatings[0], &floatings[t..]);
            Mask::add_mutations(out, base | floatings[0], &floatings[t..]);
        }
    }
}

struct Processor {
    mask: Mask,
    memory: Vec<u64>,
}

impl Processor {
    fn new() -> Self {
        Self {
            mask: Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"),
            memory: vec![0; 65536],
        }
    }

    fn execute<T: Into<String>>(&mut self, src: Vec<T>) -> u64 {
        let re = Regex::new(r"^mem\[(\d+)]$").unwrap();
        for line in src {
            let str: String = line.into();
            let parts: Vec<_> = str.split(" = ").collect();

            if let Some(captures) = re.captures(parts[0]) {
                let mem_address = captures[1].parse::<usize>().unwrap();
                if mem_address > 65535 {
                    panic!("OH NO! It was {}", mem_address);
                }
                let value = parts[1].parse().unwrap();
                self.memory[mem_address] = self.mask.apply(value);
            } else {
                self.mask = Mask::new(parts[1]);
            }
        }

        self.memory.iter().sum()
    }
}

struct ProcessorV2 {
    mask: Mask,
    memory: HashMap<u64, u64>,
}

impl ProcessorV2 {
    fn new() -> Self {
        Self {
            mask: Mask::new("00000000000000000000000000000000000000"),
            memory: HashMap::new(),
        }
    }

    fn execute<T: Into<String>>(&mut self, src: Vec<T>) -> u64 {
        let re = Regex::new(r"^mem\[(\d+)]$").unwrap();
        for line in src {
            let str: String = line.into();
            let parts: Vec<_> = str.split(" = ").collect();

            if let Some(captures) = re.captures(parts[0]) {
                let mem_address = captures[1].parse::<u64>().unwrap();
                let value = parts[1].parse().unwrap();
                let addresses = self.mask.apply_mem(mem_address);
                for address in addresses {
                    //println!("mem[{}] = {}", address, value);
                    self.memory.insert(address, value);
                }
            } else {
                self.mask = Mask::new(parts[1]);
            }
        }

        self.memory.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn set_and_clear() {
        let mask = Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);
    }

    #[test]
    fn program() {
        let src =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        let mut processor = Processor::new();
        let sum = processor.execute(src.split("\n").collect());

        assert_eq!(sum, 165);
    }

    #[test]
    fn part_1() {
        let src: Vec<String> = read_file("./src/day14/input.txt").unwrap();
        let sum = Processor::new().execute(src);

        assert_eq!(sum, 18630548206046);
    }

    #[test]
    fn v2() {
        let src = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        let mut processor = ProcessorV2::new();
        let sum = processor.execute(src.split("\n").collect());

        assert_eq!(sum, 208);
    }

    #[test]
    fn part_2() {
        let src: Vec<String> = read_file("./src/day14/input.txt").unwrap();
        let sum = ProcessorV2::new().execute(src);

        assert_eq!(sum, 4254673508445);

    }
}