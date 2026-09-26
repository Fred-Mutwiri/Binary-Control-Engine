use crate::flags::Flags;

use super::program_counter::ProgramCounter;


pub struct Machine {
    pub accumulator: u8,
    pub flags: Flags,
    pub pc: ProgramCounter,
    pub halted: bool,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            flags: Flags::empty(),
            pc: ProgramCounter::new(),
            halted: false,
        }
    }
}