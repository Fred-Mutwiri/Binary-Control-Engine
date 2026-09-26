#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction{
    Nop,
    Load(u8),
    Add(u8),
    Sub(u8),
    Jump(u8),
    JumpIfZero(u8),
    Halt,
}