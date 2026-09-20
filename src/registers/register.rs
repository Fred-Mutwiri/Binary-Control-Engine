#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register{
    value: u8,
}

impl Register {
    pub fn new( value: u8) -> Self {
        Self { value}
    }

    pub fn read( &self ) -> u8 {
        self.value
    }

    pub fn write( &mut self, value: u8) {
        self.value = value;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn create_register() {
        let register = Register::new(10);
        assert_eq!(register.read(), 10);
    }

    #[test]
    fn reads_value() {
        let register = Register::new(25);
        assert_eq!(register.read(), 25);
    }

    #[test]
    fn writes_value() {
        let mut register = Register::new(0);
        register.write(20);
        assert_eq!(register.read(), 20);
    }

    #[test]
    fn stores_all_u8_values() {
        let mut register = Register::new(0);

        register.write(255);
        assert_eq!(register.read(), 255);
    }
}