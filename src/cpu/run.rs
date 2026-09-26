

use super::cycle::step;
use super::machine::Machine;
use super::memory::Memory;

pub fn run(program: Vec<u8>) -> Machine {
    let memory = Memory::new(program);

    let mut machine = Machine::new();

    while !machine.halted {
        if !step( &mut machine, &memory){
            break;
        }
    }

    machine
}