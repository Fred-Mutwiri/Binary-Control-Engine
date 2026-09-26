use crate::flags::Flags;
use super::program_counter::ProgramCounter;


pub fn jump_if_zero(pc: &mut ProgramCounter,  flags: &Flags, address: u8) {
    if flags.zero{
        pc.jump(address);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jumps_when_zero() {
        let mut pc = ProgramCounter::new();

        let flags = Flags {
                zero: true,
                carry: false,
                borrow: false,
                negative: false,
                overflow: false,
        };

        jump_if_zero( &mut pc, &flags, 20);

        assert_eq!(pc.value, 20);
    }


    #[test]
    fn does_not_jump_when_not_zero() {
        let mut pc = ProgramCounter::new();

        let flags = Flags {
            zero: false,
            carry: false,
            borrow: false,
            negative: false,
            overflow: false,
        };

        jump_if_zero( &mut pc, &flags, 20);

        assert_eq!(pc.value, 0);
    }
}