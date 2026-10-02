use super::memory::Memory;

pub fn read( memory: &Memory, address: u8) -> Option<u8> {
    memory.read(address)
}