use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Seat(u32);

impl Seat {
    fn new(s: &str) -> Self {
        let bin_str = s
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1");
        let seat_id = u32::from_str_radix(&bin_str, 2).unwrap();
        Seat(seat_id)
    }
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seat::new(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn convert_to_seat_id() {
        assert_eq!(Seat::new("BFFFBBFRRR"), Seat(567));
        assert_eq!(Seat::new("FFFBBBFRRR"), Seat(119));
        assert_eq!(Seat::new("BBFFBBFRLL"), Seat(820));
    }

    #[test]
    fn part_1() {
        let seats: Vec<Seat> = read_file("./src/day05/input.txt").unwrap();
        let highest = seats.iter().max().unwrap();
        println!("Highest seat: {}", highest.0);
    }

    #[test]
    fn part_2() {
        let mut seats: Vec<Seat> = read_file("./src/day05/input.txt").unwrap();
        seats.sort();

        let Seat(mut previous) = seats.pop().unwrap();
        loop {
            if let Some(Seat(current)) = seats.pop() {
                if current != previous - 1 {
                    println!("My seat: {}", previous - 1);
                    break;
                }
                previous = current;
            } else {
                break;
            }
        }
    }
}