#[derive(Debug, PartialEq, Eq)]
pub enum Ordering {
    Less,
    Equal,
    Greater,
}

pub fn compare( a: u8, b: u8 ) -> Ordering {
    for position in (0..8).rev() {
        let a_bit = (a >> position) & 1;
        let b_bit = (b >> position) & 1;

        if a_bit < b_bit {
            return Ordering::Less;
        }

        if a_bit > b_bit {
            return Ordering::Greater;
        }
    }

    Ordering::Equal

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_equal_values() {
        assert_eq!(
            compare(100, 100),
            Ordering::Equal
        );
    }

    #[test]
    fn finds_less_than() {
        assert_eq!(
            compare(10, 20),
            Ordering::Less
        );
    }

    #[test]
    fn finds_greater_than() {
        assert_eq!(
            compare(20, 10),
            Ordering::Greater
        );
    }

    #[test]
    fn compares_highest_bit() {
        assert_eq!(
            compare(0b10000000, 0b01111111),
            Ordering::Greater
        );
    }
}