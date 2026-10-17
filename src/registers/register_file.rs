use super::Register;


pub struct RegisterFile {
    registers: [Register; 4],
}


impl RegisterFile {
    pub fn new() -> Self {
        Self {
            registers: [
                Register::new(0),
                Register::new(0),
                Register::new(0),
                Register::new(0),
            ],
        }
    }

    pub fn read(&self, index: u8) -> Option<u8> {
        self.registers
            .get(index as usize)
            .map(Register::read)
    }

    pub fn write( &mut self, index: u8, value: u8 ) -> bool {
        match self.registers.get_mut(index as usize) {
            Some(register) => {
                register.write(value);
                true
            }

            None => false,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_with_zeroes() {
        let registers = RegisterFile::new();

        assert_eq!(registers.read(0), Some(0));
        assert_eq!(registers.read(1), Some(0));
        assert_eq!(registers.read(2), Some(0));
        assert_eq!(registers.read(3), Some(0));
    }

    #[test]
    fn writes_register() {
        let mut registers = RegisterFile::new();

        assert!(registers.write(2, 55));
        assert_eq!(registers.read(2), Some(55));
    }

    #[test]
    fn reject_invalid_register() {
        let mut registers = RegisterFile::new();

        assert!(!registers.write(4, 55));
        assert_eq!(registers.read(4), None);
    }
}