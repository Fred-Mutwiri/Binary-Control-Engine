pub fn rotate_left(value: u8, amount: u8) -> u8 {
    let amount = amount % 8;

    if amount == 0 {
        return value;
    }

    let left = value << amount;
    let right = value >> (8 - amount);

    left | right
}


pub fn rotate_right( value: u8, amount: u8) -> u8 {
    let amount = amount % 8;

    if amount == 0 {
        return value;
    }

    let right = value >> amount;
    let left = value << (8 - amount);

    right | left
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn rotate_left_test_one() {
        assert_eq!( rotate_left(0b1011_0110, 1), 0b0110_1101)
    }

    #[test]
    fn rotate_left_test_three() {
        assert_eq!( rotate_left(0b1011_0110, 3), 0b1011_0101)
    }

    #[test]
    fn rotate_left_test_eight() {
        assert_eq!( rotate_left(0b1011_0110, 8), 0b1011_0110)
    }

    #[test]
    fn rotate_left_test_eleven() {
        assert_eq!( rotate_left(0b1011_0110, 11), 0b1011_0101)
    }

    #[test]
    fn rotate_right_test_one() {
        assert_eq!( rotate_right(0b1011_0110, 1), 0b0101_1011)
    }

    #[test]
    fn rotate_left_then_right_by_same_amount_restores_value() {
        let value = 0b10110110;

        let rotated = rotate_left(value, 3);
        let restored = rotate_right(rotated, 3);

        assert_eq!(value, restored)        
    }

    #[test]
    fn rotate_right_then_left_by_same_amount_restores_value() {
        let value = 0b10110110;

        let rotated = rotate_right(value, 4);
        let restored = rotate_left(rotated, 4);

        assert_eq!(value, restored)
    }
    
}