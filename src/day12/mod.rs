use crate::day12::Direction::East;
use crate::day12::Direction::South;
use crate::day12::Direction::North;
use crate::day12::Direction::West;

struct Ferry {
    pos: (i32, i32),
    waypoint: (i32, i32),
    heading: Direction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_steps(&self, mut steps: i32) -> Direction {
        let mut res = *self;
        while steps > 0 {
            res = match res {
                North => East,
                East => South,
                South => West,
                West => North,
            };
            steps -= 1;
        }
        while steps < 0 {
            res = match res {
                North => West,
                West => South,
                South => East,
                East => North,
            };
            steps += 1;
        }
        res
    }
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            pos: (0, 0),
            waypoint: (10, 1),
            heading: East,
        }
    }

    fn go(&mut self, instruction: &str) {
        let argument: i32 = instruction[1..].parse().unwrap();
        match &instruction[0..1] {
            "F" => {
                self.apply_movement(self.heading, argument);
            }
            "R" => {
                self.apply_turn(argument / 90);
            }
            "L" => {
                self.apply_turn(-argument / 90);
            }
            "N" => {
                self.apply_movement(North, argument);
            }
            "E" => {
                self.apply_movement(East, argument);
            }
            "S" => {
                self.apply_movement(South, argument);
            }
            "W" => {
                self.apply_movement(West, argument);
            }
            _ => {
                panic!("Unhandled instruction: {}", instruction);
            }
        }
    }

    fn apply_turn(&mut self, turns: i32) {
        self.heading = self.heading.turn_steps(turns);
    }

    fn apply_movement(&mut self, direction: Direction, distance: i32) {
        self.pos = match direction {
            East => {
                (self.pos.0 + distance, self.pos.1)
            }
            North => {
                (self.pos.0, self.pos.1 + distance)
            }
            West => {
                (self.pos.0 - distance, self.pos.1)
            }
            South => {
                (self.pos.0, self.pos.1 - distance)
            }
        }
    }

    fn nav(&mut self, instruction: &str) {
        let argument: i32 = instruction[1..].parse().unwrap();
        match &instruction[0..1] {
            "F" => {
                self.move_to_waypoint(argument);
            }
            "R" => {
                self.turn_waypoint(argument / 90);
            }
            "L" => {
                self.turn_waypoint(-argument / 90);
            }
            "N" => {
                self.move_waypoint(North, argument);
            }
            "E" => {
                self.move_waypoint(East, argument);
            }
            "S" => {
                self.move_waypoint(South, argument);
            }
            "W" => {
                self.move_waypoint(West, argument);
            }
            _ => {
                panic!("Unhandled instruction: {}", instruction);
            }
        }
    }

    fn turn_waypoint(&mut self, mut turns: i32) {
        while turns > 0 {
            self.waypoint = (self.waypoint.1, -self.waypoint.0);
            turns -= 1;
        }
        while turns < 0 {
            self.waypoint = (-self.waypoint.1, self.waypoint.0);
            turns += 1;
        }
    }

    fn move_waypoint(&mut self, direction: Direction, distance: i32) {
        self.waypoint = match direction {
            East => {
                (self.waypoint.0 + distance, self.waypoint.1)
            }
            North => {
                (self.waypoint.0, self.waypoint.1 + distance)
            }
            West => {
                (self.waypoint.0 - distance, self.waypoint.1)
            }
            South => {
                (self.waypoint.0, self.waypoint.1 - distance)
            }
        }
    }

    fn move_to_waypoint(&mut self, times: i32) {
        self.pos = (
            self.pos.0 + self.waypoint.0 * times,
            self.pos.1 + self.waypoint.1 * times
        )
    }

    fn manhattan_distance(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn move_forward() {
        let mut ferry = Ferry::new();
        ferry.go("F10");
        assert_eq!(ferry.pos, (10, 0));
    }

    #[test]
    fn turn_right() {
        let mut ferry = Ferry::new();
        ferry.go("R90");
        assert_eq!(ferry.heading, South);
    }

    #[test]
    fn turn_left() {
        let mut ferry = Ferry::new();
        ferry.go("L90");
        assert_eq!(ferry.heading, North);
    }

    #[test]
    fn move_north() {
        let mut ferry = Ferry::new();
        ferry.go("N2");
        assert_eq!(ferry.pos, (0, 2));
    }

    #[test]
    fn move_west() {
        let mut ferry = Ferry::new();
        ferry.go("W3");
        assert_eq!(ferry.pos, (-3, 0));
    }

    #[test]
    fn move_south() {
        let mut ferry = Ferry::new();
        ferry.go("S1");
        assert_eq!(ferry.pos, (0, -1));
    }

    #[test]
    fn move_east() {
        let mut ferry = Ferry::new();
        ferry.go("E5");
        assert_eq!(ferry.pos, (5, 0));
    }

    #[test]
    fn multiple_movements() {
        let movements = vec!["F10", "N3", "F7", "R90", "F11"];
        let mut ferry = Ferry::new();
        for m in movements {
            ferry.go(m);
        }
        assert_eq!(ferry.pos, (17, -8));
        assert_eq!(ferry.heading, South);

        assert_eq!(ferry.manhattan_distance(), 25);
    }

    #[test]
    fn part_1() {
        let movements: Vec<String> = read_file("./src/day12/input.txt").unwrap();
        let mut ferry = Ferry::new();
        for ref m in movements {
            ferry.go(m);
        }

        assert_eq!(ferry.manhattan_distance(), 441);
    }

    #[test]
    fn waypoint() {
        let movements = vec!["F10", "N3", "F7", "R90", "F11"];
        let mut ferry = Ferry::new();
        for m in movements {
            ferry.nav(m);
        }
        assert_eq!(ferry.pos, (214, -72));

        assert_eq!(ferry.manhattan_distance(), 286);
    }

    #[test]
    fn part_2() {
        let movements: Vec<String> = read_file("./src/day12/input.txt").unwrap();
        let mut ferry = Ferry::new();
        for ref m in movements {
            ferry.nav(m);
        }

        assert_eq!(ferry.manhattan_distance(), 40014);
    }
}

