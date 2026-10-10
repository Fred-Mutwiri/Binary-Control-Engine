use super::Register;

pub struct Accumulator {
    register: Register,
}

impl Accumulator {
    pub fn new() -> Self {
        Self {
            register: Register::new(0),
        }
    }

    pub fn read( &self ) -> u8 {
        self.register.read()
    }

    pub fn write( &mut self, value: u8 ) {
        self.register.write(value)
    }

    pub fn clear( &mut self ) {
        self.register.clear()
    }
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn starts_at_zero() {
        let accumulator = Accumulator::new();

        assert_eq!(accumulator.read(), 0);
    }

    #[test]
    fn stores_value() {
        let mut accumulator = Accumulator::new();

        accumulator.write(20);
        assert_eq!(accumulator.read(), 20);
    }

    #[test]
    fn clears_value() {
        let mut accumulator = Accumulator::new();

        accumulator.write(100);
        accumulator.clear();
        assert_eq!(accumulator.read(), 0);
    }
}