




pub fn reverse_bits(value: u8) -> u8 {
    let mut result = 0u8;

    for position in 0..8 {
        let bit = (value >> position) & 1;

        result |= bit << (7 - position);
    }

    result
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn reverse_this_bit() {
        assert_eq!( reverse_bits(0b10110110), 0b01101101)
    }

    #[test]
    fn reverse_the_reverse() {
        assert_eq!( reverse_bits( reverse_bits( 0b1011_0110)), 0b1011_0110)
    }
}