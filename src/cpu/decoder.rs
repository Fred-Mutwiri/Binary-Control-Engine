use crate::bits::extract_field;

use super::instruction::Instruction;
use super::opcode;


pub fn decode(value: u8) -> Option<Instruction> {
    let operation = extract_field(value, 4, 4)?;

    let operand = extract_field(value, 0, 4)?;

    match operation {
        opcode::NOP => Some(Instruction::Nop),

        opcode::LOAD => Some(Instruction::Load(operand)),

        opcode::ADD => Some(Instruction::Add(operand)),

        opcode::SUB => Some(Instruction::Sub(operand)),

        opcode::JUMP => Some(Instruction::Jump(operand)),

        opcode::JUMP_IF_ZERO => Some(Instruction::JumpIfZero(operand)),

        opcode::HALT => Some(Instruction::Halt),

        _ => None,
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use super::super::instruction::Instruction;

    #[test]
    fn decodes_nop() {
        assert_eq!( decode(0b0000_0000), Some(Instruction::Nop));
    }

    #[test]
    fn decodes_load() {
        assert_eq!( decode(0b0001_0011), Some(Instruction::Load(3)));
    }

    #[test]
    fn decodes_add() {
        assert_eq!( decode(0b0010_0011), Some(Instruction::Add(3)));
    }

    #[test]
    fn decodes_sub() {
        assert_eq!( decode(0b0011_0010), Some(Instruction::Sub(2)));
    }

    #[test]
    fn decodes_jump() {
        assert_eq!( decode(0b0100_0101), Some(Instruction::Jump(5)));
    }

    #[test]
    fn decodes_halt() {
        assert_eq!( decode(0b1111_0000), Some(Instruction::Halt));
    }
}