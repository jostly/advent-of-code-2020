use std::str::FromStr;
use crate::util::read_file;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        let argument = parts[1].parse().unwrap();
        match parts[0] {
            "nop" => Ok(Instruction::Nop(argument)),
            "acc" => Ok(Instruction::Acc(argument)),
            "jmp" => Ok(Instruction::Jmp(argument)),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct Processor {
    program: Vec<Instruction>,
    counter: usize,
    accumulator: i32,
}

impl Processor {
    fn from_file(filename: &str) -> Self {
        let program = read_file(filename).unwrap();
        Processor {
            program,
            counter: 0,
            accumulator: 0,
        }
    }

    fn is_terminated(&self) -> bool {
        self.counter >= self.program.len()
    }

    fn execute(&mut self) {
        self.counter = 0;
        self.accumulator = 0;
        let program_size = self.program.len();
        let mut visited = vec![false; program_size];

        while self.counter < program_size && !visited[self.counter] {
            visited[self.counter] = true;
            self.handle_operation();
        }
    }

    fn handle_operation(&mut self) {
        match self.program[self.counter] {
            Instruction::Acc(delta) => {
                self.accumulator += delta;
                self.counter += 1;
            },
            // Make counter signed while adding the signed delta value
            Instruction::Jmp(delta) => self.counter = ((self.counter as i32) + delta) as usize,
            _ => self.counter += 1,
        }
    }

    fn swap_at(&mut self, index: usize) -> bool {
        let new_instr = match self.program[index] {
            Instruction::Nop(delta) => Instruction::Jmp(delta),
            Instruction::Jmp(delta) => Instruction::Nop(delta),
            Instruction::Acc(delta) => Instruction::Acc(delta),
        };
        if new_instr != self.program[index] {
            self.program[index] = new_instr;
            return true;
        }
        return false;
    }

    fn change_to_terminate(self) -> (Self, usize) {
        for i in 0..self.program.len() {
            let mut cloned_processor = self.clone();
            if cloned_processor.swap_at(i) {
                cloned_processor.execute();
                if cloned_processor.is_terminated() {
                    return (cloned_processor, i);
                }
            }
        }
        return (self, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        assert_eq!("nop +0".parse::<Instruction>().unwrap(), Instruction::Nop(0));
        assert_eq!("acc +12".parse::<Instruction>().unwrap(), Instruction::Acc(12));
        assert_eq!("jmp -3".parse::<Instruction>().unwrap(), Instruction::Jmp(-3));
    }

    #[test]
    fn processor_from_file() {
        let mut processor = Processor::from_file("./src/day08/input_test.txt");
        processor.execute();
        assert_eq!(processor.accumulator, 5);
    }

    #[test]
    fn part_1() {
        let mut processor = Processor::from_file("./src/day08/input.txt");
        processor.execute();
        assert_eq!(processor.accumulator, 1475);
    }

    #[test]
    fn change_to_terminate() {
        let processor = Processor::from_file("./src/day08/input_test.txt");
        let (_, i) = processor.change_to_terminate();
        assert_eq!(i, 7);
    }

    #[test]
    fn part_2() {
        let processor = Processor::from_file("./src/day08/input.txt");
        let (changed_processor, i) = processor.change_to_terminate();
        println!("Switching instruction at position {} to {:?} caused program to terminate", i, &changed_processor.program[i]);
        println!("Accumulator after termination: {}", changed_processor.accumulator);
    }
}