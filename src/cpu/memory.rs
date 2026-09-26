

pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new(program: Vec<u8>) -> Self {
        Self {
            data: program,
        }
    }

    pub fn read(&self, address: u8) -> Option<u8> {
        self.data.get(address as usize).copied()
    }


}