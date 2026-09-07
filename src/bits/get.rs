

pub fn get_bit( value: u8, position: u8 ) -> Option<u8> {
    let mask = 1u8 << position;
    Some( value & mask )
}


pub fn get_bit_bit( value: u8, position: u8) -> Option<u8> {
    if position >= 8 {
        return None;
    }
    let mask = 1u8 << position;
    let bit_value = (value & mask) >> position;

    Some(bit_value)
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_zero_bit() {
        let value = 0b10110110;

        assert_eq!(get_bit(value, 0), Some(0));
    }

    #[test]
    fn get_zero_bit_from_bit_bit() {
        let value = 0b10110110;

        assert_eq!(get_bit_bit(value, 0), Some(0));
    }

    #[test]
    fn get_bit_two() {
        let value = 0b10110110;

        assert_eq!( get_bit(value, 2), Some(4))
    }

    #[test]
    fn get_bit_two_from_bit_bit() {
        let value = 0b10110110;

        assert_eq!( get_bit_bit(value, 2), Some(1))
    }

    #[test]
    fn get_highest_bit() {
        let value = 0b10110110;

        assert_eq!( get_bit_bit(value, 7), Some(1))
    }
}