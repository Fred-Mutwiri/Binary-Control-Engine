pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn read( &self, address: u8) -> Option<u8> {
        self.data
            .get(address as usize)
            .copied()
    }

    pub fn write( &mut self, address: u8, value: u8 ) -> bool {
        match self.data.get_mut(address as usize) {
            Some(byte) => {
                *byte = value;
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
    fn creates_memory() {
        let memory = Memory::new(16);

        assert_eq!(memory.size(), 16);
    }

    #[test]
    fn new_memory_contains_zeroes() {
        let memory = Memory::new(16);

        assert_eq!(memory.read(0), Some(0));
        assert_eq!(memory.read(15), Some(0));
    }

    #[test]
    fn writes_and_reads() {
        let mut memory = Memory::new(16);

        assert!(memory.write(5, 42));

        assert_eq!(memory.read(5), Some(42));
    }

    #[test]
    fn rejects_invalid_address() {
        let mut memory = Memory::new(16);

        assert!(!memory.write(16, 42));

        assert_eq!(memory.read(16), None);
    }
}