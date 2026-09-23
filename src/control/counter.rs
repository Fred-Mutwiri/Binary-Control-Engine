use crate::arithmetic;

pub struct Counter {
    pub value: u8,
}

impl Counter {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn increment( &mut self) -> bool {
        let (value, carry) = arithmetic::increment(self.value);

        self.value = value;
        carry != 0
    }

    pub fn decrement(&mut self) -> bool {
        let (value, borrow) = arithmetic::decrement(self.value);

        self.value = value;
        borrow
    }
}




#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn creates_counter() {
        let counter = Counter::new(10);

        assert_eq!(counter.value, 10);
    }

    #[test]
    fn increments_counter() {
        let mut counter = Counter::new(10);

        let overflow = counter.increment();

        assert_eq!(counter.value, 11);
        assert!(!overflow);
    }

    #[test]
    fn decrements_counter() {
        let mut counter = Counter::new(10);

        let borrow = counter.decrement();

        assert_eq!(counter.value, 9);
        assert!( !borrow );
    }

    #[test]
    fn increment_wraps() {
        let mut counter = Counter::new(255);

        let overflow = counter.increment();

        assert_eq!(counter.value, 0);
        assert!(overflow);
    }

    #[test]
    fn decrement_wraps() {
        let mut counter = Counter::new(0);

        let borrow = counter.decrement();

        assert_eq!(counter.value, 255);
        assert!(borrow);
    }

}