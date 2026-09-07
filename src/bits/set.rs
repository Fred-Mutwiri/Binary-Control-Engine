

pub fn set_bit( value: u8, position: u8) -> Option<u8> {
    if position >= 8 {
        return None;
    }

    let mask = 1u8 << position;
    
    Some(value | mask )
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_zero_bit() {
        let value = 0b10110110;

        assert_eq!( set_bit( value, 0), Some(0b10110111))
    }

    #[test]
    fn set_already_set_bit() {
        let value = 0b10110110;

        assert_eq!( set_bit( value, 2), Some(0b10110110))
    }

    #[test]
    fn set_highest_bit() {
        let value = 0b10110110;

        assert_eq!( set_bit( value, 7), Some(0b10110110))
    }

    #[test]
    fn reject_invalid_position() {
        let value = 0b10110110;

        assert_eq!( set_bit( value, 8), None)
    }
}