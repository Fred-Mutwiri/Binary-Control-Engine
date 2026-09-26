

use crate::arithmetic;
// use crate::flags::Flags;

use super::control_flow::jump_if_zero;
use super::instruction::Instruction;
use super::machine::Machine;

pub fn execute( machine: &mut Machine, instruction: Instruction ) {
    match instruction {
        Instruction::Nop => {}

        Instruction::Load(value) => {
            machine.accumulator = value;
            machine.flags.zero = value == 0;
            machine.flags.negative = (value & 0b1000_000) != 0;
        }

        Instruction::Add(value) => {
            let old_value = machine.accumulator;

            let ( result, carry ) = arithmetic::add( machine.accumulator, value );

            machine.accumulator = result;
            machine.flags.zero = result == 0;
            machine.flags.carry = carry != 0;
            machine.flags.negative = (result & 0b1000_0000) != 0;
            machine.flags.overflow = crate::flags::addition_overflow( old_value, value, result );   
        }

        Instruction::Sub(value) => {
            let (result, borrow) = arithmetic::subtract( machine.accumulator, value);

            machine.accumulator = result;
            machine.flags.zero = result == 0;
            machine.flags.borrow = borrow;
            machine.flags.negative = (result & 0b1000_0000) != 0;
        }

        Instruction::Jump(address) => {
            machine.pc.jump(address);
        }

        Instruction::JumpIfZero(address) => {
            jump_if_zero( &mut machine.pc, &machine.flags, address );
        }

        Instruction::Halt => {
            machine.halted = true;
        }
    }
}