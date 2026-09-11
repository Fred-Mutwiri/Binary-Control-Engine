

pub fn population_count(mut value: u8) -> u8 {
    let mut count = 0;

    for _ in 0..8 {
        count += value & 1;
        value >>= 1;
    }

    count
}

pub fn population_count_fast(mut value: u8) -> u8 {
    let mut count = 0;

    while value != 0 {
        value &= value - 1;
        count += 1;
    }

    count
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn population_zero_count() {
        assert_eq!( population_count(0b0000_0000), 0)
    }

    #[test]
    fn population_one_count() {
        assert_eq!( population_count(0b0000_1000), 1)
    }

    #[test]
    fn population_two_count() {
        assert_eq!( population_count(0b0001_0001), 2)
    }

    #[test]
    fn population_seven_count() {
        assert_eq!( population_count(0b1111_1110), 7)
    }
}