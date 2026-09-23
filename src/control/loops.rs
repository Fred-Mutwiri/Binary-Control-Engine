use super::counter::Counter;

pub fn count_up(start: u8, end: u8) -> u8 {
    let mut counter = Counter::new(start);

    while counter.value < end {
        counter.increment();
    }

    counter.value
}

pub fn count_down(start: u8) -> u8 {
    let mut counter = Counter::new(start);

    while counter.value != 0 {
        counter.decrement();
    }

    counter.value
}





#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn counts_up() {
        assert_eq!(count_up(0,5), 5);
    }

    #[test]
    fn already_at_end() {
        assert_eq!(count_up(5,5), 5);
    }

    #[test]
    fn counts_down() {
        assert_eq!(count_down(10), 0)
    }
}