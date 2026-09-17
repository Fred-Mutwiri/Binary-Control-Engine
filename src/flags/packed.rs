
use crate::bits;

pub const ZERO: u8 = 0;
pub const CARRY: u8 = 1;
pub const BORROW: u8 = 2;
pub const NEGATIVE: u8 = 3;
pub const OVERFLOW: u8 = 4;

pub fn set_flag( flags: u8, position: u8 ) -> u8 {
    bits::set_bit(flags, position).unwrap()
}

pub fn clear_flag( flags: u8, position: u8) -> u8 {
    bits::clear_bit(flags, position).unwrap()
}

pub fn read_flag( flags: u8, position: u8 ) -> Option<u8> {
    bits::get_bit_bit(flags, position)
}