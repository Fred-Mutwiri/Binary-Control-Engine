use crate::bits::get_bit_bit;

pub fn is_negative(value: u8) -> bool {
    get_bit_bit(value, 7) == Some(1)
}