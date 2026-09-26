use crate::arithmetic;

pub struct ProgramCounter {
    pub value: u8,
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self {
            value: 0,
        }
    }

    pub fn jump( &mut self, address: u8 ) {
        self.value = address;
    }

    pub fn advance(&mut self ) -> bool {
        let (value, carry) = arithmetic::increment(self.value);

        self.value = value;

        carry != 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_at_zero() {
        let pc = ProgramCounter::new();

        assert_eq!(pc.value, 0);
    }

    #[test]
    fn advances() {
        let mut pc = ProgramCounter::new();

        pc.advance();

        assert_eq!(pc.value, 1);
    }

    #[test]
    fn jumps() {
        let mut pc = ProgramCounter::new();

        pc.jump(20);

        assert_eq!(pc.value, 20);
    }

    #[test]
    fn wraps() {
        let mut pc = ProgramCounter::new();

        pc.jump(255);

        let overflow = pc.advance();

        assert_eq!(pc.value, 0);
        assert!(overflow);
    }
}