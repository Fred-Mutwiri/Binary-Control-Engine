

pub fn toggle_bit( value: u8, position: u8) -> Option<u8>{
    if position >= 8 {
        return None;
    }

    let mask = 1u8 << position;

    Some(value ^ mask)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggles_one_to_zero() {
        let value = 0b10110110;

        assert_eq!(toggle_bit(value, 1), Some(0b10110100));
    }

    #[test]
    fn toggles_zero_to_one() {
        let value = 0b10110110;

        assert_eq!(toggle_bit(value, 0), Some(0b10110111));
    }

    #[test]
    fn toggling_twice_restores_value() {
        let value = 0b10110110;

        let once = toggle_bit(value, 2).unwrap();
        let twice = toggle_bit(once, 2).unwrap();

        assert_eq!(twice, value);
    }

    #[test]
    fn rejects_invalid_position() {
        let value = 0b10110110;

        assert_eq!(toggle_bit(value, 8), None);
    }
}