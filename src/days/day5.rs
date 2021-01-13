
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn new(row: &str, column: &str) -> Seat {
        let row = usize::from_str_radix(row, 2).unwrap();
        let column = usize::from_str_radix(column, 2).unwrap();

        Seat {
            row,
            column,
        }
    }

    fn calculate_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

pub fn run(lines: Vec<String>, part: usize) -> Result<(), &'static str> {
    let seats_as_binary = lines
        .iter()
        .map(|x| convert_seat_to_binary(x));

    let ids = seats_as_binary
        .map(|x| Seat::new(&x.0, &x.1).calculate_id());

    if part == 1 {
        println!("Highest ID = {}", ids.max().unwrap());
    }

    Ok(())
}

fn convert_seat_to_binary(seat: &str) -> (String, String) {
    let seat = seat
        .replace(&['F', 'L'][..], "0")
        .replace(&['B', 'R'][..], "1");
    let (row, column) = seat.split_at(7);
    (row.to_string(), column.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn converts_seat_to_binary() {
        let seat = "FFBBFFBLLR";
        assert_eq!(("0011001".to_string(), "001".to_string()), convert_seat_to_binary(&seat));
    }

    #[test]
    fn determines_seat_and_column() {
        let seat = Seat::new("1000110", "111");
        assert_eq!(70, seat.row);
        assert_eq!(7, seat.column);
    }

    #[test]
    fn calculates_id() {
        let seat = Seat::new("1100110", "100");
        assert_eq!(820, seat.calculate_id());
    }
}