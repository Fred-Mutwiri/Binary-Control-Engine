
use super::decoder::decode;
use super::executor::execute;
use super::machine::Machine;
use super::memory::Memory;


pub fn step( machine: &mut Machine, memory: &Memory ) -> bool {
    if machine.halted {
        return false;
    }

    let instruction = match memory.read(machine.pc.value)
        {
            Some(value) => value,
            None => {
                machine.halted = true;
                return false;
        }
    };

    machine.pc.advance();

    let instruction = match decode(instruction) {
        Some(instruction) => instruction,
        None => {
            machine.halted = true;
            return false;
        }
    };

    execute(machine, instruction);

    true

}