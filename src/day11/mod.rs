use std::fmt;
use std::fmt::Write;
use crate::util::read_file;
use crate::day11::SeatAlgorithm::Neighbour;

const FLOOR: u8 = '.' as u8;
const EMPTY_SEAT: u8 = 'L' as u8;
const TAKEN_SEAT: u8 = '#' as u8;

#[derive(Eq, PartialEq, Copy, Clone)]
enum SeatAlgorithm {
    Neighbour,
    Closest,
}

struct WaitingArea {
    rows: i32,
    columns: i32,
    seat_algorithm: SeatAlgorithm,
    seats: Vec<u8>,
}

impl WaitingArea {
    fn new(data: &[String]) -> Self {
        let rows = data.len();
        let columns = data[0].len();
        let mut seats = Vec::with_capacity(rows * columns);

        for row in data {
            for p in row.as_bytes() {
                seats.push(*p);
            }
        }

        Self {
            rows: rows as i32,
            columns: columns as i32,
            seat_algorithm: Neighbour,
            seats,
        }
    }

    fn from_file(filename: &str) -> Self {
        let data = read_file(filename).unwrap();
        WaitingArea::new(&data)
    }

    fn with_seat_algorithm(mut self, seat_algorithm: SeatAlgorithm) -> Self {
        self.seat_algorithm = seat_algorithm;
        self
    }

    fn pos(&self, row: i32, column: i32) -> u8 {
        if row < 0 || row >= self.rows || column < 0 || column >= self.columns {
            return FLOOR;
        }
        self.seats[(row * self.columns + column) as usize]
    }

    fn surrounding_taken_seats(&self, row: i32, column: i32) -> u8 {
        let mut count = 0;

        let directions = vec![
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1)
        ];
        for (dy, dx) in directions {
            if self.seat_algorithm == SeatAlgorithm::Neighbour {
                if self.pos(row + dy, column + dx) == TAKEN_SEAT {
                    count += 1;
                }
            } else {
                let mut r = row + dy;
                let mut c = column + dx;
                while r >= 0 && r < self.rows && c >= 0 && c < self.columns {
                    match self.pos(r, c) {
                        TAKEN_SEAT => {
                            count += 1;
                            break;
                        }
                        EMPTY_SEAT => {
                            break;
                        }
                        _ => {}
                    }
                    r += dy;
                    c += dx;
                }
            }
        }

        count
    }

    fn count_taken_seats(&self) -> Vec<u8> {
        let mut taken = Vec::with_capacity((self.rows * self.columns) as usize);
        for r in 0..self.rows {
            for c in 0..self.columns {
                taken.push(self.surrounding_taken_seats(r, c));
            }
        }
        taken
    }

    fn iterate(&mut self) -> bool {
        let taken = self.count_taken_seats();
        let mut index = 0;
        let mut unchanged = true;
        let taken_limit = if self.seat_algorithm == Neighbour { 4 } else { 5 };
        for _ in 0..self.rows {
            for _ in 0..self.columns {
                match self.seats[index] {
                    EMPTY_SEAT if taken[index] == 0 => {
                        self.seats[index] = TAKEN_SEAT;
                        unchanged = false;
                    }
                    TAKEN_SEAT if taken[index] >= taken_limit => {
                        self.seats[index] = EMPTY_SEAT;
                        unchanged = false;
                    }
                    _ => {}
                }
                index += 1;
            }
        }
        unchanged
    }

    fn count_occupied_seats(&self) -> usize {
        self.seats.iter().filter(|c| **c == TAKEN_SEAT).count()
    }
}

impl fmt::Display for WaitingArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut index = 0;
        for _ in 0..self.rows {
            for _ in 0..self.columns {
                f.write_char(self.seats[index] as char)?;
                index += 1;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iterate_until_stable(mut area: WaitingArea) -> WaitingArea {
        while !area.iterate() {
            //println!("{}", area);
        }
        area
    }

    #[test]
    fn small_area_1() {
        let small_area = WaitingArea::from_file("./src/day11/input_test.txt");
        let small_area = iterate_until_stable(small_area);
        assert_eq!(small_area.count_occupied_seats(), 37);
    }

    #[test]
    fn small_area_2() {
        let small_area = WaitingArea::from_file("./src/day11/input_test.txt")
            .with_seat_algorithm(SeatAlgorithm::Closest);
        let small_area = iterate_until_stable(small_area);
        assert_eq!(small_area.count_occupied_seats(), 26);
    }

    #[test]
    fn part_1() {
        let area = WaitingArea::from_file("./src/day11/input.txt");
        let area = iterate_until_stable(area);
        assert_eq!(area.count_occupied_seats(), 2354);
    }

    #[test]
    fn part_2() {
        let area = WaitingArea::from_file("./src/day11/input.txt")
            .with_seat_algorithm(SeatAlgorithm::Closest);
        let area = iterate_until_stable(area);
        assert_eq!(area.count_occupied_seats(), 2072);
    }
}
