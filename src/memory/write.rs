use super::memory::Memory;

pub fn write( memory: &mut Memory, address: u8, value: u8) -> bool {
    memory.write(address, value)
}