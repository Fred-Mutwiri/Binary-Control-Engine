use super::compare::{compare, Ordering};
use super::subtract::subtract;

pub fn divide( dividend: u8, divisor: u8) -> Option<(u8, u8)> {
    if divisor == 0 {
        return None;
    }

    let mut quotient = 0u8;
    let mut remainder = 0u8;

    for position in (0..8).rev() {
        let bit = (dividend >> position) & 1;

        remainder = (remainder << 1) | bit;

        if compare(remainder, divisor) != Ordering::Less {
            let (new_remainder, _) = subtract(remainder, divisor);

            remainder = new_remainder;

            quotient |= 1u8 << position;
        }
    }

    Some((quotient, remainder))
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_evenly() {
        assert_eq!(
            divide(20, 5),
            Some((4, 0))
        );
    }

    #[test]
    fn division_with_remainder() {
        assert_eq!(
            divide(23, 5),
            Some((4, 3))
        );
    }

    #[test]
    fn divides_by_one() {
        assert_eq!(
            divide(100, 1),
            Some((100, 0))
        );
    }

    #[test]
    fn zero_dividend() {
        assert_eq!(
            divide(0, 5),
            Some((0, 0))
        );
    }

    #[test]
    fn division_by_zero_fails() {
        assert_eq!(
            divide(10, 0),
            None
        );
    }
}