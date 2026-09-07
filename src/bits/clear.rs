

pub fn clear_bit ( value: u8, position: u8 ) -> Option<u8> {
    if position >= 8 {
        return None;
    }

    let mask = 1u8 << position;

    Some( value & !mask )
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_one_bit() {
        let value = 0b10110110;

        assert_eq!( clear_bit( value, 1), Some(0b10110100))
    }

    #[test]
    fn clear_zero_bit() {
        let value = 0b10110110;

        assert_eq!( clear_bit( value, 3), Some(0b10110110))
    }

    #[test]
    fn reject_invalid_position() {
        let value = 0b10110110;

        assert_eq!( clear_bit(value, 8), None)
    }
}